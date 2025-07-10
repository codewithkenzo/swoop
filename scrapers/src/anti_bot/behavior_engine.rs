//! Behavioral mimicking engine for human-like automation
//! 
//! This module implements comprehensive human behavior simulation:
//! - Human mouse movement simulation with Bézier curves
//! - Natural typing patterns with realistic variance
//! - Content-aware scroll behavior and timing
//! - Timing variation engine with statistical variance
//! - Session & navigation simulation

use std::time::{Duration, Instant};
use rand::{Rng, thread_rng};
use serde::{Deserialize, Serialize};
use tokio::time::sleep;

/// Behavioral engine for simulating human-like interactions
pub struct BehaviorEngine {
    mouse_simulator: MouseSimulator,
    typing_simulator: TypingSimulator,
    scroll_simulator: ScrollSimulator,
    timing_engine: TimingEngine,
    navigation_simulator: NavigationSimulator,
}

impl BehaviorEngine {
    /// Create a new behavior engine
    pub async fn new() -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        Ok(Self {
            mouse_simulator: MouseSimulator::new(),
            typing_simulator: TypingSimulator::new(),
            scroll_simulator: ScrollSimulator::new(),
            timing_engine: TimingEngine::new(),
            navigation_simulator: NavigationSimulator::new(),
        })
    }

    /// Apply human-like timing delay
    pub async fn apply_timing_delay(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let delay = self.timing_engine.calculate_natural_delay().await;
        sleep(delay).await;
        Ok(())
    }

    /// Simulate human mouse movement to target coordinates
    pub async fn simulate_mouse_movement(&self, start: (f64, f64), end: (f64, f64)) -> Vec<MouseEvent> {
        self.mouse_simulator.generate_natural_movement(start, end).await
    }

    /// Simulate human typing for given text
    pub async fn simulate_typing(&self, text: &str) -> Vec<TypingEvent> {
        self.typing_simulator.generate_typing_sequence(text).await
    }

    /// Simulate natural scroll behavior
    pub async fn simulate_scroll(&self, scroll_distance: i32, content_height: u32) -> Vec<ScrollEvent> {
        self.scroll_simulator.generate_scroll_sequence(scroll_distance, content_height).await
    }

    /// Simulate page navigation behavior
    pub async fn simulate_navigation(&self, navigation_type: NavigationType) -> NavigationBehavior {
        self.navigation_simulator.generate_navigation_behavior(navigation_type).await
    }

    /// Generate comprehensive behavioral profile
    pub async fn generate_behavior_profile(&self) -> BehaviorProfile {
        BehaviorProfile {
            mouse_characteristics: self.mouse_simulator.get_characteristics().await,
            typing_characteristics: self.typing_simulator.get_characteristics().await,
            scroll_characteristics: self.scroll_simulator.get_characteristics().await,
            timing_characteristics: self.timing_engine.get_characteristics().await,
        }
    }
}

/// Mouse movement simulator with Bézier curves
pub struct MouseSimulator {
    movement_speed: f64,
    acceleration_factor: f64,
    jitter_intensity: f64,
    pause_probability: f64,
}

impl MouseSimulator {
    fn new() -> Self {
        let mut rng = thread_rng();
        Self {
            movement_speed: rng.gen_range(200.0..800.0), // pixels per second
            acceleration_factor: rng.gen_range(0.8..1.2),
            jitter_intensity: rng.gen_range(0.1..0.3),
            pause_probability: rng.gen_range(0.05..0.15),
        }
    }

    /// Generate natural mouse movement using Bézier curves
    async fn generate_natural_movement(&self, start: (f64, f64), end: (f64, f64)) -> Vec<MouseEvent> {
        let mut events = Vec::new();
        let distance = ((end.0 - start.0).powi(2) + (end.1 - start.1).powi(2)).sqrt();
        let duration = (distance / self.movement_speed) * 1000.0; // milliseconds
        
        // Generate control points for Bézier curve
        let control_points = self.generate_control_points(start, end);
        
        // Sample points along the curve
        let num_points = (duration / 16.0) as usize; // ~60 FPS
        let mut current_time = 0.0;
        
        for i in 0..num_points {
            let t = i as f64 / num_points as f64;
            let point = self.bezier_curve(t, &control_points);
            
            // Add natural jitter
            let jittered_point = self.add_jitter(point);
            
            events.push(MouseEvent {
                x: jittered_point.0,
                y: jittered_point.1,
                timestamp: current_time,
                event_type: MouseEventType::Move,
            });

            // Occasionally add micro-pauses
            if thread_rng().gen_bool(self.pause_probability) {
                current_time += thread_rng().gen_range(10.0..50.0);
            }
            
            current_time += 16.0; // 60 FPS
        }

        events
    }

