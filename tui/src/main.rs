mod dashboard;
mod dashboard_main;

use crossterm::{
    event::{Event, KeyCode, KeyEventKind, EventStream},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{
        Axis, Block, Borders, Cell, Chart, Dataset, List, ListItem, ListState, Paragraph, Row,
        Table, Tabs, Wrap,
    },
    Frame, Terminal,
};
use futures::StreamExt;
use std::{
    collections::{HashMap, VecDeque},
    fs,
    io::{self, stdout},
    panic,
    path::PathBuf,
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};
use tokio::sync::Semaphore;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sysinfo::System;
use tracing::{error, info, instrument};
use tracing::level_filters::LevelFilter;
use tracing_subscriber::{
    fmt,
    layer::SubscriberExt,
    util::SubscriberInitExt,
    EnvFilter,
};

fn setup_logging() -> Result<(), Box<dyn std::error::Error>> {
    let log_dir = PathBuf::from("logs");
    fs::create_dir_all(&log_dir)?;
    let log_file = fs::File::create(log_dir.join("swoop-tui.log"))?;

    let filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::INFO.into())
        .from_env_lossy();

    tracing_subscriber::registry()
        .with(filter)
        .with(fmt::layer().with_writer(log_file).json())
        .init();
    
    Ok(())
}


/// Simple HTTP fetch function to avoid dependency issues
#[instrument]
async fn fetch_url_simple(url: &str) -> Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync>> {
    info!("Fetching URL: {}", url);
    let client = reqwest::Client::new();
    let response = client.get(url).send().await?;
    let bytes = response.bytes().await?;
    info!("Finished fetching URL: {}", url);
    Ok(bytes.to_vec())
}

/// Application state for the TUI dashboard
#[derive(Debug, Clone)]
struct AppState {
    /// URL queue scroll position
    target_scroll: usize,
    /// Current active tab
    current_tab: usize,
    /// Metrics data
    metrics: Metrics,
    /// Proxy pool status
    proxy_status: ProxyStatus,
    /// Log entries
    logs: LogBuffer,
    /// Control state
    controls: ControlState,
    /// URL queue
    targets: VecDeque<Target>,
    /// Scraped data storage
    scraped_data: VecDeque<ScrapedData>,
    /// Export state
    export_state: ExportState,
    /// Settings state
    settings_state: SettingsState,
    /// Whether the app should quit
    should_quit: bool,
    /// System information
    system_info: SystemInfo,
    /// Flag to trigger data export
    export_requested: bool,
    /// Show the startup banner
    show_banner: bool,
    /// Currently focused pane
    focused_pane: FocusedPane,
    /// Is the app in input mode
    input_mode: bool,
    /// Buffer for the input box
    input_buffer: String,
}

/// System information
#[derive(Debug, Clone, Default)]
struct SystemInfo {
    cpu_usage: f32,
    mem_usage: u64,
    uptime: u64,
    threads: usize,
}

#[derive(Debug, Clone, PartialEq)]
enum TargetStatus {
    Pending,
    InProgress,
    Completed,
    Failed,
}

#[derive(Debug, Clone)]
struct Target {
    url: String,
    status: TargetStatus,
    response_time: Option<u64>,
    status_code: Option<u16>,
}

/// Metrics data for monitoring
#[derive(Debug, Clone)]
struct Metrics {
    requests_per_second: VecDeque<f64>,
    request_timestamps: VecDeque<Instant>,
    success_rate: VecDeque<f64>,
    response_time: VecDeque<f64>,
    total_requests: u64,
    total_successful: u64,
    total_failed: u64,
    active_connections: u32,
    data_processed: u64,
}

/// Proxy pool status information
#[derive(Debug, Clone)]
struct ProxyStatus {
    total_proxies: u32,
    active_proxies: u32,
    failed_proxies: u32,
    residential_health: f32,
    datacenter_health: f32,
    mobile_health: f32,
    current_rotation: u32,
}

/// Log buffer for system events
#[derive(Debug, Clone)]
struct LogBuffer {
    entries: VecDeque<LogEntry>,
    max_size: usize,
    scroll_position: usize,
}

/// Individual log entry
#[derive(Debug, Clone)]
struct LogEntry {
    timestamp: Instant,
    level: LogLevel,
    message: String,
}

/// Log levels
#[derive(Debug, Clone)]
#[allow(dead_code)]
enum LogLevel {
    Info,
    Warning,
    Error,
    Success,
}

/// Control state for user interactions
#[derive(Debug, Clone)]
#[allow(dead_code)]
struct ControlState {
    is_paused: bool,
    rate_limit: f64,
    concurrency: usize,
    url_file: PathBuf,
    request_timeout: u64,
    export_dir: PathBuf,
    auto_export: bool,
}

/// Scraped data entry
#[derive(Debug, Clone, Serialize, Deserialize)]
struct ScrapedData {
    url: String,
    timestamp: DateTime<Utc>,
    content: String,
    status_code: Option<u16>,
    headers: HashMap<String, String>,
    response_time: u64,
    content_length: usize,
    content_type: Option<String>,
    title: Option<String>,
    success: bool,
    error: Option<String>,
}

/// Export format options
#[derive(Debug, Clone, Copy, PartialEq)]
enum ExportFormat {
    Json,
    Csv,
}

/// Export state
#[derive(Debug, Clone)]
struct ExportState {
    format: ExportFormat,
    file_path: String,
    is_exporting: bool,
    progress: u8,
    status: String,
    recent_exports: VecDeque<String>,
    scroll_position: usize,
}

