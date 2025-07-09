use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{
        Axis, Block, Borders, Chart, Dataset, Gauge, List, ListItem, Paragraph, Row, Table, Tabs,
        Wrap,
    },
    Frame, Terminal,
};
use std::{
    collections::VecDeque,
    io,
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};

/// Application state for the TUI dashboard
#[derive(Debug)]
struct AppState {
    /// Current active tab
    current_tab: usize,
    /// Metrics data
    metrics: Arc<Mutex<Metrics>>,
    /// Proxy pool status
    proxy_status: Arc<Mutex<ProxyStatus>>,
    /// Log entries
    logs: Arc<Mutex<LogBuffer>>,
    /// Control state
    controls: Arc<Mutex<ControlState>>,
    /// Whether the app should quit
    should_quit: bool,
}

/// Metrics data for monitoring
#[derive(Debug, Clone)]
struct Metrics {
    /// Requests per second (last 60 seconds)
    requests_per_second: VecDeque<f64>,
    /// Success rate (last 60 seconds)
    success_rate: VecDeque<f64>,
    /// Response time (last 60 seconds)
    response_time: VecDeque<f64>,
    /// Total requests made
    total_requests: u64,
    /// Total successful requests
    total_successful: u64,
    /// Total failed requests
    total_failed: u64,
    /// Current active connections
    active_connections: u32,
    /// Data processed (bytes)
    data_processed: u64,
}

/// Proxy pool status information
#[derive(Debug, Clone)]
struct ProxyStatus {
    /// Total proxies available
    total_proxies: u32,
    /// Active proxies
    active_proxies: u32,
    /// Failed proxies
    failed_proxies: u32,
    /// Proxy health by type
    residential_health: f32,
    datacenter_health: f32,
    mobile_health: f32,
    /// Current rotation index
    current_rotation: u32,
}

/// Log buffer for system events
#[derive(Debug, Clone)]
struct LogBuffer {
    entries: VecDeque<LogEntry>,
    max_size: usize,
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
enum LogLevel {
    Info,
    Warning,
    Error,
    Success,
}

/// Control state for user interactions
#[derive(Debug, Clone)]
struct ControlState {
    /// Whether scraping is paused
    is_paused: bool,
    /// Current rate limit (requests per second)
    rate_limit: f64,
    /// Selected target index
    selected_target: usize,
    /// Active targets
    targets: Vec<String>,
}

impl Default for Metrics {
    fn default() -> Self {
        Self {
            requests_per_second: VecDeque::with_capacity(60),
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
            selected_target: 0,
            targets: vec![
                "https://example.com".to_string(),
                "https://httpbin.org".to_string(),
                "https://jsonplaceholder.typicode.com".to_string(),
            ],
        }
    }
}

impl AppState {
    fn new() -> Self {
        let mut logs = LogBuffer::default();
        logs.add_entry(LogLevel::Info, "Swoop TUI Dashboard initialized".to_string());
        logs.add_entry(LogLevel::Success, "All systems operational".to_string());
        
        Self {
            current_tab: 0,
            metrics: Arc::new(Mutex::new(Metrics::default())),
            proxy_status: Arc::new(Mutex::new(ProxyStatus::default())),
            logs: Arc::new(Mutex::new(logs)),
            controls: Arc::new(Mutex::new(ControlState::default())),
            should_quit: false,
        }
    }

