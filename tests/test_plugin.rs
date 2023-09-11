
#[cfg(test)]
mod tests {
    use flex_sqlite_rs::sqlite3_flexsqliters_init;
    use rusqlite::{ffi::sqlite3_auto_extension, Connection};
    use flexbuffers::Reader;
    use base64::{Engine as _, engine::general_purpose};

    #[test]
    fn test_specific_flexing_tuples() {
        unsafe {
            sqlite3_auto_extension(Some(std::mem::transmute(sqlite3_flexsqliters_init as *const ())));
        }

        let conn = Connection::open_in_memory().unwrap();

        let _ = conn
            .execute("CREATE TABLE t3(x TEXT, y INTEGER)", ());

        let _ = conn
            .execute("INSERT INTO t3 VALUES ('a', 4), ('b', 5), ('c', 3), ('d', 8), ('e', 1)", ());

        let result: String = conn.query_row("SELECT flex_string_int(x,y) FROM t3",
            (), |x| x.get(0)).unwrap();

        assert_eq!(result, "YQABAwEBAQQEYgABAwEBAQUEYwABAwEBAQMEZAABAwEBAQgEZQABAwEBAQEEBScfFw8HJCQkJCQKKAE");
    }

    #[test]
    fn test_generalized_flexing() {
        unsafe {
            sqlite3_auto_extension(Some(std::mem::transmute(sqlite3_flexsqliters_init as *const ())));
        }

        let conn = Connection::open_in_memory().unwrap();

        let _ = conn.execute(".headers ON", ());

        let _ = conn
            .execute("CREATE TABLE t3(x TEXT, y INTEGER)", ());

        let _ = conn
            .execute("INSERT INTO t3 VALUES ('a', 4), ('b', 5), ('c', 3), ('d', 8), ('e', 1)", ());

        let result: String = conn.query_row("SELECT flex(\"x, y\", x, y) FROM t3",
            (), |x| x.get(0)).unwrap();

        let expected = "eAABZQB5AAIIBAIBAgoBFAQBZQACFRECAQIIARQEAWUAAiIeAgECCAEUBAFlAAIvKwIBAggBFAQBZQACPDgCAQIIARQEBTktIRUJJCQkJCQKKAE";
        assert_eq!(result, expected);

        let buffer = general_purpose::STANDARD_NO_PAD.decode(expected).expect("should be bytes");
        let reader = Reader::get_root(buffer.as_slice()).unwrap();
        let vec = reader.as_vector();
        for i in 0..vec.len() {
            match vec.index(i) {
                Ok(map_key_value) => {
                    let map = map_key_value.as_map();
                    assert_eq!(map.len(), 2);
                },
                Err(_) => {},
            }
        }

    }
}
