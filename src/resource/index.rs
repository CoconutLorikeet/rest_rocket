use rocket::http::Status;
use rocket::response::status;
use rocket::serde::{Serialize, json::Json};
use rocket::{delete, get, post, put};

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Response {
    status: u8,
    message: String,
}

#[delete("/resource")]
pub fn delete_resource() -> status::Custom<Json<Response>> {
    status::Custom(
        Status::Ok, // 200 OK explicitly
        Json(Response {
            status: 200,
            message: String::from("Deleted"),
        }),
    )
}

#[get("/resource")]
pub fn get_resource() -> status::Custom<Json<Response>> {
    status::Custom(
        Status::Ok, // 200 OK explicitly
        Json(Response {
            status: 200,
            message: String::from("Ok"),
        }),
    )
}

#[post("/resource")]
pub fn post_resource() -> status::Custom<Json<Response>> {
    status::Custom(
        Status::Created, // 201 OK explicitly
        Json(Response {
            status: 201,
            message: String::from("Created"),
        }),
    )
}

#[put("/resource")]
pub fn put_resource() -> status::Custom<Json<Response>> {
    status::Custom(
        Status::Ok, // 200 OK explicitly
        Json(Response {
            status: 200,
            message: String::from("Updated"),
        }),
    )
}
