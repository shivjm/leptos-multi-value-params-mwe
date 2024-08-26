use crate::error_template::{AppError, ErrorTemplate};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/leptos-multi-value-param-mwe.css"/>

        <Title text="Multi-value parameters example"/>

        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! { <ErrorTemplate outside_errors/> }.into_view()
        }>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    let location = use_location();
    let search = location.search;
    let params = use_query_map();

    view! {
        <p>Query string: <code>{move || search.get()}</code></p>
        <p>Params: <code>{move || params.with(|p| format!("{p:?}"))}</code></p>
        <Form action="" method="GET">
            <label>
                <input type="checkbox" name="param" value="a"/>
                " Enable A"
            </label>
            <label>
                <input type="checkbox" name="param" value="b"/>
                " Enable B"
            </label>
            <button>Submit</button>
        </Form>
    }
}
