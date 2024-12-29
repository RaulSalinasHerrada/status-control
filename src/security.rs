use std::{
    collections::HashMap,
    str::FromStr,
    sync::{Arc, Mutex},
};

use actix_web::{
    dev::ServiceRequest, error, get, web::Data, Error, HttpMessage, HttpResponse, Responder,
};
use actix_web_httpauth::extractors::{
    basic::BasicAuth,
    bearer::{BearerAuth, Config},
    AuthenticationError,
};
use hmac::{Hmac, Mac};
use jwt::{SignWithKey, VerifyWithKey};
use serde::{Deserialize, Serialize};
use sha2::Sha256;

use argon2::{
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

const JWT_SECRET: &str = "supersecretjwt";
const HASH_SECRET: &str = "c3VwZXJzZWNyZXRoYXNo";
const PASS_SECRET: &str = "thisisapassword";

#[derive(Serialize, Deserialize, Clone)]
struct TokenClaims {
    id: String,
}

impl TokenClaims {
    fn new<T: Into<String>>(id: T) -> Self {
        Self { id: id.into() }
    }
}

/// Validator that:
/// - accepts Bearer auth;
/// - returns a custom response for requests without a valid Bearer Authorization header;
/// - rejects tokens containing an "x" (for quick testing using command line HTTP clients).
pub async fn validator(
    req: ServiceRequest,
    credentials: Option<BearerAuth>,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    let jwt_secret = match std::env::var("JWT_SECRET") {
        Ok(x) => x,
        Err(_) => String::from_str(JWT_SECRET).expect("Fallback on jwt failed"),
    };

    let key: Hmac<Sha256> = Hmac::new_from_slice(jwt_secret.as_bytes()).unwrap();

    match credentials {
        Some(credentials) => {
            let claims: Result<TokenClaims, &str> = credentials
                .token()
                .verify_with_key(&key)
                .map_err(|_| "Invalid token");

            match claims {
                Ok(value) => {
                    req.extensions_mut().insert(value);
                    Ok(req)
                }

                _ => {
                    let config = req
                        .app_data::<Config>()
                        .cloned()
                        .unwrap_or_default()
                        .scope("");

                    Err((AuthenticationError::from(config).into(), req))
                }
            }
        }

        _ => Err((error::ErrorBadRequest("no bearer header"), req)),
    }
}

#[get("/auth")]
pub async fn basic_auth(credentials: BasicAuth) -> impl Responder {
    let jwt_secret: Hmac<Sha256> = Hmac::new_from_slice(JWT_SECRET.as_bytes()).unwrap();

    let user = credentials.user_id();
    let pass_auth = credentials.password();

    match pass_auth {
        None => HttpResponse::Unauthorized().json("Must provide password"),
        Some(pass) => {
            let argon = Argon2::default();
            let salt = SaltString::from_b64(HASH_SECRET).unwrap();

            let hashed = argon.hash_password(pass.as_bytes(), &salt).unwrap();
            match argon.verify_password(PASS_SECRET.as_bytes(), &hashed) {
                Err(_) => HttpResponse::Unauthorized().body("Invalid user or pass"),
                Ok(_) => {
                    let token = TokenClaims::new(user).sign_with_key(&jwt_secret).unwrap();

                    HttpResponse::Ok().json(token)
                }
            }
        }
    }
}
