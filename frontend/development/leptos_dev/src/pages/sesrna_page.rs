// external 
use leptos::*;

#[component]
pub fn SesRnaPage() -> impl IntoView {
    use crate::components::general::banner::Banner;

    view! {
        <Banner current_page="sesrna".to_string()/>
        
        <div class="flex items-center"> 
            <h1 class="mx-auto font-bold mt-10 text-xl">"SesRNA Generation"</h1>
        </div>

        //<EntryForm/>
    }
}