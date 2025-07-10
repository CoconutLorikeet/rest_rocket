#[macro_use]
extern crate rocket;

use rocket::routes;
use rocket::{Build, Rocket};

use rest_rocket::resource::index::{delete_resource, get_resource, post_resource, put_resource};

#[launch]
pub fn rocket() -> Rocket<Build> {
    rocket::build().mount(
        "/api/v1.0.0",
        routes![delete_resource, get_resource, post_resource, put_resource],
    )
}

#[cfg(test)]
mod tests {
    use super::*; // bring in rocket() and handlers
    use rocket::http::Status;
    use rocket::local::blocking::Client;

    #[test]
    fn test_get_resource() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.get("/api/v1.0.0/resource").dispatch();

        assert_eq!(response.status(), Status::Ok);
        let body = response.into_string().unwrap();
        assert!(body.contains("\"message\":\"Ok\""));
    }

    #[test]
    fn test_post_resource() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.post("/api/v1.0.0/resource").dispatch();

        assert_eq!(response.status(), Status::Created);
        let body = response.into_string().unwrap();
        assert!(body.contains("\"message\":\"Created\""));
    }

    #[test]
    fn test_delete_resource() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.delete("/api/v1.0.0/resource").dispatch();

        assert_eq!(response.status(), Status::Ok);
        let body = response.into_string().unwrap();
        assert!(body.contains("\"message\":\"Deleted\""));
    }

    #[test]
    fn test_put_resource() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.put("/api/v1.0.0/resource").dispatch();

        assert_eq!(response.status(), Status::Ok);
        let body = response.into_string().unwrap();
        assert!(body.contains("\"message\":\"Updated\""));
    }
}
