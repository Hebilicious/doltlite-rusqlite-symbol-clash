fn main() {
    // rustc passes the two crates to the linker in an order that follows this
    // source. Here doltlite's archive comes last, so rusqlite's SQLite wins.
    // Swap the two `println!` lines and doltlite's engine wins instead.
    println!("doltlite::version() = {}", doltlite::version());

    #[cfg(feature = "rusqlite")]
    println!("rusqlite::version() = {}", rusqlite::version());

    let db = doltlite::Connection::open_in_memory().expect("open an in-memory database");
    match db.query_row("SELECT dolt_version()", &[]) {
        Ok(row) => println!("SELECT dolt_version() = {:?}", row.as_ref().and_then(|r| r.get(0))),
        Err(error) => {
            println!("SELECT dolt_version() failed: {error}");
            std::process::exit(1);
        }
    }
}
