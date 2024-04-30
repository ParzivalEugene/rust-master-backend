use crate::{
    dto::{
        auth::{
            request::{ChangePasswordRequest, SignInRequest, SignUpRequest},
            response::Tokens,
        },
        device::internal::DeviceInfo,
    },
    handlers::{auth::{change_password, logout, refresh, sign_in, sign_up}},
};
use utoipa::{
    openapi::security::SecurityScheme,
    {Modify, OpenApi},
};

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.as_mut().unwrap();
        components.add_security_scheme(
            "access_token",
            SecurityScheme::Http(utoipa::openapi::security::Http::new(
                utoipa::openapi::security::HttpAuthScheme::Bearer,
            )),
        );
        components.add_security_scheme(
            "refresh_token",
            SecurityScheme::Http(utoipa::openapi::security::Http::new(
                utoipa::openapi::security::HttpAuthScheme::Bearer,
            )),
        );
    }
}

#[derive(OpenApi)]
#[openapi(
    paths(sign_in::sign_in, sign_up::sign_up, refresh::refresh, logout::logout, change_password::change_password),
    components(schemas(SignInRequest, SignUpRequest, DeviceInfo, Tokens, ChangePasswordRequest)),
    modifiers(&SecurityAddon),
)]
pub struct ApiDoc;
