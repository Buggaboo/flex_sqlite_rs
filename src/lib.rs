use flexbuffers::Builder;
use sqlite_loadable::errors::{Error, Result};
use sqlite_loadable::prelude::{sqlite3_context, sqlite3_value, sqlite3, sqlite_entrypoint};
use sqlite_loadable::prelude::{c_char, c_uint};
use sqlite_loadable::prelude::register_entrypoint;

use sqlite_loadable::prelude::sqlite3_api_routines;

use sqlite_loadable::window::define_window_function_with_aux;
use sqlite_loadable::{api, FunctionFlags};

use base64::{Engine as _, engine::general_purpose};


#[derive(Debug, Clone)]
struct FlexError;

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


/// General implementation
pub fn x_step_flex(_context: *mut sqlite3_context, values: &[*mut sqlite3_value], aux: &mut Vec<Vec::<*mut sqlite3_value>>) -> Result<()> {
    aux.push(values.to_vec());

    Ok(())
}

/// This will only work if .headers ON
pub fn x_final_flex(context: *mut sqlite3_context, aux: &mut Vec<Vec<*mut sqlite3_value>>) -> Result<()> {

    // first row of values must be all strings
    let first_row = aux.first().expect("first row values");

    let first_row_are_strings = first_row.iter()
        .all(|&ptr|api::value_text_notnull(&ptr).is_ok());

    if !first_row_are_strings {
        return Err(Error::new_message("First rows must be all strings".to_string()));
    }

    let mut builder = Builder::default();
    let mut vector_builder = builder.start_vector();    

    for r in 1..aux.len() {
        let row = aux.get(r).expect("should be a vec");
        for c in 0..row.len() {
            let mut map = vector_builder.start_map();
            let key_result = api::value_text(first_row.get(0).expect("should be text 1"));
            let value_ptr = aux.get(r).expect("should be >1 row").get(c).expect("should be a columnt value");
            let value_type = api::value_type(value_ptr);
            match value_type {
                api::ValueType::Text => 
                    map.push(key_result.expect("should be a key"), api::value_text(value_ptr).expect("should be a string")),
                api::ValueType::Integer =>
                    map.push(key_result.expect("should be a key"), api::value_int64(value_ptr)),
                api::ValueType::Float =>
                    map.push(key_result.expect("should be a key"), api::value_double(value_ptr)),
                api::ValueType::Blob =>
                    map.push(key_result.expect("should be a key"), api::value_blob(value_ptr)),
                api::ValueType::Null =>
                    map.push(key_result.expect("should be a key"), 0),
            }
            map.end_map();
        }
    }

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

    define_window_function_with_aux(
        db, "flex", -1, flags,
        x_step_flex, x_final_flex, None, None,
        Vec::<Vec::<*mut sqlite3_value>>::new()
    )?;

    Ok(())
}

