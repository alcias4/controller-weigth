use dioxus::prelude::*;

use crate::db_connection;

pub fn use_reade() -> Signal<Vec<(String, i32, f64, String)>>{

    let mut data = use_signal(|| vec![("".to_string(), 0, 0.0, "".to_string())]);

    use_effect(move ||  {
        let restl = db_connection::reade_data() ;

        data.set(restl);
    });

    data
    
}