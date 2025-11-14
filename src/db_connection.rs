
use rusqlite::{Connection, Result};


pub fn create_table() -> Result<()>{

    // Crear o abrir el archivo
    let conn = Connection::open("datos_weight.db")?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS tabla_weight (
                id  INTERGER PRIMARY KEY,
                day INTERGER NOT NULL,
                peso REAL NOT NULL,
                ejercicio  INTERGER DEFAULT 0
        )"
        
    , [])?;

    Ok(())
}