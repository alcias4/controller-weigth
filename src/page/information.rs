


use std::f64;

use crate::db_connection;
use crate::hook::use_read;

use dioxus::prelude::*;
use plotters::prelude::*;
use image::{RgbImage, ImageFormat};
use base64::engine::general_purpose;
use base64::Engine as _;

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

                    if data().len() > 0 { 
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

    let mut poinst: Signal<Vec<(f64, f64)>> = use_signal(|| Vec::new());

    let mut valore =use_signal(|| (0.0,0.0, 0.0, 0.0) );
        
    {
        let data = single_data();
        if data.len() > 0 {
            
            let mut point: Vec<(f64, f64)> =  Vec::new();

            let mut max_x =  Vec::new();

            let mut max_y =  Vec::new();


            for (_, day,peso, _) in data {
                point.push((day as f64,peso));
                max_x.push(day);
                max_y.push(peso as i32);

            }

            let mx_x = max_x.iter().max().unwrap();
            let mx_y = max_y.iter().max().unwrap();

            let min_x = max_x.iter().min().unwrap();
            let min_y = max_y.iter().min().unwrap();


            let va = ((*mx_x) as f64,(*mx_y) as f64, (*min_x) as f64,(*min_y) as f64) ;

            valore.set(va);
            poinst.set(point);
        }
    };

    
    let img_src = format!("data:image/png;base64,{}", make_chart_base64(poinst(), valore()));
    rsx! {
        div {
            class: "grafica_content",

            if single_data().len() > 0 {
                h3 { "Gráfico dia vs peso" }
                img {
                    src: "{img_src}",
                    alt: "Gráfico de prueba",
                    style: "border: 1px solid #ccc;"
                }
            }

        }
    }
}



fn make_chart_base64(points: Vec<(f64, f64)>, scala: (f64,f64, f64, f64)) -> String{

    println!("{:?}", points.len());
    if points.len() <= 0 {
        return "".to_string();
    }
    let width: u32 = 400;
    let height: u32 = 300;

    // Buffer RGB plano: width * height * 3
    let mut buffer = vec![0u8; (width * height * 3) as usize];

    {
        // 1) Dibujar con Plotters dentro del buffer
        let root = BitMapBackend::with_buffer(&mut buffer, (width, height))
            .into_drawing_area();

        root.fill(&WHITE).expect("Error fill color");

        // *** OJO AQUÍ: ejes en f64, igual que tus puntos ***
        let mut chart = ChartBuilder::on(&root)
            .margin(10)
            .set_label_area_size(LabelAreaPosition::Left, 30)
            .set_label_area_size(LabelAreaPosition::Bottom, 30)
            .build_cartesian_2d(scala.2..scala.0, scala.3..scala.1)
            .expect("Error char create");

        chart.configure_mesh().draw().expect("Error configure char");

        // Línea y = x
        chart
            .draw_series(LineSeries::new(points, &RED))
            .expect("Errro char contex");

        root.present().expect("Error expect root char");
    }

    // 2) Pasar el buffer RGB a una RgbImage
    let image = RgbImage::from_raw(width, height, buffer).expect("buffer -> imagen");

    // 3) Codificar la imagen como PNG en memoria (Vec<u8>)
    let mut png_bytes: Vec<u8> = Vec::new();
    image
        .write_to(
            &mut std::io::Cursor::new(&mut png_bytes),
            ImageFormat::Png, // <- AQUÍ el cambio: ya no ImageOutputFormat
        )
        .expect("Error image bytes");

    // 4) Convertir PNG (bytes) a base64
    general_purpose::STANDARD.encode(&png_bytes)
}