    fn handle_key_event(&mut self, key: KeyCode) {
        match key {
            KeyCode::Char('q') => self.should_quit = true,
            KeyCode::Tab => {
                self.current_tab = (self.current_tab + 1) % 4;
            }
            KeyCode::Char('1') => self.current_tab = 0,
            KeyCode::Char('2') => self.current_tab = 1,
            KeyCode::Char('3') => self.current_tab = 2,
            KeyCode::Char('4') => self.current_tab = 3,
            KeyCode::Char(' ') => {
                if let Ok(mut controls) = self.controls.lock() {
                    controls.is_paused = !controls.is_paused;
                }
                if let Ok(mut logs) = self.logs.lock() {
                    let state = if self.controls.lock().unwrap().is_paused {
                        "paused"
                    } else {
                        "resumed"
                    };
                    logs.add_entry(LogLevel::Info, format!("Scraping {}", state));
                }
            }
            KeyCode::Char('+') => {
                if let Ok(mut controls) = self.controls.lock() {
                    controls.rate_limit = (controls.rate_limit + 0.1).min(10.0);
                }
            }
            KeyCode::Char('-') => {
                if let Ok(mut controls) = self.controls.lock() {
                    controls.rate_limit = (controls.rate_limit - 0.1).max(0.1);
                }
            }
            _ => {}
        }
    }
}

/// Simulate real-time metrics updates
async fn update_metrics(metrics: Arc<Mutex<Metrics>>, logs: Arc<Mutex<LogBuffer>>) {
    let mut interval = tokio::time::interval(Duration::from_secs(1));
    let mut counter = 0u64;
    
    loop {
        interval.tick().await;
        counter += 1;
        
        if let Ok(mut m) = metrics.lock() {
            // Simulate request rate (0-10 requests per second)
            let rate = 2.0 + 3.0 * (counter as f64 * 0.1).sin();
            m.requests_per_second.push_back(rate);
            if m.requests_per_second.len() > 60 {
                m.requests_per_second.pop_front();
            }
            
            // Simulate success rate (85-98%)
            let success = 0.85 + 0.13 * (counter as f64 * 0.05).cos();
            m.success_rate.push_back(success);
            if m.success_rate.len() > 60 {
                m.success_rate.pop_front();
            }
            
            // Simulate response time (100-500ms)
            let response_time = 200.0 + 150.0 * (counter as f64 * 0.08).sin();
            m.response_time.push_back(response_time);
            if m.response_time.len() > 60 {
                m.response_time.pop_front();
            }
            
            // Update totals
            m.total_requests += rate as u64;
            m.total_successful += (rate * success) as u64;
            m.total_failed += (rate * (1.0 - success)) as u64;
            m.active_connections = (10.0 + 5.0 * (counter as f64 * 0.1).cos()) as u32;
            m.data_processed += (rate * 1024.0) as u64;
        }
        
        // Add occasional log entries
        if counter % 10 == 0 {
            if let Ok(mut logs) = logs.lock() {
                logs.add_entry(
                    LogLevel::Info,
                    format!("Processed {} requests", counter * 2),
                );
            }
        }
        
        if counter % 30 == 0 {
            if let Ok(mut logs) = logs.lock() {
                logs.add_entry(
                    LogLevel::Success,
                    "Proxy rotation completed successfully".to_string(),
                );
            }
        }
        if counter % 45 == 0 {
             if let Ok(mut logs) = logs.lock() {
                logs.add_entry(
                    LogLevel::Warning,
                    "High latency detected on datacenter proxies".to_string(),
                );
            }
        }
         if counter % 60 == 0 {
             if let Ok(mut logs) = logs.lock() {
                logs.add_entry(
                    LogLevel::Error,
                    "Failed to connect to target: example.com".to_string(),
                );
            }
        }
    }
}

/// Render the dashboard UI
fn render_dashboard(f: &mut Frame, app: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)])
        .split(f.area());

    // Render tabs
    let tabs = Tabs::new(vec!["Overview", "Metrics", "Proxies", "Logs"])
        .block(Block::default().borders(Borders::ALL).title("Swoop Dashboard"))
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
        .select(app.current_tab);
    f.render_widget(tabs, chunks[0]);

    // Render content based on current tab
    match app.current_tab {
        0 => render_overview(f, chunks[1], app),
        1 => render_metrics(f, chunks[1], app),
        2 => render_proxies(f, chunks[1], app),
        3 => render_logs(f, chunks[1], app),
        _ => {}
    }
}

