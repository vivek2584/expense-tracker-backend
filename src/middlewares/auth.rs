use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use axum_extra::headers::{Authorization, HeaderMapExt, authorization::Bearer};
use jsonwebtoken::{Algorithm, DecodingKey, Validation};
use uuid::Uuid;

use crate::{errors::GlobalAppError, helpers::users::Claims, middlewares::GlobalAppState};

pub async fn validate_jwt(
    State(state): State<GlobalAppState>,
    mut request: Request,
    next: Next,
) -> Result<Response, GlobalAppError> {
    let token = request
        .headers()
        .typed_get::<Authorization<Bearer>>()
        .ok_or_else(|| {
            GlobalAppError::new(
                StatusCode::BAD_REQUEST,
                "auth bearer token missing!".to_owned(),
            )
        })?
        .token()
        .to_owned();

    let decoded_data = jsonwebtoken::decode::<Claims>(
        &token,
        &DecodingKey::from_base64_secret(&state.hmac).map_err(|_| {
            GlobalAppError::new(StatusCode::BAD_REQUEST, "invalid hmac!".to_string())
        })?,
        &Validation::new(Algorithm::HS384),
    )
    .map_err(|_| {
        GlobalAppError::new(
            StatusCode::UNAUTHORIZED,
            "error decoding token! token might have expired or invalid!".to_string(),
        )
    })?;

    let claim = decoded_data.claims;
    let uuid = Uuid::parse_str(&claim.sub).map_err(|_| {
        GlobalAppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "error parsing uuid!".to_string(),
        )
    })?;

    request.extensions_mut().insert(uuid);

    Ok(next.run(request).await)
}
