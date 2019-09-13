#![feature(proc_macro_hygiene,decl_macro)]
#[macro_use] extern crate rocket;
use chrono::prelude::*;
use std::fs::File;
use std::path::Path;
use std::io::prelude::*;
use rocket::request::Form;
use rocket::Request;

fn ifi(i:u32) -> u32 {
    if i <=0 {
        return 0;
    }

    if i == 1{
        1
    }else {
        i * ifi(i-1)
    }
}

#[derive(FromForm)]
struct User {
    name:String,
    password:String
}
#[get("/user?<user..>")]
fn user(user:Form<User>) -> String{
    format!("name is {} password is {}",&user.name,&user.password)
}

#[get("/echo?<name>")]
fn echo(name :String) ->String{
    format!("echo {} ,number is {}",name,ifi(8))
}


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
#[catch(404)]
fn not_found(req :&Request) -> String{
    format!("sorry {} is not a valid url",req.uri())
}
fn main() {
    rocket::ignite()
        .register(catchers![not_found])
        .mount("/",routes![hello])
        .mount("/lord",routes![get_time])
        .mount("/lord",routes![res,echo,user])
        .launch();
}
