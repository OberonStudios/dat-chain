use actix_web::{web, App, HttpResponse, HttpServer, Responder};

async fn index() -> impl Responder {
    let response = "<div id=\"app\"><div><script>fetch('/api').then(res => res.json()).then(res => document.getElementById(\"app\").innerHTML = res.status)</script>";
    HttpResponse::Ok().body(response)
}

async fn api() -> impl Responder {
    HttpResponse::Ok().body("{\"status\":\"running\"}")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/api", web::get().to(api))
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}
