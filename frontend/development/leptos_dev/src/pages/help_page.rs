// external 
use leptos::*;

#[component]
pub fn HelpPage() -> impl IntoView {
    use crate::components::general::banner::Banner;

    view! {
        <Banner current_page="help".to_string()/>

        <div class="flex items-center"> 
            <h1 class="mx-auto font-bold mt-10 text-xl">"Help Page"</h1>
        </div>

    }
}
