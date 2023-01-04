
  use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

  async fn index() -> impl Responder {
      HttpResponse::Ok().body("Hello world!")
  }

  #[post("/echo")]
  async fn echo(req_body: String) -> impl Responder {
      HttpResponse::Ok().body(req_body)
  }

  async fn manual_hello() -> impl Responder {
      HttpResponse::Ok().body("Hey there!")
  }

  #[actix_web::main]
  async fn main() -> std::io::Result<()> {
      HttpServer::new(|| {
          App::new()
            .service(echo)
            .service(
                web::scope("/app")
                    .route("/index.html", web::get().to(index))
            )
      })
      .bind(("127.0.0.1", 8080))?
      .run()
      .await
  }
