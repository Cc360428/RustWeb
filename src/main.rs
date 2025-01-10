#[macro_use]
extern crate rocket;

// #[get("/")]
// fn index() -> &'static str {
//     connection_redis();
//     "get 请求"
// }

#[post("/config")]
fn get_config() -> &'static str {
    "post 请求"
}

#[get("/")]
fn hello_world() -> &'static str {
    "hello_world"
}


#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![hello_world])
        .mount("/v1", routes![get_config])
}
