
/* Import macros and others */
use crate::schema::*;

/* For beeing able to serialize */
use serde::Serialize;


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