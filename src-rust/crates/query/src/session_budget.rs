use dashmap::mapref::entry::Entry;
use dashmap::DashMap;
use once_cell::sync::Lazy;
use parking_lot::Mutex;
use std::cell::RefCell;
use std::collections::HashMap;
use std::future::Future;
use std::sync::Arc;
use tokio_util::sync::CancellationToken;

static SESSION_BUDGET_REGISTRY: Lazy<DashMap<String, (Arc<SessionBudget>, usize)>> =
    Lazy::new(DashMap::new);

tokio::task_local! {
    static TASK_SESSION_BUDGET_STACKS: RefCell<HashMap<String, Vec<Arc<SessionBudget>>>>;
}

#[derive(Debug)]
pub struct SessionBudget {
    budget_usd: f64,
    spent: Mutex<f64>,
    root_token: CancellationToken,
    parent: Option<Arc<SessionBudget>>,
}

impl SessionBudget {
    pub fn new(budget_usd: f64) -> Self {
        Self {
            budget_usd,
            spent: Mutex::new(0.0),
            root_token: CancellationToken::new(),
            parent: None,
        }
    }

    pub fn child_scope(parent: Arc<SessionBudget>, budget_usd: f64) -> Self {
        Self {
            budget_usd,
            spent: Mutex::new(0.0),
            root_token: parent.child_cancel_token(),
            parent: Some(parent),
        }
    }

    pub fn record_cost(&self, cost_usd: f64) {
        *self.spent.lock() += cost_usd;
        if let Some(parent) = &self.parent {
            parent.record_cost(cost_usd);
        }
    }

    pub fn check_and_cancel(&self) {
        if *self.spent.lock() >= self.budget_usd {
            self.root_token.cancel();
        }
        if let Some(parent) = &self.parent {
            parent.check_and_cancel();
        }
    }

    pub fn child_cancel_token(&self) -> CancellationToken {
        self.root_token.child_token()
    }

    pub fn is_cancelled(&self) -> bool {
        self.root_token.is_cancelled()
    }

    pub fn shared_budget(self: &Arc<Self>) -> Arc<Self> {
        self.parent
            .as_ref()
            .map(|parent| parent.shared_budget())
            .unwrap_or_else(|| self.clone())
    }
}

#[derive(Debug)]
pub struct SessionBudgetRegistration {
    session_id: String,
}

impl Drop for SessionBudgetRegistration {
    fn drop(&mut self) {
        pop_task_session_budget(&self.session_id);
        if let Entry::Occupied(mut entry) = SESSION_BUDGET_REGISTRY.entry(self.session_id.clone()) {
            let should_remove = {
                let (_, registrations) = entry.get_mut();
                *registrations -= 1;
                *registrations == 0
            };
            if should_remove {
                entry.remove();
            }
        }
    }
}

pub fn register_session_budget(
    session_id: &str,
    budget: &Arc<SessionBudget>,
) -> SessionBudgetRegistration {
    let shared_budget = budget.shared_budget();
    push_task_session_budget(session_id, budget.clone());
    match SESSION_BUDGET_REGISTRY.entry(session_id.to_string()) {
        Entry::Occupied(mut entry) => {
            // Preserve the first shared root budget for this session key; nested
            // child-local scopes resolve through the task-local stack instead.
            let (_, registrations) = entry.get_mut();
            *registrations += 1;
        }
        Entry::Vacant(entry) => {
            entry.insert((shared_budget, 1));
        }
    }

    SessionBudgetRegistration {
        session_id: session_id.to_string(),
    }
}

pub fn session_budget_for_session(session_id: &str) -> Option<Arc<SessionBudget>> {
    if let Some(current_budget) = current_task_session_budget(session_id) {
        return Some(current_budget);
    }

    SESSION_BUDGET_REGISTRY
        .get(session_id)
        .map(|entry| entry.value().0.clone())
}

pub async fn with_registered_session_budget<Fut, T>(
    session_id: &str,
    budget: Option<Arc<SessionBudget>>,
    future: Fut,
) -> T
where
    Fut: Future<Output = T>,
{
    if budget.is_none() {
        return future.await;
    }

    let session_id = session_id.to_string();
    let run = async move {
        let _registration = budget
            .as_ref()
            .map(|budget| register_session_budget(&session_id, budget));
        future.await
    };

    match TASK_SESSION_BUDGET_STACKS.try_with(|_| ()) {
        Ok(()) => run.await,
        Err(_) => {
            TASK_SESSION_BUDGET_STACKS
                .scope(RefCell::new(HashMap::new()), run)
                .await
        }
    }
}

fn current_task_session_budget(session_id: &str) -> Option<Arc<SessionBudget>> {
    TASK_SESSION_BUDGET_STACKS
        .try_with(|stacks| {
            stacks
                .borrow()
                .get(session_id)
                .and_then(|budgets| budgets.last().cloned())
        })
        .ok()
        .flatten()
}

fn push_task_session_budget(session_id: &str, budget: Arc<SessionBudget>) {
    let _ = TASK_SESSION_BUDGET_STACKS.try_with(|stacks| {
        stacks
            .borrow_mut()
            .entry(session_id.to_string())
            .or_default()
            .push(budget);
    });
}

