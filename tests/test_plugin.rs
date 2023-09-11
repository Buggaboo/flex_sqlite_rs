
#[cfg(test)]
mod tests {
    use flex_sqlite_rs::sqlite3_flexsqliters_init;
    use rusqlite::{ffi::sqlite3_auto_extension, Connection};

    #[test]
    fn test_rusqlite_auto_extension() {
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
    fn test_missing_headers() {
        unsafe {
            sqlite3_auto_extension(Some(std::mem::transmute(sqlite3_flexsqliters_init as *const ())));
        }

        let conn = Connection::open_in_memory().unwrap();

        let _ = conn
            .execute("CREATE TABLE t3(x TEXT, y INTEGER)", ());

        let _ = conn
            .execute("INSERT INTO t3 VALUES ('a', 4), ('b', 5), ('c', 3), ('d', 8), ('e', 1)", ());

        let result: String = conn.query_row("SELECT flex(x,y) FROM t3",
            (), |x| x.get(0)).unwrap();

        assert_eq!(result, "YQABAwEBAQQEYgABAwEBAQUEYwABAwEBAQMEZAABAwEBAQgEZQABAwEBAQEEBScfFw8HJCQkJCQKKAE");
    }
}