/// Settings UI state
#[derive(Debug, Clone)]
struct SettingsState {
    selected_index: usize,
    is_editing: bool,
    edit_value: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum FocusedPane {
    SystemStatus,
    QuickStats,
    InfrastructureStatus,
    RecentActivity,
}

impl Default for FocusedPane {
    fn default() -> Self {
        FocusedPane::SystemStatus
    }
}

impl Default for Metrics {
    fn default() -> Self {
        Self {
            requests_per_second: VecDeque::with_capacity(60),
            request_timestamps: VecDeque::with_capacity(1000),
            success_rate: VecDeque::with_capacity(60),
            response_time: VecDeque::with_capacity(60),
            total_requests: 0,
            total_successful: 0,
            total_failed: 0,
            active_connections: 0,
            data_processed: 0,
        }
    }
}

impl Default for ProxyStatus {
    fn default() -> Self {
        Self {
            total_proxies: 100,
            active_proxies: 85,
            failed_proxies: 15,
            residential_health: 0.92,
            datacenter_health: 0.88,
            mobile_health: 0.85,
            current_rotation: 0,
        }
    }
}

impl Default for LogBuffer {
    fn default() -> Self {
        Self {
            entries: VecDeque::new(),
            max_size: 1000,
            scroll_position: 0,
        }
    }
}

impl LogBuffer {
    fn add_entry(&mut self, level: LogLevel, message: String) {
        if self.entries.len() >= self.max_size {
            self.entries.pop_front();
        }
        self.entries.push_back(LogEntry {
            timestamp: Instant::now(),
            level,
            message,
        });
    }
}

impl Default for ControlState {
    fn default() -> Self {
        Self {
            is_paused: false,
            rate_limit: 1.0,
            concurrency: 10,
            url_file: PathBuf::from("test_urls.txt"),
            request_timeout: 30,
            export_dir: PathBuf::from("exports"),
            auto_export: false,
        }
    }
}

impl Default for ExportState {
    fn default() -> Self {
        Self {
            format: ExportFormat::Json,
            file_path: "export.json".to_string(),
            is_exporting: false,
            progress: 0,
            status: "Ready".to_string(),
            recent_exports: VecDeque::new(),
            scroll_position: 0,
        }
    }
}

impl Default for SettingsState {
    fn default() -> Self {
        Self {
            selected_index: 0,
            is_editing: false,
            edit_value: String::new(),
        }
    }
}

impl ExportFormat {
    fn as_str(&self) -> &str {
        match self {
            ExportFormat::Json => "JSON",
            ExportFormat::Csv => "CSV",
        }
    }
}

impl AppState {
    fn new() -> Self {
        let mut logs = LogBuffer::default();
        logs.add_entry(
            LogLevel::Info,
            "Swoop TUI Dashboard initialized".to_string(),
        );
        logs.add_entry(LogLevel::Success, "All systems operational".to_string());

        Self {
            target_scroll: 0,
            current_tab: 0,
            metrics: Metrics::default(),
            proxy_status: ProxyStatus::default(),
            logs,
            controls: ControlState::default(),
            targets: VecDeque::new(),
            scraped_data: VecDeque::with_capacity(10000),
            export_state: ExportState::default(),
            settings_state: SettingsState::default(),
            should_quit: false,
            system_info: SystemInfo::default(),
            export_requested: false,
            show_banner: true,
            focused_pane: FocusedPane::default(),
            input_mode: false,
            input_buffer: String::new(),
        }
    }

