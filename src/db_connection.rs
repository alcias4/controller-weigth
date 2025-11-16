


use rusqlite::{Connection, Result, params};
use nanoid::nanoid;

const  PATH_DB: &str = "datos_weight.db";

pub fn create_table() -> Result<()>{

    // Crear o abrir el archivo
    let conn = Connection::open(PATH_DB)?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS tabla_weight (
                id  TEXT PRIMARY KEY,
                day INTERGER NOT NULL,
                peso REAL NOT NULL,
                ejercicio  TEXT NOT NULL
        )"
        
    , [])?;

    Ok(())
}

pub fn new_data(day: u32, weight: f64, ejercice: String) -> Result<()>{
    let conn =  Connection::open(PATH_DB)?;

    let id = nanoid!(4);
    conn.execute(
        "INSERT INTO tabla_weight (id,day,peso, ejercicio) 
            VALUES (?1, ?2, ?3, ?4)
        "
        , params![id,day,weight, ejercice])?;
    Ok(())
}

pub fn reade_data() -> Vec<(String, i32, f64, String)> {
    let conn = Connection::open(PATH_DB).unwrap();

    let mut stmt = conn.prepare(
        "SELECT id, day, peso, ejercicio FROM tabla_weight"
    ).unwrap();

    

    let filas_iter = stmt.query_map([], |fila| {
        Ok((
            fila.get::<_, String>(0)?, // id
            fila.get::<_, i32>(1)?,    // day
            fila.get::<_, f64>(2)?,    // peso
            fila.get::<_, String>(3)?, // ejercicio
        ))
    }).unwrap();

    // Aquí creamos un Vec y lo llenamos
    let mut filas = Vec::new();

    for fila_res in filas_iter {
        let fila = fila_res.unwrap(); // si hay error en una fila, se propaga
        filas.push(fila);
    }
    
    filas
}


pub fn remove_id(id: String) -> Result<usize> {
    let conn = Connection::open(PATH_DB)?;
    
    let del_con =  conn.execute(
        "DELETE FROM tabla_weight WHERE id = ?1",
        [id]
    )?;

    Ok(del_con)
}

pub fn remove_all() -> Result<usize>{
    let conn = Connection::open(PATH_DB)?;

    let filas_elimina = conn.execute(
        "DELETE FROM tabla_weight"
        , [])?;

    Ok(filas_elimina)
}