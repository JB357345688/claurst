use parking_lot::Mutex;
use tokio_util::sync::CancellationToken;

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

#[cfg(test)]
mod tests {
    use super::SessionBudget;

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
}