    #[instrument(skip(self))]
    fn handle_key_event(&mut self, key: KeyCode) {
        info!(?key, "Handling key event");

        if self.input_mode {
            match key {
                KeyCode::Esc => {
                    self.input_mode = false;
                    self.input_buffer.clear();
                }
                KeyCode::Enter => {
                    // TODO: Implement intelligent URL parsing
                    let urls: Vec<&str> = self.input_buffer.lines().collect();
                    let url_count = urls.len();
                    for url in urls {
                        if !url.trim().is_empty() {
                            self.targets.push_back(Target {
                                url: url.trim().to_string(),
                                status: TargetStatus::Pending,
                                response_time: None,
                                status_code: None,
                            });
                        }
                    }
                    self.logs.add_entry(LogLevel::Info, format!("Added {} URLs from input", url_count));
                    self.input_mode = false;
                    self.input_buffer.clear();
                }
                KeyCode::Char(c) => {
                    self.input_buffer.push(c);
                }
                KeyCode::Backspace => {
                    self.input_buffer.pop();
                }
                _ => {}
            }
        } else {
            match key {
                KeyCode::Char('q') => {
                    self.should_quit = true;
                    info!("Quit key pressed, should_quit set to true");
                }
                KeyCode::Char('i') => {
                    self.input_mode = true;
                }
                KeyCode::Tab => {
                    self.current_tab = (self.current_tab + 1) % 7;
                }
                KeyCode::BackTab => {
                    self.current_tab = (self.current_tab + 6) % 7;
                }
                KeyCode::Up => {
                    self.scroll(-1);
                }
                KeyCode::Down => {
                    self.scroll(1);
                }
                KeyCode::Char('1') => self.current_tab = 0,
                KeyCode::Char('2') => self.current_tab = 1,
                KeyCode::Char('3') => self.current_tab = 2,
                KeyCode::Char('4') => self.current_tab = 3,
                KeyCode::Char('5') => self.current_tab = 4,
                KeyCode::Char('6') => self.current_tab = 5,
                KeyCode::Char('7') => self.current_tab = 6,
                KeyCode::Char(' ') => {
                    self.controls.is_paused = !self.controls.is_paused;
                    let state = if self.controls.is_paused {
                        "paused"
                    } else {
                        "resumed"
                    };
                    self.logs
                        .add_entry(LogLevel::Info, format!("Scraping {}", state));
                }
                KeyCode::Char('+') => {
                    self.controls.rate_limit = (self.controls.rate_limit + 10.0).min(500.0);
                }
                KeyCode::Char('-') => {
                    self.controls.rate_limit = (self.controls.rate_limit - 10.0).max(10.0);
                }
                KeyCode::Left => self.navigate_panes(-1),
                KeyCode::Right => self.navigate_panes(1),
                KeyCode::Char('l') => {
                    self.load_urls_from_file();
                }
                KeyCode::Char('e') => {
                    self.current_tab = 5; // Export tab
                }
                KeyCode::Char('s') => {
                    self.current_tab = 6; // Settings tab
                }
                KeyCode::Char('d') => {
                    // Launch advanced dashboard
                    tokio::spawn(async {
                        if let Err(e) = dashboard_main::run_dashboard().await {
                            eprintln!("Dashboard error: {}", e);
                        }
                    });
                }
                KeyCode::Char('f') => {
                    if self.current_tab == 5 {
                        self.export_state.format = match self.export_state.format {
                            ExportFormat::Json => ExportFormat::Csv,
                            ExportFormat::Csv => ExportFormat::Json,
                        };
                        self.export_state.file_path = match self.export_state.format {
                            ExportFormat::Json => "export.json".to_string(),
                            ExportFormat::Csv => "export.csv".to_string(),
                        };
                    }
                }
                KeyCode::Enter => {
                    if self.current_tab == 5 {
                        self.export_requested = true;
                    }
                }
                _ => {}
            }
        }
    }

    fn scroll(&mut self, direction: i32) {
        match self.current_tab {
            3 => { // Logs
                let len = self.logs.entries.len();
                if len > 0 {
                    let new_pos = self.logs.scroll_position as i32 + direction;
                    self.logs.scroll_position = new_pos.max(0).min((len - 1) as i32) as usize;
                }
            }
            4 => { // Targets
                let len = self.targets.len();
                if len > 0 {
                    let new_pos = self.target_scroll as i32 + direction;
                    self.target_scroll = new_pos.max(0).min((len - 1) as i32) as usize;
                }
            }
            5 => { // Export
                let len = self.export_state.recent_exports.len();
                if len > 0 {
                    let new_pos = self.export_state.scroll_position as i32 + direction;
                    self.export_state.scroll_position = new_pos.max(0).min((len - 1) as i32) as usize;
                }
            }
            _ => {}
        }
    }

    fn navigate_panes(&mut self, direction: i32) {
        let panes = [
            FocusedPane::SystemStatus,
            FocusedPane::InfrastructureStatus,
            FocusedPane::QuickStats,
            FocusedPane::RecentActivity,
        ];
        let current_index = panes.iter().position(|p| p == &self.focused_pane).unwrap_or(0);
        let next_index = (current_index as i32 + direction + panes.len() as i32) % panes.len() as i32;
        self.focused_pane = panes[next_index as usize].clone();
    }

