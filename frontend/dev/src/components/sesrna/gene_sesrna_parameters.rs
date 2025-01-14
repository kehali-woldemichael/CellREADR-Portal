// external
use leptos::*;
// internal
use crate::api::test_sesrna_parameters::{load_parameter_test};
// for creating random search id 
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

#[allow(unused_variables)] 
#[component]
pub fn GeneSesrnaParameters(target_sequence_rw: RwSignal<std::option::Option<String>>, submit_sesrna_parameters_rw: RwSignal<bool>, 
                                vec_sesrna_parameters: RwSignal<std::option::Option<Vec<String>>> 
                                ) -> impl IntoView {

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
            None => "Waiting for parameters to be submitted".to_string(),
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
        <label>
            "Target region: "
            <input 
                type="checkbox"
                name="min_length"
                //value = {move || min_sesrna_length()} 
                //on:input=move |ev| min_sesrna_length.set(event_target_value(&ev))
                //prob:value = {move || min_sesrna_length()}
                class="border rounded"
            />
        </label>
        <label>
            "Minimum length of sesRNA: "
            <input 
                type="number"
                name="min_length"
                value = {move || min_sesrna_length()} 
                on:input=move |ev| min_sesrna_length.set(event_target_value(&ev))
                prob:value = {move || min_sesrna_length()}
                class="border rounded"
            />
        </label>
        <br></br>

        <label>
            "Maximum length of sesRNA: "
            <input 
                type="number"
                name="min_length"
                value = {move || max_sesrna_length()} 
                on:input=move |ev| max_sesrna_length.set(event_target_value(&ev))
                prop:value={move || max_sesrna_length()} 
                class="border rounded"
            />
        </label>
        <br></br>

        <label>
            "Maximum number of mismatches: "
            <input 
                type="number"
                name="min_length"
                value = {move || num_mismatches_max()} 
                on:input=move |ev| num_mismatches_max.set(event_target_value(&ev))
                prop:value={move || num_mismatches_max()} 
                class="border rounded"
            />
        </label>
        <br></br>

        <label>
            "Minimum distance from TGG to closest mismatch: "
            <input 
                type="number"
                name="min_length"
                value = {move || dist_tgg_mismatch()} 
                on:input=move |ev| dist_tgg_mismatch.set(event_target_value(&ev))
                prop:value={move || dist_tgg_mismatch()} 
                class="border rounded"
            />
        </label>
        <br></br>

        <button 
            type="submit"
            on:click=on_submit
            class="border rounded-full w-40 h-10"
            inner_html="Submit"
        />
        <button 
            type="clear"
            on:click=on_clear
            class="border rounded-full w-40 h-10"
            inner_html="Clear"
        />

        <p>"Status of testing parameters : " {status_test_sesrna_parameters}</p>
        <p>"Search token = "{search_token}</p>
    }
}