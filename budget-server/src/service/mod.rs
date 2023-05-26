use std::sync::{Arc, Mutex};

use uuid::Uuid;

use budget_common::domain::{Budget, BudgetCreationRequest, BudgetItem, BudgetItemCreationRequest};

use crate::repository::BudgetRepository;

pub async fn get_all_budgets(repository: Arc<Mutex<BudgetRepository>>) -> Vec<Budget> {
    repository.lock().unwrap().get_all_budgets()
}

pub async fn get_budget_by_id(
    repository: Arc<Mutex<BudgetRepository>>,
    id: Uuid,
) -> Option<Budget> {
    repository.lock().unwrap().get_budget_by_id(id).cloned()
}

pub async fn add_budget(repository: Arc<Mutex<BudgetRepository>>, budget: BudgetCreationRequest) {
    let budget = Budget::new(budget);
    repository.lock().unwrap().add_budget(budget);
}

pub async fn update_budget(repository: Arc<Mutex<BudgetRepository>>, budget: Budget) {
    repository.lock().unwrap().update_budget(budget);
}

pub async fn get_all_budget_items(repository: Arc<Mutex<BudgetRepository>>) -> Vec<BudgetItem> {
    repository.lock().unwrap().get_all_items()
}

pub async fn get_item_by_id(
    repository: Arc<Mutex<BudgetRepository>>,
    id: Uuid,
) -> Option<BudgetItem> {
    repository.lock().unwrap().get_item_by_id(id)
}

pub async fn add_budget_item(
    repository: Arc<Mutex<BudgetRepository>>,
    item: BudgetItemCreationRequest,
) {
    let item = BudgetItem::new(item);
    repository.lock().unwrap().add_item(item)
}

pub async fn add_budget_items(
    repository: Arc<Mutex<BudgetRepository>>,
    items: Vec<BudgetItemCreationRequest>,
) {
    let items = items.into_iter().map(BudgetItem::new).collect();
    repository.lock().unwrap().add_items(items)
}

pub async fn update_budget_item(
    repository: Arc<Mutex<BudgetRepository>>,
    item: BudgetItem,
) -> bool {
    repository.lock().unwrap().update_item(item)
}
