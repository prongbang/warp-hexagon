use chrono::{DateTime, Utc};
use mobc_postgres::tokio_postgres::Row;
use crate::core::error_postgres::Error::DBQueryError;
use crate::database;
use crate::database::postgres;
use crate::database::postgres::get_db_con;
use crate::todo::domain::model::{Todo, TodoRequest, TodoUpdateRequest};

const TABLE: &str = "todo";
const SELECT_FIELDS: &str = "id, name, created_at, checked";

pub async fn fetch_todos(db_pool: &postgres::DBPool, search: Option<String>) -> database::Result<Vec<Todo>> {
    let con = get_db_con(db_pool).await?;
    let where_clause = match search {
        Some(_) => "WHERE name like $1",
        None => "",
    };
    let query = format!(
        "SELECT {} FROM {} {} ORDER BY created_at DESC",
        SELECT_FIELDS, TABLE, where_clause
    );
    let q = match search {
        Some(v) => con.query(query.as_str(), &[&v]).await,
        None => con.query(query.as_str(), &[]).await,
    };
    let rows = q.map_err(DBQueryError)?;

    Ok(rows.iter().map(|r| row_to_todo(&r)).collect())
}

pub async fn create_todo(db_pool: &postgres::DBPool, body: TodoRequest) -> database::Result<Todo> {
    let con = get_db_con(db_pool).await?;
    let query = format!("INSERT INTO {} (name) VALUES ($1) RETURNING *", TABLE);
    let row = con
        .query_one(query.as_str(), &[&body.name])
        .await
        .map_err(DBQueryError)?;
    Ok(row_to_todo(&row))
}

pub async fn update_todo(db_pool: &postgres::DBPool, id: i32, body: TodoUpdateRequest) -> database::Result<Todo> {
    let con = get_db_con(db_pool).await?;
    let query = format!(
        "UPDATE {} SET name = $1, checked = $2 WHERE id = $3 RETURNING *",
        TABLE
    );
    let row = con
        .query_one(query.as_str(), &[&body.name, &body.checked, &id])
        .await
        .map_err(DBQueryError)?;
    Ok(row_to_todo(&row))
}

pub async fn delete_todo(db_pool: &postgres::DBPool, id: i32) -> database::Result<u64> {
    let con = get_db_con(db_pool).await?;
    let query = format!("DELETE FROM {} WHERE id = $1", TABLE);
    con.execute(query.as_str(), &[&id])
        .await
        .map_err(DBQueryError)
}

fn row_to_todo(row: &Row) -> Todo {
    let id: i32 = row.get(0);
    let name: String = row.get(1);
    let created_at: DateTime<Utc> = row.get(2);
    let checked: bool = row.get(3);
    Todo {
        id,
        name,
        created_at,
        checked,
    }
}