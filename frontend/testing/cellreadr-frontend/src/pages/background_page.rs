// external 
use leptos::*;

#[component]
pub fn BackgroundPage() -> impl IntoView {
    use crate::components::general::banner::Banner;

    view! {
        <Banner current_page="introduction".to_string()/>

        <div class="border grid grid-cols-2 m-10"> 
            <div class="items-center mx-auto">
                <h1 class="mx-auto font-bold mt-10 text-xl">"Introduction"</h1>
            </div>

            <div class="border items-center p-10">
                <img src="/assets/images/background/CellREADR-Mechanism.gif" alt="mechanism"/>
            </div>
        </div>
    }
}