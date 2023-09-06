use flexbuffers::Builder;
use sqlite_loadable::prelude::*;
use sqlite_loadable::window::define_window_function_with_aux;
use sqlite_loadable::{api, Result};

use base64::{Engine as _, engine::general_purpose};

pub fn x_step(_context: *mut sqlite3_context, values: &[*mut sqlite3_value], aux: &mut Vec<(String, i64)>) -> Result<()> {
    let string_value = api::value_text(values.get(0).expect("should be text 1"));
    let int_value = api::value_int64(values.get(1).expect("should be int64"));

    aux.push((string_value.expect("should be text 2").to_string(), int_value));

    Ok(())
}

pub fn x_final(context: *mut sqlite3_context, aux: &mut Vec<(String, i64)>) -> Result<()> {
    let mut builder = Builder::default();
    let mut vector_builder = builder.start_vector();

    aux.iter().for_each(|t| {
        let mut map = vector_builder.start_map();
        map.push(&t.0, t.1);
        map.end_map();
    });

    vector_builder.end_vector();

    let buffer = builder.take_buffer();
    
    api::result_text(context, general_purpose::STANDARD_NO_PAD.encode(buffer))?;
    Ok(())
}

#[sqlite_entrypoint]
pub fn sqlite3_flexsqliters_init(db: *mut sqlite3) -> Result<()> {
    let flags = FunctionFlags::UTF8 | FunctionFlags::DETERMINISTIC;

    define_window_function_with_aux(
        db, "flex_string_int", 2, flags,
        x_step, x_final, None, None,
        Vec::<(String, i64)>::new()
    )?;
    Ok(())
}

