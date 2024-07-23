use std::process::exit;

use sqlx::{Pool, Row, Sqlite, SqlitePool};

pub struct DB{
    pool: Pool<Sqlite>
}

impl DB {
    pub async fn new(db_dir: &str) -> DB{
        
        let pool: Pool<Sqlite> = SqlitePool::connect(&format!("sqlite://./{db_dir}.db")).await.unwrap_or_else(|e: sqlx::Error| {
            eprintln!("Could not connect to database: {e}"); 
            exit(1);
        });

        DB{pool}
    }

    pub async fn verify_user_credentials(&self, username: &str, password: &str) -> bool {
        let query = "SELECT COUNT(*) FROM users WHERE username = ? AND password = ?";
        match sqlx::query(query)
            .bind(username)
            .bind(password)
            .fetch_one(&self.pool)
            .await
        {
            Ok(row) => {
                let count: i64 = row.get(0);
                count > 0
            }
            Err(e) => {
                eprintln!("Database query error: {}", e);
                false
            }
        }
    }

    pub async fn add_user(&self, username: &str, password: &str) -> Result<(), sqlx::Error> {
        let query = "INSERT INTO users (username, password) VALUES (?, ?)";
        sqlx::query(query)
            .bind(username)
            .bind(password)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}