// use dotenv::dotenv;
// use surrealdb::engine::remote::ws::{Client, Ws};
// use surrealdb::opt::auth::Root;
// use surrealdb::{Result, Surreal};
// use std::env;
// use std::path::PathBuf;

// async fn init_db() -> Result<Surreal<Client>> {
//     println!("Loading .env file...");
//     let current_dir = std::env::current_dir().unwrap();
//     println!("Current directory: {:?}", current_dir);
    
//     // Try to load .env file from current directory
//     let env_path = current_dir.join(".env");
//     println!("Looking for .env file at: {:?}", env_path);
    
//     if env_path.exists() {
//         println!("Found .env file");
//         dotenv::from_path(env_path).ok();
//     } else {
//         println!("No .env file found at {:?}", env_path);
//     }
    
//     println!("Checking environment variables...");
    
//     let db_user = match env::var("SURREALDB_USER") {
//         Ok(val) => {
//             println!("Found SURREALDB_USER: {}", val);
//             val
//         },
//         Err(e) => {
//             println!("Error loading SURREALDB_USER: {}", e);
//             panic!("SURREALDB_USER must be set: {}", e);
//         }
//     };
    
//     let db_pass = match env::var("SURREALDB_PASS") {
//         Ok(val) => {
//             println!("Found SURREALDB_PASS: {}", val);
//             val
//         },
//         Err(e) => {
//             println!("Error loading SURREALDB_PASS: {}", e);
//             panic!("SURREALDB_PASS must be set: {}", e);
//         }
//     };

//     let db = Surreal::new::<Ws>("127.0.0.1:8000").await?;
    
//     // Authenticate
//     db.signin(Root {
//         username: &db_user,
//         password: &db_pass,
//     })
//     .await?;

//     // Select namespace and database
//     db.use_ns("testign").use_db("testing").await?;

//     // Test query to verify connection
//     let version = db.version().await?;
//     println!("Successfully connected to SurrealDB version: {}", version);
    
//     Ok(db)
// }

// #[tokio::main]
// async fn main() -> Result<()> {
//     let db = init_db().await?;
//     println!("Database connection test successful!");
//     Ok(())
// }



use serde::{Deserialize, Serialize};
use surrealdb::engine::any;
use surrealdb::opt::auth::Root;
use tokio;

#[derive(Debug, Serialize, Deserialize)]
struct Person {
    name: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Open a connection
    let db = any::connect("wss://tenantship-.aws-euw1.surreal.cloud").await?;

    // Select a namespace and database
    db.use_ns("").use_db("").await?;

    // Authenticate
    db.signin(Root {
        username: "",
        password: "",
    }).await?;

	db.query("CREATE person:john SET name = 'John Doe', age = 25").await?.check()?;

	// Query that person
	let john: Option<Person> = db.select(("person", "john")).await?;
	dbg!(john);

  Ok(())
}
