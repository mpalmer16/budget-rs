mod domain;
mod handler;
mod repository;
mod service;

use std::sync::{Arc, Mutex};

use axum::{
    routing::{get, post},
    Router,
};
use handler::{
    add_budget, add_item, add_items, fetch_all_budgets, fetch_all_items, fetch_budget_by_id,
    fetch_item_by_id, update_budget, update_item,
};
use repository::BudgetRepository;

#[tokio::main]
async fn main() {
    let repository = Arc::new(Mutex::new(BudgetRepository::new()));

    let app = Router::new()
        .route("/budget/budgets", get(fetch_all_budgets))
        .route("/budget/budgets/:id", get(fetch_budget_by_id))
        .route("/budget/budgets/add", post(add_budget))
        .route("/budget/budgets/update", post(update_budget))
        .route("/budget/items", get(fetch_all_items))
        .route("/budget/items/:id", get(fetch_item_by_id))
        .route("/budget/items/add", post(add_item))
        .route("/budget/items/addAll", post(add_items))
        .route("/budget/items/update", post(update_item))
        .with_state(repository);

    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
