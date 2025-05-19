use jsonwebtoken::{DecodingKey, EncodingKey};
use rand::rng;
use serde::{Deserialize, Serialize};

pub struct Keys {
    pub encoding: EncodingKey,
    pub decoding: DecodingKey,
}

impl Keys {
    pub fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    pub iss: String,
    pub nonce: i32,
}

impl Claims {
    pub fn new(duration: usize) -> Self {
        let exp = chrono::Utc::now().timestamp() as usize + duration;
        let nonce = rand::random_range(100000..999999);
        Self {
            sub: "IMAGE_HOSTING_ADMIN".into(),
            exp,
            iss: "AXUM.EU.ORG".into(),
            nonce,
        }
    }
}

pub fn token(claims: &Claims, keys: &EncodingKey) -> String {
    jsonwebtoken::encode(&jsonwebtoken::Header::default(), claims, &keys).unwrap()
}

pub fn validate(token: &str, keys: &DecodingKey) -> Result<Claims, jsonwebtoken::errors::Error> {
    let claims =
        jsonwebtoken::decode::<Claims>(token, &keys, &jsonwebtoken::Validation::default())?.claims;
    Ok(claims)
}

#[cfg(test)]
mod tests {
    use super::*;
    const JWT_SECERT: &str = "gA8qRajn9gYacWxt76qtEm47";
    const JWT_EXP: usize = 3600;

    fn key() -> Keys {
        Keys::new(JWT_SECERT.as_bytes())
    }

    #[test]
    fn test_token() {
        let claims = &Claims::new(JWT_EXP);
        let token = token(claims, &key().encoding);
        println!("token: {}", token);
    }

    #[test]
    fn test_validate() {
        let token = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJJTUFHRV9IT1NUSU5HX0FETUlOIiwiZXhwIjoxNzQ3NjMzNDI2LCJpc3MiOiJBWFVNLkVVLk9SRyJ9.WMgwP4ilH86F-O6GfjpAn-WNcW-dV-kXx1DWUN2ASVQ";
        let claims = validate(token, &key().decoding).unwrap();
        println!("claims: {:#?}", claims);
    }
}
