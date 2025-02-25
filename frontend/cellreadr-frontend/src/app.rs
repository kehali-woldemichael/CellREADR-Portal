use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    //use pages;
    use crate::pages::welcome_page::WelcomePage;
    use crate::pages::background_page::BackgroundPage;
    use crate::pages::protocols_page::ProtocolsPage;
    use crate::pages::help_page::HelpPage;

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/app.css"/>

        // sets the document title
        <Title text="CellREADR Portal"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes>
                    <Route path="/" view=WelcomePage/>
                    <Route path="/background" view=BackgroundPage/>
                    <Route path="/protocols" view=ProtocolsPage/>
                    <Route path="/help" view=HelpPage/>

                    <Route path="/*any" view=NotFound/>
                </Routes>
            </main>
        </Router>
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