    fn load_urls_from_file(&mut self) {
        let path = &self.controls.url_file;
        if let Ok(contents) = fs::read_to_string(path) {
            for url in contents.lines() {
                if !url.trim().is_empty() {
                    self.targets.push_back(Target {
                        url: url.trim().to_string(),
                        status: TargetStatus::Pending,
                        response_time: None,
                        status_code: None,
                    });
                }
            }
            self.logs.add_entry(
                LogLevel::Success,
                format!("Loaded {} URLs from {:?}", self.targets.len(), path),
            );
        } else {
            self.logs.add_entry(
                LogLevel::Error,
                format!("Failed to load URLs from {:?}", path),
            );
        }
    }
}

async fn scraping_engine(app: Arc<Mutex<AppState>>) {
    info!("Scraping engine started");
    loop {
        let (concurrency, is_paused) = {
            let app_guard = app.lock().unwrap();
            (app_guard.controls.concurrency, app_guard.controls.is_paused)
        };

        if is_paused {
            tokio::time::sleep(Duration::from_millis(500)).await;
            continue;
        }

        let url_to_process_index = {
            let mut app_guard = app.lock().unwrap();
            app_guard.targets.iter().position(|t| t.status == TargetStatus::Pending)
        };

        if let Some(index) = url_to_process_index {
            let url = {
                let mut app_guard = app.lock().unwrap();
                app_guard.targets[index].status = TargetStatus::InProgress;
                app_guard.targets[index].url.clone()
            };
            
            let semaphore = Arc::new(Semaphore::new(concurrency));
            let permit_fut = semaphore.clone().acquire_owned();
            let app_clone = Arc::clone(&app);

            tokio::spawn(async move {
                let _permit = permit_fut.await.unwrap();
                let start_time = Instant::now();
                match fetch_url_simple(&url).await {
                    Ok(data) => {
                        let duration = start_time.elapsed();
                        let mut app_guard = app_clone.lock().unwrap();
                        if let Some(target) = app_guard.targets.get_mut(index) {
                            target.status = TargetStatus::Completed;
                            target.response_time = Some(duration.as_millis() as u64);
                            target.status_code = Some(200);
                        }
                        app_guard.metrics.total_requests += 1;
                        app_guard.metrics.total_successful += 1;
                        app_guard.metrics.request_timestamps.push_back(Instant::now());
                        app_guard.metrics.data_processed += data.len() as u64;
                        app_guard.metrics.response_time.push_back(duration.as_millis() as f64);
                        if app_guard.metrics.response_time.len() > 60 {
                            app_guard.metrics.response_time.pop_front();
                        }
                        app_guard.metrics.success_rate.push_back(1.0);
                        if app_guard.metrics.success_rate.len() > 60 {
                            app_guard.metrics.success_rate.pop_front();
                        }

                        let scraped_entry = ScrapedData {
                            url: url.clone(),
                            timestamp: Utc::now(),
                            content: String::from_utf8_lossy(&data).to_string(),
                            status_code: Some(200),
                            headers: HashMap::new(),
                            response_time: duration.as_millis() as u64,
                            content_length: data.len(),
                            content_type: Some("text/html".to_string()),
                            title: None,
                            success: true,
                            error: None,
                        };
                        app_guard.scraped_data.push_back(scraped_entry);
                        if app_guard.scraped_data.len() > 10000 {
                            app_guard.scraped_data.pop_front();
                        }

                        app_guard.logs.add_entry(
                            LogLevel::Success,
                            format!("Successfully fetched from {}", url),
                        );
                    }
                    Err(e) => {
                        let mut app_guard = app_clone.lock().unwrap();
                        if let Some(target) = app_guard.targets.get_mut(index) {
                            target.status = TargetStatus::Failed;
                            target.response_time = None;
                            target.status_code = None;
                        }
                        app_guard.metrics.total_requests += 1;
                        app_guard.metrics.total_failed += 1;
                        app_guard.metrics.request_timestamps.push_back(Instant::now());
                        app_guard.metrics.success_rate.push_back(0.0);
                        if app_guard.metrics.success_rate.len() > 60 {
                            app_guard.metrics.success_rate.pop_front();
                        }

                        let scraped_entry = ScrapedData {
                            url: url.clone(),
                            timestamp: Utc::now(),
                            content: String::new(),
                            status_code: None,
                            headers: HashMap::new(),
                            response_time: 0,
                            content_length: 0,
                            content_type: None,
                            title: None,
                            success: false,
                            error: Some(e.to_string()),
                        };
                        app_guard.scraped_data.push_back(scraped_entry);
                        if app_guard.scraped_data.len() > 10000 {
                            app_guard.scraped_data.pop_front();
                        }

                        app_guard.logs.add_entry(
                            LogLevel::Error,
                            format!("Failed to fetch from {}: {}", url, e),
                        );
                    }
                }
            });
        } else {
            tokio::time::sleep(Duration::from_secs(1)).await;
        }
    }
}

#[instrument(skip(f, app))]
fn render_dashboard(f: &mut Frame, app: &AppState) {
    if app.show_banner {
        let banner = Paragraph::new("🕸️  Unstoppable Scraper v0.9.3")
            .style(Style::default().fg(Color::Blue).add_modifier(Modifier::BOLD))
            .alignment(ratatui::layout::Alignment::Center);
        f.render_widget(banner, f.area());
        return;
    }

    info!("Rendering dashboard");
    let constraints = if app.input_mode {
        vec![Constraint::Length(3), Constraint::Min(0), Constraint::Length(3)]
    } else {
        vec![Constraint::Length(3), Constraint::Min(0)]
    };

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints)
        .split(f.area());

    let tabs = Tabs::new(vec!["Overview", "Metrics", "Proxies", "Logs", "Targets", "Export", "Settings"])
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Swoop Dashboard"),
        )
        .style(Style::default().fg(Color::White))
        .highlight_style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )
        .select(app.current_tab);
    f.render_widget(tabs, chunks[0]);

    match app.current_tab {
        0 => render_overview(f, chunks[1], app),
        1 => render_metrics(f, chunks[1], app),
        2 => render_proxies(f, chunks[1], app),
        3 => render_logs(f, chunks[1], app),
        4 => render_targets(f, chunks[1], app),
        5 => render_export(f, chunks[1], app),
        6 => render_settings(f, chunks[1], app),
        _ => {}
    }

    if app.input_mode {
        render_input_box(f, chunks[2], app);
    }
}

fn render_input_box(f: &mut Frame, area: Rect, app: &AppState) {
    let input = Paragraph::new(app.input_buffer.as_str())
        .style(Style::default().fg(Color::Yellow))
        .block(Block::default().borders(Borders::ALL).title("Input URLs (Press Esc to cancel, Enter to submit)"));
    f.render_widget(input, area);
    f.set_cursor_position((area.x + app.input_buffer.len() as u16 + 1, area.y + 1));
}

