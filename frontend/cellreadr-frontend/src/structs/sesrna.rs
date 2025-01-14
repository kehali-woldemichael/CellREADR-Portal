// external
use leptos::*;
use leptos::{component, MaybeSignal, view, IntoView, window};
use serde::{Serialize, Deserialize};
use leptos_struct_table::*;
// internal
use crate::components::general::tailwind::TailwindClassesPreset;

#[derive(Debug, PartialEq, PartialOrd, Eq, Serialize, Deserialize, TableRow, Clone)]
#[table(
    sortable,
    classes_provider = "TailwindClassesPreset",
    impl_vec_data_provider,
)]
pub struct SesRNA {
   pub index_tgg: usize,
   pub index_begin: usize,
   pub index_end: usize,
    #[table(
        cell_class = "text-red-600 dark:text-red-400 text-center",
        head_class = "text-red-700 dark:text-red-300"
    )]
   pub num_mismatches: usize,
   pub num_start: usize,
   pub num_stop: usize,
   pub length: usize,
   pub gc_content: usize,
   #[table(renderer = "SequenceCellRenderer")]
   pub dna_sequence: String,
}

// Cell renderer that just displays a button
#[allow(unused_variables)] 
#[component]
pub fn SequenceCellRenderer<F>(
    class: String,
    #[prop(into)] value: MaybeSignal<String>,
    on_change: F,
    index: usize,
) -> impl IntoView
where
    F: Fn(String) + 'static,
{
    let button_value = create_rw_signal("Copy".to_string());
    let button_style = "rounded-full px-2 py-1 bg-gray-200 hover:bg-gray-500";

    view! {
        <td class=class>
            <button
                name="test_copy_text"
                on:click={move |_| { 
                    let _ = window().navigator().clipboard().write_text(&value.get()); 
                    button_value.set("Copied!".to_string())
                }}
                class=button_style
                inner_html={button_value}
            />
        </td>
    }
}