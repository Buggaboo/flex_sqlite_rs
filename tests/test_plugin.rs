
#[cfg(test)]
mod tests {
    use flex_sqlite_rs::sqlite3_flex_init;
    use rusqlite::{ffi::sqlite3_auto_extension, Connection};

    #[test]
    fn test_rusqlite_auto_extension() {
        unsafe {
            sqlite3_auto_extension(Some(std::mem::transmute(sqlite3_flex_init as *const ())));
        }

        let conn = Connection::open_in_memory().unwrap();

        let _ = conn
            .execute("CREATE TABLE t3(x TEXT, y INTEGER)", ());

        let _ = conn
            .execute("INSERT INTO t3 VALUES ('a', 4), ('b', 5), ('c', 3), ('d', 8), ('e', 1)", ());

        let result: String = conn.query_row("SELECT flex(*) 
          FROM t3 ORDER BY x", (), |x| x.get(0)).unwrap();

        // assert_eq!(result, 9);
    }
}