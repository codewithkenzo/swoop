# Text-to-Speech (TTS) Integration Guide 🎙️

> **Add voice capabilities to Swoop with open-source TTS libraries**

Based on 2024 research, here are the best open-source TTS options for Rust web applications with real-time streaming audio capabilities.

## 🎯 Recommended TTS Stack

### 1. **xd-tts** (Pure Rust) ⭐ **Recommended**

**Why Choose xd-tts:**
- Nearly pure Rust implementation
- ONNX Runtime integration for neural models
- Designed for web applications
- Active development in 2024

**Installation:**
```toml
[dependencies]
xd-tts = "0.1"
onnxruntime = "1.16"
tokio = { version = "1.0", features = ["full"] }
```

**Basic Implementation:**
```rust
use xd_tts::{TtsEngine, Voice, SynthesisConfig};
use tokio::io::AsyncWriteExt;

pub struct SwoopTtsService {
    engine: TtsEngine,
    config: SynthesisConfig,
}

impl SwoopTtsService {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let engine = TtsEngine::new("models/tts-model.onnx").await?;
        let config = SynthesisConfig {
            sample_rate: 22050,
            voice: Voice::default(),
            speed: 1.0,
            pitch: 1.0,
        };
        
        Ok(Self { engine, config })
    }
    
    pub async fn synthesize_streaming(&self, text: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let audio_data = self.engine.synthesize(text, &self.config).await?;
        Ok(audio_data)
    }
    
    pub async fn synthesize_to_stream(&self, text: &str, mut output: impl AsyncWriteExt + Unpin) -> Result<(), Box<dyn std::error::Error>> {
        let chunks = self.engine.synthesize_chunked(text, &self.config).await?;
        
        for chunk in chunks {
            output.write_all(&chunk).await?;
            output.flush().await?;
            
            // Add small delay for streaming effect
            tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
        }
        
        Ok(())
    }
}
```

### 2. **Piper TTS** (External Service) 🔄 **Alternative**

**Why Consider Piper:**
- High-quality voice synthesis
- Multiple language support
- Well-documented API
- Active community

**Integration via HTTP API:**
```rust
use reqwest::Client;
use serde_json::json;

pub struct PiperTtsService {
    client: Client,
    base_url: String,
}

impl PiperTtsService {
    pub fn new(base_url: String) -> Self {
        Self {
            client: Client::new(),
            base_url,
        }
    }
    
    pub async fn synthesize(&self, text: &str, voice: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let response = self.client
            .post(&format!("{}/api/tts", self.base_url))
            .json(&json!({
                "text": text,
                "voice": voice,
                "output_format": "wav"
            }))
            .send()
            .await?;
            
        let audio_bytes = response.bytes().await?;
        Ok(audio_bytes.to_vec())
    }
}
```

### 3. **Microsoft Edge Read Aloud** (Cloud API) ☁️ **Fallback**

**For production with multiple voices:**
```rust
use reqwest::Client;

pub struct EdgeTtsService {
    client: Client,
}

impl EdgeTtsService {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }
    
    pub async fn synthesize(&self, text: &str, voice: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let ssml = format!(
            r#"<speak version="1.0" xmlns="http://www.w3.org/2001/10/synthesis" xml:lang="en-US">
                <voice name="{}">
                    {}
                </voice>
            </speak>"#,
            voice, text
        );
        
        let response = self.client
            .post("https://speech.platform.bing.com/synthesize")
            .header("Content-Type", "application/ssml+xml")
            .header("X-Microsoft-OutputFormat", "audio-16khz-32kbitrate-mono-mp3")
            .body(ssml)
            .send()
            .await?;
            
        let audio_bytes = response.bytes().await?;
        Ok(audio_bytes.to_vec())
    }
}
```

## 🎵 Streaming Audio Implementation

### Server-Side Streaming (Axum)

```rust
use axum::{
    extract::{Query, State},
    http::{header, StatusCode},
    response::{Response, IntoResponse},
    routing::post,
    Router,
};
use tokio_util::io::ReaderStream;
use std::collections::HashMap;

#[derive(serde::Deserialize)]
struct TtsRequest {
    text: String,
    voice: Option<String>,
    format: Option<String>,
}

async fn tts_stream(
    Query(params): Query<TtsRequest>,
    State(tts_service): State<SwoopTtsService>,
) -> impl IntoResponse {
    let text = params.text;
    let voice = params.voice.unwrap_or_else(|| "default".to_string());
    
    match tts_service.synthesize_streaming(&text).await {
        Ok(audio_data) => {
            let stream = ReaderStream::new(std::io::Cursor::new(audio_data));
            
            Response::builder()
                .status(StatusCode::OK)
                .header(header::CONTENT_TYPE, "audio/wav")
                .header(header::CACHE_CONTROL, "no-cache")
                .header("X-Voice-Used", voice)
                .body(axum::body::Body::from_stream(stream))
                .unwrap()
        }
        Err(_) => {
            Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(axum::body::Body::empty())
                .unwrap()
        }
    }
}

// Add to your Axum router
pub fn tts_routes() -> Router<AppState> {
    Router::new()
        .route("/api/tts/stream", post(tts_stream))
        .route("/api/tts/synthesize", post(tts_synthesize))
}
```

