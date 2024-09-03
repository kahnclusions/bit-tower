use leptos::prelude::*;

#[cfg(feature = "ssr")]
pub mod ssr {
    use crate::qbittorrent::client::QbtClient;
    use base64::Engine;
    use http::header;
    use leptos::prelude::*;
    use serde::{Deserialize, Serialize};

    pub static AUTH_COOKIE: &str = "bt-session";
    pub static REMOVE_COOKIE: &str = "bt-session=; path=/; expires=Thu, 01 Jan 1970 00:00:00 GMT";

    pub fn use_qbt() -> Result<QbtClient, ServerFnError> {
        use_context::<QbtClient>()
            .ok_or_else(|| ServerFnError::ServerError("Qbt client missing.".into()))
    }

    pub fn auth() -> Result<Option<Session>, ServerFnError> {
        let session = use_context::<Session>();
        Ok(session)
    }

    #[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
    pub struct AuthSession {
        pub session: Option<Session>,
    }
    impl AuthSession {
        pub fn new(session: Option<Session>) -> Self {
            Self { session }
        }
    }

    #[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
    pub struct Session {
        pub sid: String,
    }
    impl Session {
        pub fn new(sid: String) -> Self {
            Self { sid }
        }
    }

    #[tracing::instrument(skip_all)]
    pub fn set_session(session: Session) -> Result<(), ServerFnError> {
        if let Some(res) = leptos::context::use_context::<leptos_axum::ResponseOptions>() {
            let encoded: Vec<u8> = bincode::serialize(&session).unwrap();
            let value = simple_crypt::encrypt(&encoded, b"test-password-please-ignore").unwrap();
            let value = base64::prelude::BASE64_STANDARD.encode(value);
            res.insert_header(
                header::SET_COOKIE,
                header::HeaderValue::from_str(&format!("bt-session={value}; path=/; HttpOnly"))
                    .expect("header value couldn't be set"),
            );
            Ok(())
        } else {
            Err(ServerFnError::ServerError("No ".to_string()))
        }
    }

    #[tracing::instrument(skip_all)]
    pub fn get_session(sealed_token: String) -> Result<Session, anyhow::Error> {
        let sealed_bytes = base64::prelude::BASE64_STANDARD.decode(sealed_token)?;
        let encoded = simple_crypt::decrypt(&sealed_bytes, b"test-password-please-ignore").unwrap();
        let session: Session = bincode::deserialize(&encoded).unwrap();
        Ok(session)
    }
}

#[server(Login, "/api")]
pub async fn login(username: String, password: String) -> Result<(), ServerFnError> {
    use self::ssr::*;

    let qbt = use_qbt()?;

    let sid = qbt.auth_login(username, password).await?;

    set_session(Session::new(sid))?;

    Ok(())
}

#[server]
pub async fn has_auth() -> Result<bool, ServerFnError> {
    let auth = self::ssr::auth()?;

    Ok(auth.is_some())
}
