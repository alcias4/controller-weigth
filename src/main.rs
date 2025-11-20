
mod db_connection;
mod components;
mod page;
mod hook;

use dioxus::prelude::*;
use dioxus::desktop::{Config, WindowBuilder, tao::dpi::LogicalSize};
use components::header::Header;
use page::{register::FormRegister, home::Home, information::Information};


use dioxus_material_icons::{
    MaterialIconStylesheet,

};

// styles
const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/css/main.css");
const INFORMATION_CSS: Asset = asset!("/assets/css/informatio.css");
const REGISTER_CSS: Asset = asset!("/assets/css/register.css");

//const HEADER_SVG: Asset = asset!("/assets/header.svg");

fn main() {
    let db_result =  db_connection::create_table();
    println!("No se pudo crear la db: {:?}", db_result);
    let cfg = Config::new().with_window(
            WindowBuilder::new()
                .with_title("Resgistros")
                .with_inner_size(LogicalSize::new(500.0, 500.0))
                .with_min_inner_size(LogicalSize::new(550.0, 600.0))
                .with_resizable(true)
                .with_transparent(true)
                .with_decorations(true),
        ).with_menu(None);
        dioxus::LaunchBuilder::desktop().with_cfg(cfg).launch(App);
}

#[component]
fn App() -> Element {
    let menu = use_signal(|| "register".to_string());
    rsx! {
        
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: INFORMATION_CSS }
        document::Link { rel: "stylesheet", href: REGISTER_CSS }
        MaterialIconStylesheet  {}
        main {         
            Header { menu_str: menu }

            if menu() == "home" {
                Home {  }
            } else if menu() == "register" {
                FormRegister {  }
            } else if menu() == "information" {
                Information {  }
            }
            
        }

    }
}








