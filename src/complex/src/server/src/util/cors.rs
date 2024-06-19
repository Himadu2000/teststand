use rocket::{
    async_trait,
    fairing::{Fairing, Info, Kind},
    http::{Header, Method::Options, Status},
    {Request, Response},
};

pub struct Cors;

#[async_trait]
impl Fairing for Cors {
    fn info(&self) -> Info {
        Info {
            name: "CORS",
            kind: Kind::Response,
        }
    }

    async fn on_response<'res>(&self, request: &'res Request<'_>, response: &mut Response<'res>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "GET, POST, OPTIONS",
        ));

        if request.method() == Options {
            response.set_status(Status::Ok);
        }
    }
}
