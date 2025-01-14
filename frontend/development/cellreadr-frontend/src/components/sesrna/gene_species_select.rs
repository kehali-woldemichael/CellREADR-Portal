// external
use leptos::*;
use leptos_struct_table::{TableContent,SortingMode, SelectionChangeEvent, Selection};
// internal
use crate::api::ensembl_info::{load_transcript_info};
use crate::api::ensembl_sequence::{load_transcript_sequence};
use crate::structs::transcripts_info::TranscriptsInfo;

#[allow(unused)]
#[component]
pub fn SelectGeneSpecies(target_sequence_rw: RwSignal<std::option::Option<String>>, submit_target_sequence_rw: RwSignal<bool>) -> impl IntoView {
   
    // Had to write to temporary string signal ... 
    // Writing to entry struct signal ... results in focus shifting with every keystroke 
    let gene_rw: RwSignal<std::option::Option<String>> = create_rw_signal(None);
    let species_rw = create_rw_signal(String::from("..."));
    let submit_gene_species_rw = create_rw_signal(false);


    // Resource for async access of mysql database with Ensembl information on transcripts for a given gene
    let transcript_info = create_resource(gene_rw, |gene_rw| async move { 
        match gene_rw {
            Some(gene) => Some(load_transcript_info(gene).await),
            None => None,
        }
    });
    let selected_index = create_rw_signal(None);
    let (target_transcript, set_target_transcript) = create_signal(None);
    let select_target_transcript = {move |evt: SelectionChangeEvent<TranscriptsInfo>| {
        set_target_transcript.update(|selected_row| {
            *selected_row = Some(evt.row);
        });
        submit_target_sequence_rw.set(true);
    }};
    let transcript_sequence = create_resource(target_transcript, |target_transcript| async move { 
        match target_transcript {
            None => None,
            Some(data) => Some(load_transcript_sequence(data.transcript_id).await),
        }
    });

    view! {

        <EnterGeneSpecies gene_rw=gene_rw species_rw=species_rw submit_gene_species_rw=submit_gene_species_rw/>

        { move || match submit_gene_species_rw() {
            false => view! {
                <div>
                    <p>"..."</p>
                </div>
            }.into_view(),
            true => view! {
                        {move || match transcript_info.get() {
                            None => view! {
                                <div>
                                    <p>"loading"</p>
                                </div>
                            },
                            Some(info) => view! {
                                <div class="mt-10 ml-20">
                                    <table>
                                        <TableContent 
                                            rows=info.unwrap()
                                            selection=Selection::Single(selected_index)
                                            row_class="select-none"
                                            on_selection_change=select_target_transcript
                                            sorting_mode=SortingMode::SingleColumn
                                        />
                                    </table>
                                </div>
                            }
                        }}
                        { move || target_transcript.get().map(|target_transcript| {
                            view! {
                                <div class="rounded-md overflow-clip m-10 border dark:border-gray-700 float-left px-5 py-3 bg-white dark:bg-gray-800 text-gray-700 dark:text-gray-400">
                                    <pre>
                                        "      Gene Name:      " {target_transcript.gene_name} "\n"
                                        "      Transcript ID:  " {&target_transcript.transcript_id} "\n"
                                        "      Biotype:        " {target_transcript.biotype} "\n"
                                    </pre>
                                    // Selecting target region
                                </div>
                            }
                        })}
            }.into_view()
        }}
        }
}


#[component]
pub fn EnterGeneSpecies(gene_rw: RwSignal<std::option::Option<String>>, species_rw: RwSignal<String>, submit_gene_species_rw: RwSignal<bool>) -> impl IntoView {
    // Styling
    let button_style = "rounded-full px-4 py-2 bg-gray-200 hover:bg-gray-500";
    let clear_button_style = "rounded-full px-6 py-2 bg-gray-200 hover:bg-gray-500";
    
    let on_clear_species_gene = move |_| {
        submit_gene_species_rw.set(false);
        gene_rw.set(None);
        species_rw.set(String::from("..."));
    };

    view! {
        <div class="grid grid-cols-3">
            <div class="flex">
                <div class="mx-auto">
                    <div class="mb-2">
                        <p>"Select species: "</p>
                    </div>
                    <div>
                        <p>"Enter gene: "</p>
                    </div>
                </div>
            </div>

            <div>
                <div class="mb-2">
                    <select
                        value={move || species_rw()}
                        on:change=move |ev| species_rw.set(event_target_value(&ev))
                        prop:value={move || species_rw()}
                        class="border w-30"
                    >
                        <option value="None".to_string()>"..."</option>
                        <option value="Mouse".to_string()>"Mouse"</option>
                        <option value="Human".to_string()>"Human"</option>
                        <option value="Macaque".to_string()>"Macaque"</option>
                    </select>
                </div>

                <div>
                    <input 
                        type="text"
                        name="gene_name"
                        value = {move || gene_rw.get().unwrap_or_default()} 
                        on:input={move |ev| gene_rw.set(Some(event_target_value(&ev)))}
                        prop:value = {move || gene_rw.get().unwrap_or_default()} 
                        class="border rounded w-20"
                    />
                </div>
            </div>

            <div class="flex">
                <div class="mx-auto">
                    <div class="mb-2">
                        <button 
                            type="submit"
                            on:click={move |_| submit_gene_species_rw.set(true)}
                            class=button_style
                            inner_html="Submit"
                        />
                    </div>

                    <div>
                        <button 
                            type="clear"
                            on:click=on_clear_species_gene
                            class=clear_button_style
                            inner_html="Clear"
                        />
                    </div>
                </div>
            </div>
        </div>
    }
}