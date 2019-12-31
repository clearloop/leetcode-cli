use diesel::prelude::*;
use diesel::sql_query;
fn main() {
    let p = "/Users/mercury/tmp/t.db";
    let conn = SqliteConnection::establish(&p)
        .unwrap_or_else(|_| panic!("Error connecting to {:?}", p));
    let r = sql_query(r#"
      CREATE TABLE posts (
        id INTEGER NOT NULL PRIMARY KEY,
        title VARCHAR NOT NULL,
        body TEXT NOT NULL,
        published BOOLEAN NOT NULL DEFAULT 0
      )
    "#).execute(&conn);
    println!("res: {:?}", r);
}
