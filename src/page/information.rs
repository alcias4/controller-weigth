

use dioxus::prelude::*;
use crate::db_connection;
use crate::hook::use_read;

#[component]
pub fn Information()-> Element {
    let mut data  = use_read::use_reade();

    let all_remove = move |_: Event<MouseData>|  {
       let res=  db_connection::remove_all();

       match res {
           Ok(filas) => {
            println!("Número de registro eliminado: {}", filas);
            let mut data_tem =  data();
            data_tem.clear();

            data.set(data_tem);
           },
           Err(err) => println!("Error en {}", err)
       }
    };
    rsx! {
        section {
            class: "seccton_info",
            div { 
                h1 { "Información" }
                button { 
                    onclick: all_remove,
                    "Eliminar todo" 
                }
            }
            table { 

                thead {
                    tr {
                        th { "Día" }
                        th { "Peso" }
                        th { "Ejercicio" }
                        th { "Funciones" }
                    }
                }
                tbody { 

                    for (id, day, peso, ejercicio) in data.read().iter() {
                        
                        tr { 
                            key: "{id}",
                            td { "{day}" }
                            td { "{peso}" }
                            td { "{ejercicio}" }
                            td { FunTable { delete_id: id, data_singla: data }}
                        }
                    }
                }
                
            }

            div { 
                class: "grafic",
                GraficInfo { single_data: data }
            }
        }
    }
}


#[component]
fn FunTable(delete_id: String, data_singla:Signal<Vec<(String, i32, f64, String)>> ) -> Element{

    
    let del_id = move |_| {
        let remove = db_connection::remove_id(delete_id.clone()).unwrap();


        if remove == 1 {
            println!("Se elimino el registro{}", remove);

            let mut data_tem  = data_singla();
            
            if let Some(pos) = data_tem.iter().position(|(id, _day,_n,_b)| *id == delete_id) {
                data_tem.remove(pos);
            }

            data_singla.set(data_tem);

        }

        
        
    };
    rsx! {
        div { 
            button { 
                onclick: del_id,
                "Eliminar" 
            }
            button {  "Cambiar" }
        }
    }
}


#[component]
fn GraficInfo(single_data: Signal<Vec<(String, i32, f64, String)>>) -> Element {

    let datos = use_signal(|| {
        let data = single_data();

        
        let mut datos = String::new();


        for (_, day, peso, _) in data.into_iter() {
            if day == 0 && peso == 0.0 {
                continue;
            }
            let tem = format!("{day},{peso} ");

            datos += &tem;
        }

        datos
    });
    let width: i32 = 400;
    let height: i32 = 200;
    let padding: i32 = 20; // margen


 


    rsx! {
        p { "{datos()}" }
        svg {
            width: "400",
            height: "200",
            style: "border: 1px solid red;",
            polyline {
                fill: "none",
                stroke: "white",
                "stroke-width": "3",
                points: "10,150 100,100 200,50 300,120",
            }
        } 
    }
}