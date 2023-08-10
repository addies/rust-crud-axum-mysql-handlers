use crate::model::{MyTable, MyTableInsert, MyTableUpdate};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json
};
use serde_json::{json, Value};
use sqlx::mysql::MySqlPool;


pub async fn health_checker_handler() -> impl IntoResponse {
    const MESSAGE: &str = "Simple CRUD API with Rust, SQLX, MySQL,and Axum";
    let json_response = serde_json::json!({
        "status": "success",
        "message": MESSAGE
    });

    Json(json_response)
}

pub async fn getall_mytable (State(pool):State<MySqlPool>) 
    -> Result<Json<Vec<MyTable>>, (StatusCode, String)> {
    let string_query = "SELECT * FROM mytable";
    let result = sqlx::query_as(string_query)
        .fetch_all(&pool)
        .await
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, format!("Error is: {}", err)))?;

    Ok(Json(result))    
}

pub async fn get_mytable (State(pool):State<MySqlPool>, Path(nomer): Path<i64>) 
    -> Result<Json<Vec<MyTable>>, (StatusCode, String)> {
    let string_query = "SELECT * FROM mytable WHERE nomer = $1";
    let result = sqlx::query_as(string_query)
        .bind(nomer)
        .fetch_all(&pool)
        .await
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, format!("Error is: {}", err)))?;

    Ok(Json(result))   
}

pub async fn create_mytable( State(pool):State<MySqlPool>, Json(my_table):Json<MyTableInsert>) 
    -> Result<Json<Value>, (StatusCode, String)> {
    let string_query = "INSERT INTO mytable (nama,alamat) VALUES (?, ?)";
    let _result = sqlx::query(string_query)
        .bind(&my_table.nama.to_string())
        .bind(&my_table.alamat.to_string())
        .execute(&pool)
        .await
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, format!("Error is: {}", err)))?;

    Ok(Json(json!({"msg": "Data inserted successfully"})))
}

pub async fn delete_mytable (State(pool):State<MySqlPool>, Path(nomer): Path<i64>) 
    -> Result<Json<Vec<MyTable>>, (StatusCode, String)> {
    let string_query = "DELETE FROM mytable WHERE nomer = ?";
    let result = sqlx::query_as(string_query)
        .bind(nomer)
        .fetch_all(&pool)
        .await
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, format!("Error is: {}", err)))?;

    Ok(Json(result))   
}

pub async fn update_mytable( State(pool):State<MySqlPool>, Path(nomer): Path<i64>, Json(my_table):Json<MyTableUpdate>) 
    -> Result<Json<Value>, (StatusCode, String)> {
    let string_query = "UPDATE mytable SET nama = ?, alamat = ? WHERE nomer = ?";
    let _result = sqlx::query(string_query)
        .bind(&my_table.nama.to_string())
        .bind(&my_table.alamat.to_string())
        .bind(nomer)
        .execute(&pool)
        .await
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, format!("Error is: {}", err)))?;

    Ok(Json(json!({"msg": "Data updated successfully"})))
}
