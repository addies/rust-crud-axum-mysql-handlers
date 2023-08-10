use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
#[allow(non_snake_case)]
pub struct MyTable {
    pub nomer: i64,
    pub nama: String,
    pub alamat: String,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct MyTableInsert {
    pub nama: String,
    pub alamat: String,
}

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct MyTableUpdate {
    pub nama: String,
    pub alamat: String,
}

