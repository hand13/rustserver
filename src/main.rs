#![feature(proc_macro_hygiene,decl_macro)]
#[macro_use] extern crate rocket;
use chrono::prelude::*;
use std::fs::File;
use std::path::Path;
use std::io::prelude::*;

#[get("/hello/<name>")]
fn hello(name : String) -> String {
    format!("hello ,{}!",name.as_str())
}
#[get("/time")]
fn get_time() -> String{
    let time = Local::now();
    format!("now is {}",time)
}

#[get("/content")]
fn res() ->String{
    let path = Path::new("test.json");
    let mut file = match File::open(&path){
        Err(why) => panic!("could not read the file the reason is {}",why),
        Ok(file) => file
    };
    let mut s = String::new();
    match file.read_to_string(&mut s){
        Err(_) => panic!("this is bad"),
        Ok(_) => println!("the result is {}",&s),
    };
    format!("content  is {}",s)
}
fn main() {
    rocket::ignite()
        .mount("/",routes![hello])
        .mount("/lord",routes![get_time])
        .mount("/lord",routes![res])
        .launch();
}
