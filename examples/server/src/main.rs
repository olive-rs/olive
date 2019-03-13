#![feature(fn_traits)]
#![feature(unboxed_closures)]

use futures::IntoFuture;

#[macro_use]
extern crate actix_web;

use actix_web::{middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer};

#[get("/resource1/{name}/index.html")]
fn index(req: HttpRequest) -> String {
    println!("REQ: {:?}", req);

    "Mehh".to_string()
}

impl FnOnce<(HttpRequest,)> for index {
    type Output = String;
    extern "rust-call" fn call_once(self, args: (HttpRequest,)) -> String {
        println!("INDEX CALL REQ: {:?}", args.0);

        "FNONCE".to_string()
    }
}

fn index_async(req: HttpRequest) -> impl IntoFuture<Item = String, Error = Error> {
    println!("REQ: {:?}", req);
    Ok(index(req))
}

#[get("/")]
fn no_params() -> &'static str {
    "Hello world!\r\n"
}

fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .middleware(middleware::DefaultHeaders::new().header("X-Version", "0.2"))
            .middleware(middleware::Compress::default())
            .middleware(middleware::Logger::default())
            .service(index)
            .service(no_params)
            .service(
                web::resource("/resource2/index.html")
                    .middleware(
                        middleware::DefaultHeaders::new().header("X-Version-R2", "0.3"),
                    )
                    .default_resource(|r| {
                        r.route(web::route().to(|| HttpResponse::MethodNotAllowed()))
                    })
                    .route(web::get().to_async(index_async)),
            )
            .service(web::resource("/test1.html").to(|| "Test\r\n"))
    })
    .bind("127.0.0.1:8080")?
    .workers(1)
    .run()
}