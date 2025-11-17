
use dioxus::prelude::*;

use crate::db_connection;
use dioxus_material_icons::{MaterialIcon,MaterialIconColor};


#[component]
pub fn FormRegister() -> Element {
    let mut day = use_signal(|| "".to_string());
    let mut peso = use_signal(|| "".to_string());
    let mut ejer = use_signal(|| 0u8);
    

    let insert_new =move |_e: Event<MouseData>| {

        if day() != "" && peso() != ""  {
            let day: u32 =  day().trim().parse().expect("Error parse day");
            let peso: f64 = peso().trim().parse().expect("Error parse peso");
            let mut ejercicio  = "No";


            if ejer() == 1 {
                ejercicio = "Si"
            }

            let result  = db_connection::new_data(day, peso, ejercicio.to_string());
            println!("result: {:?}", result);
        } 
    };

    rsx! {
       section { 
            class: "form",
            h2 { "Agregar nuevo dato" }

            section { 
                class: "inputs_seccton",
                div { 
                    class: "group_input",
                    label { "Ingrese dia" }
                    input { 
                        oninput:move |e| day.set(e.value()),
                }
            }

                div { 
                    class: "group_input",
                    label { "Ingrese peso" }
                    input { 
                        oninput:move |e| peso.set(e.value()),
                    }
                }
            }

            div { 
                class: "group_radio",
                label { "Ejercicio" }
                section { 
                    div { 
                        class: "radio",
                        span { 
                            class: if ejer() == 1 { "radio_input span_click"} else {"radio_input span_no_click"} ,
                            onclick: move |_| ejer.set(1) 
                        
                        }
                        "si" 
                    }
                    div { 
                        class: "radio" ,
                        span { 
                            class: if ejer() == 0 {"radio_input span_click"} else {"radio_input span_no_click"},
                            onclick: move |_| ejer.set(0) 
                        }
                        "no"
                    }
                }
            }

            button { 
                class: "btn_guardar",
                onclick:insert_new,
                MaterialIcon {
                    name: "save",
                    color: MaterialIconColor::Light
                }
                "Guardar info"
            }
        }


    }
}