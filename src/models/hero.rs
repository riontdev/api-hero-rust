 
use serde::{Serialize, Deserialize};
//use actix_web::web::Data;
//use actix_web::Responder;
//use actix_web::HttpResponse;


//use diesel::pg::PgConnection;
use diesel::prelude::*;
use crate::schema::hero;
use crate::db;
use diesel;
use crate::api_error::ApiError;
use chrono::{NaiveDateTime, Utc};
//use uuid::Uuid;

#[derive(AsChangeset, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "hero"]
pub struct Hero {
	pub id: i32,
	pub name: String,
	pub identity: String,
	pub hometown: String,
	pub age: i32,
	pub created_at: NaiveDateTime,
	pub updated_at: Option<NaiveDateTime>,
	//pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Serialize, Deserialize, AsChangeset, Insertable)]
#[table_name = "hero"]
pub struct NewHero {
	pub name: String,
	pub identity: String,
	pub hometown: String,
	pub age: i32,
}

impl Hero {
	pub fn list() -> Result<Vec<Self>, ApiError> {
		let conn = db::connection()?;
		
		//get all heroes
		let heroes = hero::table
			.order(hero::created_at.desc())
	  		.load::<Hero>(&conn)?;
		
		Ok(heroes)
	}

	pub fn find(id: i32) -> Result<Self, ApiError> {
        let conn = db::connection()?;

		//get hero by id
        let heroes = hero::table
            .filter(hero::id.eq(id))
            .first::<Hero>(&conn)?;

        Ok(heroes)
	}
	
	pub fn create(hero_data: NewHero) -> Result<Self, ApiError> {
		let conn = db::connection()?;

		//let new_hero = Hero::from(hero);
		//insert new hero
        let heroes = diesel::insert_into(hero::table)
            .values(hero_data)
            .get_result(&conn)?;

        Ok(heroes)
	}

	pub fn update(id: i32, hero_data: NewHero) -> Result<Self, ApiError> {
        let conn = db::connection()?;

		//update hero
        let heroes = diesel::update(hero::table)
            .filter(hero::id.eq(id))
            .set(hero_data)
            .get_result(&conn)?;

        Ok(heroes)
	}
	
	pub fn delete(id: i32) -> Result<Self, ApiError> {
        let conn = db::connection()?;

		//get hero after delete
		let heroes = hero::table
		.filter(hero::id.eq(id))
		.first::<Hero>(&conn)?;

		//delete hero
        diesel::delete(hero::table.filter(hero::id.eq(id))).execute(&conn)?;

        Ok(heroes)
    }
}