#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, from Rocket Repo!"
}

#[get("/hi")]
fn hi() -> &'static str {
    "Hi again, from Rocket Repo!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, hi])
}
