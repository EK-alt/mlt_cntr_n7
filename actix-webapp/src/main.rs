// use mysql::{prelude::Queryable, Pool};
// use std::env;
fn main() {
    println!("Hello, world from Rust!");
    // let DATABASE_URL = env::var("DATABASE_URL").expect("DATABASE_URL environment variable not set");
    // let pool = Pool::new(DATABASE_URL.as_str()).expect("Failed to create a pool");
    // let mut conn = pool.get_conn().expect("Failed to get connection.");
    // let result: Vec<String> = conn
    //     // .query("SELECT CURRENT_TIMESTAMP() AS now")
    //     .query("SELECT DATE_FORMAT(UTC_TIMESTAMP(6), '%Y-%m-%dT%H:%i:%s.%fZ') AS now")
    //     .expect("Query failed");

    // println!("aRes {:?} ", result);
}