#[instrument(skip(f, app))]
fn render_overview(f: &mut Frame, area: Rect, app: &AppState) {
    info!("Start rendering overview");
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);

    let left_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[0]);

    let right_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[1]);

    let controls = &app.controls;
    let status_text = if controls.is_paused {
        "🔴 PAUSED"
    } else {
        "🟢 RUNNING"
    };

    let active_style = Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD);

    let system_status_block = if app.focused_pane == FocusedPane::SystemStatus {
        Block::default().title("▶ System Status").borders(Borders::ALL).border_style(active_style)
    } else {
        Block::default().title("System Status").borders(Borders::ALL)
    };

    let system_status = Paragraph::new(format!(
        "System Status: {}\n\nControls:\n• Press 'q' to quit\n• Press 'i' to input URLs\n• Press 'Space' to pause/resume\n• Press '+/-' to adjust RPS\n• Press 'l' to load URLs from file\n• Press 'Tab'/'Shift+Tab' to switch tabs\n• Press '←/→' to navigate panes",
        status_text
    ))
    .block(system_status_block)
    .wrap(Wrap { trim: true });
    f.render_widget(system_status, left_chunks[0]);
    info!("Rendered system status");

    let metrics = &app.metrics;
    let rate_limit = controls.rate_limit;
    let stats_text = format!(
        "Total Requests: {}\nSuccessful: {}\nFailed: {}\nActive Connections: {}\nData Processed: {} KB\nRate Limit: {:.1} req/s",
        metrics.total_requests,
        metrics.total_successful,
        metrics.total_failed,
        metrics.active_connections,
        metrics.data_processed / 1024,
        rate_limit
    );

    let quick_stats_block = if app.focused_pane == FocusedPane::QuickStats {
        Block::default().title("▶ Quick Stats").borders(Borders::ALL).border_style(active_style)
    } else {
        Block::default().title("Quick Stats").borders(Borders::ALL)
    };

    let quick_stats = Paragraph::new(stats_text)
        .block(quick_stats_block)
        .wrap(Wrap { trim: true });
    f.render_widget(quick_stats, left_chunks[1]);
    info!("Rendered quick stats");

    let proxy_status = &app.proxy_status;
    let db_status = "🟢 Healthy";
    let proxy_text = format!(
        "Proxy Pool:\n- Total: {}\n- Active: {}\n- Failed: {}\n\nDB Status: {}",
        proxy_status.total_proxies,
        proxy_status.active_proxies,
        proxy_status.failed_proxies,
        db_status
    );

    let infra_status_block = if app.focused_pane == FocusedPane::InfrastructureStatus {
        Block::default().title("▶ Infrastructure Status").borders(Borders::ALL).border_style(active_style)
    } else {
        Block::default().title("Infrastructure Status").borders(Borders::ALL)
    };

    let proxy_status_widget = Paragraph::new(proxy_text)
        .block(infra_status_block)
        .wrap(Wrap { trim: true });
    f.render_widget(proxy_status_widget, right_chunks[0]);
    info!("Rendered proxy status");

    let logs = &app.logs;
    let recent_logs: Vec<ListItem> = logs
        .entries
        .iter()
        .rev()
        .take(10)
        .map(|entry| {
            let style = match entry.level {
                LogLevel::Info => Style::default().fg(Color::Cyan),
                LogLevel::Warning => Style::default().fg(Color::Yellow),
                LogLevel::Error => Style::default().fg(Color::LightRed),
                LogLevel::Success => Style::default().fg(Color::LightGreen),
            };
            ListItem::new(entry.message.clone()).style(style)
        })
        .collect();

    let recent_activity_block = if app.focused_pane == FocusedPane::RecentActivity {
        Block::default().title("▶ Recent Activity").borders(Borders::ALL).border_style(active_style)
    } else {
        Block::default().title("Recent Activity").borders(Borders::ALL)
    };

    let recent_activity = List::new(recent_logs).block(recent_activity_block);
    f.render_widget(recent_activity, right_chunks[1]);
    info!("Rendered recent activity");
    info!("Finished rendering overview");
}

fn render_metrics(f: &mut Frame, area: Rect, app: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);

    let top_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[0]);

    let metrics = &app.metrics;
    if metrics.total_requests == 0 {
        let msg = Paragraph::new("📊 0 metrics yet — waiting for first scrape…")
            .style(Style::default().fg(Color::DarkGray))
            .block(Block::default().borders(Borders::ALL).title("Metrics"));
        f.render_widget(msg, area);
        return;
    }

    if !metrics.requests_per_second.is_empty() {
        let data: Vec<(f64, f64)> = metrics
            .requests_per_second
            .iter()
            .enumerate()
            .map(|(i, &value)| (i as f64, value))
            .collect();

        let dataset = Dataset::default()
            .marker(ratatui::symbols::Marker::Braille)
            .style(Style::default().fg(Color::Cyan))
            .data(&data);

        let chart = Chart::new(vec![dataset])
            .block(
                Block::default()
                    .title("Request Rate (req/s)")
                    .borders(Borders::ALL),
            )
            .x_axis(
                Axis::default()
                    .title("Time (seconds)")
                    .bounds([0.0, 60.0])
                    .style(Style::default().fg(Color::Gray)),
            )
            .y_axis(
                Axis::default()
                    .title("Requests/sec")
                    .bounds([0.0, 10.0])
                    .style(Style::default().fg(Color::Gray)),
            );
        f.render_widget(chart, top_chunks[0]);
    }

    if !metrics.success_rate.is_empty() {
        let data: Vec<(f64, f64)> = metrics
            .success_rate
                .iter()
                .enumerate()
                .map(|(i, &value)| (i as f64, value * 100.0))
                .collect();

            let dataset = Dataset::default()
                .marker(ratatui::symbols::Marker::Braille)
                .style(Style::default().fg(Color::Green))
                .data(&data);

            let chart = Chart::new(vec![dataset])
                .block(
                    Block::default()
                        .title("Success Rate (%)")
                        .borders(Borders::ALL),
                )
                .x_axis(
                    Axis::default()
                        .title("Time (seconds)")
                        .bounds([0.0, 60.0])
                        .style(Style::default().fg(Color::Gray)),
                )
                .y_axis(
                    Axis::default()
                        .title("Success %")
                        .bounds([80.0, 100.0])
                        .style(Style::default().fg(Color::Gray)),
                );
            f.render_widget(chart, top_chunks[1]);
    }

    if !metrics.response_time.is_empty() {
        let data: Vec<(f64, f64)> = metrics
            .response_time
                .iter()
                .enumerate()
                .map(|(i, &value)| (i as f64, value))
                .collect();

            let dataset = Dataset::default()
                .marker(ratatui::symbols::Marker::Braille)
                .style(Style::default().fg(Color::Yellow))
                .data(&data);

            let chart = Chart::new(vec![dataset])
                .block(
                    Block::default()
                        .title("Response Time (ms)")
                        .borders(Borders::ALL),
                )
                .x_axis(
                    Axis::default()
                        .title("Time (seconds)")
                        .bounds([0.0, 60.0])
                        .style(Style::default().fg(Color::Gray)),
                )
                .y_axis(
                    Axis::default()
                        .title("Response Time (ms)")
                        .bounds([0.0, 600.0])
                        .style(Style::default().fg(Color::Gray)),
                );
            f.render_widget(chart, chunks[1]);
    }
}

