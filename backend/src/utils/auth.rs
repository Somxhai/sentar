// use std::env;
//
// use chrono::{DateTime, Utc};
// use eyre::Result;
// use jsonwebtoken::{TokenData, Validation, decode, decode_header, jwk::JwkSet};
// use reqwest;
// use serde::{Deserialize, Serialize};
//
// #[derive(Debug, Clone, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Claims {
//     pub iat: i64,
//     pub name: String,
//     pub email: String,
//     pub email_verified: bool,
//     pub image: Option<String>,
//     pub created_at: DateTime<Utc>,
//     pub updated_at: DateTime<Utc>,
//     pub id: String,
//     pub sub: String,
//     pub exp: usize,
//     pub iss: String,
//     pub aud: String,
// }
//
// pub async fn get_jwks() -> Result<JwkSet> {
//     let jwks_url = env::var("JWT_ISSUER")? + "/api/auth/jwks";
//     let jwks_text = reqwest::get(&jwks_url).await?.text().await?;
//     let jwks: JwkSet = serde_json::from_str(&jwks_text)?;
//     Ok(jwks)
// }
//
// pub fn create_jwt_verifier(token: &str, jwks: &JwkSet) -> eyre::Result<TokenData<Claims>> {
//     use eyre::Context;
//     use eyre::eyre;
//
//     let header = decode_header(token).wrap_err("invalid JWT header")?;
//
//     let kid = header
//         .kid
//         .ok_or_else(|| eyre!("missing kid in JWT header"))?;
//     let jwk = jwks
//         .find(&kid)
//         .ok_or_else(|| eyre!("no JWK found for kid {kid}"))?;
//
//     let issuer = env::var("JWT_ISSUER").wrap_err("JWT_ISSUER env missing")?;
//
//     let mut validation = Validation::new(header.alg);
//     validation.set_issuer(std::slice::from_ref(&issuer));
//     validation.set_audience(&[issuer]);
//
//     let key: jsonwebtoken::DecodingKey = jwk.try_into().wrap_err("invalid JWK")?;
//
//     let decoded = decode::<Claims>(token, &key, &validation).wrap_err("JWT validation failed")?;
//
//     Ok(decoded)
// }