fn pop_task_session_budget(session_id: &str) {
    let _ = TASK_SESSION_BUDGET_STACKS.try_with(|stacks| {
        let mut stacks = stacks.borrow_mut();
        if let Some(budgets) = stacks.get_mut(session_id) {
            budgets.pop();
            if budgets.is_empty() {
                stacks.remove(session_id);
            }
        }
    });
}

#[cfg(test)]
mod tests {
    use super::{
        register_session_budget, session_budget_for_session, with_registered_session_budget,
        SessionBudget,
    };
    use std::sync::Arc;

    #[test]
    fn record_cost_accumulates_spend() {
        let budget = SessionBudget::new(10.0);

        budget.record_cost(1.25);
        budget.record_cost(2.75);

        assert_eq!(*budget.spent.lock(), 4.0);
        assert!(!budget.is_cancelled());
    }

    #[test]
    fn check_and_cancel_triggers_at_threshold() {
        let budget = SessionBudget::new(5.0);

        budget.record_cost(4.99);
        budget.check_and_cancel();
        assert!(!budget.is_cancelled());

        budget.record_cost(0.01);
        budget.check_and_cancel();
        assert!(budget.is_cancelled());
    }

    #[test]
    fn child_token_is_cancelled_with_root() {
        let budget = SessionBudget::new(2.0);
        let child = budget.child_cancel_token();

        assert!(!child.is_cancelled());

        budget.record_cost(2.0);
        budget.check_and_cancel();

        assert!(budget.is_cancelled());
        assert!(child.is_cancelled());
    }

    #[test]
    fn child_scope_records_against_local_and_parent_budget() {
        let root = Arc::new(SessionBudget::new(10.0));
        let child = SessionBudget::child_scope(root.clone(), 3.0);

        child.record_cost(2.25);
        child.check_and_cancel();

        assert_eq!(*child.spent.lock(), 2.25);
        assert_eq!(*root.spent.lock(), 2.25);
        assert!(!child.is_cancelled());
        assert!(!root.is_cancelled());

        child.record_cost(1.0);
        child.check_and_cancel();

        assert!(child.is_cancelled());
        assert!(!root.is_cancelled());
        assert_eq!(*root.spent.lock(), 3.25);
    }

    #[test]
    fn descendant_scope_chains_all_active_budget_caps() {
        let root = Arc::new(SessionBudget::new(10.0));
        let child = Arc::new(SessionBudget::child_scope(root.clone(), 4.0));
        let grandchild = SessionBudget::child_scope(child.clone(), 1.5);

        grandchild.record_cost(1.0);
        grandchild.check_and_cancel();

        assert_eq!(*grandchild.spent.lock(), 1.0);
        assert_eq!(*child.spent.lock(), 1.0);
        assert_eq!(*root.spent.lock(), 1.0);
        assert!(!grandchild.is_cancelled());
        assert!(!child.is_cancelled());
        assert!(!root.is_cancelled());

        grandchild.record_cost(0.6);
        grandchild.check_and_cancel();

        assert!(grandchild.is_cancelled());
        assert!(!child.is_cancelled());
        assert!(!root.is_cancelled());
        assert_eq!(*child.spent.lock(), 1.6);
        assert_eq!(*root.spent.lock(), 1.6);
    }

    #[test]
    fn registered_budget_is_visible_for_session() {
        let budget = Arc::new(SessionBudget::new(3.0));
        let _registration = register_session_budget("session-budget-visible", &budget);

        let inherited =
            session_budget_for_session("session-budget-visible").expect("budget must register");

        assert!(Arc::ptr_eq(&budget, &inherited));
    }

    #[test]
    fn registration_releases_when_last_guard_drops() {
        let budget = Arc::new(SessionBudget::new(4.0));

        {
            let _registration = register_session_budget("session-budget-release", &budget);
            assert!(session_budget_for_session("session-budget-release").is_some());
        }

        assert!(session_budget_for_session("session-budget-release").is_none());
    }

    #[tokio::test]
    async fn task_local_scope_prefers_nearest_active_budget() {
        let root = Arc::new(SessionBudget::new(8.0));

        with_registered_session_budget(
            "session-budget-task-local",
            Some(root.clone()),
            async move {
                let inherited = session_budget_for_session("session-budget-task-local")
                    .expect("root budget must be visible");
                assert!(Arc::ptr_eq(&root, &inherited));

                let child = Arc::new(SessionBudget::child_scope(root.clone(), 2.0));
                with_registered_session_budget(
                    "session-budget-task-local",
                    Some(child.clone()),
                    async move {
                        let inherited = session_budget_for_session("session-budget-task-local")
                            .expect("child budget must be visible");
                        assert!(Arc::ptr_eq(&child, &inherited));
                    },
                )
                .await;

                let inherited = session_budget_for_session("session-budget-task-local")
                    .expect("root budget must be restored");
                assert!(Arc::ptr_eq(&root, &inherited));
            },
        )
        .await;

        assert!(session_budget_for_session("session-budget-task-local").is_none());
    }
}
