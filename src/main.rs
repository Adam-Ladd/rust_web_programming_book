use actix_web as aw;
mod views;

#[aw::main]
async fn main() -> std::io::Result<()> {
    aw::HttpServer::new(|| {
        let app = aw::App::new().configure(views::auth::auth_views_factory);
        app
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
