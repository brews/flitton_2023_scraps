use actix_web::dev::Payload;
use actix_web::error::ErrorUnauthorized;
use actix_web::{Error, FromRequest, HttpRequest};
use chrono::Utc;
use futures::future::{err, ok, Ready};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::config::Config;

#[derive(Debug, Serialize, Deserialize)]
pub struct JwToken {
    pub user_id: i32,
    pub exp: usize,
}

impl JwToken {
    pub fn new(user_id: i32) -> Self {
        // Logic for token expiration.
        let config = Config::new();
        let minutes = config.expire_minutes;
        let expiration = Utc::now()
            .checked_add_signed(chrono::Duration::minutes(minutes))
            .expect("valid timestamp")
            .timestamp();

        return JwToken {
            user_id,
            exp: expiration as usize,
        };
    }

    /// Gets the secret key for the serialization and deserialization from config file.
    pub fn get_key() -> String {
        return Config::new().secret_key.to_owned();
    }

    /// Encodes the data from the JwToken struct as a token.
    pub fn encode(self) -> String {
        let key = EncodingKey::from_secret(JwToken::get_key().as_ref());
        let token = encode(&Header::default(), &self, &key).unwrap();
        return token;
    }

    /// Creates JwToken from a raw header token. If there is a failure in the deserialization it returns a None as there can be failures in deserialiation.
    pub fn from_token(token: String) -> Result<Self, String> {
        let key = DecodingKey::from_secret(JwToken::get_key().as_ref());
        let token_result = decode::<JwToken>(&token, &key, &Validation::default());

        match token_result {
            Ok(data) => return Ok(data.claims),
            Err(error) => {
                // I'm not crazy about handling errors with string messages...
                // Relates to how we handle errors/strings in the impl FromRequest for JwToken.
                let message = format!("{}", error);
                return Err(message);
            }
        }
    }
}

impl FromRequest for JwToken {
    type Error = Error;
    type Future = Ready<Result<JwToken, Error>>;

    /// This gets fired when the JwToken is attached to a request. It fires before the request hits the view.
    /// # Arguments
    /// The arguments are needed in order for the impl of FromRequest to work.
    ///
    /// * req (&HttpRequest): the request that the token is going to be extracted from
    /// * _ (Payload): the payload stream (not used in this function but is needed)
    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        // Can we get token from header?
        match req.headers().get("token") {
            Some(data) => {
                let raw_token = data.to_str().unwrap().to_string();
                let token_result = JwToken::from_token(raw_token);

                // Can we decode the raw header token?
                match token_result {
                    Ok(token) => return ok(token),
                    Err(message) => {
                        if message == "ExpiredSignature".to_owned() {
                            return err(ErrorUnauthorized("token expired"));
                        }
                        return err(ErrorUnauthorized("token can't be decoded"));
                    }
                }
            }
            None => {
                return err(ErrorUnauthorized("token not in header under key 'token'"));
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use actix_web::test::{init_service, call_service};
    use actix_web::{HttpRequest, HttpResponse,
                    test::TestRequest, web, App};
    use actix_web::http::header::{HeaderValue,
                                  HeaderName, ContentType};
    use actix_web;
    use serde_json::json;
    use serde::{Deserialize, Serialize};


    #[derive(Debug, Serialize, Deserialize)]
    pub struct ResponseFromTest {
        pub user_id: i32,
        pub exp_minutes: i32,
    }

    #[test]
    fn get_key() {
        assert_eq!(String::from("secret"), JwToken::get_key());
    }

    #[test]
    fn get_exp() {
        // This is testing Config. Not sure why it's here. Likely because other tests depend on it.
        let expected: i64 = 120;
        assert_eq!(expected, Config::new().expire_minutes);
    }

    #[test]
    fn decode_incorrect_token() {
        let encoded_token = String::from("invalid_token");
        match JwToken::from_token(encoded_token) {
            Err(msg) => assert_eq!("InvalidToken", msg),
            _ => panic!("Should not be able to encode an incorrect token"),
        }
    }

    #[test]
    fn encode_decode() {
        let test_token = JwToken::new(5);
        let encoded_token = test_token.encode();
        let new_token = JwToken::from_token(encoded_token).unwrap();
        assert_eq!(5, new_token.user_id);
    }

    /// Basic view function that only handles the authentication of JwToken, returning the user ID.
    async fn test_handler(token: JwToken, _: HttpRequest) -> HttpResponse {
        return HttpResponse::Ok().json(json!({"user_id": token.user_id, "exp_minutes": 60}))
    }

    #[actix_web::test]
    async fn test_no_token_request() {
        let app = init_service(App::new().route("/", web::get().to(test_handler))).await;
        let req = TestRequest::default().insert_header(ContentType::plaintext()).to_request();
        let resp = call_service(&app, req).await;
        assert_eq!("401", resp.status().as_str());
    }

    #[actix_web::test]
    async fn test_passing_token_request() {
        let test_token = JwToken::new(5);
        let encoded_token = test_token.encode();

        let app = init_service(App::new().route("/", web::get().to(test_handler))).await;
        let mut req = TestRequest::default().insert_header(ContentType::plaintext()).to_request();
        let header_name = HeaderName::from_static("token");
        let header_value = HeaderValue::from_str(encoded_token.as_str()).unwrap();
        req.headers_mut().insert(header_name, header_value);

        let resp: ResponseFromTest = actix_web::test::call_and_read_body_json(&app, req).await;
        assert_eq!(5, resp.user_id);
    }

    #[actix_web::test]
    async fn test_false_token_request() {
        let app = init_service(App::new().route("/", web::get().to(test_handler))).await;
        let mut req = TestRequest::default().insert_header(ContentType::plaintext()).to_request();
        let header_name = HeaderName::from_static("token");
        let header_value = HeaderValue::from_str("invalidtoken").unwrap();
        req.headers_mut().insert(header_name, header_value);

        let resp = call_service(&app, req).await;
        assert_eq!("401", resp.status().as_str());
    }
}