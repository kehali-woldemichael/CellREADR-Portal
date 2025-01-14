// external 
use leptos::*;

#[component]
pub fn ProtocolsPage() -> impl IntoView {
    use crate::components::general::banner::Banner;

    view! {
        <Banner current_page="protocols".to_string()/>

        <div class="border flex items-center m-10"> 
            <h1 class="mx-auto font-bold text-xl">"Getting started"</h1>
        </div>

        <div class="border flex items-center m-10"> 
            <h1 class="mx-auto font-bold text-xl">"Designing sesRNAs"</h1>
        </div>

        <div class="border flex items-center m-10"> 
            <h1 class="mx-auto font-bold text-xl">"Selecting expression vectors"</h1>
        </div>

        <div class="border flex items-center m-10"> 
            <h1 class="mx-auto font-bold text-xl">"Screening sesRNAs"</h1>
        </div>

        <div class="border flex items-center m-10"> 
            <h1 class="mx-auto font-bold text-xl">"Delivering in vivo"</h1>
        </div>
    }
}
