use rocket::Request;

#[catch(400)]
pub fn bad_request() -> &'static str {
    "Bad Request, please make sure your request body is valid"
}

#[catch(401)]
pub fn unauthorized() -> &'static str {
    "Unauthorized, please include a valid Authentication header, or check your request body"
}

#[catch(404)]
pub fn not_found(req: &Request) -> String {
    format!("I couldn't find '{}'. Try something else?", req.uri())
}

#[catch(409)]
pub fn conflict(_req: &Request) -> String {
    format!("Data Conflict, please make sure you are not trying to insert duplicate data")
}

#[catch(422)]
pub fn unprocessable_entity(_req: &Request) -> String {
    format!("The body data is invalid, please make sure you are following the correct structure")
}

#[catch(500)]
pub fn internal_error() -> &'static str {
    "Whoops! Looks like we messed up."
}
