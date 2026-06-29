use sqlx::SqlitePool;

pub async fn connect_pool() -> SqlitePool {
    dotenvy::dotenv().ok();
    let url = std::env::var("DATABASE_URL").expect("DATABASE_URL not set");
    SqlitePool::connect(&url)
        .await
        .expect("Failed to connect to database")
}
