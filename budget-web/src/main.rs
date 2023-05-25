use leptos::*;

fn main() {
    mount_to_body(|cx| {
        view! { cx, <App/> }
    })
}


#[component]
fn ProgressBar(
    cx: Scope,
    #[prop(default=100)]
    max: u16,
    #[prop(into)]
    progress: Signal<i32>,
) -> impl IntoView {
    view! { cx, 
        <progress
            max=max
            value=move || progress.get()
        >
        </progress>
        <br/>
    }
}

#[component]
fn App(cx: Scope) -> impl IntoView {
    let (count, set_count) = create_signal(cx, 0);
    let double_count = move || count.get() * 2;

    view! { cx,
        <button
            on:click=move |_| {
                set_count.update(|c| *c += 1);
            }
            class:red=move || count.get() % 2 == 1
        >
            "Click me: "
        </button>

        <br/>

        <ProgressBar
            max=50
            progress=count
        />

        <ProgressBar
            max=50
            progress=Signal::derive(cx, double_count)
        />

        <ProgressBar progress=count/>

        <p>
            <strong>"Count: "</strong>
            {count} 
        </p>

        <p>
            <strong>"Double Count: "</strong>
            {double_count} 
        </p>
    }
}
