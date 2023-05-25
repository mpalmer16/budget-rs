use std::sync::{Arc, Mutex};

use axum::{
    extract::{Path, State},
    Json,
};
use uuid::Uuid;

use budget_common::domain::{Budget, BudgetCreationRequest, BudgetItem, BudgetItemCreationRequest};

use crate::{
    repository::BudgetRepository,
    service,
};

pub async fn fetch_all_budgets(
    State(repository): State<Arc<Mutex<BudgetRepository>>>,
) -> Json<Vec<Budget>> {
    Json(service::get_all_budgets(repository).await)
}

pub async fn fetch_budget_by_id(
    State(repository): State<Arc<Mutex<BudgetRepository>>>,
    Path(id): Path<Uuid>,
) -> Json<Option<Budget>> {
    Json(service::get_budget_by_id(repository, id).await)
}

pub async fn add_budget(
    State(repository): State<Arc<Mutex<BudgetRepository>>>,
    Json(budget): Json<BudgetCreationRequest>,
) {
    service::add_budget(repository, budget).await
}

pub async fn update_budget(
    State(repository): State<Arc<Mutex<BudgetRepository>>>,
    Json(budget): Json<Budget>,
) {
    service::update_budget(repository, budget).await
}

pub async fn fetch_all_items(
    State(repository): State<Arc<Mutex<BudgetRepository>>>,
) -> Json<Vec<BudgetItem>> {
    Json(service::get_all_budget_items(repository).await)
}

pub async fn fetch_item_by_id(
    State(repository): State<Arc<Mutex<BudgetRepository>>>,
    Path(id): Path<Uuid>,
) -> Json<Option<BudgetItem>> {
    Json(service::get_item_by_id(repository, id).await)
}

pub async fn add_item(
    State(repository): State<Arc<Mutex<BudgetRepository>>>,
    Json(item): Json<BudgetItemCreationRequest>,
) {
    service::add_budget_item(repository, item).await
}

pub async fn add_items(
    State(repository): State<Arc<Mutex<BudgetRepository>>>,
    Json(items): Json<Vec<BudgetItemCreationRequest>>,
) {
    service::add_budget_items(repository, items).await
}

pub async fn update_item(
    State(repository): State<Arc<Mutex<BudgetRepository>>>,
    Json(item): Json<BudgetItem>,
) -> Json<bool> {
    Json(service::update_budget_item(repository, item).await)
}
