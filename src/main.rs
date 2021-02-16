#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/hello/<name>/<age>/<religion>")]
fn hello(name: String, age: u8, religion: String) -> String {
    if religion == "Islam" {
        format!("You're a {} year old Muslim. And you are the best creation. Allah loves you, {}!", age, name)
    } else{
        format!("{}, You don't still believe in your real lord, your creator. Embrace Islam and go towards eternal peace, in Jannah.", name)
    }
}


fn main() {
    rocket::ignite().mount("/", routes![index, hello]).launch();
}
