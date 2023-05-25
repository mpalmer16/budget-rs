
use leptos::*;

use budget_common::domain::{Budget, BudgetCreationRequest, BudgetMonth};

use gloo_timers::future::TimeoutFuture;

fn main() {
    mount_to_body(|cx| {
        view! { cx, <App/> }
    })
}

// This will eventually be used to pull data from the server
// for now just use dummy data
async fn fetch_from_server(value: i32) -> Vec<Budget> {
    TimeoutFuture::new(1_000).await;
    let budget = Budget::new(BudgetCreationRequest {
        name: format!("Budget {}", value),
        total_salary: 100.0,
        month: BudgetMonth::January,
    });
    vec![budget]
}

#[component]
fn BudgetTable(
    cx: Scope,
    #[prop(into)]
    budgets: Vec<Budget>,
) -> impl IntoView {
    view! { cx,
        <ul>
            {
                budgets.into_iter()
                    .map(|n| view! { cx, <li>{n.name}</li> } )
                    .collect_view(cx)
            }
        </ul>
    }
}



#[component]
fn App(cx: Scope) -> impl IntoView {
    let (count, set_count) = create_signal(cx, 0);

    let budgets_resource = create_resource(
        cx, 
        move || count.get(), 
        |value| async move { fetch_from_server(value).await }
    );

    let budget_result = move || {
        budgets_resource
            .read(cx)
            .map(|budgets| format!("Current budgets: {budgets:?}"))
            .unwrap_or_else(|| "Loading budgets...".into())
    };

    let loading = budgets_resource.loading();
    let is_loading = move || if loading.get() { "Loading budgets ..." } else { "Idle." };

    view! { cx,
        <button
            on:click=move |_| {
                set_count.update(|c| *c += 1);
            }
            class:red=move || count.get() % 2 == 1
        >
            "Fetch Budgets"
        </button>

        <br/>

        <p>
            <code>"budgets"</code>": "
            {budget_result}
            <br/>
            {is_loading}
        </p>

        <BudgetTable budgets=vec![]/>
    }
}
