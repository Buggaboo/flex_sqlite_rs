use sqlite_loadable::prelude::*;
use sqlite_loadable::window::define_window_function;
use sqlite_loadable::{api, Result};

use base64::{Engine as _, engine::general_purpose};

fn pack_string_and_int(string_value: &str, int_value: i64) -> String {
    // Create a FlexbufferBuilder to build the Flexbuffer.
    let mut builder = flexbuffers::Builder::default();

    // Start the root object.
    let mut map = builder.start_map();

    // Add the string value with a key.
    map.push("string_key", string_value);

    // Add the integer value with a key.
    map.push("int_key", int_value);

    map.end_map();

    let buffer = builder.view();

    // Encode the Flexbuffer data as base64.
    general_purpose::STANDARD_NO_PAD.encode(buffer)
}

pub fn x_step(context: *mut sqlite3_context, values: &[*mut sqlite3_value]) -> Result<()> {
    assert!(values.len() == 1);
    let string_value = api::value_text(values.get(0).expect("should be text 1"));
    let int_value = api::value_int64(values.get(1).expect("should be integer"));
    let _ = api::result_text(context, pack_string_and_int(string_value.expect("should be text 2"), int_value));
    Ok(())
}


pub fn x_final(_: *mut sqlite3_context) -> Result<()> {
    Ok(())
}

#[sqlite_entrypoint]
pub fn sqlite3_flex_init(db: *mut sqlite3) -> Result<()> {
    let flags = FunctionFlags::UTF8 | FunctionFlags::DETERMINISTIC;
    define_window_function(
        db, "flex_string_int", -1, flags,
        x_step, x_final, None, None,
    )?;
    Ok(())
}

