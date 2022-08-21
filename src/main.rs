#[macro_use]
extern crate diesel;

use actix_web::{get, web, App, HttpServer, Responder};
mod db;
mod model;
mod schema;

use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel::RunQueryDsl;

use crate::model::User;

#[get("/users/{id}")]
async fn get(db: web::Data<db::Pool>, path: web::Path<i32>) -> std::io::Result<impl Responder> {
    let conn = db.get().unwrap();
    let id = path.into_inner();
    let message = schema::user::table
        .select(schema::user::all_columns)
        .filter(schema::user::id.eq(id))
        .load::<User>(&conn)
        .expect("error");

    Ok(web::Json(message))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = db::establish_connection();
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(get)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
