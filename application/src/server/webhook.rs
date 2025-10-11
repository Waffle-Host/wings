use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};

use tracing::{debug, error};

use crate::config::Config;
use crate::server::event::Event;

pub async fn send(config: &Config, event: &Event) {
    if config.api.webhook.url.is_empty() {
        return;
    }

    debug!(event = ?event, "sending webhook");

    let client = reqwest::Client::new();
    let mut req = client
        .post(&config.api.webhook.url)
        .header(CONTENT_TYPE, "application/json");

    if !config.api.webhook.token.is_empty() {
        req = req.header(
            AUTHORIZATION,
            format!("Bearer {}", config.api.webhook.token),
        );
    }

    match req.json(&event).send().await {
        Ok(response) => {
            if !response.status().is_success() {
                error!(status = %response.status(), "webhook request failed");
            }
        }
        Err(err) => {
            error!(error = ?err, "failed to send webhook");
        }
    }
}
