use actix_web::web;

use crate::handlers::{menus, orders};

pub fn configure(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/api")
            .service(
                web::scope("/menus")
                    .route("", web::get().to(menus::get_menus))
                    .route("", web::post().to(menus::create_menu))
                    .route("/{id}", web::get().to(menus::get_menu))
                    .route("/{id}", web::put().to(menus::update_menu))
                    .route("/{id}", web::patch().to(menus::patch_menu))
                    .route("/{id}", web::delete().to(menus::delete_menu)),
            )
            .service(
                web::scope("/orders")
                    .route("", web::get().to(orders::get_orders))
                    .route("", web::post().to(orders::create_order))
                    .route("/{id}", web::get().to(orders::get_order))
                    .route("/{id}", web::put().to(orders::update_order))
                    .route("/{id}", web::patch().to(orders::patch_order))
                    .route("/{id}", web::delete().to(orders::delete_order)),
            ),
    );
}