/// Render overview tab
fn render_overview(f: &mut Frame, area: Rect, app: &AppState) {
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

    // System Status
    let status_text = if app.controls.lock().unwrap().is_paused {
        "🔴 PAUSED"
    } else {
        "🟢 RUNNING"
    };
    
    let system_status = Paragraph::new(format!(
        "System Status: {}\n\nControls:\n• Press 'q' to quit\n• Press 'Space' to pause/resume\n• Press '+/-' to adjust rate limit\n• Press '1-4' to switch tabs",
        status_text
    ))
    .block(Block::default().title("System Status").borders(Borders::ALL))
    .wrap(Wrap { trim: true });
    f.render_widget(system_status, left_chunks[0]);

    // Quick Stats
    if let Ok(metrics) = app.metrics.lock() {
        let rate_limit = app.controls.lock().unwrap().rate_limit;
        let stats_text = format!(
            "Total Requests: {}\nSuccessful: {}\nFailed: {}\nActive Connections: {}\nData Processed: {} KB\nRate Limit: {:.1} req/s",
            metrics.total_requests,
            metrics.total_successful,
            metrics.total_failed,
            metrics.active_connections,
            metrics.data_processed / 1024,
            rate_limit
        );
        
        let quick_stats = Paragraph::new(stats_text)
            .block(Block::default().title("Quick Stats").borders(Borders::ALL))
            .wrap(Wrap { trim: true });
        f.render_widget(quick_stats, left_chunks[1]);
    }

    // Proxy Status
    if let Ok(proxy_status) = app.proxy_status.lock() {
        let controls = app.controls.lock().unwrap();
        let proxy_text = format!(
            "Total Proxies: {}\nActive: {}\nFailed: {}\n\nHealth Status:\n• Residential: {:.1}%\n• Datacenter: {:.1}%\n• Mobile: {:.1}%\n\nSelected Target: {}",
            proxy_status.total_proxies,
            proxy_status.active_proxies,
            proxy_status.failed_proxies,
            proxy_status.residential_health * 100.0,
            proxy_status.datacenter_health * 100.0,
            proxy_status.mobile_health * 100.0,
            controls.targets.get(controls.selected_target).map_or("None", |s| s.as_str())
        );
        
        let proxy_status_widget = Paragraph::new(proxy_text)
            .block(Block::default().title("Proxy Status & Targets").borders(Borders::ALL))
            .wrap(Wrap { trim: true });
        f.render_widget(proxy_status_widget, right_chunks[0]);
    }

    // Recent Activity
    if let Ok(logs) = app.logs.lock() {
        let recent_logs: Vec<ListItem> = logs.entries
            .iter()
            .rev()
            .take(10)
            .map(|entry| {
                let style = match entry.level {
                    LogLevel::Info => Style::default().fg(Color::Blue),
                    LogLevel::Warning => Style::default().fg(Color::Yellow),
                    LogLevel::Error => Style::default().fg(Color::Red),
                    LogLevel::Success => Style::default().fg(Color::Green),
                };
                ListItem::new(entry.message.clone()).style(style)
            })
            .collect();
        
        let recent_activity = List::new(recent_logs)
            .block(Block::default().title("Recent Activity").borders(Borders::ALL));
        f.render_widget(recent_activity, right_chunks[1]);
    }
}

/// Render metrics tab
fn render_metrics(f: &mut Frame, area: Rect, app: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);

    let top_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[0]);

    if let Ok(metrics) = app.metrics.lock() {
        // Request Rate Chart
        if !metrics.requests_per_second.is_empty() {
            let data: Vec<(f64, f64)> = metrics.requests_per_second
                .iter()
                .enumerate()
                .map(|(i, &value)| (i as f64, value))
                .collect();

            let dataset = Dataset::default()
                .marker(ratatui::symbols::Marker::Braille)
                .style(Style::default().fg(Color::Cyan))
                .data(&data);

            let chart = Chart::new(vec![dataset])
                .block(Block::default().title("Request Rate (req/s)").borders(Borders::ALL))
                .x_axis(
                    Axis::default()
                        .title("Time (seconds)")
                        .bounds([0.0, 60.0])
                        .style(Style::default().fg(Color::Gray))
                )
                .y_axis(
                    Axis::default()
                        .title("Requests/sec")
                        .bounds([0.0, 10.0])
                        .style(Style::default().fg(Color::Gray))
                );
            f.render_widget(chart, top_chunks[0]);
        }

        // Success Rate Chart
        if !metrics.success_rate.is_empty() {
            let data: Vec<(f64, f64)> = metrics.success_rate
                .iter()
                .enumerate()
                .map(|(i, &value)| (i as f64, value * 100.0))
                .collect();

            let dataset = Dataset::default()
                .marker(ratatui::symbols::Marker::Braille)
                .style(Style::default().fg(Color::Green))
                .data(&data);

            let chart = Chart::new(vec![dataset])
                .block(Block::default().title("Success Rate (%)").borders(Borders::ALL))
                .x_axis(
                    Axis::default()
                        .title("Time (seconds)")
                        .bounds([0.0, 60.0])
                        .style(Style::default().fg(Color::Gray))
                )
                .y_axis(
                    Axis::default()
                        .title("Success %")
                        .bounds([80.0, 100.0])
                        .style(Style::default().fg(Color::Gray))
                );
            f.render_widget(chart, top_chunks[1]);
        }

        // Response Time Chart
        if !metrics.response_time.is_empty() {
            let data: Vec<(f64, f64)> = metrics.response_time
                .iter()
                .enumerate()
                .map(|(i, &value)| (i as f64, value))
                .collect();

            let dataset = Dataset::default()
                .marker(ratatui::symbols::Marker::Braille)
                .style(Style::default().fg(Color::Yellow))
                .data(&data);

            let chart = Chart::new(vec![dataset])
                .block(Block::default().title("Response Time (ms)").borders(Borders::ALL))
                .x_axis(
                    Axis::default()
                        .title("Time (seconds)")
                        .bounds([0.0, 60.0])
                        .style(Style::default().fg(Color::Gray))
                )
                .y_axis(
                    Axis::default()
                        .title("Response Time (ms)")
                        .bounds([0.0, 600.0])
                        .style(Style::default().fg(Color::Gray))
                );
            f.render_widget(chart, chunks[1]);
        }
    }
}

