#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;


#[get("/add/<num>")]
fn add(num:i32) -> String {
    format!("Number after adding 10 to it will be: '{}'", num+10)
}

fn main() {
    rocket::ignite().mount("/", routes![add]).launch();
}