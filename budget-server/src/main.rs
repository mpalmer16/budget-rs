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
use hyper::Method;
use repository::BudgetRepository;
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};
use tracing_subscriber::{prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                "budget_server=debug,tower_http=debug,axum::rejection=trace".into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let repository = Arc::new(Mutex::new(BudgetRepository::new()));

    let router = create_router(repository);

    axum::Server::bind(&"0.0.0.0:8081".parse().unwrap())
        .serve(router.into_make_service())
        .await
        .unwrap();
}

fn create_router(state: Arc<Mutex<BudgetRepository>>) -> Router {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any);

    Router::new()
        .route("/budget/budgets", get(fetch_all_budgets))
        .route("/budget/budgets/:id", get(fetch_budget_by_id))
        .route("/budget/budgets/add", post(add_budget))
        .route("/budget/budgets/update", post(update_budget))
        .route("/budget/items", get(fetch_all_items))
        .route("/budget/items/:id", get(fetch_item_by_id))
        .route("/budget/items/add", post(add_item))
        .route("/budget/items/addAll", post(add_items))
        .route("/budget/items/update", post(update_item))
        .with_state(state)
        .layer(TraceLayer::new_for_http())
        .layer(cors)
}

#[cfg(test)]
mod tests {

    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use budget_common::domain::{Budget, BudgetItem, BudgetItemType, BudgetMonth};
    use tower::ServiceExt;
    use uuid::Uuid;

    use super::*;

    fn create_budget_body_and_response(
        name: &str,
        month: BudgetMonth,
        total_salary: f32,
    ) -> (Budget, String) {
        let id = Uuid::new_v4();
        let budget = Budget {
            id,
            name: name.to_string(),
            month,
            total_salary,
        };
        let expected_response = format!(
            "{{\"id\":\"{}\",\"name\":\"{}\",\"month\":\"{}\",\"total_salary\":{}}}",
            budget.id, budget.name, budget.month, budget.total_salary
        );
        (budget, expected_response)
    }

    fn create_budget_item(
        name: &str,
        amount: f32,
        item_type: BudgetItemType,
        budget_id: Uuid,
    ) -> (BudgetItem, String) {
        let item = BudgetItem {
            id: Uuid::new_v4(),
            name: name.to_string(),
            amount,
            item_type,
            budget_id,
        };
        let expected_response = format!("{{\"id\":\"{}\",\"name\":\"{}\",\"amount\":1234.56,\"item_type\":\"{}\",\"budget_id\":\"{}\"}}", item.id, item.name, item.item_type, item.budget_id);
        (item, expected_response)
    }

    #[tokio::test]
    async fn budgets() {
        let repository = Arc::new(Mutex::new(BudgetRepository::new()));
        let app = create_router(repository);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/budget/budgets")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        assert_eq!(&body[..], b"[]");
    }

    #[tokio::test]
    async fn budget_by_id() {
        let repository = Arc::new(Mutex::new(BudgetRepository::new()));
        let (budget, expected_response) =
            create_budget_body_and_response("January Budget", BudgetMonth::January, 1234.56);
        let id = budget.id;
        repository.lock().unwrap().add_budget(budget);

        let app = create_router(repository);

        let response = app
            .oneshot(
                Request::builder()
                    .uri(&format!("/budget/budgets/{}", id))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();

        assert_eq!(&body[..], expected_response.as_bytes());
    }

    #[tokio::test]
    async fn item_by_id() {
        let repository = Arc::new(Mutex::new(BudgetRepository::new()));
        let (budget, _) =
            create_budget_body_and_response("January Budget", BudgetMonth::January, 1234.56);
        let budget_id = budget.id;
        let (item, expected_response) =
            create_budget_item("January Item", 1234.56, BudgetItemType::Misc, budget_id);
        let item_id = item.id;

        repository.lock().unwrap().add_budget(budget);
        repository.lock().unwrap().add_item(item);

        let app = create_router(repository);

        let response = app
            .oneshot(
                Request::builder()
                    .uri(&format!("/budget/items/{}", item_id))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();

        assert_eq!(&body[..], expected_response.as_bytes());
    }
}
