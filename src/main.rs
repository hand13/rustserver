#![feature(proc_macro_hygiene,decl_macro)]
#[macro_use] extern crate rocket;
#[get("/hello/<name>")]
fn hello(name : String) -> String {
    format!("hello ,{}!",name.as_str())
}
fn main() {
    rocket::ignite().mount("/",routes![hello]).launch();
}
