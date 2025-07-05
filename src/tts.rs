//! Text-to-Speech synthesis helpers.
//!
//! When the `tts` feature is enabled we compile with the `xd-tts` crate and stream real
//! audio bytes.  Without the feature, handlers will return 501 Not Implemented so the
//! rest of the application can compile on platforms without ONNX runtime.

#[cfg(feature = "tts")]
mod with_tts {
    //! Runtime implementation that calls the ElevenLabs HTTP API.
    //!
    //! Requirements (runtime):
    //! - Set the environment variable `ELEVENLABS_API_KEY` with your API key.
    //! - Optionally set `ELEVENLABS_VOICE_ID` to override the default voice.
    //!
    //! The function returns raw WAV bytes that can be streamed to clients.

    use anyhow::{anyhow, Context, Result};
    use reqwest::header::{ACCEPT, CONTENT_TYPE};
    use serde_json::json;

    /// Synthesize `text` using ElevenLabs and return WAV bytes.
    pub async fn synthesize_to_wav(text: &str, voice: &str) -> Result<Vec<u8>> {
        // Grab credentials from environment.
        let api_key = std::env::var("ELEVENLABS_API_KEY")
            .context("ELEVENLABS_API_KEY environment variable not set")?;

        // Determine which voice ID to use: explicit argument wins, else env var, else fallback.
        let voice_id = if voice.is_empty() {
            std::env::var("ELEVENLABS_VOICE_ID").unwrap_or_else(|_| "EXAVITQu4vr4xnSDxMaL".to_string())
        } else {
            voice.to_string()
        };

        let url = format!(
            "https://api.elevenlabs.io/v1/text-to-speech/{}?optimize_streaming_latency=0",
            voice_id
        );

        let client = reqwest::Client::new();
        let resp = client
            .post(url)
            .header("xi-api-key", api_key)
            .header(CONTENT_TYPE, "application/json")
            .header(ACCEPT, "audio/wav")
            .json(&json!({ "text": text }))
            .send()
            .await
            .context("Failed to send TTS request")?;

        if !resp.status().is_success() {
            let status = resp.status();
            let err_body = resp.text().await.unwrap_or_default();
            return Err(anyhow!("TTS request failed: {} - {}", status, err_body));
        }

        let bytes = resp
            .bytes()
            .await
            .context("Failed to read TTS response body")?;

        Ok(bytes.to_vec())
    }
}

#[cfg(not(feature = "tts"))]
mod without_tts {
    use anyhow::{anyhow, Result};

    pub async fn synthesize_to_wav(_text: &str, _voice: &str) -> Result<Vec<u8>> {
        Err(anyhow!("TTS feature not compiled; rebuild with --features tts"))
    }
}

// Re-export symbol depending on feature flag
#[cfg(feature = "tts")]
pub use with_tts::synthesize_to_wav;
#[cfg(not(feature = "tts"))]
pub use without_tts::synthesize_to_wav; 