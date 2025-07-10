#[cfg(test)]
mod test {
    use super::rocket;
    use rocket::http::Status;
    use rocket::local::blocking::Client;

    #[test]
    fn get_resource_should_return_200() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let mut response = client.get(uri!(super::get_resource)).dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.into_string().unwrap(), "Hello, world!");
    }
}
