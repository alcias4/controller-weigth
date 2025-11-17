use dioxus::prelude::*;
use dioxus_material_icons::{MaterialIcon};


#[component]
pub fn Header(menu_str: Signal<String>) -> Element{
    rsx ! {
        header {
            class: "header",
            nav { 
                ul {
                    // li { onclick: move |_e| menu_str.set("home".to_string()), "Home" }
                    li { 
                        onclick: move |_e| menu_str.set("register".to_string()), 
                        MaterialIcon {
                            name: "book",
                        },
                        "Register" }
                    li { 
                        onclick: move |_e| menu_str.set("information".to_string()), 
                        MaterialIcon {
                            name: "article",
                        },
                        "Information" }

                }
             }
        }

    }
}