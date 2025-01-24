use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/lifameleptos.css"/>
        // <Stylesheet id="leptos" href="style/main.scss" />

        // sets the document title
        <Title text="Lifame"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                    <Route path="/*any" view=NotFound/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    view! {
        <div class="w-screen h-screen bg-linear-to-br from-purple-500 bg-blue-950">
            <Button />
            <Input />
        </div>
    }
}

/// 404 - Not Found
#[component]
fn NotFound() -> impl IntoView {
    // set an HTTP status code 404
    // this is feature gated because it can only be done during
    // initial server-side rendering
    // if you navigate to the 404 page subsequently, the status
    // code will not be set because there is not a new HTTP request
    // to the server
    #[cfg(feature = "ssr")]
    {
        // this can be done inline because it's synchronous
        // if it were async, we'd use a server function
        let resp = expect_context::<leptos_actix::ResponseOptions>();
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! {
        <h1>"Not Found"</h1>
    }
}

#[component]
fn Button() -> impl IntoView {
    view! {
        <div class="w-full">
            <div class="flex justify-center content-center items-center">
                <button class="p-5 m-2 rounded-3xl text-white bg-linear-to-r from-purple-500 to-red-500">
                    Button
                </button>
            </div>
        </div>
    }
}

#[component]
fn Input() -> impl IntoView {
    view! {
        <div class="w-full">
            <div class="flex justify-center content-center items-center">
                <input class="text-white text-2xl rounded-3xl p-5 bg-linear-to-r from-purple-500 to-red-500">
                </input>
            </div>
        </div>

    }
}
