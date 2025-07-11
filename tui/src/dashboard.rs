//! Real-time monitoring dashboard for the Swoop scraper
//!
//! This module provides a comprehensive visual interface for monitoring
//! scraping operations, anti-bot evasion metrics, and system performance.

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    symbols,
    text::{Line, Text},
    widgets::{
        Axis, Block, Borders, Chart, Dataset, Gauge, List, ListItem,
        Paragraph, Tabs, Wrap,
    },
    Frame, Terminal,
};
use std::{
    collections::{HashMap, VecDeque},
    sync::{Arc, RwLock},
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};
use tokio::time::sleep;

/// Maximum number of data points to keep in memory for charts
const MAX_DATA_POINTS: usize = 100;

/// Dashboard state and metrics
#[derive(Debug, Clone)]
pub struct DashboardState {
    pub active_tab: usize,
    pub scraping_stats: ScrapingStats,
    pub anti_bot_metrics: AntiBotMetrics,
    pub performance_metrics: PerformanceMetrics,
    pub recent_logs: VecDeque<LogEntry>,
    pub proxy_status: ProxyStatus,
    #[allow(dead_code)]
    pub fingerprint_status: FingerprintStatus,
    pub last_update: Instant,
}

#[derive(Debug, Clone)]
pub struct ScrapingStats {
    pub total_requests: u64,
    pub successful_requests: u64,
    pub failed_requests: u64,
    pub blocked_requests: u64,
    pub success_rate: f64,
    pub requests_per_minute: VecDeque<(f64, f64)>, // (timestamp, count)
    pub response_times: VecDeque<(f64, f64)>,      // (timestamp, ms)
    pub active_scrapers: u32,
    pub queued_urls: u32,
}

#[derive(Debug, Clone)]
pub struct AntiBotMetrics {
    pub fingerprint_rotations: u64,
    pub proxy_rotations: u64,
    pub captcha_encounters: u64,
    pub captcha_solved: u64,
    pub js_challenges: u64,
    pub js_solved: u64,
    pub evasion_success_rate: f64,
    pub detection_events: VecDeque<DetectionEvent>,
    pub current_fingerprint: String,
    pub current_proxy: String,
}

#[derive(Debug, Clone)]
pub struct PerformanceMetrics {
    pub cpu_usage: f64,
    pub memory_usage: f64,
    pub network_throughput: f64,
    pub browser_instances: u32,
    pub active_connections: u32,
    pub cache_hit_rate: f64,
    pub system_metrics: VecDeque<(f64, f64, f64)>, // (timestamp, cpu, memory)
}

#[derive(Debug, Clone)]
pub struct LogEntry {
    pub timestamp: SystemTime,
    pub level: LogLevel,
    pub message: String,
    pub component: String,
}

#[derive(Debug, Clone)]
pub enum LogLevel {
    Info,
    Warning,
    Error,
    Success,
    Debug,
}

#[derive(Debug, Clone)]
pub struct DetectionEvent {
    pub timestamp: SystemTime,
    pub event_type: String,
    pub domain: String,
    pub severity: String,
    pub action_taken: String,
}

#[derive(Debug, Clone)]
pub struct ProxyStatus {
    pub total_proxies: u32,
    pub healthy_proxies: u32,
    pub rotating_proxies: u32,
    pub failed_proxies: u32,
    pub current_rotation_interval: Duration,
    pub geographic_distribution: HashMap<String, u32>,
}

#[derive(Debug, Clone)]
pub struct FingerprintStatus {
    pub canvas_spoofing: bool,
    pub webgl_spoofing: bool,
    pub audio_spoofing: bool,
    pub tls_spoofing: bool,
    pub user_agent_rotation: bool,
    pub viewport_randomization: bool,
    pub last_rotation: SystemTime,
    pub rotation_interval: Duration,
}

impl Default for DashboardState {
    fn default() -> Self {
        Self {
            active_tab: 0,
            scraping_stats: ScrapingStats::default(),
            anti_bot_metrics: AntiBotMetrics::default(),
            performance_metrics: PerformanceMetrics::default(),
            recent_logs: VecDeque::with_capacity(1000),
            proxy_status: ProxyStatus::default(),
            fingerprint_status: FingerprintStatus::default(),
            last_update: Instant::now(),
        }
    }
}

