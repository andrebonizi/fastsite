
/* Import macros and others */
use crate::schema::*;

/* For beeing able to serialize */
use serde::Serialize;

//use diesel::prelude::*;
use chrono::{NaiveDateTime};

#[derive(Debug, Queryable, Serialize)]
pub struct Text {
    pub id: i32, 
    pub site_id: i32, 
    pub text_ref: String,
    pub content: Option<String>
}

#[derive(Debug, Queryable, Serialize)]
pub struct Image {
    pub id: i32, 
    pub site_id: i32, 
    pub ref_name: String,
    pub ref_path: String
}

#[derive(Debug, Queryable, Serialize)]
pub struct Site {
    pub id: i32, 
    pub ref_name: String
}

#[derive(Debug, Queryable, Serialize)]
pub struct User {
    pub id: i32, 
    pub site_id: i32, 
    pub ref_name: String,
    pub email: String,
    pub pass: String
}

#[derive(Debug, Insertable, Serialize)]
#[ table_name = "users"]
pub struct NewUser {
    pub site_id: i32, 
    pub ref_name: String,
    pub email: String,
    pub pass: String
}

#[derive(Debug, Queryable, Serialize)]
pub struct About {
    pub id: i32, 
    pub site_id: i32, 
    pub ref_name: String,
    pub content: Option<String>
}

#[derive(Debug, Queryable, Serialize)]
pub struct Service {
    pub id: i32, 
    pub site_id: i32, 
    pub content: Option<String>
}

#[derive(Debug, Queryable, Serialize)]
pub struct Blog {
    pub id: i32, 
    pub site_id: i32, 
    pub post: Option<String>,
    pub img_path: Option<String>,
    pub posted_at: NaiveDateTime
}

#[derive(Debug, Queryable, Serialize)]
pub struct Contact {
    pub id: i32, 
    pub site_id: i32, 
    pub name: String,
    pub email: String,
    pub message: String
}

#[derive(FromForm)]
pub struct LoginForm {
    pub user: String,
    pub pass: String
}