    /// Generate control points for natural Bézier curve
    fn generate_control_points(&self, start: (f64, f64), end: (f64, f64)) -> Vec<(f64, f64)> {
        let mut rng = thread_rng();
        let mid_x = (start.0 + end.0) / 2.0;
        let mid_y = (start.1 + end.1) / 2.0;
        
        // Add randomness to control points for natural curve
        let offset_x = rng.gen_range(-50.0..50.0);
        let offset_y = rng.gen_range(-50.0..50.0);
        
        vec![
            start,
            (mid_x + offset_x, mid_y + offset_y),
            end,
        ]
    }

    /// Calculate point on cubic Bézier curve
    fn bezier_curve(&self, t: f64, control_points: &[(f64, f64)]) -> (f64, f64) {
        if control_points.len() == 3 {
            // Quadratic Bézier curve
            let p0 = control_points[0];
            let p1 = control_points[1];
            let p2 = control_points[2];
            
            let x = (1.0 - t).powi(2) * p0.0 + 2.0 * (1.0 - t) * t * p1.0 + t.powi(2) * p2.0;
            let y = (1.0 - t).powi(2) * p0.1 + 2.0 * (1.0 - t) * t * p1.1 + t.powi(2) * p2.1;
            
            (x, y)
        } else {
            // Linear interpolation fallback
            let start = control_points[0];
            let end = control_points[control_points.len() - 1];
            (
                start.0 + t * (end.0 - start.0),
                start.1 + t * (end.1 - start.1),
            )
        }
    }

    /// Add natural jitter to mouse movement
    fn add_jitter(&self, point: (f64, f64)) -> (f64, f64) {
        let mut rng = thread_rng();
        let jitter_x = rng.gen_range(-self.jitter_intensity..self.jitter_intensity);
        let jitter_y = rng.gen_range(-self.jitter_intensity..self.jitter_intensity);
        
        (point.0 + jitter_x, point.1 + jitter_y)
    }

    async fn get_characteristics(&self) -> MouseCharacteristics {
        MouseCharacteristics {
            movement_speed: self.movement_speed,
            acceleration_factor: self.acceleration_factor,
            jitter_intensity: self.jitter_intensity,
            pause_probability: self.pause_probability,
        }
    }
}

/// Typing pattern simulator
pub struct TypingSimulator {
    base_typing_speed: f64, // characters per minute
    speed_variance: f64,
    error_rate: f64,
    pause_after_word_probability: f64,
}

impl TypingSimulator {
    fn new() -> Self {
        let mut rng = thread_rng();
        Self {
            base_typing_speed: rng.gen_range(200.0..400.0), // WPM * 5
            speed_variance: rng.gen_range(0.2..0.4),
            error_rate: rng.gen_range(0.01..0.05),
            pause_after_word_probability: rng.gen_range(0.1..0.3),
        }
    }

    /// Generate realistic typing sequence
    async fn generate_typing_sequence(&self, text: &str) -> Vec<TypingEvent> {
        let mut events = Vec::new();
        let mut current_time = 0.0;
        let chars: Vec<char> = text.chars().collect();

        for (i, &char) in chars.iter().enumerate() {
            // Calculate typing delay with variance
            let base_delay = 60000.0 / self.base_typing_speed; // milliseconds per char
            let variance = thread_rng().gen_range(-self.speed_variance..self.speed_variance);
            let char_delay = base_delay * (1.0 + variance);

            // Simulate typing errors and corrections
            if thread_rng().gen_bool(self.error_rate) {
                // Type wrong character first
                let wrong_char = self.generate_wrong_character(char);
                events.push(TypingEvent {
                    character: wrong_char,
                    timestamp: current_time,
                    event_type: TypingEventType::KeyPress,
                });
                current_time += char_delay * 0.5;

                // Backspace to correct
                events.push(TypingEvent {
                    character: '\u{0008}', // backspace
                    timestamp: current_time,
                    event_type: TypingEventType::Backspace,
                });
                current_time += char_delay * 0.3;
            }

            // Type the correct character
            events.push(TypingEvent {
                character: char,
                timestamp: current_time,
                event_type: TypingEventType::KeyPress,
            });

            current_time += char_delay;

            // Pause after words
            if char.is_whitespace() && thread_rng().gen_bool(self.pause_after_word_probability) {
                current_time += thread_rng().gen_range(100.0..500.0);
            }

            // Longer pause after sentences
            if char == '.' || char == '!' || char == '?' {
                current_time += thread_rng().gen_range(200.0..800.0);
            }
        }

        events
    }

