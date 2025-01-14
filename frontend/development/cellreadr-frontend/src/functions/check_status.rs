// external
use leptos::*;
// internal

#[component]
pub fn DisplayStatus<T: 'static + Clone>(status: Resource<Option<T>, String>) -> impl IntoView {
    view! {
        {move || match status() {
            Some(output) if output == "pass" => view! {
                // color code output 
                <div>
                    <p style="color:green;font-style:italic;">"Status of entry: "{output}</p>
                </div>
            },
            Some(output) if output == "waiting" => view! {
                // color code output 
                <div>
                    <p style="color:gray;font-style:italic;">"Status of entry: waiting for entry"</p>
                </div>
            },
            Some(output) => view! {
                // color code output 
                <div>
                    <p style="color:red;font-style:italic;">"Status of entry: fail"</p>
                    <p style="color:red;font-style:italic;">{output}</p>
                </div>
            },
            None => view! {
                <div>
                    <p>"Entry is empty"</p>
                </div>
            },
        }}

    }
}