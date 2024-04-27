use std::sync::Arc;
use std::{env, fmt};

#[cfg(feature = "blocking")]
use reqwest::blocking::Client as ReqwestClient;
#[cfg(not(feature = "blocking"))]
use reqwest::Client as ReqwestClient;

use crate::config::Config;
use crate::services::{ApiKeysSvc, AudiencesSvc, ContactsSvc, DomainsSvc, EmailsSvc};

/// A minimal [Resend](https://resend.com) client.
///
/// #### Example
///
/// ```rust,no_run
/// use resend_rs::{Client, Result};
/// use resend_rs::types::SendEmail;
///
/// # #[tokio::main]
/// # async fn main() -> Result<()> {
/// let from = "Acme <onboarding@resend.dev>";
/// let to = ["delivered@resend.dev"];
/// let sub = "Hello World!";
///
/// let email = SendEmail::new(from, to, sub)
///     .with_text("Hello World!");
///
/// let resend = Client::default();
/// let id = resend.emails.send(email).await?;
/// println!("id: {id}");
/// # Ok(())
/// # }
/// ```
#[must_use]
#[derive(Clone)]
pub struct Client {
    /// `Resend` APIs for `/emails` endpoints.
    pub emails: EmailsSvc,
    /// `Resend` APIs for `/api-keys` endpoints.
    pub api_keys: ApiKeysSvc,
    /// `Resend` APIs for `/audiences` endpoints.
    pub audiences: AudiencesSvc,
    /// `Resend` APIs for `/audiences/:id/contacts` endpoints.
    pub contacts: ContactsSvc,
    /// `Resend` APIs for `/domains` endpoints.
    pub domains: DomainsSvc,
}

impl Client {
    /// Creates a new [`Resend`] client.
    ///
    /// ### Panics
    ///
    /// - Panics if the environment variable `RESEND_BASE_URL` is present but is not a valid `URL`.
    /// - Panics if the environment variable `RESEND_RATE_LIMIT` is present but is not a valid `u64`.
    ///
    /// [`Resend`]: https://resend.com
    pub fn new(api_key: &str) -> Self {
        Self::with_client(api_key, ReqwestClient::default())
    }

    /// Creates a new [`Resend`] client with a provided [`reqwest::Client`].
    ///
    /// ### Panics
    ///
    /// - Panics if the environment variable `RESEND_BASE_URL` is present but is not a valid `URL`.
    /// - Panics if the environment variable `RESEND_RATE_LIMIT` is present but is not a valid `u64`.
    ///
    /// [`Resend`]: https://resend.com
    /// [`reqwest::Client`]: ReqwestClient
    pub fn with_client(api_key: &str, client: ReqwestClient) -> Self {
        let inner = Arc::new(Config::new(api_key, client));

        Self {
            api_keys: ApiKeysSvc(inner.clone()),
            audiences: AudiencesSvc(inner.clone()),
            contacts: ContactsSvc(inner.clone()),
            domains: DomainsSvc(inner.clone()),
            emails: EmailsSvc(inner),
        }
    }

    /// Returns the used `User-Agent` header value.
    #[must_use]
    pub fn user_agent(&self) -> String {
        self.emails.0.user_agent.clone()
    }

    /// Returns the provided API key.
    #[must_use]
    pub fn api_key(&self) -> String {
        self.emails.0.api_key.clone()
    }

    /// Returns the underlying [`reqwest::Client`].
    ///
    /// [`reqwest::Client`]: ReqwestClient
    #[must_use]
    pub fn client(&self) -> ReqwestClient {
        self.emails.0.client()
    }
}

impl Default for Client {
    /// Creates a new [`Client`] from the `RESEND_API_KEY` environment variable .
    ///
    /// ### Panics
    ///
    /// - Panics if the environment variable `RESEND_API_KEY` is not set.
    /// - Panics if the environment variable `RESEND_BASE_URL` is present but is not a valid `URL`.
    /// - Panics if the environment variable `RESEND_RATE_LIMIT` is present but is not a valid `u64`.
    fn default() -> Self {
        let api_key = env::var("RESEND_API_KEY")
            .expect("env variable `RESEND_API_KEY` should be a valid API key");

        Self::new(api_key.as_str())
    }
}

impl fmt::Debug for Client {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.emails, f)
    }
}