    /// Generate a plausible wrong character
    fn generate_wrong_character(&self, intended_char: char) -> char {
        // Simulate common typing errors (adjacent keys, etc.)
        let keyboard_layout = "qwertyuiopasdfghjklzxcvbnm";
        let mut rng = thread_rng();
        
        if let Some(pos) = keyboard_layout.find(intended_char.to_ascii_lowercase()) {
            // Pick an adjacent character
            let adjacent_chars = match pos {
                0..=9 => &keyboard_layout[0..10], // top row
                10..=18 => &keyboard_layout[10..19], // middle row
                19..=25 => &keyboard_layout[19..26], // bottom row
                _ => keyboard_layout,
            };
            
            adjacent_chars.chars().nth(rng.gen_range(0..adjacent_chars.len())).unwrap_or('x')
        } else {
            // Random character as fallback
            char::from(rng.gen_range(b'a'..=b'z'))
        }
    }

    async fn get_characteristics(&self) -> TypingCharacteristics {
        TypingCharacteristics {
            typing_speed: self.base_typing_speed,
            speed_variance: self.speed_variance,
            error_rate: self.error_rate,
        }
    }
}

/// Scroll behavior simulator
pub struct ScrollSimulator {
    scroll_speed: f64,
    pause_probability: f64,
    reading_speed: f64, // pixels per second when "reading"
}

impl ScrollSimulator {
    fn new() -> Self {
        let mut rng = thread_rng();
        Self {
            scroll_speed: rng.gen_range(300.0..800.0),
            pause_probability: rng.gen_range(0.2..0.4),
            reading_speed: rng.gen_range(50.0..150.0),
        }
    }

    /// Generate natural scroll sequence
    async fn generate_scroll_sequence(&self, total_distance: i32, content_height: u32) -> Vec<ScrollEvent> {
        let mut events = Vec::new();
        let mut current_position = 0;
        let mut current_time = 0.0;

        while current_position < total_distance.abs() {
            // Determine scroll chunk size
            let chunk_size = thread_rng().gen_range(50..200);
            let actual_chunk = chunk_size.min(total_distance.abs() - current_position);

            // Calculate scroll duration
            let duration = (actual_chunk as f64 / self.scroll_speed) * 1000.0;

            events.push(ScrollEvent {
                delta_y: if total_distance > 0 { actual_chunk } else { -actual_chunk },
                timestamp: current_time,
                scroll_type: ScrollType::Wheel,
            });

            current_position += actual_chunk;
            current_time += duration;

            // Simulate reading pauses
            if thread_rng().gen_bool(self.pause_probability) {
                let reading_pause = thread_rng().gen_range(500.0..2000.0);
                current_time += reading_pause;
            }
        }

        events
    }

    async fn get_characteristics(&self) -> ScrollCharacteristics {
        ScrollCharacteristics {
            scroll_speed: self.scroll_speed,
            pause_probability: self.pause_probability,
            reading_speed: self.reading_speed,
        }
    }
}

/// Timing engine for natural delays
pub struct TimingEngine {
    base_delay: Duration,
    variance_factor: f64,
    context_aware: bool,
}

impl TimingEngine {
    fn new() -> Self {
        let mut rng = thread_rng();
        Self {
            base_delay: Duration::from_millis(rng.gen_range(2000..8000)),
            variance_factor: rng.gen_range(0.3..0.7),
            context_aware: true,
        }
    }

    /// Calculate natural delay with statistical variance
    async fn calculate_natural_delay(&self) -> Duration {
        let mut rng = thread_rng();
        let base_ms = self.base_delay.as_millis() as f64;
        
        // Apply variance using normal distribution approximation
        let variance = base_ms * self.variance_factor;
        let random_factor = rng.gen_range(-1.0..1.0);
        let actual_delay = base_ms + (variance * random_factor);
        
        // Ensure minimum delay
        Duration::from_millis(actual_delay.max(500.0) as u64)
    }

    async fn get_characteristics(&self) -> TimingCharacteristics {
        TimingCharacteristics {
            base_delay_ms: self.base_delay.as_millis() as u64,
            variance_factor: self.variance_factor,
            context_aware: self.context_aware,
        }
    }
}

/// Navigation behavior simulator
pub struct NavigationSimulator {
    tab_switch_probability: f64,
    back_navigation_probability: f64,
    new_tab_probability: f64,
}

impl NavigationSimulator {
    fn new() -> Self {
        let mut rng = thread_rng();
        Self {
            tab_switch_probability: rng.gen_range(0.1..0.3),
            back_navigation_probability: rng.gen_range(0.05..0.15),
            new_tab_probability: rng.gen_range(0.02..0.08),
        }
    }

