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

use chrono::Utc;

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


#[get("/sites")]
pub fn all_sites() -> Json<Vec<Site>>{

    /* Get all our texts from database */
    let sites: Vec<Site> = sites::table
        .select(sites::all_columns)
        .load::<Site>(&crate::establish_connection())
        .expect("Whoops, like this went bananas!");
    
    Json(sites)
}

#[get("/site/<id>")]
pub fn site_by_id(id: i32) -> Json<Vec<Site>> {
    /* Get all our texts from database */
    let sites: Vec<Site> = sites::table
        .select(sites::all_columns)
        .filter(sites::id.eq(id))
        .load::<Site>(&crate::establish_connection())
        .expect("Whoops, like this went bananas!");
    Json(sites)
}

#[get("/users")]
pub fn all_users() -> Json<Vec<User>>{

    /* Get all our texts from database */
    let users: Vec<User> = users::table
        .select(users::all_columns)
        .load::<User>(&crate::establish_connection())
        .expect("Whoops, like this went bananas!");
    Json(users)
}

#[get("/site/<id>/users")]
pub fn users_by_site(id: i32) -> Json<Vec<User>>{

    /* Get all our texts from database */
    let users: Vec<User> = users::table
        .select(users::all_columns)
        .filter(users::site_id.eq(id))
        .load::<User>(&crate::establish_connection())
        .expect("Whoops, like this went bananas!");
    
    Json(users)
}

#[get("/site/<id>/about")]
pub fn get_about(id: i32) -> Json<Vec<About>>{

    /* Get all our texts from database */
    let about: Vec<About> = about::table
        .select(about::all_columns)
        .filter(about::site_id.eq(id))
        .load::<About>(&crate::establish_connection())
        .expect("Whoops, like this went bananas!");
    
    Json(about)
}

#[get("/site/<id>/contact")]
pub fn get_contact(id: i32) -> Json<Vec<Contact>>{

    /* Get all our texts from database */
    let contacts: Vec<Contact> = contact::table
        .select(contact::all_columns)
        .filter(contact::site_id.eq(id))
        .load::<Contact>(&crate::establish_connection())
        .expect("Whoops, like this went bananas!");
    
    Json(contacts)
}
//SERVICES
#[get("/site/<id>/services")]
pub fn get_services(id: i32) -> Json<Vec<Service>>{

    /* Get all our texts from database */
    let services: Vec<Service> = service::table
        .select(service::all_columns)
        .filter(service::site_id.eq(id))
        .load::<Service>(&crate::establish_connection())
        .expect("Whoops, like this went bananas!");
    
    Json(services)
}
#[post("/site/<id>/newservice", data = "<service_data>")]
pub fn new_service(id: i32, content_type: &ContentType, service_data: Data){
    let mut context = HashMap::new();
    context.insert("id", id);
    let mut options = MultipartFormDataOptions::new();
    options.allowed_fields = vec![
        MultipartFormDataField::text("content"),
    ];
    let multipart_form_data = MultipartFormData::parse(content_type, service_data, options);
    match multipart_form_data {
        Ok(form) => {
            let insert = diesel::insert_into(service::table)
                .values( NewService{
                    site_id: id,
                    content: match form.texts.get("content") { Some(val) => &val[0].text, None => "...", },
                    
                })
                .execute(&crate::establish_connection());
            match insert {
                Ok(_) => { println!("Service saved!"); },
                Err(err_msg) =>{ println!("Error saving service... {}", err_msg); }
            }
        }
        Err(err) => { print!("Error on insertion... {}", err); }
    }
}

//BLOG
#[get("/site/<id>/blog")]
pub fn get_blog(id: i32) -> Json<Vec<Blog>>{
    /* Get all our texts from database */
    let blog: Vec<Blog> = blog::table
        .select(blog::all_columns)
        .filter(blog::site_id.eq(id))
        .load::<Blog>(&crate::establish_connection())
        .expect("Whoops, like this went bananas!");
    Json(blog)
}