fn render_proxies(f: &mut Frame, area: Rect, app: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)])
        .split(area);

    let proxy_status = &app.proxy_status;

    let failure_color = if proxy_status.failed_proxies >= 50 {
        Color::Red
    } else if proxy_status.failed_proxies > 0 {
        Color::Rgb(255, 165, 0) // Orange
    } else {
        Color::Green
    };

    let failure_text = format!(
        "⚠ {} proxy failures – rotating…",
        proxy_status.failed_proxies
    );
    let failure_paragraph = Paragraph::new(failure_text)
        .style(Style::default().fg(failure_color))
        .block(Block::default().title("Proxy Pool").borders(Borders::ALL));
    f.render_widget(failure_paragraph, chunks[0]);

    let proxy_data = vec![
        Row::new(vec![
            "Total Proxies".to_string(),
            proxy_status.total_proxies.to_string(),
        ]),
        Row::new(vec![
            "Active Proxies".to_string(),
            proxy_status.active_proxies.to_string(),
        ]),
        Row::new(vec![
            "Failed Proxies".to_string(),
            proxy_status.failed_proxies.to_string(),
        ]),
        Row::new(vec![
            "Current Rotation".to_string(),
            proxy_status.current_rotation.to_string(),
        ]),
        Row::new(vec![
            "Residential Health".to_string(),
            format!("{:.1}%", proxy_status.residential_health * 100.0),
        ]),
        Row::new(vec![
            "Datacenter Health".to_string(),
            format!("{:.1}%", proxy_status.datacenter_health * 100.0),
        ]),
        Row::new(vec![
            "Mobile Health".to_string(),
            format!("{:.1}%", proxy_status.mobile_health * 100.0),
        ]),
    ];

    let proxy_table = Table::new(
        proxy_data,
        [Constraint::Percentage(50), Constraint::Percentage(50)],
    )
    .block(
        Block::default()
            .title("Proxy Details")
            .borders(Borders::ALL),
    )
    .header(Row::new(vec!["Metric", "Value"]).style(Style::default().add_modifier(Modifier::BOLD)));
    f.render_widget(proxy_table, chunks[1]);
}

fn render_logs(f: &mut Frame, area: Rect, app: &AppState) {
    let logs = &app.logs;
    let log_items: Vec<ListItem> = logs
        .entries
        .iter()
        .rev()
        .map(|entry| {
            let style = match entry.level {
                LogLevel::Info => Style::default().fg(Color::Cyan),
                LogLevel::Warning => Style::default().fg(Color::Yellow),
                LogLevel::Error => Style::default().fg(Color::LightRed),
                LogLevel::Success => Style::default().fg(Color::LightGreen),
            };

            let elapsed = entry.timestamp.elapsed();
            let time_str = if elapsed.as_secs() < 60 {
                format!("{}s ago", elapsed.as_secs())
            } else {
                format!("{}m ago", elapsed.as_secs() / 60)
            };

            ListItem::new(format!("[{}] {}", time_str, entry.message)).style(style)
        })
        .collect();

    let mut list_state = ListState::default();
    list_state.select(Some(logs.scroll_position));

    let logs_widget = List::new(log_items)
        .block(Block::default().title("System Logs").borders(Borders::ALL))
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .highlight_symbol(">> ");
    f.render_stateful_widget(logs_widget, area, &mut list_state);
}