    /// Generate navigation behavior pattern
    async fn generate_navigation_behavior(&self, nav_type: NavigationType) -> NavigationBehavior {
        let mut rng = thread_rng();
        
        match nav_type {
            NavigationType::PageLoad => NavigationBehavior {
                actions: vec![
                    NavigationAction::LoadPage,
                    NavigationAction::WaitForLoad(Duration::from_millis(rng.gen_range(1000..3000))),
                    NavigationAction::ScrollToTop,
                ],
                referrer_behavior: self.generate_referrer_behavior().await,
            },
            NavigationType::LinkClick => NavigationBehavior {
                actions: vec![
                    NavigationAction::MouseHover(Duration::from_millis(rng.gen_range(200..800))),
                    NavigationAction::Click,
                    NavigationAction::WaitForLoad(Duration::from_millis(rng.gen_range(800..2000))),
                ],
                referrer_behavior: self.generate_referrer_behavior().await,
            },
            NavigationType::BackNavigation => NavigationBehavior {
                actions: vec![
                    NavigationAction::BackButton,
                    NavigationAction::WaitForLoad(Duration::from_millis(rng.gen_range(500..1500))),
                ],
                referrer_behavior: ReferrerBehavior::KeepReferrer,
            },
        }
    }

    /// Generate referrer behavior
    async fn generate_referrer_behavior(&self) -> ReferrerBehavior {
        let mut rng = thread_rng();
        let behavior_type = rng.gen_range(0..4);
        
        match behavior_type {
            0 => ReferrerBehavior::DirectNavigation,
            1 => ReferrerBehavior::SearchEngine(self.generate_search_referrer()),
            2 => ReferrerBehavior::SocialMedia(self.generate_social_referrer()),
            _ => ReferrerBehavior::KeepReferrer,
        }
    }

    fn generate_search_referrer(&self) -> String {
        let search_engines = vec![
            "https://www.google.com/",
            "https://www.bing.com/",
            "https://duckduckgo.com/",
        ];
        let mut rng = thread_rng();
        search_engines[rng.gen_range(0..search_engines.len())].to_string()
    }

    fn generate_social_referrer(&self) -> String {
        let social_sites = vec![
            "https://www.facebook.com/",
            "https://twitter.com/",
            "https://www.linkedin.com/",
        ];
        let mut rng = thread_rng();
        social_sites[rng.gen_range(0..social_sites.len())].to_string()
    }
}

// Event structures

/// Mouse event representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MouseEvent {
    pub x: f64,
    pub y: f64,
    pub timestamp: f64,
    pub event_type: MouseEventType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MouseEventType {
    Move,
    Click,
    Hover,
}

/// Typing event representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypingEvent {
    pub character: char,
    pub timestamp: f64,
    pub event_type: TypingEventType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TypingEventType {
    KeyPress,
    Backspace,
    Pause,
}

/// Scroll event representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScrollEvent {
    pub delta_y: i32,
    pub timestamp: f64,
    pub scroll_type: ScrollType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ScrollType {
    Wheel,
    Trackpad,
    Scrollbar,
}

/// Navigation types
#[derive(Debug, Clone)]
pub enum NavigationType {
    PageLoad,
    LinkClick,
    BackNavigation,
}

/// Navigation behavior pattern
#[derive(Debug, Clone)]
pub struct NavigationBehavior {
    pub actions: Vec<NavigationAction>,
    pub referrer_behavior: ReferrerBehavior,
}

#[derive(Debug, Clone)]
pub enum NavigationAction {
    LoadPage,
    WaitForLoad(Duration),
    MouseHover(Duration),
    Click,
    ScrollToTop,
    BackButton,
}

#[derive(Debug, Clone)]
pub enum ReferrerBehavior {
    DirectNavigation,
    SearchEngine(String),
    SocialMedia(String),
    KeepReferrer,
}

// Characteristic structures

/// Complete behavior profile
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BehaviorProfile {
    pub mouse_characteristics: MouseCharacteristics,
    pub typing_characteristics: TypingCharacteristics,
    pub scroll_characteristics: ScrollCharacteristics,
    pub timing_characteristics: TimingCharacteristics,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MouseCharacteristics {
    pub movement_speed: f64,
    pub acceleration_factor: f64,
    pub jitter_intensity: f64,
    pub pause_probability: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypingCharacteristics {
    pub typing_speed: f64,
    pub speed_variance: f64,
    pub error_rate: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScrollCharacteristics {
    pub scroll_speed: f64,
    pub pause_probability: f64,
    pub reading_speed: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimingCharacteristics {
    pub base_delay_ms: u64,
    pub variance_factor: f64,
    pub context_aware: bool,
}
