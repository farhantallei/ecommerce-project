use std::{env, thread};
use std::sync::mpsc;
use std::time::{Duration, Instant};
use dotenv::dotenv;
use crate::infrastructure::db::connection::establish_connection;

#[test]
fn test_establish_connection() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    println!("Connecting to database...");
    let start_time = Instant::now();

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        establish_connection(&database_url);
        tx.send(()).unwrap(); // Kirim sinyal selesai
    });

    let timeout = Duration::from_secs(30);

    match rx.recv_timeout(timeout) {
        Ok(_) => {
            println!("Connected to database.");
            let elapsed = start_time.elapsed();
            println!("Done waiting in {:?}", elapsed);
            assert!(elapsed < timeout, "Wait took too long: {:?}", elapsed);
        }
        Err(_) => {
            let elapsed = start_time.elapsed();
            panic!("Operation timed out after {:?}", elapsed);
        }
    }
}