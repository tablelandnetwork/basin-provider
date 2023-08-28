mod crypto;
mod db;
mod publications;

use dotenvy::dotenv;
use sqlx::postgres::PgPool;
use std::env;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().expect(".env file not found");

    let pool = PgPool::connect(&env::var("DATABASE_URL")?).await?;

    let args: Vec<String> = env::args().collect();
    if args.len() >= 2 {
        match &args[1][..] {
            "start" => return publications::listen(pool).await,
            _ => (),
        }
    }

    println!("usage: {} [start] ADDRESS", args[0]);
    Ok(())
}
