// external
use leptos::*;
// internal

#[component]
pub fn EnterTargetSequence(target_sequence_rw: RwSignal<std::option::Option<String>>, submit_target_sequence_rw: RwSignal<bool>) -> impl IntoView {
    // internal 
    use crate::api::test_sequences::{load_sequence_test};
    use crate::functions::check_status::DisplayStatus;

    // Styling
    let button_style = "rounded px-4 py-1 bg-gray-200 hover:bg-gray-500";
    let clear_button_style = "border rounded px-4 py-1 bg-white hover:bg-gray-200";

    // For storing value directly from input ... on submit transfer to official target_sequence_rw
    // Want to initiate child processes when submit is clicked 
    let temp_sequence = create_rw_signal(None);
    // For indicating submit button has been selected ... but does not allow for proceeding 
    let temp_submit = create_rw_signal(false);

    // check if sequence is valid DNA sequnece
    let status_sequence_entry_rs = create_resource(target_sequence_rw, |target_sequence_rw| async move {
        //match target_sequence_rw {
        match load_sequence_test(target_sequence_rw).await {
            //Some(sequence) => { Some(test_if_sequence_valid(Some(sequence)).await) }
            Some(sequence) => sequence,
            None => "none".to_string(),
        }
    });



    create_effect(move |_| match status_sequence_entry_rs() {
        Some(status) if status == "pass".to_string() => submit_target_sequence_rw.set(true),
        Some(_) => submit_target_sequence_rw.set(false),
        None => submit_target_sequence_rw.set(false),
        }
    );

    let on_submit = move |_| match temp_sequence() {
        Some(sequence) => {
            temp_submit.set(true);
            target_sequence_rw.set(Some(sequence));
        },
        None => {
            temp_submit.set(false);
            target_sequence_rw.set(None);
        }
    };
    
    let on_click_clear = move |_| {
        target_sequence_rw.set(None);
        submit_target_sequence_rw.set(false);
            temp_submit.set(false);
    };
    
    view! {
        <div class="grid grid-cols-2 mt-5">
            <div class="ml-5">
                <div class="mb-2">
                    <p>"Enter target sequence: "</p>
                </div>

                <div>
                    <DisplayStatus status=status_sequence_entry_rs/> 
                </div>
            </div>

            <div>
                <div>
                    <input 
                        type="text"
                        name="gene"
                        // fills in input box with current value 
                        value = {move || target_sequence_rw().unwrap_or_default()}
                        on:input = move |ev| temp_sequence.set(Some(event_target_value(&ev).to_lowercase()))
                        prop:value={move || target_sequence_rw().unwrap_or_default()} 
                        class="border rounded"
                    />
                </div>

                <div class="space-x-2 mt-2 ml-2">
                    <button
                        name="submit_target_sequence"
                        on:click=on_submit
                        class=button_style
                        inner_html="Submit"
                    />
                    <button
                        name="clear_target_sequence_entry"
                        on:click=on_click_clear
                        class=clear_button_style
                        inner_html="Clear"
                    />
                </div>
            </div>
        </div>

    }
}