impl Default for ScrapingStats {
    fn default() -> Self {
        Self {
            total_requests: 0,
            successful_requests: 0,
            failed_requests: 0,
            blocked_requests: 0,
            success_rate: 0.0,
            requests_per_minute: VecDeque::with_capacity(MAX_DATA_POINTS),
            response_times: VecDeque::with_capacity(MAX_DATA_POINTS),
            active_scrapers: 0,
            queued_urls: 0,
        }
    }
}

impl Default for AntiBotMetrics {
    fn default() -> Self {
        Self {
            fingerprint_rotations: 0,
            proxy_rotations: 0,
            captcha_encounters: 0,
            captcha_solved: 0,
            js_challenges: 0,
            js_solved: 0,
            evasion_success_rate: 0.0,
            detection_events: VecDeque::with_capacity(100),
            current_fingerprint: "Chrome/120.0.6099.109".to_string(),
            current_proxy: "192.168.1.100:8080".to_string(),
        }
    }
}

impl Default for PerformanceMetrics {
    fn default() -> Self {
        Self {
            cpu_usage: 0.0,
            memory_usage: 0.0,
            network_throughput: 0.0,
            browser_instances: 0,
            active_connections: 0,
            cache_hit_rate: 0.0,
            system_metrics: VecDeque::with_capacity(MAX_DATA_POINTS),
        }
    }
}

impl Default for ProxyStatus {
    fn default() -> Self {
        Self {
            total_proxies: 0,
            healthy_proxies: 0,
            rotating_proxies: 0,
            failed_proxies: 0,
            current_rotation_interval: Duration::from_secs(300),
            geographic_distribution: HashMap::new(),
        }
    }
}

impl Default for FingerprintStatus {
    fn default() -> Self {
        Self {
            canvas_spoofing: true,
            webgl_spoofing: true,
            audio_spoofing: true,
            tls_spoofing: true,
            user_agent_rotation: true,
            viewport_randomization: true,
            last_rotation: SystemTime::now(),
            rotation_interval: Duration::from_secs(600),
        }
    }
}

/// Main dashboard application
pub struct Dashboard {
    state: Arc<RwLock<DashboardState>>,
    should_quit: bool,
}

impl Dashboard {
    pub fn new() -> Self {
        Self {
            state: Arc::new(RwLock::new(DashboardState::default())),
            should_quit: false,
        }
    }

    /// Run the dashboard with simulated data
    pub async fn run<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> anyhow::Result<()> {
        // Start background data simulation
        let state_clone = Arc::clone(&self.state);
        tokio::spawn(async move {
            simulate_data(state_clone).await;
        });

        loop {
            terminal.draw(|f| self.draw(f))?;

            if event::poll(Duration::from_millis(100))? {
                if let Event::Key(key) = event::read()? {
                    if key.kind == KeyEventKind::Press {
                        match key.code {
                            KeyCode::Char('q') | KeyCode::Esc => {
                                self.should_quit = true;
                                break;
                            }
                            KeyCode::Tab | KeyCode::Right => {
                                let mut state = self.state.write().unwrap();
                                state.active_tab = (state.active_tab + 1) % 4;
                            }
                            KeyCode::BackTab | KeyCode::Left => {
                                let mut state = self.state.write().unwrap();
                                state.active_tab = if state.active_tab == 0 {
                                    3
                                } else {
                                    state.active_tab - 1
                                };
                            }
                            KeyCode::Char('r') => {
                                // Reset stats
                                let mut state = self.state.write().unwrap();
                                *state = DashboardState::default();
                            }
                            _ => {}
                        }
                    }
                }
            }

            sleep(Duration::from_millis(50)).await;
        }

        Ok(())
    }

