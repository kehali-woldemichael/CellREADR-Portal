// external 
use leptos::*;

#[component]
pub fn WelcomePage() -> impl IntoView {
    use crate::components::general::banner::Banner;

    view! {
        <Banner current_page="welcome".to_string()/>

        //<div class="border p-10 flex items-center"> 
        <div class="grid grid-cols-2 mt-10 mr-10 mb-20"> 
            <div class="border mx-auto">
                <div class="mt-5 ml-5">
                    <div>
                        <h1 class="mx-auto font-bold text-xl">"Welcome to CellREADR Portal!"</h1>
                    </div>

                    <div class="mt-2">
                        <p>"CellREADR allows you to design simple RNA molecules capable of detecting specific RNA molecules"</p> 
                        <p>"and selectively translating a protein effector in cells expressing that particular RNA transcript."</p>
                        <p>"This portal serves as a resource for implementing the technology tailored for your needs."</p>
                    </div>
                </div>

                <div class="items-center mx-auto">
                    <img src="/assets/images/welcome/CellREADR_Main_Schematic.png" alt="central_schematic"/>
                </div>
            </div>


            <div class="border">
                <EntryForm/>
            </div>
        </div>
    }
}

#[component]
pub fn EntryForm() -> impl IntoView {
    // components
    use crate::components::sesrna::define_target_sequence::EntryType;
    use crate::components::sesrna::enter_sesrna_parameters::EnterSesrnaParameters;
    use crate::components::sesrna::visualize_sesrna::VisualizeSesrna;
    // structs
    use crate::structs::sesrna::SesRNA;


    let entry_type_rw: RwSignal<std::option::Option<String>> = create_rw_signal(None);
    let target_sequence_rw: RwSignal<std::option::Option<String>> = create_rw_signal(None);
    let submit_target_sequence_rw: RwSignal<bool> = create_rw_signal(false);

    let submit_sesrna_parameters_rw: RwSignal<bool> = create_rw_signal(false);
    let vec_sesrna_parameters: RwSignal<std::option::Option<Vec<String>>> = create_rw_signal(None);
    let vec_sesrnas_rw: RwSignal<std::option::Option<Vec<SesRNA>>> = create_rw_signal(None);

    view! {
        <div class="flex items-center mt-5"> 
            <h1 class="mx-auto font-bold text-xl">"SesRNA Design Tool"</h1>
        </div> 

        <div class="grid grid-cols-2 mt-5">
            <div>
                <div class="flex mb-2">
                    <h2 class="font-bold mx-auto">"Step 1: Define target sequence"</h2>
                </div>

                <div>
                    <div class="mb-2">
                        <EntryType entry_type_rw=entry_type_rw target_sequence_rw=target_sequence_rw
                                submit_target_sequence_rw=submit_target_sequence_rw 
                        />
                    </div>
                </div>
            </div>

            <div class="">
                <div class="flex mb-2">
                    <h2 class="font-bold mx-auto">"Step 2: Enter sesRNA parameters"</h2>
                </div>

                <div>
                    <EnterSesrnaParameters entry_type_rw=entry_type_rw target_sequence_rw=target_sequence_rw
                                            submit_target_sequence_rw=submit_target_sequence_rw 
                                            submit_sesrna_parameters_rw=submit_sesrna_parameters_rw
                                            vec_sesrna_parameters=vec_sesrna_parameters
                    />
                </div>
            </div>
        </div>


        <div class="border-t mt-10">
            <div class="flex mb-2 mt-2">
                <h2 class="font-bold mx-auto">"Step 3: Analyze sesRNA candidates"</h2>
            </div>

            <div>
                <VisualizeSesrna submit_sesrna_parameters_rw=submit_sesrna_parameters_rw
                    target_sequence_rw=target_sequence_rw
                    vec_sesrna_parameters=vec_sesrna_parameters
                    vec_sesrnas_rw=vec_sesrnas_rw
                />

            </div>
        </div>
    }
}