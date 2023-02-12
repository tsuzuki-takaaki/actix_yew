use actix_web::{
    get,
    post,
    App,
    HttpServer,
    HttpResponse,
    ResponseError,
    middleware::Logger,
    web
};
use thiserror::Error;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::params;
use serde::{ Serialize, Deserialize };

#[derive(Error, Debug)]
enum MyError {
    #[error("Failed to get connection")]
    ConnectionPoolError(#[from] r2d2::Error),

    #[error("Failed to SQL execution")]
    SQLiteError(#[from] rusqlite::Error),

}
impl ResponseError for MyError {}

// Todo [mkdir for models]
#[derive(Debug, Serialize)]
struct Post {
    id: i32,
    title: String,
    body: String,
}

#[derive(Debug, Deserialize)]
struct NewPost {
    title: String,
    body: String,
}

#[post("/posts")]
async fn add_post(
    form: web::Json<NewPost>, // Deserializeされてる
    db: web::Data<r2d2::Pool<SqliteConnectionManager>>,
) -> Result<HttpResponse, MyError> {
    let conn = db.get()?;

    // print_typename(&form.title);
    // // => actix_web::types::json::Json<actix_api::NewPost>
    // print_typename(&form.title);
    // // => &alloc::string::String

    conn.execute("INSERT INTO post (title, body) VALUES (?1, ?2)", params![form.title, form.body])?;
    Ok(HttpResponse::Accepted().into())
}

#[get("/")]
async fn index(
    db: web::Data<Pool<SqliteConnectionManager>>
) -> Result<web::Json<Vec<Post>>, MyError> {//Result<HttpResponse, MyError> {

    let conn = db.get()?;
    let mut stmt = conn.prepare("SELECT id, title, body FROM post")?;

    // execute SQL from prepared statement
    let rows = stmt.query_map(params![], |row| {
        let id = row.get(0)?;
        let title = row.get(1)?;
        let body = row.get(2)?;
        Ok(Post {id, title, body})
    })?;

    let mut entries = Vec::new();
    for row in rows {
        entries.push(row?);
    }

    println!("{:?}", entries);
    Ok(web::Json(entries))
}

#[actix_web::main]
async fn main() -> Result<(), actix_web::Error>{
    // set up logger
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    // establish connection to db
    let manager = SqliteConnectionManager::file("actix_yew.db");
    let pool = Pool::new(manager).expect("Failed to initialize connection pool");
    let conn = pool
        .get()
        .expect("Failed to connect");

    conn.execute(
        "CREATE TABLE IF NOT EXISTS post(
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT NOT NULL,
            body TEXT NOT NULL
        )",
        params![]
    ).expect("Failed to create table");
    
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .service(index)
            .service(sample)
            .service(add_post)
            .data(pool.clone())
    })
    .bind("127.0.0.1:3000")?
    .run()
    .await?;
    Ok(())
}


// not related
#[get("/sample")]
async fn sample() -> Result<std::string::String, serde_json::Error> {
    let sample = Post {
        id: 1,
        title: "hello".to_string(),
        body: "world".to_string(),
    };
    let serded = serde_json::to_string(&sample);
    serded
}

fn print_typename<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}
