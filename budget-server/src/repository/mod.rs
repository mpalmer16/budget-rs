use serde::{Deserialize, Serialize};
use uuid::Uuid;

use budget_common::domain::{Budget, BudgetItem};

#[derive(Serialize, Deserialize)]
pub struct BudgetRepository {
    budgets: Vec<Budget>,
    items: Vec<BudgetItem>,
}

impl BudgetRepository {
    pub fn new() -> Self {
        BudgetRepository {
            budgets: Vec::new(),
            items: Vec::new(),
        }
    }

    // budgets

    pub fn get_all_budgets(&self) -> Vec<Budget> {
        self.budgets.clone()
    }

    pub fn get_budget_by_id(&self, id: Uuid) -> Option<&Budget> {
        self.budgets.iter().find(|budget| budget.id == id)
    }

    pub fn add_budget(&mut self, budget: Budget) {
        self.budgets.push(budget);
    }

    pub fn update_budget(&mut self, budget: Budget) {
        self.budgets.retain(|b| b.id != budget.id);
        self.budgets.push(budget);
    }

    // items

    pub fn get_all_items(&self) -> Vec<BudgetItem> {
        self.items.clone()
    }

    pub fn get_item_by_id(&self, id: Uuid) -> Option<BudgetItem> {
        self.items.iter().find(|item| item.id == id).cloned()
    }

    pub fn add_item(&mut self, item: BudgetItem) {
        if self.check_budget_id(item.budget_id) {
            self.items.push(item);
        }
    }

    pub fn add_items(&mut self, items: Vec<BudgetItem>) {
        items.iter().for_each(|item| {
            if self.check_budget_id(item.budget_id) {
                self.items.push(item.clone());
            }
        });
    }

    pub fn update_item(&mut self, item: BudgetItem) -> bool {
        self.items.retain(|i| i.id != item.id);
        self.items.push(item);
        true
    }

    fn check_budget_id(&self, budget_id: Uuid) -> bool {
        self.budgets.iter().any(|budget| budget.id == budget_id)
    }
}