#[post("/site/<id>/newpost", data = "<post_data>")]
pub fn new_post(id: i32,content_type: &ContentType, post_data: Data){
    let mut context = HashMap::new();
    context.insert("id", id);
    use std::fs;
    let mut options = MultipartFormDataOptions::new();
    options.allowed_fields = vec![
        MultipartFormDataField::file("image"),
        MultipartFormDataField::text("post"),
    ];
    let multipart_form_data = MultipartFormData::parse(content_type, post_data, options);
    match multipart_form_data {
        Ok(form) => {
            let image = match form.files.get("image") {
                Some(file) => {
                    let file_field = &file[0];
                    let _content_type = &file_field.content_type;
                    let _file_name = &file_field.file_name;
                    let _path = &file_field.path;
                    let format: Vec<&str> = _file_name.as_ref().unwrap().split('.').collect();
                    let absolute_path: String = format!("imgs/{}", _file_name.clone().unwrap());
                    fs::copy(_path, &absolute_path).unwrap();
                    Some(format!("{}", _file_name.clone().unwrap()))
                } None => None,
            };
            let insert = diesel::insert_into(blog::table)
                .values( NewBlog{
                    post: match form.texts.get("post") { Some(val) => &val[0].text, None => "...", },
                    site_id: id,
                    img_path: image,
                    posted_at: Utc::now().naive_utc(),
                })
                .execute(&crate::establish_connection());
            match insert {
                Ok(_) => { println!("Posted!"); },
                Err(err_msg) =>{ println!("Error posting... {}", err_msg); }
            }
        }
        Err(err) => { print!("Error on insertion... {}", err); }
    }
}

#[post("/site/<id>/blog/update/<postid>", data = "<post_data>")]
pub fn update_post(id: i32,content_type: &ContentType, post_data: Data, postid: i32){
    use std::fs;
    let mut options = MultipartFormDataOptions::new();
    options.allowed_fields = vec![
        MultipartFormDataField::file("image"),
        MultipartFormDataField::text("post"),
    ];
    let multipart_form_data = MultipartFormData::parse(content_type, post_data, options);
    match multipart_form_data {
        Ok(form) => {
            let image = match form.files.get("image") {
                Some(file) => {
                    let file_field = &file[0];
                    let _content_type = &file_field.content_type;
                    let _file_name = &file_field.file_name;
                    let _path = &file_field.path;
                    let format: Vec<&str> = _file_name.as_ref().unwrap().split('.').collect();
                    let absolute_path: String = format!("imgs/{}", _file_name.clone().unwrap());
                    fs::copy(_path, &absolute_path).unwrap();
                    Some(format!("{}", _file_name.clone().unwrap()))
                } None => None,
            };
            let update = diesel::update(
                blog::table
                    .filter(blog::site_id.eq(id))
                    .filter(blog::id.eq(postid))
            ).set( NewBlog{
                post: match form.texts.get("post") { Some(val) => &val[0].text, None => "...", },
                site_id: id,
                img_path: image,
                posted_at: Utc::now().naive_utc(),
            })
            .execute(&crate::establish_connection());
            match update {
                Ok(_) => { println!("Updated!"); },
                Err(err_msg) =>{ println!("Error posting... {}", err_msg); }
            }
        } Err(err) => { print!("Error on updating... {}", err); }
    }
}

#[get("/site/<id>/blog/delete/<postid>")]
pub fn delete_post(id: i32, postid: i32) -> Redirect {
    diesel::delete(blog::table
        .filter(blog::site_id.eq(id))
        .filter(blog::id.eq(postid)))
        .execute(&crate::establish_connection())
        .expect("Post couldn't be deleted...");
    Redirect::to("/admin")
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

#[get("/admin")]
pub fn admin() -> Template{
    let context = "loginForm";
    Template::render("login", &context)
}

#[post("/admin", data = "<form_data>")]
pub fn login(content_type: &ContentType, form_data: Data) -> Template {
    let mut context = HashMap::new();
    
    let mut options = MultipartFormDataOptions::new();
    options.allowed_fields = vec![
        MultipartFormDataField::text("user"),
        MultipartFormDataField::text("password")
    ];
    let multipart_form_data = MultipartFormData::parse(content_type, form_data, options);
    match multipart_form_data {
        Ok(form) => {
            let email = match form.texts.get("user") {
                Some(value) => &value[0].text,
                None => "no email...",
            };
            let pass = match form.texts.get("password") {
                Some(value) => &value[0].text,
                None => "no password...",
            };



            let users: Vec<User> = users::table
                .select(users::all_columns)
                .filter(users::email.eq(email))
                .filter(users::pass.eq(pass))
                .load::<User>(&crate::establish_connection())
                .expect("Whoops, like this went bananas!");
            if users.is_empty() {
                println!("{} : {}", email, pass);
                //context.insert("text", "User not found...");
                return Template::render("index", &context)
            }else{
                //println!("{} : {}", email, pass);
                let id = users[0].site_id;
                
                //let id = 1;
                /*let about_data = about::table
                    .select(about::all_columns)
                    .filter(about::site_id.eq(id))
                    .load::<About>(&crate::establish_connection())
                    .expect("Whoops, like this went bananas!");
                context.insert("about", about_data);
                */
                context.insert("id", id);
                return Template::render("admin", &context);

            }
        }
        Err(_err_msg) => {
            //context.insert("text", "Error on login...");
            return Template::render("index", &context)
        }
    }
}
