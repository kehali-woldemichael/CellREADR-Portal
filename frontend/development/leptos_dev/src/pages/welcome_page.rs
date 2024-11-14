// external 
use leptos::*;

#[component]
pub fn WelcomePage() -> impl IntoView {
    use crate::components::general::banner::Banner;

    view! {
        <Banner current_page="welcome".to_string()/>

        <div class="flex items-center"> 
            <h1 class="mx-auto font-bold mt-10 text-xl">"Welcome to CellREADR Portal!"</h1>
        </div>
    }
}