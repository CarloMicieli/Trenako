use actix_web::http::header::LOCATION;
use actix_web::HttpResponse;

pub trait ToCreated {
    /// Creates a http response with the CREATED http status
    fn to_created(&self) -> HttpResponse {
        HttpResponse::Created()
            .insert_header((LOCATION, self.location()))
            .finish()
    }

    /// The location for the resource created
    fn location(&self) -> String;
}
