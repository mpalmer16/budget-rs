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

    let app = app(repository);

    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

fn app(repository: Arc<Mutex<BudgetRepository>>) -> Router {
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
        .with_state(repository)
}


#[cfg(test)]
mod tests {

    use axum::{http::{Request, StatusCode}, body::Body};
    use tower::ServiceExt;
    use uuid::Uuid;

    use crate::domain::{Budget, BudgetMonth::{January, self}};

    use super::*;

    #[tokio::test]
    async fn budgets() {
        let repository = Arc::new(Mutex::new(BudgetRepository::new()));
        let app = app(repository);

        let response = app
            .oneshot(Request::builder().uri("/budget/budgets").body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        assert_eq!(&body[..], b"[]");
    }

    fn create_budget_body_and_response(
        title: &str,
        month: BudgetMonth,
        total_salary: f32
    ) -> (Budget, String) {
        let id = Uuid::new_v4();
        let budget = Budget {
            id,
            name: title.to_string(),
            month,
            total_salary,
        };
        let expected_response = format!("{{\"id\":\"{}\",\"name\":\"January Budget\",\"month\":\"January\",\"total_salary\":1234.56}}", id);
        (budget, expected_response)
    }

    #[tokio::test]
    async fn budget_by_id() {
        let repository = Arc::new(Mutex::new(BudgetRepository::new()));
        let (budget, expected_response) = create_budget_body_and_response("January Budget", January, 1234.56);
        let id = budget.id;
        repository.lock().unwrap().add_budget(budget);
        let app = app(repository);

        let response = app
            .oneshot(Request::builder()
                .uri(&format!("/budget/budgets/{}", id))
                .body(Body::empty())
                .unwrap())
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();

        assert_eq!(&body[..], expected_response.as_bytes());
        }


}
