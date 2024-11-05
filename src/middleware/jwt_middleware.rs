// middleware/jwt_auth.rs
use actix_service::{Service, Transform};
use actix_web::error::ErrorUnauthorized;
use actix_web::{
    dev::{ServiceRequest, ServiceResponse},
    Error,
};
use futures_util::future::{ok, Ready};
use jsonwebtoken::{decode, DecodingKey, Validation};
use std::{
    env,
    future::Future,
    pin::Pin,
    rc::Rc,
    task::{Context, Poll},
};

use crate::config::auth::Claims;

#[derive(Debug, Clone)]
pub struct JwtAuthMiddleware;

impl<S, B> Transform<S, ServiceRequest> for JwtAuthMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = JwtAuthMiddlewareMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(JwtAuthMiddlewareMiddleware {
            service: Rc::new(service),
        })
    }
}

pub struct JwtAuthMiddlewareMiddleware<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for JwtAuthMiddlewareMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&self, _ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        // Always ready to accept requests
        Poll::Ready(Ok(()))
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let headers = req.headers().clone();

        // Extract Authorization header
        if let Some(auth_header) = headers.get("Authorization") {
            if let Ok(token) = auth_header.to_str() {
                if let Some(token) = token.strip_prefix("Bearer ") {
                    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
                    let validation = Validation::default();

                    // Validate the JWT
                    if decode::<Claims>(
                        token,
                        &DecodingKey::from_secret(secret.as_ref()),
                        &validation,
                    )
                    .is_ok()
                    {
                        // Proceed with the next middleware/service if JWT is valid
                        let fut = self.service.call(req);
                        return Box::pin(async move { fut.await });
                    }
                }
            }
        }

        // Return 401 Unauthorized if JWT is invalid or missing
        Box::pin(async { Err(ErrorUnauthorized("JWT invalid or missing")) })
    }
}
