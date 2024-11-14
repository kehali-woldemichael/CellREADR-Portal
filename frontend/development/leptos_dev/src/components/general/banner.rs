use leptos::*;
#[component]
pub fn Banner(current_page: String) -> impl IntoView {

    view!{
        // Banner
        <nav class="bg-white border-gray-200 dark:bg-gray-900" aria-label="Main Navigation">
            <div class="pt-5 h-5 pl-5">
                <h1 class="text-white dark:text-white font-bold text-xl">"CellREADR Portal"</h1>
            </div>
        </nav>

        <nav class="bg-white border-gray-200 dark:bg-gray-900 pt-5 pl-10" aria-label="Main Navigation">
            <div class="flex flex-wrap items-center justify-between mx-auto p-4">
                <div class="hidden w-full md:block md:w-auto" id="navbar-default">
                    <ul class="font-medium flex flex-col p-4 md:p-0 mt-4 border border-gray-100 rounded-lg bg-gray-50 md:flex-row md:space-x-8 rtl:space-x-reverse md:mt-0 md:border-0 md:bg-white dark:bg-gray-800 md:dark:bg-gray-900 dark:border-gray-700">
                        <li>
                            <a href="/welcome" class=assign_color_selected(current_page == "welcome".to_string())>Welcome</a>
                        </li>
                        <li>
                            <a href="/" class=assign_color_selected(current_page == "sesrna".to_string())>sesRNA</a>
                        </li>
                        <li>
                            <a href="/help" class=assign_color_selected(current_page == "help".to_string())>Help</a>
                        </li>
                    </ul>
                </div>
            </div>
        </nav>
    }
}

pub fn assign_color_selected(bool_current_page: bool) -> String {
    match bool_current_page {
        true => return "block py-2 px-3 text-white bg-blue-700 rounded md:bg-transparent md:text-blue-700 md:p-0 dark:text-white md:dark:text-blue-500".to_string(),
        false => return "block py-2 px-3 text-gray-900 rounded hover:bg-gray-100 md:hover:bg-transparent md:border-0 md:hover:text-blue-700 md:p-0 dark:text-white md:dark:hover:text-blue-500 dark:hover:bg-gray-700 dark:hover:text-white md:dark:hover:bg-transparent".to_string(),
    }
}