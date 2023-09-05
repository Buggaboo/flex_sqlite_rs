use flexbuffers::Builder;
use sqlite_loadable::prelude::*;
use sqlite_loadable::window::define_window_function_with_aux;
use sqlite_loadable::{api, Result};

use base64::{Engine as _, engine::general_purpose};

impl Aux {
    pub fn pack_string_and_int(&mut self, string_value: &str, int_value: i64) {
        // Start the root object.
        let mut map = self.builder.start_map();
    
        // Add the string value with a key.
        map.push("string_key", string_value);
    
        // Add the integer value with a key.
        map.push("int_key", int_value);
    
        map.end_map();
    
        self.base64 = Self::vec_to_base64(self.builder.take_buffer());
        
        // debugging
        // println!("{}", self.base64);
    }

    fn vec_to_base64(v: Vec<u8>) -> String {
        general_purpose::STANDARD_NO_PAD.encode(v.as_slice())
    }
}

pub fn x_step(context: *mut sqlite3_context, values: &[*mut sqlite3_value], aux: &mut Aux) -> Result<()> {
    if values.len() != 2 {
        return Ok(());
    }

    let string_value = api::value_text(values.get(0).expect("should be text"));
    let int_value = api::value_int64(values.get(1).expect("should be int64"));

    aux.pack_string_and_int(string_value.expect("should be string"), int_value);


    Ok(())
}

pub fn x_value(context: *mut sqlite3_context, aux: &mut Aux) -> Result<()> {
    let _ = api::result_text(context, aux.base64.as_str());
    Ok(())
}

pub fn x_final(_context: *mut sqlite3_context, _aux: &mut Aux) -> Result<()> {
    Ok(())
}

pub struct Aux {
    builder: Builder,
    base64: String,
}


#[sqlite_entrypoint]
pub fn sqlite3_flexsqliters_init(db: *mut sqlite3) -> Result<()> {
    let flags = FunctionFlags::UTF8 | FunctionFlags::DETERMINISTIC;

    define_window_function_with_aux(
        db, "flex_string_int", 2, flags,
        x_step, x_final, Some(x_value), None,
        Aux { builder: Builder::default(), base64: String::new() }
    )?;
    Ok(())
}

