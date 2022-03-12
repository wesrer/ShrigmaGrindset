use rusqlite::{
    params as sqliteparams, Connection as SqliteConnection, Error as SqliteError, MappedRows,
    Result as SqliteResult,
};
use uuid::Uuid;

use crate::cli::MainCommands;
use crate::data_structures::TaskTypes;
use crate::defaults::{db_name, task_id_tracker_table};

pub fn create_connection() -> SqliteResult<SqliteConnection> {
    SqliteConnection::open(db_name)
}

pub fn add_table(conn: &SqliteConnection, project_name: &str) -> SqliteResult<()> {
    let query_string = format!(
        "
        CREATE TABLE IF NOT EXISTS {} (
            task_id INTEGER NOT NULL,
            task_string TEXT NOT NULL,
            task_type TEXT NOT NULL,
            priority INTEGER NOT NULL,
            uuid STRING PRIMARY KEY
        );
        ",
        project_name,
    );
    conn.execute(query_string.as_str(), [])?;

    let query_string = format!(
        "
        CREATE TABLE IF NOT EXISTS {} (
           task_id INTEGER NOT NULL,
           task_type TEXT NOT NULL,
           project TEXT NOT NULL
        );
        ",
        task_id_tracker_table
    );
    conn.execute(query_string.as_str(), [])?;

    Ok(())
}

pub fn add_available_id(
    conn: &SqliteConnection,
    project_name: &str,
    task_type: TaskTypes,
    task_id: u64,
) -> SqliteResult<()> {
    let query_string = format!(
        "
        INSERT INTO {} (task_id, task_type, project) VALUES (?, ?, ?);
        ",
        task_id_tracker_table
    );
    conn.execute(
        query_string.as_str(),
        sqliteparams![task_id, task_type.to_string(), project_name],
    )?;

    Ok(())
}

pub fn fetch_available_id(
    conn: &SqliteConnection,
    project_name: &str,
    task_type: TaskTypes,
) -> SqliteResult<u64> {
    let fetch_query_string = format!(
        "
        SELECT
            task_id
        FROM
            {}
        WHERE
            task_type=\"{}\" AND
            project=\"{}\"
        ORDER BY
            task_id ASC
        LIMIT
            1;
        ",
        task_id_tracker_table,
        task_type.to_string(),
        project_name
    );
    let mut stmt = conn.prepare(&fetch_query_string)?;

    let mut rows = stmt.query_map([], |row| row.get::<_, u64>(0)).unwrap();

    match rows.next() {
        None => {
            let fetch_last_used_id = format!(
                "
                SELECT
                    task_id
                FROM
                    {}
                WHERE
                    task_type=\"{}\"
                ORDER BY
                    task_id ASC
                LIMIT
                    1;
                ",
                project_name,
                task_type.to_string()
            );

            let mut stmt = conn.prepare(&fetch_last_used_id)?;
            let mut rows = stmt.query_map([], |row| row.get::<_, u64>(0)).unwrap();

            match rows.next() {
                None => Ok(1),
                Some(x) => Ok(x.unwrap() + 1),
            }
        }
        Some(x) => {
            let x = x.unwrap();
            let delete_query_string = format!(
                "
                DELETE FROM
                    {}
                WHERE
                    task_type=\"{}\" AND
                    project=\"{}\" AND
                    task_id={};
                ",
                task_id_tracker_table,
                task_type.to_string(),
                project_name,
                x
            );
            conn.execute(&delete_query_string, [])?;
            Ok(x)
        }
    }
}

pub fn add_task(
    conn: &SqliteConnection,
    project_name: &str,
    task_type: TaskTypes,
    priority: u64,
    task_string: &str,
) -> SqliteResult<()> {
    let uuid = Uuid::new_v4().to_simple().to_string();
    let task_id = 1; // FIXME

    let query_string = format!(
        "
        INSERT INTO {} (task_id, task_string, task_type, priority, uuid) VALUES (?, ?, ?, ?, ?);
        ",
        project_name,
    );
    conn.execute(
        query_string.as_str(),
        sqliteparams![task_id, task_string, task_type.to_string(), priority, uuid],
    )?;

    Ok(())
}

pub fn list_all(conn: &SqliteConnection, table_name: &str) -> SqliteResult<()> {
    let query_string = format!(
        "
        SELECT
            task_string,
            task_type,
            priority
        FROM
            {};
        ",
        table_name
    );
    let mut stmt = conn.prepare(&query_string)?;

    let tasks = stmt
        .query_map([], |row| {
            Ok(MainCommands::Add {
                task: row.get(0)?,
                project: table_name.to_string(),
                tasktype: row.get::<_, String>(1)?.parse().unwrap(),
                priority: row.get::<_, u64>(2)?,
            })
        })
        .unwrap();

    let mut tasks_vec = Vec::new();

    for tas in tasks {
        tasks_vec.push(tas.unwrap());
    }

    dbg!(tasks_vec);
    Ok(())
}
