use rusqlite::{Connection as SqliteConnection, Result as SqliteResult};

fn create_connection() -> SqliteResult<SqliteConnection> {
    SqliteConnection::open("Shrigma.db")
}

fn add_table(conn: SqliteConnection, table_name: &str) -> SqliteResult<()> {
    let query_string = format!(
        "
        CREATE TABLE IF NOT EXISTS {}
        ",
        table_name
    );
    // conn.execute()
    Ok(())
}

fn add_task(conn: SqliteConnection) -> SqliteResult<()> {
    let query_string = format!(
        "

        "
    );
    // conn.execute

    todo!();
}
