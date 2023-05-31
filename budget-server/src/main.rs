mod handler;
mod repository;
mod service;

use std::{sync::{Arc, Mutex}, time::Duration};

use axum::{
    routing::{get, post},
    Router, extract::State,
};


use handler::{
    add_budget, add_item, add_items, fetch_all_budgets, fetch_all_items, fetch_budget_by_id,
    fetch_item_by_id, update_budget, update_item,
};
use hyper::{Method, StatusCode};
use repository::BudgetRepository;
use sqlx::{postgres::PgPoolOptions, Postgres, Pool};
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

    let db_connection_str = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/budget_db".into());

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&db_connection_str)
        .await
        .expect("can't connect to database");

    let repository = Arc::new(Mutex::new(BudgetRepository::new()));

    let _router = create_router(repository);

    let db_router = create_db_router(pool);

    axum::Server::bind(&"0.0.0.0:8082".parse().unwrap())
        .serve(db_router.into_make_service())
        .await
        .unwrap();

    // axum::Server::bind(&"0.0.0.0:8081".parse().unwrap())
    //     .serve(router.into_make_service())
    //     .await
    //     .unwrap();
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

fn create_db_router(state: Pool<Postgres>) -> Router {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any);
    Router::new()
        .route("/budget/budgets", get(fetch_all_budgets_db))
        .with_state(state)
        .layer(TraceLayer::new_for_http())
        .layer(cors)
}

async fn fetch_all_budgets_db(
    State(pool): State<Pool<Postgres>>,
) -> Result<String, (StatusCode, String)> {
    sqlx::query_scalar("select 'hello world from pg'")
        .fetch_one(&pool)
        .await
        .map_err(internal_error)

}

fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
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
