// external
use leptos::*;
use leptos_struct_table::{TableContent,SortingMode};
// internal
use crate::structs::sesrna::{SesRNA};


#[allow(unused_variables)] 
#[component]
pub fn VisualizeSesrna(submit_sesrna_parameters_rw: RwSignal<bool>,
                    target_sequence_rw: RwSignal<std::option::Option<String>>,
                    vec_sesrna_parameters: RwSignal<std::option::Option<Vec<String>>>,
                    vec_sesrnas_rw: RwSignal<std::option::Option<Vec<SesRNA>>>, 
                    ) -> impl IntoView {

    let vec_sesrna_search_inputs = move || (target_sequence_rw(), vec_sesrna_parameters());

    use crate::api::sesrna_search::{load_sesrnas};
    let vec_sesrnas = create_resource(vec_sesrna_search_inputs, |input| async move { load_sesrnas(input).await });

    create_effect(move |_| match vec_sesrnas() {
        Some(vec) => vec_sesrnas_rw.set(vec),
        None => (),
    });

    let button_style = "rounded-full px-4 py-2 bg-gray-200 hover:bg-gray-500";
    
    use crate::api::genome_map::{load_genomic_map};
    let target_sesrna_map = create_resource(vec_sesrnas_rw, move |vec_sesrnas_rw| async move { 
        let length_target = create_rw_signal(target_sequence_rw().unwrap_or_default().len());
        match vec_sesrnas_rw {
            Some(vec) => load_genomic_map(length_target(), vec).await, 
            None => "loading...".to_string(),
        }
    });

    let perform_analysis: RwSignal<bool> = create_rw_signal(false);

    
    let select_blastn_rw: RwSignal<bool> = create_rw_signal(false);
    let select_rna_rna_rw: RwSignal<bool> = create_rw_signal(false);
    let submit_analysis_selections: RwSignal<bool> = create_rw_signal(false);



    use crate::components::sesrna::analyze_sesrnas::{AnalyzeSesrnas};


    //let interaction_prediction = create_resource();

    view! {
        { move || match submit_sesrna_parameters_rw() {
            false => view! {
                {move || match target_sequence_rw() {
                    Some(_) => view! {
                        <div class="flex"> 
                            <div class="mx-auto">
                                <p style="color:gray;font-style:italic;">"Submit parameters for filtering potential sesRNAs"</p>
                            </div>
                        </div>
                    },
                    None => view! {
                        <div>
                            <p></p>
                        </div>
                    }

                }}
            }.into_view(),
            true => view! { 
                <div>
                    {move || match target_sesrna_map() {
                        Some(message) if message == "loading..." => view! {
                            <div class="flex">
                                <div class="mx-auto">
                                    <p>{message}</p>
                                </div>
                            </div>
                        },
                        Some(url) => view! {
                            <div>
                                <img src={url} alt="sequence_map"/>
                            </div>
                        },
                        None => view! {
                            <div class="flex">
                                <div class="mx-auto">
                                    <p>"Failed to load map"</p>
                                </div>
                            </div>
                        }
                    }}


                    {move || match vec_sesrnas().unwrap() {
                        Some(vec) => view! {
                            <div>
                                <div class="flex mb-5">
                                    <div class="mx-auto space-x-2">
                                        <button
                                            name="analyze"
                                            on:click={move |_| {
                                                perform_analysis.set(!(perform_analysis.get()));
                                            }}
                                            class=button_style
                                            inner_html="Perform additional analysis"
                                        />

                                        <button
                                            name="download_csv"
                                            on:click={move |_| {
                                                let csv_url = format!("/data/{}.csv", vec_sesrna_parameters().unwrap()[5]);
                                                // set_href allows downloading without opening new window or tab 
                                                let _ = window().location().set_href(&csv_url);
                                            }}
                                            class=button_style
                                            inner_html="Download csv"
                                        />
                                        <button
                                            name="download_fasta"
                                            on:click={move |_| {
                                                let fasta_url = format!("/data/{}.fasta", vec_sesrna_parameters().unwrap()[5]);
                                                // set_href allows downloading without opening new window or tab 
                                                let _ = window().location().set_href(&fasta_url);
                                            }}
                                            class=button_style
                                            inner_html="Download fasta"
                                        />
                                    </div>
                                </div>

                                <AnalyzeSesrnas perform_analysis=perform_analysis
                                                select_blastn_rw=select_blastn_rw
                                                select_rna_rna_rw=select_rna_rna_rw
                                                submit_analysis_selections=submit_analysis_selections
                                />
                                // <div>
                                //     <p>"Variables"</p>
                                //     <p>"Perform analysis: "{perform_analysis}</p>
                                //     <p>"BLASTN: "{select_blastn_rw}</p>
                                //     <p>"RNA-RNA: "{select_rna_rna_rw}</p>
                                //     <p>"Submit analysis: "{submit_analysis_selections}</p>
                                // </div>

                                <div class="flex">
                                    <div class="mx-auto">
                                        <TableContent 
                                            rows=vec.clone()
                                            row_class="select-none"
                                            sorting_mode=SortingMode::SingleColumn
                                        />
                                    </div>
                                </div>

                            </div>
                        },
                        None => view! {
                            <div>
                                <p>"No sesrnas can be generated using provided parameters"</p>
                            </div>
                        },
                    }}
                </div>
            }.into_view(),
        }}
    }
}


