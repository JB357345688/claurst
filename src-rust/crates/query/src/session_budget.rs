use dashmap::mapref::entry::Entry;
use dashmap::DashMap;
use once_cell::sync::Lazy;
use parking_lot::Mutex;
use std::sync::Arc;
use tokio_util::sync::CancellationToken;

static SESSION_BUDGET_REGISTRY: Lazy<DashMap<String, (Arc<SessionBudget>, usize)>> =
    Lazy::new(DashMap::new);

#[derive(Debug)]
pub struct SessionBudget {
    budget_usd: f64,
    spent: Mutex<f64>,
    root_token: CancellationToken,
}

impl SessionBudget {
    pub fn new(budget_usd: f64) -> Self {
        Self {
            budget_usd,
            spent: Mutex::new(0.0),
            root_token: CancellationToken::new(),
        }
    }

    pub fn record_cost(&self, cost_usd: f64) {
        *self.spent.lock() += cost_usd;
    }

    pub fn check_and_cancel(&self) {
        if *self.spent.lock() >= self.budget_usd {
            self.root_token.cancel();
        }
    }

    pub fn child_cancel_token(&self) -> CancellationToken {
        self.root_token.child_token()
    }

    pub fn is_cancelled(&self) -> bool {
        self.root_token.is_cancelled()
    }
}

#[derive(Debug)]
pub struct SessionBudgetRegistration {
    session_id: String,
}

impl Drop for SessionBudgetRegistration {
    fn drop(&mut self) {
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
    match SESSION_BUDGET_REGISTRY.entry(session_id.to_string()) {
        Entry::Occupied(mut entry) => {
            let (registered_budget, registrations) = entry.get_mut();
            *registered_budget = budget.clone();
            *registrations += 1;
        }
        Entry::Vacant(entry) => {
            entry.insert((budget.clone(), 1));
        }
    }

    SessionBudgetRegistration {
        session_id: session_id.to_string(),
    }
}

pub fn session_budget_for_session(session_id: &str) -> Option<Arc<SessionBudget>> {
    SESSION_BUDGET_REGISTRY
        .get(session_id)
        .map(|entry| entry.value().0.clone())
}

#[cfg(test)]
mod tests {
    use super::{register_session_budget, session_budget_for_session, SessionBudget};
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
}
