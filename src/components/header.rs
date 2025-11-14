use dioxus::prelude::*;

#[component]
pub fn Header() -> Element{
    rsx ! {
        header {
            class: "header",
            nav { 
                ul {
                    li { "Home" }
                    li { "Add weight"  }
                    li { "Table" }
                }
             }
        }

    }
}