fn render_targets(f: &mut Frame, area: Rect, app: &AppState) {
    let header_cells = ["URL", "Status", "Response Time", "Status Code"]
        .iter()
        .map(|h| Cell::from(*h).style(Style::default().fg(Color::Yellow)));
    let header = Row::new(header_cells).height(1).bottom_margin(1);

    let rows = app.targets.iter().map(|target| {
        let status_style = match target.status {
            TargetStatus::Pending => Style::default().fg(Color::DarkGray),
            TargetStatus::InProgress => Style::default().fg(Color::Blue),
            TargetStatus::Completed => Style::default().fg(Color::Green),
            TargetStatus::Failed => Style::default().fg(Color::Red),
        };
        let status_text = format!("{:?}", target.status);
        let response_time_text = target.response_time.map_or("N/A".to_string(), |t| format!("{}ms", t));
        let status_code_text = target.status_code.map_or("N/A".to_string(), |s| s.to_string());

        Row::new(vec![
            Cell::from(target.url.clone()),
            Cell::from(status_text).style(status_style),
            Cell::from(response_time_text),
            Cell::from(status_code_text),
        ])
    });

    let mut table_state = ratatui::widgets::TableState::default();
    table_state.select(Some(app.target_scroll));

    let table = Table::new(
        rows,
        [
            Constraint::Percentage(50),
            Constraint::Length(15),
            Constraint::Length(20),
            Constraint::Length(15),
        ],
    )
    .header(header)
    .block(Block::default().borders(Borders::ALL).title("Targets"))
    .highlight_style(Style::default().add_modifier(Modifier::BOLD))
    .highlight_symbol(">> ");

    f.render_stateful_widget(table, area, &mut table_state);
}

fn render_export(f: &mut Frame, area: Rect, app: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(8),
            Constraint::Length(6),
            Constraint::Min(0),
        ])
        .split(area);

    let export_state = &app.export_state;
    let controls_text = format!(
        "Export Controls:\n\n• Format: {}\n• File: {}\n• Status: {}\n\nPress 'Enter' to export data\nPress 'f' to toggle format (JSON/CSV)",
        export_state.format.as_str(),
        export_state.file_path,
        export_state.status
    );

    let controls = Paragraph::new(controls_text)
        .block(Block::default().title("Export Controls").borders(Borders::ALL))
        .wrap(Wrap { trim: true });
    f.render_widget(controls, chunks[0]);

    let progress_text = if export_state.is_exporting {
        format!("Exporting... {}%", export_state.progress)
    } else {
        "Ready to export".to_string()
    };

    let progress = Paragraph::new(progress_text)
        .block(Block::default().title("Export Progress").borders(Borders::ALL))
        .wrap(Wrap { trim: true });
    f.render_widget(progress, chunks[1]);

    let recent_items: Vec<ListItem> = export_state
        .recent_exports
        .iter()
        .map(|export| ListItem::new(export.as_str()))
        .collect();

    let mut list_state = ListState::default();
    list_state.select(Some(export_state.scroll_position));

    let recent_list = List::new(recent_items)
        .block(Block::default().title("Recent Exports").borders(Borders::ALL))
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .highlight_symbol(">> ");
    f.render_stateful_widget(recent_list, chunks[2], &mut list_state);

    let summary_text = format!(
        "Scraped Data Summary:\n\n• Total Entries: {}\n• Ready for Export",
        app.scraped_data.len()
    );

    let _summary = Paragraph::new(summary_text)
        .block(Block::default().title("Data Summary").borders(Borders::ALL))
        .wrap(Wrap { trim: true });
}

fn render_settings(f: &mut Frame, area: Rect, app: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);

    let controls = &app.controls;
    let settings_data = vec![
        Row::new(vec!["Concurrency".to_string(), controls.concurrency.to_string()]),
        Row::new(vec!["Rate Limit (req/s)".to_string(), format!("{:.1}", controls.rate_limit)]),
        Row::new(vec!["Request Timeout (s)".to_string(), controls.request_timeout.to_string()]),
        Row::new(vec!["URL File".to_string(), controls.url_file.to_string_lossy().to_string()]),
        Row::new(vec!["Export Directory".to_string(), controls.export_dir.to_string_lossy().to_string()]),
        Row::new(vec!["Auto Export".to_string(), if controls.auto_export { "Enabled" } else { "Disabled" }.to_string()]),
    ];

    let settings_table = Table::new(
        settings_data,
        [Constraint::Percentage(50), Constraint::Percentage(50)],
    )
    .block(
        Block::default()
            .title("Configuration Settings")
            .borders(Borders::ALL),
    )
    .header(
        Row::new(vec!["Setting", "Value"]).style(Style::default().add_modifier(Modifier::BOLD)),
    );
    f.render_widget(settings_table, chunks[0]);

    let metrics = &app.metrics;
    let info = &app.system_info;
    let system_info = format!(
        "🔥 Scraper CPU {:.2}% | RAM {} MB ({:.1}%) | Threads: {}\n\nPerformance:\n• Total Requests: {}\n• Success Rate: {:.1}%\n• Avg Response Time: {:.0}ms",
        info.cpu_usage,
        info.mem_usage / 1024,
        (info.mem_usage as f64 * 100.0) / (System::new().total_memory() as f64),
        info.threads,
        metrics.total_requests,
        if metrics.total_requests > 0 {
            (metrics.total_successful as f64 / metrics.total_requests as f64) * 100.0
        } else {
            0.0
        },
        metrics.response_time.iter().sum::<f64>() / metrics.response_time.len().max(1) as f64
    );

    let system_info_widget = Paragraph::new(system_info)
        .block(Block::default().title("System Information").borders(Borders::ALL))
        .wrap(Wrap { trim: true });
    f.render_widget(system_info_widget, chunks[1]);
}

