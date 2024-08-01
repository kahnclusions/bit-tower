use leptos::prelude::*;

#[cfg(feature = "ssr")]
pub mod ssr {
    use base64::Engine;
    use http::header;
    use leptos::prelude::*;
    use qbittorrent_rs::QbtClient;
    use serde::{Deserialize, Serialize};

    pub fn use_qbt() -> Result<QbtClient, ServerFnError> {
        use_context::<QbtClient>()
            .ok_or_else(|| ServerFnError::ServerError("Qbt client missing.".into()))
    }

    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    pub struct Session {
        sid: String,
    }
    impl Session {
        pub fn new(sid: String) -> Self {
            Self { sid }
        }
    }

    #[tracing::instrument]
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

    #[tracing::instrument]
    pub fn get_session(session: Session) -> Result<(), ServerFnError> {
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
}

#[server(Login, "/api")]
pub async fn login(username: String, password: String) -> Result<(), ServerFnError> {
    use self::ssr::*;

    let qbt = use_qbt()?;

    let sid = qbt.auth_login(username, password).await?;

    set_session(Session::new(sid))?;

    Ok(())
}
