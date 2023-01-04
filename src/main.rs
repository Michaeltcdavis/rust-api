
  use actix_web::{get, post, web, App, HttpResponse, HttpServer};
  use std::sync::Mutex;

  struct AppStateWithCounter {
    counter: Mutex<i32>,
  }

  async fn index(data: web::Data<AppStateWithCounter>) -> String {
    let mut counter = data.counter.lock().unwrap();
    *counter += 1; // accessing counter inside mutex guard
      format!("Request Number: {counter}!")
  }

  //this can be moved to a different module
  fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
      web::resource("/test")
        .route(web::get().to(|| async {HttpResponse::Ok().body("test")}))
        .route(web::head().to(HttpResponse::MethodNotAllowed)),
    );
  }

  #[actix_web::main]
  async fn main() -> std::io::Result<()> {
    let counter = web::Data::new(AppStateWithCounter {
      counter: Mutex::new(0),
    });
      HttpServer::new(move || {
          App::new()
            .configure(config)
            .app_data(counter.clone())
            .route("/", web::get().to(index))
      })
      .bind(("127.0.0.1", 8080))?
      .run()
      .await
  }
