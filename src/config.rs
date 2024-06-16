#[cfg(not(feature = "blocking"))]
use governor::{
    clock::{QuantaClock, QuantaInstant},
    middleware::NoOpMiddleware,
    state::{InMemoryState, NotKeyed},
    Quota, RateLimiter,
};
#[cfg(feature = "blocking")]
use reqwest::blocking::{Client, RequestBuilder, Response};
use reqwest::header::USER_AGENT;
#[cfg(not(feature = "blocking"))]
use reqwest::{Client, RequestBuilder, Response};
use reqwest::{Method, Url};
use std::{env, fmt};
#[cfg(not(feature = "blocking"))]
use std::{num::NonZeroU32, sync::Arc, time::Duration};

use crate::{error::types::ErrorResponse, Error, Result};

pub struct Config {
    pub(crate) user_agent: String,
    pub(crate) api_key: String,
    pub(crate) base_url: Url,
    pub(crate) client: Client,
    #[cfg(not(feature = "blocking"))]
    limiter: Arc<RateLimiter<NotKeyed, InMemoryState, QuantaClock, NoOpMiddleware<QuantaInstant>>>,
}

impl Config {
    /// Creates a new [`Config`].
    pub fn new(api_key: &str, client: Client) -> Self {
        let env_base_url = env::var("RESEND_BASE_URL")
            .map_or_else(
                |_| Url::parse("https://api.resend.com"),
                |env_var| Url::parse(env_var.as_str()),
            )
            .expect("env variable `RESEND_BASE_URL` should be a valid URL");

        let env_user_agent = format!("{}/{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));

        // ==== Rate limiting is a non-blocking thing only ====
        #[cfg(not(feature = "blocking"))]
        let rate_limit_per_sec = env::var("RESEND_RATE_LIMIT")
            .unwrap_or_else(|_| "9".to_owned())
            .parse::<u32>()
            .expect("env variable `RESEND_RATE_LIMIT` should be a valid u32");

        #[cfg(not(feature = "blocking"))]
        let quota = Quota::with_period(Duration::from_millis(1100))
            .expect("Valid quota")
            .allow_burst(
                NonZeroU32::new(rate_limit_per_sec).expect("Rate limit is a valid non zero u32"),
            );

        #[cfg(not(feature = "blocking"))]
        let limiter = Arc::new(RateLimiter::direct(quota));
        // ====================================================

        Self {
            user_agent: env_user_agent,
            api_key: api_key.to_owned(),
            base_url: env_base_url,
            client,
            #[cfg(not(feature = "blocking"))]
            limiter,
        }
    }

    /// Constructs a new [`RequestBuilder`].
    pub fn build(&self, method: Method, path: &str) -> RequestBuilder {
        let path = self
            .base_url
            .join(path)
            .expect("should be a valid API endpoint");

        self.client
            .request(method, path)
            .bearer_auth(self.api_key.as_str())
            .header(USER_AGENT, self.user_agent.as_str())
    }

    #[maybe_async::maybe_async]
    pub async fn send(&self, request: RequestBuilder) -> Result<Response> {
        #[cfg(not(feature = "blocking"))]
        {
            let jitter =
                governor::Jitter::new(Duration::from_millis(10), Duration::from_millis(50));
            self.limiter.until_ready_with_jitter(jitter).await;
        }

        let request = request.build()?;

        let response = self.client.execute(request).await?;

        match response.status() {
            x if x.is_client_error() || x.is_server_error() => {
                let error = response.json::<ErrorResponse>().await?;
                Err(Error::Resend(error))
            }
            _ => Ok(response),
        }
    }
}

impl fmt::Debug for Config {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Don't output API key.
        f.debug_struct("Client")
            .field("api_key", &"re_*********")
            .field("user_agent", &self.user_agent.as_str())
            .field("base_url", &self.base_url.as_str())
            .finish_non_exhaustive()
    }
}
