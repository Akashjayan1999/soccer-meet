use actix_web::{http::header, web, App, HttpServer};
use dotenv::dotenv;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

#[actix_web::main]
async fn main(){
     if std::env::var_os("RUST_LOG").is_none() {
        unsafe{
            std::env::set_var("RUST_LOG", "actix_web=info");
        }
    }

    dotenv().ok();
    env_logger::init();

     let database_url: String = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
      let pool:Pool<Postgres> = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
    {
        Ok(pool) => {
            println!("Connection to the db is successful!");
            pool
        }
        Err(err) => {
            println!("Failed to connect to the database {:?}", err);
            std::process::exit(1);
        }
    };

    HttpServer::new(move || {
        let cors = actix_cors::Cors::default()
        .allowed_origin("http://localhost:3000")
        .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
        .allowed_headers(vec![header::CONTENT_TYPE,header::AUTHORIZATION, header::ACCEPT])
        .supports_credentials();
        App::new()
            .wrap(cors)
            .app_data(web::Data::new(AppState {db: pool.clone()}))

            // .configure(routes::routes)
    })
    .bind(("127.0.0.1", 8080))
    .unwrap()
    .run()
    .await
    .unwrap();
}

struct AppState {
    db: Pool<Postgres>
}
/*
setup db
-setup db pool

decalre routes
declare cors middleware

start listening for requests in port x

*/ 