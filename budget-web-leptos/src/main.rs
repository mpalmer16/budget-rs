use leptos::*;

use budget_common::{domain::Budget, client::BudgetClient};



fn main() {
    // GetAllBudgets::register();
    mount_to_body(|cx| {
        view! { cx, <App/> }
    })
}

// #[server(GetAllBudgets, "/budgets")]
async fn fetch_from_server() -> Vec<Budget> {
    let budget_client = BudgetClient::new(
        "localhost",
        8081,
        "budget",
        false,
    );
    match budget_client.get_budgets().await {
        Ok(budgets) => budgets,
        Err(_e) => vec![],
    }
}

#[component]
fn BudgetTable(cx: Scope, #[prop(into)] budgets: Signal::<Vec<Budget>>) -> impl IntoView {
    log!("budgets: {:?}", budgets.get());
    view! { cx,
        <h1>"Budgets"</h1>
        
        <ul>
            {
                budgets.get().into_iter()
                    .map(|n| view! { cx, <li>{n.name}</li> } )
                    .collect_view(cx)
            }
        </ul>
    }
}

#[component]
fn App(cx: Scope) -> impl IntoView {

    let (_budgets, _set_budgets) = create_signal(cx, fetch_from_server());

    let budgets_resource = create_resource(
        cx,
        || (),
        |_| async move { fetch_from_server().await },
    );

    let budget_result = move || {
        budgets_resource
            .read(cx).unwrap_or_default()
    };

    let loading = budgets_resource.loading();
    let is_loading = move || {
        if loading.get() {
            "Loading budgets ..."
        } else {
            "Idle."
        }
    };

    view! { cx,
        <button
            on:click=move |_| {
                budgets_resource.refetch()
            }
        >
            "refresh"
        </button>

        <br/>

        <p>
            <code>"budgets"</code>": "
            <BudgetTable budgets=Signal::derive(cx, budget_result)/>
            <br/>
            {is_loading}
        </p>

        
    }
}
