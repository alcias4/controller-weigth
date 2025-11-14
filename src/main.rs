
mod db_connection;
mod components;
use dioxus::prelude::*;
use dioxus::desktop::{Config, WindowBuilder, tao::dpi::LogicalSize};
use components::header::Header;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
//const HEADER_SVG: Asset = asset!("/assets/header.svg");

fn main() {
    let db_result =  db_connection::create_table();
    println!("No se pudo crear la db: {:?}", db_result);
    let cfg = Config::new().with_window(
            WindowBuilder::new()
                .with_title("Resgistros")
                .with_inner_size(LogicalSize::new(500.0, 600.0))
                //.with_min_inner_size(LogicalSize::new(500.0, 300.0))
                .with_resizable(true)
                .with_transparent(true)
                .with_decorations(true),
        ).with_menu(None);
        dioxus::LaunchBuilder::desktop().with_cfg(cfg).launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        Header {  }
        FormRegister {  }
    }
}




#[component]
fn FormRegister() -> Element {
    let mut day = use_signal(|| "".to_string());
    let mut peso = use_signal(|| "".to_string());
    let mut ejer = use_signal(|| "".to_string());
    let mut handleClick = use_signal(|| false); 

    rsx! {
       section { 
            class: "form",
            h2 { "Agregar nuevo dato" }
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

            div { 
                class: "group_input",
                label { "Ingrese si hizo ejericicio " }
                input { 
                    oninput:move |e| ejer.set(e.value()),
                }
            }

            button { 
                onclick: move |_e| handleClick.set(!handleClick()),
                "Guardar info"
            }
        }

        if handleClick() {
            h2 { "dia: {day}, peso: {peso}, ejercicio: {ejer} " }
        }
    }
}