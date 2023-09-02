mod login;
mod logout;

use actix_web as aw;

pub fn auth_views_factory(app: &mut aw::web::ServiceConfig) {
    app.service(
        aw::web::scope("v1/auth")
            .route("login", aw::web::get().to(login::login))
            .route("logout", aw::web::get().to(logout::logout)),
    );
}