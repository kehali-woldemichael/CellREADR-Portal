// external 
use leptos::*;

#[component]
pub fn HelpPage() -> impl IntoView {
    use crate::components::general::banner::Banner;

    view! {
        <Banner current_page="help".to_string()/>

        <div class="border flex items-center m-10 grid grid-cols-2"> 
            <div class="border mx-auto">
                <h1 class="mx-auto font-bold text-xl">"Troubleshooting in culture"</h1>
            </div>

            <div class="border mx-auto">
                <h1 class="mx-auto font-bold text-xl">"Troubleshooting in vivo"</h1>
            </div>
        </div>

        <div class="border flex items-center m-10"> 
            <h1 class="mx-auto font-bold text-xl">"FAQ"</h1>
        </div>

        <div class="border flex items-center m-10"> 
            <h1 class="mx-auto font-bold text-xl">"Contact"</h1>
        </div>
    }
}
