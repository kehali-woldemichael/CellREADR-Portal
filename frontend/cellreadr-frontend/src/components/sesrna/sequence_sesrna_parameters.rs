// external
use leptos::*;
// for creating random search id 
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

// internal
// structs
use crate::api::test_sesrna_parameters::{load_parameter_test};

#[allow(unused_variables)] 
#[component]
pub fn SequenceSesrnaParameters(target_sequence_rw: RwSignal<std::option::Option<String>>, submit_sesrna_parameters_rw: RwSignal<bool>, 
                                vec_sesrna_parameters: RwSignal<std::option::Option<Vec<String>>> 
                                ) -> impl IntoView {

    //internal
    use crate::functions::check_status::DisplayStatus;

    // style 
    // TODO: figure out how to justify text inside of input box
    let input_box_class = "border rounded w-20";
    let submit_button_style = "rounded px-3 py-1 bg-gray-200 hover:bg-gray-500";
    let clear_button_style = "border rounded px-3 py-1 bg-white hover:bg-gray-500";

    // Option to few full of
    let option_advanced_rw: RwSignal<bool> = create_rw_signal(false);
    let basic_option_style: RwSignal<String> = create_rw_signal(submit_button_style.to_string());
    let advanced_option_style: RwSignal<String> = create_rw_signal(clear_button_style.to_string());


    // Need length of target sequence to test if sesRNA lenghts are less than or equal to target sequence length
    let target_sequence_length_rw: RwSignal<String> = create_rw_signal(target_sequence_rw().unwrap().len().to_string());

    // SesRNA parameters
    let min_sesrna_length: RwSignal<String> = create_rw_signal("201".to_string());
    let max_sesrna_length: RwSignal<String> = create_rw_signal("300".to_string());
    let num_mismatches_max: RwSignal<String> = create_rw_signal("3".to_string());
    let dist_tgg_mismatch: RwSignal<String> = create_rw_signal("12".to_string());
    // generate random token for each search 
    let search_token: RwSignal<String> = create_rw_signal(thread_rng()
                                                            .sample_iter(&Alphanumeric)
                                                            .take(30)
                                                            .map(char::from)
                                                            .collect());


    // On clicking submit button, triggers test of parameters
    let submit_test_sesrna_parameters: RwSignal<bool> = create_rw_signal(false);
    create_effect(move |_| match submit_test_sesrna_parameters() {
        false => (),
        true => vec_sesrna_parameters.set(Some(vec![
            target_sequence_length_rw(), 
            min_sesrna_length(), 
            max_sesrna_length(), 
            num_mismatches_max(),
            dist_tgg_mismatch(),
            search_token(),
            ])),
    });
    // on_submit: parse temp input string to i32 sesrna parameters Result
    let status_test_sesrna_parameters = create_resource(vec_sesrna_parameters, |vec_sesrna_parameters| async move {
        match vec_sesrna_parameters {
            Some(vector) => load_parameter_test(vector).await,
            None => "waiting".to_string(),
        }
    });

    create_effect(move |_| match status_test_sesrna_parameters() {
        Some(status) if status == "pass".to_string() => submit_sesrna_parameters_rw.set(true),
        Some(_) => submit_sesrna_parameters_rw.set(false),
        None => submit_sesrna_parameters_rw.set(false),
    });

    // on submit
    let on_submit = move |_| {
        submit_test_sesrna_parameters.set(true);
    };

    let on_clear = move |_| {
        submit_test_sesrna_parameters.set(false);
    };





    view! {
        <div>
            <div>
                <button 
                    name="basic_parameters_choice"
                    on:click={move |_| {
                        option_advanced_rw.set(false);
                        basic_option_style.set(submit_button_style.to_string());
                        advanced_option_style.set(clear_button_style.to_string());
                    }}
                    class=basic_option_style
                    inner_html="Basic"
                />

                <button 
                    name="advanced_parameters_choice"
                    on:click={move |_| {
                        option_advanced_rw.set(true);
                        advanced_option_style.set(submit_button_style.to_string());
                        basic_option_style.set(clear_button_style.to_string());
                    }}
                    class=advanced_option_style
                    inner_html="Advanced"
                />
            </div>
        </div>

        <div class="grid grid-cols-3 mt-1.5">
            <div class="col-span-2 space-y-1.5">
                <p>"Minimum length of sesRNA: "</p>
                <p>"Maximum length of sesRNA: "</p>
                <p>"Maximum number of mismatches: "</p>
            </div>

            <div class="col-span-1">
                <div class="flex">
                    <div class="ml-10 space-y-1">
                        <div>
                            <input 
                                type="number"
                                name="min_length"
                                value = {move || min_sesrna_length()} 
                                on:input=move |ev| min_sesrna_length.set(event_target_value(&ev))
                                prob:value = {move || min_sesrna_length()}
                                class=input_box_class
                            />
                        </div>

                        <div>
                            <input 
                                type="number"
                                name="min_length"
                                value = {move || max_sesrna_length()} 
                                on:input=move |ev| max_sesrna_length.set(event_target_value(&ev))
                                prop:value={move || max_sesrna_length()} 
                                class=input_box_class
                            />
                        </div>

                        <div>
                            <input 
                                type="number"
                                name="min_length"
                                value = {move || num_mismatches_max()} 
                                on:input=move |ev| num_mismatches_max.set(event_target_value(&ev))
                                prop:value={move || num_mismatches_max()} 
                                class=input_box_class
                            />
                        </div>

                    </div>
                </div>
            </div>

        </div>


        { move || match option_advanced_rw() {
            false => view! {
                <div>
                </div>
            },
            true => view! {
                <div class="mt-1.5 grid grid-cols-3">
                    <div class="col-span-2 space-y-1.5">
                        <p>"Minimum distance from TGG to mismatch: "</p>
                        <p>"Mininum GC: "</p>
                        <p>"Max GC: "</p>
                        <p>"Exclude overlapping sesRNAs: "</p>
                    </div>

                    <div class="col-span-1 ml-10 space-y-1">
                        <div>
                            <input 
                                type="number"
                                name="min_dist_tgg"
                                value = {move || dist_tgg_mismatch()} 
                                on:input=move |ev| dist_tgg_mismatch.set(event_target_value(&ev))
                                prop:value={move || dist_tgg_mismatch()} 
                                class=input_box_class
                            />
                        </div>
                        <div>
                            <input 
                                type="number"
                                name="min_gc"
                                //value = {move || dist_tgg_mismatch()} 
                                //on:input=move |ev| dist_tgg_mismatch.set(event_target_value(&ev))
                                //prop:value={move || dist_tgg_mismatch()} 
                                class=input_box_class
                            />
                        </div>
                        <div>
                            <input 
                                type="number"
                                name="max_gc"
                                //value = {move || dist_tgg_mismatch()} 
                                //on:input=move |ev| dist_tgg_mismatch.set(event_target_value(&ev))
                                //prop:value={move || dist_tgg_mismatch()} 
                                class=input_box_class
                            />
                        </div>
                        <div class="ml-8">
                            <input 
                                type="checkbox"
                                name="exclude_overlaps"
                                on:input=move |ev| {
                                    let is_checked = event_target_checked(&ev);
                                    let new_value = if is_checked { true } else { false };
                                    //select_rna_rna_rw.set(new_value);
                                }
                            />

                        </div>
                    </div>
                </div>
            },
        }}


        <div class="grid grid-cols-3 mt-1.5">
            <div class="col-span-2">
                <div>
                    <DisplayStatus status=status_test_sesrna_parameters/>
                </div>
                //<p>"Search token = "{search_token}</p>
            </div>

            <div class="col-span-1">
                <button 
                    name="submit_parameters"
                    type="submit"
                    on:click=on_submit
                    class=submit_button_style
                    inner_html="Submit"
                />

                <button 
                    name="clear_parameters"
                    type="clear"
                    on:click=on_clear
                    class=clear_button_style
                    inner_html="Clear"
                />
            </div>
        </div>

    }
}