#[tokio::main]
async fn main() -> io::Result<()> {
    setup_logging().expect("Failed to set up logging.");
    info!("Swoop TUI starting up");
    let original_hook = panic::take_hook();
    panic::set_hook(Box::new(move |panic_info| {
        error!("A panic occurred: {:?}", panic_info);
        disable_raw_mode().unwrap();
        execute!(stdout(), LeaveAlternateScreen).unwrap();
        original_hook(panic_info);
    }));

    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    enable_raw_mode()?;
    execute!(terminal.backend_mut(), EnterAlternateScreen)?;

    let app = Arc::new(Mutex::new(AppState::new()));
    let app_clone = Arc::clone(&app);

    tokio::spawn(async move {
        scraping_engine(app_clone).await;
    });

    let res = run_app(&mut terminal, app).await;

    info!("Swoop TUI shutting down");
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("Error: {:?}", err);
    }

    Ok(())
}

#[instrument(skip(terminal, app))]
async fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    app: Arc<Mutex<AppState>>,
) -> io::Result<()> {
    info!("Entering main application loop");
    let mut event_stream = EventStream::new();
    let mut interval = tokio::time::interval(Duration::from_millis(250));
    let mut sys = System::new_all();

    // Banner fade-out
    let app_clone = app.clone();
    tokio::spawn(async move {
        tokio::time::sleep(Duration::from_secs(3)).await;
        app_clone.lock().unwrap().show_banner = false;
    });

    loop {
        let mut app_guard = app.lock().unwrap();

        // Update RPS
        let now = Instant::now();
        app_guard.metrics.request_timestamps.retain(|&t| now.duration_since(t).as_secs() < 1);
        let rps = app_guard.metrics.request_timestamps.len() as f64;
        app_guard.metrics.requests_per_second.push_back(rps);
        if app_guard.metrics.requests_per_second.len() > 60 {
            app_guard.metrics.requests_per_second.pop_front();
        }

        // Handle export requests
        if app_guard.export_requested {
            app_guard.export_requested = false;
            let state_clone = app_guard.clone();
            tokio::spawn(async move {
                export_data(state_clone).await;
            });
        }

        // Update system info
        let pid = sysinfo::get_current_pid().unwrap();
        sys.refresh_process(pid);
        if let Some(p) = sys.process(pid) {
            app_guard.system_info.cpu_usage = p.cpu_usage();
            app_guard.system_info.mem_usage = p.memory();
            app_guard.system_info.threads = 0; // TODO: p.threads().len(),
        }
        app_guard.system_info.uptime = System::uptime();


        // Draw UI
        let app_clone = app_guard.clone();
        terminal.draw(|f| render_dashboard(f, &app_clone))?;

        let should_quit = app_guard.should_quit;
        drop(app_guard);

        if should_quit {
            return Ok(());
        }

        tokio::select! {
            _ = interval.tick() => {}
            maybe_event = event_stream.next() => {
                if let Some(Ok(Event::Key(key))) = maybe_event {
                    if key.kind == KeyEventKind::Press {
                        app.lock().unwrap().handle_key_event(key.code);
                    }
                } else {
                    app.lock().unwrap().should_quit = true;
                }
            }
        }
    }
}

async fn export_data(mut app_state: AppState) {
    if app_state.export_state.is_exporting {
        return;
    }
    app_state.export_state.is_exporting = true;
    app_state.export_state.progress = 0;
    app_state.export_state.status = "Starting export...".to_string();

    let data_clone = app_state.scraped_data.clone();
    let export_format = app_state.export_state.format;
    let file_path = app_state.export_state.file_path.clone();

    if data_clone.is_empty() {
        app_state.export_state.is_exporting = false;
        app_state.export_state.status = "No data to export".to_string();
        app_state.logs.add_entry(
            LogLevel::Warning,
            "No scraped data available for export".to_string(),
        );
        return;
    }

    for i in 0..=100 {
        app_state.export_state.progress = i;
        app_state.export_state.status = format!("Exporting... {}%", i);
        tokio::time::sleep(Duration::from_millis(20)).await;
    }

    let export_result = match export_format {
        ExportFormat::Json => {
            let json_data = serde_json::to_string_pretty(&data_clone);
            match json_data {
                Ok(json) => fs::write(&file_path, json).map_err(|e| e.to_string()),
                Err(e) => Err(e.to_string()),
            }
        }
        ExportFormat::Csv => {
            let mut csv_content = "URL,Timestamp,Status Code,Success,Response Time,Content Length,Title,Error\n".to_string();
            for item in data_clone.iter() {
                csv_content.push_str(&format!(
                    "{},{},{},{},{},{},{},{}\n",
                    item.url,
                    item.timestamp.format("%Y-%m-%d %H:%M:%S"),
                    item.status_code.unwrap_or(0),
                    item.success,
                    item.response_time,
                    item.content_length,
                    item.title.as_deref().unwrap_or(""),
                    item.error.as_deref().unwrap_or("")
                ));
            }
            fs::write(&file_path, csv_content).map_err(|e| e.to_string())
        }
    };

    app_state.export_state.is_exporting = false;
    match export_result {
        Ok(_) => {
            app_state.export_state.status = "Export completed successfully".to_string();
            app_state.export_state.recent_exports.push_back(format!(
                "{} - {} entries",
                file_path,
                data_clone.len()
            ));
            if app_state.export_state.recent_exports.len() > 10 {
                app_state.export_state.recent_exports.pop_front();
            }
            app_state.logs.add_entry(
                LogLevel::Success,
                format!("Exported {} entries to {}", data_clone.len(), file_path),
            );
        }
        Err(e) => {
            app_state.export_state.status = format!("Export failed: {}", e);
            app_state
                .logs
                .add_entry(LogLevel::Error, format!("Export failed: {}", e));
        }
    }
}
