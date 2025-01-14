// external
use leptos::*;
// internal

#[component]
pub fn EntryType(entry_type_rw: RwSignal<std::option::Option<String>>, target_sequence_rw: RwSignal<std::option::Option<String>>, 
                 submit_target_sequence_rw: RwSignal<bool>) -> impl IntoView {
    // internal
    use crate::components::sesrna::sequence_target_enter::EnterTargetSequence;
    use crate::components::sesrna::gene_species_select::SelectGeneSpecies;

    let submit_button_style = "rounded-full px-4 py-2 bg-gray-200 hover:bg-gray-500";
    let clear_button_style = "border rounded-full px-4 py-2 bg-white hover:bg-gray-200";

    view! {
        <div class="flex">
            <div class="mx-auto mb-2 mt-2 space-x-2">
                <button
                    name="enter_sequence"
                    type="submit"
                    on:click={move |_| entry_type_rw.set(Some("sequence".to_string()))}
                    class=submit_button_style
                    inner_html="Sequence"
                />

                <button
                    name="enter_species_gene"
                    type="submit"
                    on:click={move |_| entry_type_rw.set(Some("gene".to_string()))}
                    class=submit_button_style
                    inner_html="Gene"
                />
                <button
                    name="clear_all"
                    type="clear"
                    on:click={move |_| entry_type_rw.set(None)}
                    class=clear_button_style
                    inner_html="Clear All"
                />
            </div>
        </div>

        <div>
            { move || match entry_type_rw().unwrap_or_default().as_str() {
                "sequence" => view! {
                    <EnterTargetSequence target_sequence_rw=target_sequence_rw submit_target_sequence_rw=submit_target_sequence_rw/>
                }.into_view(),
                "gene" => view! {
                    <SelectGeneSpecies target_sequence_rw=target_sequence_rw submit_target_sequence_rw=submit_target_sequence_rw/>
                }.into_view(),
                _ => view! { 
                    <div class="flex">
                        <div class="mx-auto">
                            <p style="color:gray;font-style:italic;">"Waiting for selection of method to define target sequence"</p>
                            //<p>"Sequence option for entering target sequence as text"</p> 
                            //<p>"Gene option for selecting transcripts for a gene and species"</p> 
                        </div>
                    </div>
                }.into_view(),
            }}
        </div>
    }
}