/// Render proxies tab
fn render_proxies(f: &mut Frame, area: Rect, app: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(7), Constraint::Min(0)])
        .split(area);

    if let Ok(proxy_status) = app.proxy_status.lock() {
        // Proxy Health Gauges
        let health_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(33), Constraint::Percentage(33), Constraint::Percentage(34)])
            .split(chunks[0]);

        let residential_gauge = Gauge::default()
            .block(Block::default().title("Residential Proxies").borders(Borders::ALL))
            .gauge_style(Style::default().fg(Color::Green))
            .percent((proxy_status.residential_health * 100.0) as u16);
        f.render_widget(residential_gauge, health_chunks[0]);

        let datacenter_gauge = Gauge::default()
            .block(Block::default().title("Datacenter Proxies").borders(Borders::ALL))
            .gauge_style(Style::default().fg(Color::Blue))
            .percent((proxy_status.datacenter_health * 100.0) as u16);
        f.render_widget(datacenter_gauge, health_chunks[1]);

        let mobile_gauge = Gauge::default()
            .block(Block::default().title("Mobile Proxies").borders(Borders::ALL))
            .gauge_style(Style::default().fg(Color::Magenta))
            .percent((proxy_status.mobile_health * 100.0) as u16);
        f.render_widget(mobile_gauge, health_chunks[2]);

        // Proxy Details Table - fix borrow checker issues by creating owned strings
        let total_proxies = proxy_status.total_proxies.to_string();
        let active_proxies = proxy_status.active_proxies.to_string();
        let failed_proxies = proxy_status.failed_proxies.to_string();
        let current_rotation = proxy_status.current_rotation.to_string();
        let residential_health = format!("{:.1}%", proxy_status.residential_health * 100.0);
        let datacenter_health = format!("{:.1}%", proxy_status.datacenter_health * 100.0);
        let mobile_health = format!("{:.1}%", proxy_status.mobile_health * 100.0);
        
        let proxy_data = vec![
            Row::new(vec!["Total Proxies".to_string(), total_proxies]),
            Row::new(vec!["Active Proxies".to_string(), active_proxies]),
            Row::new(vec!["Failed Proxies".to_string(), failed_proxies]),
            Row::new(vec!["Current Rotation".to_string(), current_rotation]),
            Row::new(vec!["Residential Health".to_string(), residential_health]),
            Row::new(vec!["Datacenter Health".to_string(), datacenter_health]),
            Row::new(vec!["Mobile Health".to_string(), mobile_health]),
        ];

        let proxy_table = Table::new(proxy_data, [Constraint::Percentage(50), Constraint::Percentage(50)])
            .block(Block::default().title("Proxy Details").borders(Borders::ALL))
            .header(Row::new(vec!["Metric", "Value"]).style(Style::default().add_modifier(Modifier::BOLD)));
        f.render_widget(proxy_table, chunks[1]);
    }
}

/// Render logs tab
fn render_logs(f: &mut Frame, area: Rect, app: &AppState) {
    if let Ok(logs) = app.logs.lock() {
        let log_items: Vec<ListItem> = logs.entries
            .iter()
            .rev()
            .map(|entry| {
                let style = match entry.level {
                    LogLevel::Info => Style::default().fg(Color::Blue),
                    LogLevel::Warning => Style::default().fg(Color::Yellow),
                    LogLevel::Error => Style::default().fg(Color::Red),
                    LogLevel::Success => Style::default().fg(Color::Green),
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

        let logs_widget = List::new(log_items)
            .block(Block::default().title("System Logs").borders(Borders::ALL))
            .style(Style::default().fg(Color::White));
        f.render_widget(logs_widget, area);
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize application state
    let mut app = AppState::new();

    // Set up terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(&mut stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Start metrics update task
    let metrics_clone = Arc::clone(&app.metrics);
    let logs_clone = Arc::clone(&app.logs);
    tokio::spawn(async move {
        update_metrics(metrics_clone, logs_clone).await;
    });

    // Main application loop
    let result = run_app(&mut terminal, &mut app).await;

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen
    )?;
    terminal.show_cursor()?;

    if let Err(err) = result {
        println!("Error: {:?}", err);
    }

    Ok(())
}

async fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    app: &mut AppState,
) -> io::Result<()> {
    loop {
        terminal.draw(|f| render_dashboard(f, app))?;

        if crossterm::event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    app.handle_key_event(key.code);
                }
            }
        }

        if app.should_quit {
            return Ok(());
        }
    }
}