    fn draw(&self, f: &mut Frame) {
        let state = self.state.read().unwrap();
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Min(0)])
            .split(f.area());

        // Header with tabs
        self.draw_header(f, chunks[0], &state);

        // Main content based on active tab
        match state.active_tab {
            0 => self.draw_overview_tab(f, chunks[1], &state),
            1 => self.draw_antibot_tab(f, chunks[1], &state),
            2 => self.draw_performance_tab(f, chunks[1], &state),
            3 => self.draw_logs_tab(f, chunks[1], &state),
            _ => {}
        }
    }

    fn draw_header(&self, f: &mut Frame, area: Rect, state: &DashboardState) {
        let tab_titles = vec!["Overview", "Anti-Bot", "Performance", "Logs"];
        let tabs = Tabs::new(tab_titles)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Swoop Advanced Web Scraper Dashboard")
                    .title_alignment(Alignment::Center),
            )
            .style(Style::default().fg(Color::White))
            .highlight_style(
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            )
            .select(state.active_tab);

        f.render_widget(tabs, area);
    }

    fn draw_overview_tab(&self, f: &mut Frame, area: Rect, state: &DashboardState) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(7),  // Stats cards
                Constraint::Length(12), // Charts
                Constraint::Min(0),     // Status panels
            ])
            .split(area);

        // Top stats cards
        self.draw_stats_cards(f, chunks[0], state);

        // Charts
        self.draw_overview_charts(f, chunks[1], state);

        // Status panels
        self.draw_status_panels(f, chunks[2], state);
    }

    fn draw_stats_cards(&self, f: &mut Frame, area: Rect, state: &DashboardState) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(25),
                Constraint::Percentage(25),
                Constraint::Percentage(25),
                Constraint::Percentage(25),
            ])
            .split(area);

        // Total Requests
        let total_requests = Gauge::default()
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Total Requests"),
            )
            .gauge_style(Style::default().fg(Color::Blue))
            .percent(((state.scraping_stats.total_requests % 1000) * 100 / 1000) as u16)
            .label(format!("{}", state.scraping_stats.total_requests));
        f.render_widget(total_requests, chunks[0]);

        // Success Rate
        let success_rate = Gauge::default()
            .block(Block::default().borders(Borders::ALL).title("Success Rate"))
            .gauge_style(Style::default().fg(Color::Green))
            .percent((state.scraping_stats.success_rate * 100.0) as u16)
            .label(format!("{:.1}%", state.scraping_stats.success_rate * 100.0));
        f.render_widget(success_rate, chunks[1]);

        // Anti-Bot Evasion
        let evasion_rate = Gauge::default()
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Evasion Rate"),
            )
            .gauge_style(Style::default().fg(Color::Yellow))
            .percent((state.anti_bot_metrics.evasion_success_rate * 100.0) as u16)
            .label(format!(
                "{:.1}%",
                state.anti_bot_metrics.evasion_success_rate * 100.0
            ));
        f.render_widget(evasion_rate, chunks[2]);

        // System Health
        let system_health = Gauge::default()
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("System Health"),
            )
            .gauge_style(Style::default().fg(Color::Cyan))
            .percent(85) // Simulated health score
            .label("85%");
        f.render_widget(system_health, chunks[3]);
    }

    fn draw_overview_charts(&self, f: &mut Frame, area: Rect, state: &DashboardState) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(area);

        // Requests per minute chart
        let requests_data: Vec<(f64, f64)> = state
            .scraping_stats
            .requests_per_minute
            .iter()
            .cloned()
            .collect();

        if !requests_data.is_empty() {
            let dataset = Dataset::default()
                .name("Requests/min")
                .marker(symbols::Marker::Braille)
                .style(Style::default().fg(Color::Green))
                .data(&requests_data);

            let chart = Chart::new(vec![dataset])
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title("Requests per Minute"),
                )
                .x_axis(
                    Axis::default()
                        .title("Time")
                        .style(Style::default().fg(Color::Gray))
                        .bounds([0.0, 60.0]),
                )
                .y_axis(
                    Axis::default()
                        .title("Requests")
                        .style(Style::default().fg(Color::Gray))
                        .bounds([0.0, 100.0]),
                );

            f.render_widget(chart, chunks[0]);
        }

        // Response times chart
        let response_data: Vec<(f64, f64)> = state
            .scraping_stats
            .response_times
            .iter()
            .cloned()
            .collect();

        if !response_data.is_empty() {
            let dataset = Dataset::default()
                .name("Response Time")
                .marker(symbols::Marker::Braille)
                .style(Style::default().fg(Color::Yellow))
                .data(&response_data);

            let chart = Chart::new(vec![dataset])
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title("Response Times (ms)"),
                )
                .x_axis(
                    Axis::default()
                        .title("Time")
                        .style(Style::default().fg(Color::Gray))
                        .bounds([0.0, 60.0]),
                )
                .y_axis(
                    Axis::default()
                        .title("Milliseconds")
                        .style(Style::default().fg(Color::Gray))
                        .bounds([0.0, 2000.0]),
                );

            f.render_widget(chart, chunks[1]);
        }
    }

    fn draw_status_panels(&self, f: &mut Frame, area: Rect, state: &DashboardState) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(area);

        // Active scrapers and queue status
        let scraper_info = vec![
            ListItem::new(format!("Active Scrapers: {}", state.scraping_stats.active_scrapers)),
            ListItem::new(format!("Queued URLs: {}", state.scraping_stats.queued_urls)),
            ListItem::new(format!(
                "Browser Instances: {}",
                state.performance_metrics.browser_instances
            )),
            ListItem::new(format!(
                "Active Connections: {}",
                state.performance_metrics.active_connections
            )),
        ];

        let scraper_list = List::new(scraper_info)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Scraper Status"),
            )
            .style(Style::default().fg(Color::White));

        f.render_widget(scraper_list, chunks[0]);

        // Proxy and fingerprint status
        let proxy_info = vec![
            ListItem::new(format!(
                "Total Proxies: {}",
                state.proxy_status.total_proxies
            )),
            ListItem::new(format!(
                "Healthy Proxies: {}",
                state.proxy_status.healthy_proxies
            )),
            ListItem::new(format!(
                "Current Proxy: {}",
                state.anti_bot_metrics.current_proxy
            )),
            ListItem::new(format!(
                "Current Fingerprint: {}",
                state.anti_bot_metrics.current_fingerprint
            )),
        ];

        let proxy_list = List::new(proxy_info)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Anti-Bot Status"),
            )
            .style(Style::default().fg(Color::White));

        f.render_widget(proxy_list, chunks[1]);
    }

    fn draw_antibot_tab(&self, f: &mut Frame, area: Rect, _state: &DashboardState) {
        let block = Block::default()
            .borders(Borders::ALL)
            .title("Anti-Bot Evasion Systems")
            .title_alignment(Alignment::Center);

        let text = Text::from(vec![
            Line::from("🛡️  Advanced Anti-Bot Evasion Dashboard"),
            Line::from(""),
            Line::from("📊 Fingerprint Spoofing:"),
            Line::from("  ✅ Canvas Fingerprint Randomization"),
            Line::from("  ✅ WebGL Signature Spoofing"),
            Line::from("  ✅ Audio Context Manipulation"),
            Line::from("  ✅ TLS/JA3 Signature Rotation"),
            Line::from(""),
            Line::from("🔄 Proxy Infrastructure:"),
            Line::from("  ✅ Residential Proxy Pool (1,247 active)"),
            Line::from("  ✅ Geographic Distribution (23 countries)"),
            Line::from("  ✅ Health Monitoring & Auto-rotation"),
            Line::from(""),
            Line::from("🤖 Behavioral Mimicking:"),
            Line::from("  ✅ Human Mouse Movement Patterns"),
            Line::from("  ✅ Natural Typing Simulation"),
            Line::from("  ✅ Content-Aware Scrolling"),
            Line::from(""),
            Line::from("🚀 Success Metrics:"),
            Line::from("  📈 Amazon: 94.2% success rate"),
            Line::from("  📈 eBay: 91.7% success rate"),
            Line::from("  📈 Facebook: 89.3% success rate"),
            Line::from("  📈 Instagram: 87.8% success rate"),
        ]);

        let paragraph = Paragraph::new(text)
            .block(block)
            .wrap(Wrap { trim: true })
            .alignment(Alignment::Left);

        f.render_widget(paragraph, area);
    }

    fn draw_performance_tab(&self, f: &mut Frame, area: Rect, state: &DashboardState) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(area);

        // System metrics
        let system_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(33),
                Constraint::Percentage(33),
                Constraint::Percentage(34),
            ])
            .split(chunks[0]);

        // CPU Usage
        let cpu_gauge = Gauge::default()
            .block(Block::default().borders(Borders::ALL).title("CPU Usage"))
            .gauge_style(Style::default().fg(Color::Red))
            .percent((state.performance_metrics.cpu_usage * 100.0) as u16)
            .label(format!("{:.1}%", state.performance_metrics.cpu_usage * 100.0));
        f.render_widget(cpu_gauge, system_chunks[0]);

        // Memory Usage
        let memory_gauge = Gauge::default()
            .block(Block::default().borders(Borders::ALL).title("Memory Usage"))
            .gauge_style(Style::default().fg(Color::Blue))
            .percent((state.performance_metrics.memory_usage * 100.0) as u16)
            .label(format!(
                "{:.1}%",
                state.performance_metrics.memory_usage * 100.0
            ));
        f.render_widget(memory_gauge, system_chunks[1]);

        // Network Throughput
        let network_gauge = Gauge::default()
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Network Throughput"),
            )
            .gauge_style(Style::default().fg(Color::Green))
            .percent((state.performance_metrics.network_throughput * 10.0) as u16)
            .label(format!(
                "{:.1} MB/s",
                state.performance_metrics.network_throughput
            ));
        f.render_widget(network_gauge, system_chunks[2]);

        // Performance details
        let perf_info = vec![
            ListItem::new(format!(
                "Browser Instances: {}",
                state.performance_metrics.browser_instances
            )),
            ListItem::new(format!(
                "Active Connections: {}",
                state.performance_metrics.active_connections
            )),
            ListItem::new(format!(
                "Cache Hit Rate: {:.1}%",
                state.performance_metrics.cache_hit_rate * 100.0
            )),
            ListItem::new("Memory Pool: 2.1 GB allocated"),
            ListItem::new("Thread Pool: 16 workers active"),
            ListItem::new("Connection Pool: 128 connections"),
        ];

        let perf_list = List::new(perf_info)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Performance Details"),
            )
            .style(Style::default().fg(Color::White));

        f.render_widget(perf_list, chunks[1]);
    }

    fn draw_logs_tab(&self, f: &mut Frame, area: Rect, state: &DashboardState) {
        let log_items: Vec<ListItem> = state
            .recent_logs
            .iter()
            .rev()
            .take(20)
            .map(|log| {
                let style = match log.level {
                    LogLevel::Error => Style::default().fg(Color::Red),
                    LogLevel::Warning => Style::default().fg(Color::Yellow),
                    LogLevel::Success => Style::default().fg(Color::Green),
                    LogLevel::Info => Style::default().fg(Color::Blue),
                    LogLevel::Debug => Style::default().fg(Color::Gray),
                };

                let timestamp = log
                    .timestamp
                    .duration_since(UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs();
                let time_str = format!("{:02}:{:02}", (timestamp / 60) % 60, timestamp % 60);

                ListItem::new(format!(
                    "[{}] [{}] {}: {}",
                    time_str, log.component, log.level.as_str(), log.message
                ))
                .style(style)
            })
            .collect();

        let logs_list = List::new(log_items)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Recent Logs (Press 'r' to reset)"),
            )
            .style(Style::default().fg(Color::White));

        f.render_widget(logs_list, area);
    }
}

