use anyhow::{Context, Result};
use rusqlite::{params, Connection};
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize, Debug)]
struct Todo {
    id: i64,
    title: String,
    content: Option<String>,
    public: i64,
    food_orange: i64,
    food_apple: i64,
    food_banana: i64,
    pub_date: Option<String>,
    qty1: i64,
    qty2: i64,
    qty3: i64,
    created_at: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct CreateReq {
    title: String,
    content: Option<String>,
    public: Option<i64>,
    food_orange: Option<i64>,
    food_apple: Option<i64>,
    food_banana: Option<i64>,
    pub_date: Option<String>,
    qty1: Option<i64>,
    qty2: Option<i64>,
    qty3: Option<i64>,
}

fn get_db_path() -> PathBuf {
    PathBuf::from("todo.db")
}

fn open_conn() -> Result<Connection> {
    let path = get_db_path();
    let conn = Connection::open(&path)
        .with_context(|| format!("データベースを開けませんでした: {}", path.display()))?;
    create_table_if_not_exists(&conn)?;
    Ok(conn)
}

fn create_table_if_not_exists(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS todos (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT NOT NULL,
            content TEXT,
            public INTEGER NOT NULL DEFAULT 0,
            food_orange INTEGER DEFAULT 0,
            food_apple INTEGER DEFAULT 0,
            food_banana INTEGER DEFAULT 0,
            pub_date TEXT,
            qty1 INTEGER DEFAULT 0,
            qty2 INTEGER DEFAULT 0,
            qty3 INTEGER DEFAULT 0,
            created_at TEXT DEFAULT (DATETIME('now', 'localtime'))
        );",
    )?;
    Ok(())
}

fn add_todo(conn: &Connection, req: &CreateReq) -> Result<i64> {
    conn.execute(
        "INSERT INTO todos (title, content, public, food_orange, food_apple, food_banana, pub_date, qty1, qty2, qty3, created_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
        params![
            req.title,
            req.content,
            req.public.unwrap_or(0),
            req.food_orange.unwrap_or(0),
            req.food_apple.unwrap_or(0),
            req.food_banana.unwrap_or(0),
            req.pub_date,
            req.qty1.unwrap_or(0),
            req.qty2.unwrap_or(0),
            req.qty3.unwrap_or(0),
            chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        ],
    )?;
    Ok(conn.last_insert_rowid())
}

pub fn add_handler(input: &str) -> std::result::Result<String, Box<dyn std::error::Error>> {
    let req: CreateReq = serde_json::from_str(input)?;

    println!("デコードされた構造体: {:?}", req);

    let conn = open_conn()?;
    let id = add_todo(&conn, &req)?;
    println!("✓ 追加しました (ID: {})", id);

    Ok(json!({ "ret": "OK", "id": id }).to_string())
}

pub fn list_todo_json(show_all: bool) -> std::result::Result<String, Box<dyn std::error::Error>> {
    let conn = open_conn()?;

    let sql = if show_all {
        "SELECT id, title, content, public, food_orange, food_apple, food_banana, pub_date, qty1, qty2, qty3, created_at FROM todos ORDER BY id"
    } else {
        "SELECT id, title, content, public, food_orange, food_apple, food_banana, pub_date, qty1, qty2, qty3, created_at FROM todos WHERE public = 1 ORDER BY id"
    };

    let mut stmt = conn.prepare(sql)?;

    let todos = stmt.query_map([], |row| {
        Ok(Todo {
            id: row.get(0)?,
            title: row.get(1)?,
            content: row.get(2)?,
            public: row.get(3)?,
            food_orange: row.get(4)?,
            food_apple: row.get(5)?,
            food_banana: row.get(6)?,
            pub_date: row.get(7)?,
            qty1: row.get(8)?,
            qty2: row.get(9)?,
            qty3: row.get(10)?,
            created_at: row.get(11)?,
        })
    })?;

    let list: Vec<Todo> = todos.collect::<rusqlite::Result<Vec<_>>>()?;
    if list.is_empty() {
        return Ok("[]".to_string());
    }
    Ok(serde_json::to_string(&list)?)
}

pub fn delete_handler(id: i64) -> std::result::Result<String, Box<dyn std::error::Error>> {
    let conn = open_conn()?;

    let rows = conn.execute("DELETE FROM todos WHERE id = ?1", params![id])?;
    if rows == 0 {
        let msg = format!("ID {} のタスクが見つかりません", id);
        println!("{}", msg);
        return Ok(json!({ "ret": "NG", "message": msg }).to_string());
    }
    println!("✓ 削除しました (ID: {})", id);
    Ok(json!({ "ret": "OK", "id": id }).to_string())
}