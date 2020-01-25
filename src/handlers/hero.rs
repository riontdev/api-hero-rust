use actix_web::{HttpRequest, HttpResponse, web, get, post, put, delete};
use crate::models::hero::Hero;
use crate::models::hero::NewHero;
use crate::api_error::ApiError;
use serde_json::json;


#[get("/heroes")]
async fn index(_req: HttpRequest) -> Result<HttpResponse, ApiError> {
  let heroes = Hero::list()?;
  Ok(HttpResponse::Ok().json(heroes))
}

#[post("/heroes")]
async fn store(hero: web::Json<NewHero>) -> Result<HttpResponse, ApiError> {
  let new_hero = Hero::create(hero.into_inner())?;
   
  Ok(HttpResponse::Ok().json(new_hero))
}

#[get("/heroes/{id}")]
async fn find(path: web::Path<(i32,)>) -> Result<HttpResponse, ApiError> {
    let hero = Hero::find(path.0)?;
    Ok(HttpResponse::Ok().json(hero))

}

#[put("/heroes/{id}")]
async fn update(path: web::Path<(i32,)>, hero: web::Json<NewHero>) -> Result<HttpResponse, ApiError> {
    let hero = Hero::update(path.0, hero.into_inner())?;
    Ok(HttpResponse::Ok().json(hero))
}

#[delete("/heroes/{id}")]
async fn destroy(path: web::Path<(i32,)>) -> Result<HttpResponse, ApiError> {
    let num_deleted = Hero::delete(path.0)?;
    Ok(HttpResponse::Ok().json(json!({ "deleted": num_deleted })))
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
  cfg.service(index);
  cfg.service(store);
  cfg.service(update);
  cfg.service(destroy);
  cfg.service(find);
}