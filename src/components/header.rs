use dioxus::prelude::*;

#[component]
pub fn Header(menu_str: Signal<String>) -> Element{
    rsx ! {
        header {
            class: "header",
            nav { 
                ul {
                    li { onclick: move |_e| menu_str.set("home".to_string()), "Home" }
                    li { onclick: move |_e| menu_str.set("register".to_string()), "Register" }
                    li { onclick: move |_e| menu_str.set("information".to_string()), "Information" }

                }
             }
        }

    }
}