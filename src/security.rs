use actix_web::guard::GuardContext;

// const API_KEY: &str = "your-secure-api-key";

// pub fn validate(ctx: &GuardContext) -> bool {
//     match ctx.head().headers().get("API-KEY") {
//         Some(header_value) if header_value == API_KEY => true,
//         _ => false,
//     }
// }

use actix_web::{
    dev::ServiceRequest, error, get, middleware::Logger, App, Error, HttpServer, Responder,
};
use actix_web_httpauth::{extractors::bearer::BearerAuth, middleware::HttpAuthentication};

/// Validator that:
/// - accepts Bearer auth;
/// - returns a custom response for requests without a valid Bearer Authorization header;
/// - rejects tokens containing an "x" (for quick testing using command line HTTP clients).
pub async fn validator(
    req: ServiceRequest,
    credentials: Option<BearerAuth>,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    match credentials {
        Some(_) => Ok(req),

        _ => Err((error::ErrorBadRequest("no bearer header"), req)),
    }
}
