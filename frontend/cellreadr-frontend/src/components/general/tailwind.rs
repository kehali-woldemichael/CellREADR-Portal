use leptos_struct_table::{ColumnSort, TableClassesProvider};

#[derive(Clone, Copy)]
pub struct TailwindClassesPreset;

impl TableClassesProvider for TailwindClassesPreset {
    fn new() -> Self {
        Self
    }

    fn thead_row(&self, template_classes: &str) -> String {
        let text_color: String = String::from("text-black");
        let bg_color: String = String::from("bg-white");
        // pub let dark_text_color: String = "text-white".to_string();

        format!(
            "{} {}",
            format!("text-xs uppercase  {} {} dark:bg-black dark:text-white", text_color, bg_color),
            template_classes
        )
    }

    fn thead_cell(&self, sort: ColumnSort, template_classes: &str) -> String {
        let sort_class = match sort {
            ColumnSort::None => "",
            _ => "text-black dark:text-white",
        };

        format!(
            "cursor-pointer px-2 py-2 {} {}",
            sort_class, template_classes
        )
    }

    fn thead_cell_inner(&self) -> String {
        "flex items-center after:content-[--sort-icon] after:pl-1 after:opacity-40 before:content-[--sort-priority] before:order-last before:pl-0.5 before:font-light before:opacity-40".to_string()
    }

    fn row(&self, row_index: usize, selected: bool, template_classes: &str) -> String {
        let bg_color = if row_index % 2 == 0 {
            if selected {
                "bg-gray-100 text-black"
            } else {
                "bg-white text-black hover:bg-gray-100"
            }
        } else if selected {
            "bg-gray-100 text-black"
        } else {
            "bg-white text-black hover:bg-gray-100"
        };

        format!(
            "{} {} {}",
            "border-b text-center", bg_color, template_classes, 
        )
    }

    fn loading_cell(&self, _row_index: usize, _col_index: usize, prop_class: &str) -> String {
        format!("{} {}", "px-5 py-2 text-center", prop_class)
    }

    fn loading_cell_inner(&self, row_index: usize, _col_index: usize, prop_class: &str) -> String {
        let width = match row_index % 4 {
            0 => "w-[calc(50%-2.5rem)]",
            1 => "w-[calc(50%-2.5rem)]",
            2 => "w-[calc(50%-2.5rem)]",
            _ => "w-[calc(50%-2.5rem)]",
        };
        format!(
            "animate-pulse h-2 bg-white rounded-full dark:bg-black inline-block align-middle {} {}",
            width, prop_class
        )
    }

    fn cell(&self, template_classes: &str) -> String {
        format!("{} {}", "border px-5 py-2", template_classes)
    }
}