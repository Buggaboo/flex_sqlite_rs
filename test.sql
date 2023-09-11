.load target/debug/libflex_sqlite_rs

PRAGMA foreign_keys=OFF;
BEGIN TRANSACTION;
CREATE TABLE t3(x TEXT, y INTEGER);
INSERT INTO t3 VALUES('a',4);
INSERT INTO t3 VALUES('b',5);
INSERT INTO t3 VALUES('c',3);
INSERT INTO t3 VALUES('d',8);
INSERT INTO t3 VALUES('e',1);
COMMIT;

.headers ON

SELECT flex("x,y", x, y) FROM t3;