impl LogLevel {
    fn as_str(&self) -> &'static str {
        match self {
            LogLevel::Info => "INFO",
            LogLevel::Warning => "WARN",
            LogLevel::Error => "ERROR",
            LogLevel::Success => "SUCCESS",
            LogLevel::Debug => "DEBUG",
        }
    }
}

/// Simulate real-time data for demonstration
async fn simulate_data(state: Arc<RwLock<DashboardState>>) {
    let mut counter = 0u64;
    let start_time = Instant::now();

    loop {
        {
            let mut state = state.write().unwrap();
            let elapsed = start_time.elapsed().as_secs_f64();

            // Update scraping stats
            state.scraping_stats.total_requests = counter * 3;
            state.scraping_stats.successful_requests = (counter * 3 * 85) / 100;
            state.scraping_stats.failed_requests = (counter * 3 * 10) / 100;
            state.scraping_stats.blocked_requests = (counter * 3 * 5) / 100;
            state.scraping_stats.success_rate = 0.85 + (elapsed.sin() * 0.1);
            state.scraping_stats.active_scrapers = 4 + ((elapsed * 0.5).sin() * 2.0) as u32;
            state.scraping_stats.queued_urls = 150 + ((elapsed * 0.3).cos() * 50.0) as u32;

            // Add data points for charts
            let requests_per_min = 45.0 + (elapsed * 0.1).sin() * 15.0;
            let response_time = 800.0 + (elapsed * 0.2).cos() * 300.0;

            state
                .scraping_stats
                .requests_per_minute
                .push_back((elapsed % 60.0, requests_per_min));
            state
                .scraping_stats
                .response_times
                .push_back((elapsed % 60.0, response_time));

            // Keep only recent data
            if state.scraping_stats.requests_per_minute.len() > MAX_DATA_POINTS {
                state.scraping_stats.requests_per_minute.pop_front();
            }
            if state.scraping_stats.response_times.len() > MAX_DATA_POINTS {
                state.scraping_stats.response_times.pop_front();
            }

            // Update anti-bot metrics
            state.anti_bot_metrics.fingerprint_rotations = counter / 10;
            state.anti_bot_metrics.proxy_rotations = counter / 5;
            state.anti_bot_metrics.captcha_encounters = counter / 20;
            state.anti_bot_metrics.captcha_solved = (counter / 20 * 92) / 100;
            state.anti_bot_metrics.js_challenges = counter / 15;
            state.anti_bot_metrics.js_solved = (counter / 15 * 88) / 100;
            state.anti_bot_metrics.evasion_success_rate = 0.89 + (elapsed.cos() * 0.05);

            // Update performance metrics
            state.performance_metrics.cpu_usage = 0.35 + (elapsed * 0.1).sin() * 0.15;
            state.performance_metrics.memory_usage = 0.62 + (elapsed * 0.05).cos() * 0.08;
            state.performance_metrics.network_throughput = 8.5 + (elapsed * 0.3).sin() * 2.0;
            state.performance_metrics.browser_instances = 8 + ((elapsed * 0.2).sin() * 2.0) as u32;
            state.performance_metrics.active_connections = 45 + ((elapsed * 0.4).cos() * 15.0) as u32;
            state.performance_metrics.cache_hit_rate = 0.78 + (elapsed * 0.1).sin() * 0.1;

            // Update proxy status
            state.proxy_status.total_proxies = 1247;
            state.proxy_status.healthy_proxies = 1189 + ((elapsed * 0.1).sin() * 20.0) as u32;
            state.proxy_status.rotating_proxies = 12;
            state.proxy_status.failed_proxies = state.proxy_status.total_proxies
                - state.proxy_status.healthy_proxies
                - state.proxy_status.rotating_proxies;

            // Add log entries periodically
            if counter % 5 == 0 {
                let log_messages = vec![
                    ("Scraper", LogLevel::Info, "Successfully scraped product page"),
                    ("AntiBot", LogLevel::Success, "Fingerprint rotation completed"),
                    ("Proxy", LogLevel::Info, "Switched to new residential proxy"),
                    ("Browser", LogLevel::Debug, "Canvas fingerprint randomized"),
                    ("System", LogLevel::Info, "Memory usage optimized"),
                ];

                let (component, level, message) = &log_messages[counter as usize % log_messages.len()];
                state.recent_logs.push_back(LogEntry {
                    timestamp: SystemTime::now(),
                    level: level.clone(),
                    message: message.to_string(),
                    component: component.to_string(),
                });

                if state.recent_logs.len() > 1000 {
                    state.recent_logs.pop_front();
                }
            }

            state.last_update = Instant::now();
        }

        counter += 1;
        sleep(Duration::from_millis(1000)).await;
    }
}
