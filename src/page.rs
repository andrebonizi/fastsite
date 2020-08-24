/* To be able to return Templates */
use rocket_contrib::templates::Template;
use rocket_contrib::json::Json;
use std::collections::HashMap;

/* Diesel query builder */
use diesel::prelude::*;

/* Database macros */
use crate::schema::*;

/* Database data structs (Hero, NewHero) */
use crate::models::*;

/* To be able to parse raw forms */
use rocket::http::ContentType;
use rocket::Data;
use rocket_multipart_form_data::{
    MultipartFormData, MultipartFormDataField, MultipartFormDataOptions,
};

/* Flash message and redirect */
use rocket::request::FlashMessage;
use rocket::response::{Flash, Redirect};

/* List our inserted texts */
#[get("/")]
pub fn index(flash: Option<FlashMessage>) -> Template {
    let mut context = HashMap::new();

    /* Get all our heroes from database */
    let texts: Vec<Text> = texts::table
        .select(texts::all_columns)
        .load::<Text>(&crate::establish_connection())
        .expect("Whoops, like this went bananas!");
        /* Insert on the template rendering
    context our new heroes vec */
    if let Some(ref msg) = flash {
        context.insert("text", (texts, msg.msg()));
    } else {
        context.insert("text", (texts, "Listing texts..."));
    }
    /* Return the template */
    Template::render("index", &context)
}

#[get("/texts")]
pub fn texts() -> Json<Vec<Text>>{

    /* Get all our texts from database */
    let texts: Vec<Text> = texts::table
        .select(texts::all_columns)
        .load::<Text>(&crate::establish_connection())
        .expect("Whoops, like this went bananas!");
    
    Json(texts)
}
