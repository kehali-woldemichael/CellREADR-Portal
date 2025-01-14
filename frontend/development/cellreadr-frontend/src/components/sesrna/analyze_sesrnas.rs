// external
use leptos::*;
//use leptos_struct_table::{TableContent,SortingMode};
// internal
//use crate::structs::sesrna::{SesRNA};

#[allow(unused_variables)] 
#[component]
pub fn AnalyzeSesrnas(perform_analysis: RwSignal<bool>, 
                        select_blastn_rw: RwSignal<bool>, select_rna_rna_rw: RwSignal<bool>,
                        submit_analysis_selections: RwSignal<bool>) -> impl IntoView {
    let button_style = "rounded-full px-4 py-2 bg-gray-200 hover:bg-gray-500";

    use crate::api::interaction_prediction::{srv_predict_interaction};
    //use crate::api::blastn::{RunBLASTn};
    // when submit_analysis = true 

    // <S: (), T: String>
    let blastn_results = create_resource(select_blastn_rw, |select_blastn| async move { srv_predict_interaction().await });
    //log::info!("blastn result = {}", blastn_results().unwrap()?);

    view! {
            {move || match perform_analysis() {
                false => view! {
                    <div>
                    </div>
                },
                true => view! {
                        <div class="">
                            <div class="flex">
                                <div class="mx-auto">
                                    <p>"Perform analysis!"</p>
                                </div>
                            </div>
                            
                            <div class="grid grid-cols-3">
                                <div>
                                    <div>
                                        <label>
                                            <input 
                                                type="checkbox"
                                                name="run_something"
                                                on:input=move |ev| {
                                                    let is_checked = event_target_checked(&ev);
                                                    let new_value = if is_checked { true } else { false };
                                                    //select_riblast_rw.set(new_value);
                                                }
                                            />
                                            " Secondary structure prediction"
                                        </label>
                                    </div>

                                    <div>
                                        <label>
                                            <input 
                                                type="checkbox"
                                                name="run_something"
                                                on:input=move |ev| {
                                                    let is_checked = event_target_checked(&ev);
                                                    let new_value = if is_checked { true } else { false };
                                                    //select_riblast_rw.set(new_value);
                                                }
                                            />
                                            " Complexity calculation"
                                        </label>
                                    </div>
                                </div>

                                <div>
                                    <div>
                                        <label>
                                            <input 
                                                type="checkbox"
                                                name="run_blastn"
                                                on:input=move |ev| {
                                                    let is_checked = event_target_checked(&ev);
                                                    let new_value = if is_checked { true } else { false };
                                                    select_blastn_rw.set(new_value);
                                                }
                                            />
                                            " BLASTn"
                                        </label>
                                    </div>

                                    <div>
                                        <label>
                                            <input 
                                                type="checkbox"
                                                name="run_something"
                                                on:input=move |ev| {
                                                    let is_checked = event_target_checked(&ev);
                                                    let new_value = if is_checked { true } else { false };
                                                    select_rna_rna_rw.set(new_value);
                                                }
                                            />
                                            " Target-sesRNA binding prediction"
                                        </label>
                                    </div>
                                </div>

                                <div>
                                    <button
                                        name="analyze_sesrnas"
                                        on:click={move |_| {
                                            submit_analysis_selections.set(true)
                                        }}
                                        class=button_style
                                        inner_html="Run analysis"
                                    />
                                </div>
                            </div>
                        </div>

                }
            }}

    }
}