### Real-time Chat TTS Integration

```rust
use tokio::sync::broadcast;

pub struct ChatTtsIntegration {
    tts_service: SwoopTtsService,
    audio_sender: broadcast::Sender<Vec<u8>>,
}

impl ChatTtsIntegration {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let tts_service = SwoopTtsService::new().await?;
        let (audio_sender, _) = broadcast::channel(100);
        
        Ok(Self {
            tts_service,
            audio_sender,
        })
    }
    
    pub async fn process_chat_response(&self, response_stream: impl Stream<Item = String>) {
        let mut accumulated_text = String::new();
        let mut sentence_buffer = String::new();
        
        pin_mut!(response_stream);
        
        while let Some(chunk) = response_stream.next().await {
            accumulated_text.push_str(&chunk);
            sentence_buffer.push_str(&chunk);
            
            // Check for sentence boundaries
            if sentence_buffer.contains('.') || sentence_buffer.contains('!') || sentence_buffer.contains('?') {
                let sentences: Vec<&str> = sentence_buffer.split(|c| c == '.' || c == '!' || c == '?').collect();
                
                for sentence in &sentences[..sentences.len()-1] {
                    if !sentence.trim().is_empty() {
                        // Synthesize completed sentence
                        if let Ok(audio) = self.tts_service.synthesize_streaming(sentence.trim()).await {
                            let _ = self.audio_sender.send(audio);
                        }
                    }
                }
                
                // Keep the last incomplete sentence
                sentence_buffer = sentences.last().unwrap_or(&"").to_string();
            }
        }
        
        // Process any remaining text
        if !sentence_buffer.trim().is_empty() {
            if let Ok(audio) = self.tts_service.synthesize_streaming(sentence_buffer.trim()).await {
                let _ = self.audio_sender.send(audio);
            }
        }
    }
    
    pub fn subscribe_audio(&self) -> broadcast::Receiver<Vec<u8>> {
        self.audio_sender.subscribe()
    }
}
```

## 🎛️ Frontend Integration

### React Audio Player Component

```typescript
import React, { useEffect, useRef, useState } from 'react';

interface AudioPlayerProps {
  audioUrl?: string;
  autoPlay?: boolean;
  onEnded?: () => void;
}

export function StreamingAudioPlayer({ audioUrl, autoPlay = false, onEnded }: AudioPlayerProps) {
  const audioRef = useRef<HTMLAudioElement>(null);
  const [isPlaying, setIsPlaying] = useState(false);
  const [isLoading, setIsLoading] = useState(false);

  useEffect(() => {
    if (audioUrl && audioRef.current) {
      setIsLoading(true);
      audioRef.current.src = audioUrl;
      
      if (autoPlay) {
        audioRef.current.play().catch(console.error);
      }
    }
  }, [audioUrl, autoPlay]);

  const handlePlay = () => {
    setIsPlaying(true);
    setIsLoading(false);
  };

  const handlePause = () => {
    setIsPlaying(false);
  };

  const handleEnded = () => {
    setIsPlaying(false);
    onEnded?.();
  };

  const togglePlayPause = () => {
    if (audioRef.current) {
      if (isPlaying) {
        audioRef.current.pause();
      } else {
        audioRef.current.play().catch(console.error);
      }
    }
  };

  return (
    <div className="flex items-center space-x-2">
      <button
        onClick={togglePlayPause}
        disabled={!audioUrl || isLoading}
        className="p-2 rounded-full bg-blue-500 text-white hover:bg-blue-600 disabled:opacity-50"
      >
        {isLoading ? (
          <div className="animate-spin w-4 h-4 border-2 border-white border-t-transparent rounded-full" />
        ) : isPlaying ? (
          <PauseIcon className="w-4 h-4" />
        ) : (
          <PlayIcon className="w-4 h-4" />
        )}
      </button>
      
      <audio
        ref={audioRef}
        onPlay={handlePlay}
        onPause={handlePause}
        onEnded={handleEnded}
        onLoadStart={() => setIsLoading(true)}
        onCanPlay={() => setIsLoading(false)}
      />
      
      {isPlaying && (
        <div className="flex items-center space-x-1">
          <div className="w-1 h-4 bg-green-500 animate-pulse" />
          <div className="w-1 h-3 bg-green-500 animate-pulse delay-75" />
          <div className="w-1 h-5 bg-green-500 animate-pulse delay-150" />
          <span className="text-sm text-gray-600">Playing...</span>
        </div>
      )}
    </div>
  );
}
```

