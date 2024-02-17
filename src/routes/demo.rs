use leptos::*;

#[server(GetDemo)]
pub async fn get_demo(value: String) -> Result<String, ServerFnError> {
    Ok(value)
}

#[island]
pub fn DemoHydrationError() -> impl IntoView {
    let items = create_resource(
        || (),
        |_| async move { get_demo("render with hydration errors".into()).await },
    );
    view! {
        <div>
            <Suspense fallback=|| view! {}>
                {move || items.get().map(Result::ok).unwrap_or_default()}
            </Suspense>
        </div>
    }
}

#[island]
pub fn DemoOk() -> impl IntoView {
    let items = create_resource(|| (), |_| async move { get_demo("render ok".into()).await });
    view! {
        <Suspense fallback=|| view! {}>
            <div>
                {move || items.get().map(Result::ok).unwrap_or_default()}
            </div>
        </Suspense>
    }
}

#[component]
pub fn Page() -> impl IntoView {
    view! {
    //    <DemoOk />
       <DemoHydrationError />
    }
}
