// external
use leptos::*;
// internal
use crate::components::sesrna::sequence_sesrna_parameters::SequenceSesrnaParameters;
use crate::components::sesrna::gene_sesrna_parameters::GeneSesrnaParameters;
// structs


#[allow(unused_variables)] 
#[allow(unused)]
#[component]
pub fn EnterSesrnaParameters(entry_type_rw: RwSignal<std::option::Option<String>>, target_sequence_rw: RwSignal<std::option::Option<String>>, 
                            submit_target_sequence_rw: RwSignal<bool>, submit_sesrna_parameters_rw: RwSignal<bool>,
                            vec_sesrna_parameters: RwSignal<std::option::Option<Vec<String>>> 
                            ) -> impl IntoView {

    view! {
        {move || match submit_target_sequence_rw() {
            false => view! {
                { move || match entry_type_rw().unwrap_or_default().as_str() {
                    "sequence" => view! {
                        <div class="flex">
                            <div class="mx-auto">
                                <p style="color:gray;font-style:italic;">"Waiting for target sequence to be defined"</p>
                            </div>
                        </div>
                    }.into_view(),
                    "gene" => view! {
                        //<p>"First select species and enter gene then select transcript"</p> 
                        <GeneSesrnaParameters target_sequence_rw=target_sequence_rw 
                                                  submit_sesrna_parameters_rw=submit_sesrna_parameters_rw
                                                  vec_sesrna_parameters=vec_sesrna_parameters
                        />
                    }.into_view(),
                    _ => view! { 
                        <p>""</p> 
                    }.into_view(),
                }}
            }.into_view(),
            true => view! {
                { move || match entry_type_rw().unwrap_or_default().as_str() {
                    "sequence" => view! {
                        <SequenceSesrnaParameters target_sequence_rw=target_sequence_rw 
                                                  submit_sesrna_parameters_rw=submit_sesrna_parameters_rw
                                                  vec_sesrna_parameters=vec_sesrna_parameters
                        />
                    }.into_view(),
                    "gene" => view! {
                        <GeneSesrnaParameters target_sequence_rw=target_sequence_rw 
                                                  submit_sesrna_parameters_rw=submit_sesrna_parameters_rw
                                                  vec_sesrna_parameters=vec_sesrna_parameters
                        />
                    }.into_view(),
                    _ => view! { 
                        <p>"..."</p> 
                    }.into_view(),
                }}
            }.into_view()
        }}
    }
}