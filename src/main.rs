#[actix_web::main]
async fn main() -> std::io::Result<()> {
     if std::env::var_os("RUST_LOG").is_none() {
        unsafe{
            std::env::set_var("RUST_LOG", "actix_web=info");
        }
    }

    dotenv().ok();
    env_logger::init();
}


/*
setup db
-setup db pool

decalre routes
declare cors middleware

start listening for requests in port x

*/ 