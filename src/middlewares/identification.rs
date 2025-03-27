/*++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
+ Copyright (c) 2025. Xodium.
+ All rights reserved.
+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++*/

use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::request::{FromRequest, Outcome};
use rocket::{async_trait, Data, Request, Response};
use uuid::Uuid;

/// Request ID middleware
pub struct Identificator;

#[async_trait]
impl Fairing for Identificator {
    fn info(&self) -> Info {
        Info {
            name: "Request ID",
            kind: Kind::Request | Kind::Response,
        }
    }

    async fn on_request(&self, request: &mut Request<'_>, _data: &mut Data<'_>) {
        request.local_cache(|| Uuid::new_v4().to_string().clone());
    }

    async fn on_response<'r>(&self, request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new(
            "X-Request-ID",
            request.local_cache::<String, _>(|| String::new()).clone(),
        ));
    }
}

/// Request ID guard
pub struct IdGuard(pub String);

#[async_trait]
impl<'r> FromRequest<'r> for IdGuard {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        Outcome::Success(IdGuard(
            request.local_cache::<String, _>(|| String::new()).clone(),
        ))
    }
}
