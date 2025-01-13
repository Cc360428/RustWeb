use actix_web::{web, HttpResponse, Responder};

async fn product_add_platform() -> impl Responder {
    HttpResponse::Ok().body("Add platform")
}

async fn product_platform_list() -> impl Responder {
    HttpResponse::Ok().body("Platform list")
}

async fn product_add_game() -> impl Responder {
    HttpResponse::Ok().body("Add game")
}

async fn product_game_list() -> impl Responder {
    HttpResponse::Ok().body("Game list")
}

async fn release_create() -> impl Responder {
    HttpResponse::Ok().body("Create release")
}

async fn get_release_list() -> impl Responder {
    HttpResponse::Ok().body("Release list")
}

/// Initializes the product module routes
pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg
        .service(
            web::scope("/product")
                .route("/add/platform", web::post().to(product_add_platform))
                .route("/platform/list", web::get().to(product_platform_list))
                .route("/add/game", web::post().to(product_add_game))
                .route("/game/list", web::get().to(product_game_list))
                .route("/release/create", web::post().to(release_create))
                .route("/release/list", web::get().to(get_release_list)),
        );
}
