use dioxus::prelude::*;


#[component]
pub fn Home()-> Element{
    rsx! {
        h1 { "Hola soy el home" }
    }
}