### TTS Hook for Chat Integration

```typescript
import { useState, useCallback } from 'react';

export function useTTS() {
  const [isGenerating, setIsGenerating] = useState(false);
  const [audioUrl, setAudioUrl] = useState<string | null>(null);

  const generateSpeech = useCallback(async (text: string, voice?: string) => {
    setIsGenerating(true);
    
    try {
      const response = await fetch('/api/tts/stream', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({
          text,
          voice: voice || 'default',
          format: 'wav'
        }),
      });

      if (response.ok) {
        const audioBlob = await response.blob();
        const url = URL.createObjectURL(audioBlob);
        setAudioUrl(url);
      }
    } catch (error) {
      console.error('TTS generation failed:', error);
    } finally {
      setIsGenerating(false);
    }
  }, []);

  const clearAudio = useCallback(() => {
    if (audioUrl) {
      URL.revokeObjectURL(audioUrl);
      setAudioUrl(null);
    }
  }, [audioUrl]);

  return {
    generateSpeech,
    clearAudio,
    audioUrl,
    isGenerating
  };
}
```

## 🔧 Configuration & Settings

### TTS Settings Component

```typescript
interface TTSSettings {
  enabled: boolean;
  voice: string;
  speed: number;
  pitch: number;
  autoPlay: boolean;
}

export function TTSSettingsPanel() {
  const [settings, setSettings] = useState<TTSSettings>({
    enabled: false,
    voice: 'default',
    speed: 1.0,
    pitch: 1.0,
    autoPlay: false
  });

  return (
    <div className="space-y-4 p-4 border rounded-lg">
      <h3 className="font-semibold">Text-to-Speech Settings</h3>
      
      <div className="flex items-center space-x-2">
        <input
          type="checkbox"
          checked={settings.enabled}
          onChange={(e) => setSettings(prev => ({ ...prev, enabled: e.target.checked }))}
        />
        <label>Enable Text-to-Speech</label>
      </div>

      {settings.enabled && (
        <>
          <div>
            <label className="block text-sm font-medium mb-1">Voice</label>
            <select
              value={settings.voice}
              onChange={(e) => setSettings(prev => ({ ...prev, voice: e.target.value }))}
              className="w-full border rounded px-3 py-2"
            >
              <option value="default">Default</option>
              <option value="female">Female</option>
              <option value="male">Male</option>
              <option value="neural">Neural (Premium)</option>
            </select>
          </div>

          <div>
            <label className="block text-sm font-medium mb-1">Speed: {settings.speed}x</label>
            <input
              type="range"
              min="0.5"
              max="2.0"
              step="0.1"
              value={settings.speed}
              onChange={(e) => setSettings(prev => ({ ...prev, speed: parseFloat(e.target.value) }))}
              className="w-full"
            />
          </div>

          <div className="flex items-center space-x-2">
            <input
              type="checkbox"
              checked={settings.autoPlay}
              onChange={(e) => setSettings(prev => ({ ...prev, autoPlay: e.target.checked }))}
            />
            <label>Auto-play responses</label>
          </div>
        </>
      )}
    </div>
  );
}
```

## 📋 Implementation Checklist

### Phase 1: Basic TTS (Week 1)
- [ ] Choose TTS library (xd-tts recommended)
- [ ] Implement basic synthesis endpoint
- [ ] Add frontend audio player component
- [ ] Test with simple text inputs

### Phase 2: Streaming Integration (Week 2)
- [ ] Implement streaming audio synthesis
- [ ] Integrate with chat responses
- [ ] Add real-time sentence processing
- [ ] Implement audio caching

### Phase 3: Advanced Features (Week 3)
- [ ] Multiple voice support
- [ ] Voice settings panel
- [ ] Audio quality optimization
- [ ] Performance monitoring

### Phase 4: Production Polish (Week 4)
- [ ] Error handling and fallbacks
- [ ] Audio compression
- [ ] CDN integration for audio files
- [ ] User preference persistence

## 🎯 Performance Targets

- **Synthesis Latency**: <2s for typical sentences
- **Audio Quality**: 22kHz, 16-bit minimum
- **Memory Usage**: <100MB for TTS models
- **Concurrent Users**: Support 50+ simultaneous synthesis

## 🚀 Getting Started

1. **Install xd-tts**: Add to your `Cargo.toml`
2. **Download models**: Get ONNX models for voice synthesis
3. **Add endpoints**: Implement `/api/tts/stream` route
4. **Frontend integration**: Add audio player components
5. **Test thoroughly**: Ensure quality and performance

---

**Ready to give Swoop a voice? Start with xd-tts for the best Rust integration! 🎙️** 