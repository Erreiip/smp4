use hmac::{Hmac, Mac};
use jwt::{SignWithKey, VerifyWithKey};
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use crate::database::repositories::users_repository::User;

pub struct JwtService {
    jwt_key: Hmac<Sha256>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TokenClaims {
    pub id: i32,
}

impl JwtService {
    pub fn new() -> Self {
        let jwt_secret: String = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
        let jwt_key: Hmac<Sha256> = Hmac::new_from_slice(jwt_secret.as_bytes()).unwrap();

        JwtService { jwt_key }
    }

    pub fn generate_token_from_user(&self, user : User) -> String {
        let claims = TokenClaims { id: user.id };
        claims.sign_with_key(&self.jwt_key).unwrap()
    }

    pub fn get_claims_from_token(&self, token : String) -> Result<TokenClaims, &str> {
        token.verify_with_key(&self.jwt_key).map_err(|_| "Invalid token")
    }
}