//! Performance benchmarks for anti-bot evasion systems
//! 
//! These benchmarks measure the performance impact of various
//! anti-bot techniques to ensure they don't significantly
//! slow down scraping operations.

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use scrapers::anti_bot::*;
use std::time::Duration;
use tokio::runtime::Runtime;

/// Benchmark fingerprint generation performance
fn benchmark_fingerprint_generation(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    
    c.bench_function("canvas_fingerprint_generation", |b| {
        b.to_async(&rt).iter(|| async {
            let manager = fingerprint_manager::FingerprintManager::new().await.unwrap();
            black_box(manager.generate_canvas_fingerprint().await.unwrap())
        })
    });

    c.bench_function("webgl_fingerprint_generation", |b| {
        b.to_async(&rt).iter(|| async {
            let manager = fingerprint_manager::FingerprintManager::new().await.unwrap();
            black_box(manager.generate_webgl_fingerprint().await.unwrap())
        })
    });

    c.bench_function("tls_signature_generation", |b| {
        b.to_async(&rt).iter(|| async {
            let manager = fingerprint_manager::FingerprintManager::new().await.unwrap();
            black_box(manager.generate_tls_signature().await.unwrap())
        })
    });
}

/// Benchmark proxy rotation performance
fn benchmark_proxy_rotation(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    
    c.bench_function("proxy_selection", |b| {
        b.to_async(&rt).iter(|| async {
            let config = proxy_rotator::ProxyConfig {
                rotation_interval: Duration::from_secs(30),
                health_check_interval: Duration::from_secs(60),
                max_failures: 3,
                timeout: Duration::from_secs(10),
            };
            let rotator = proxy_rotator::ProxyRotator::new(config).await.unwrap();
            black_box(rotator.get_next_proxy().await.unwrap())
        })
    });

    c.bench_function("proxy_health_check", |b| {
        b.to_async(&rt).iter(|| async {
            let config = proxy_rotator::ProxyConfig {
                rotation_interval: Duration::from_secs(30),
                health_check_interval: Duration::from_secs(60),
                max_failures: 3,
                timeout: Duration::from_secs(10),
            };
            let rotator = proxy_rotator::ProxyRotator::new(config).await.unwrap();
            let proxy = create_test_proxy();
            black_box(rotator.check_proxy_health(&proxy).await.unwrap())
        })
    });
}

/// Benchmark behavioral simulation performance
fn benchmark_behavior_simulation(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    
    c.bench_function("mouse_movement_generation", |b| {
        b.to_async(&rt).iter(|| async {
            let engine = behavior_engine::BehaviorEngine::new().await.unwrap();
            let start = behavior_engine::Point { x: 0, y: 0 };
            let end = behavior_engine::Point { x: 500, y: 300 };
            black_box(engine.generate_mouse_movement(start, end, Duration::from_millis(1000)).await.unwrap())
        })
    });

    c.bench_function("typing_simulation", |b| {
        b.to_async(&rt).iter(|| async {
            let engine = behavior_engine::BehaviorEngine::new().await.unwrap();
            let text = "Hello, this is a test typing simulation!";
            black_box(engine.simulate_typing(text).await.unwrap())
        })
    });

    c.bench_function("scroll_behavior_generation", |b| {
        b.to_async(&rt).iter(|| async {
            let engine = behavior_engine::BehaviorEngine::new().await.unwrap();
            black_box(engine.generate_scroll_behavior(1000, 5000).await.unwrap())
        })
    });
}

/// Benchmark session management performance
fn benchmark_session_management(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    
    c.bench_function("session_creation", |b| {
        b.to_async(&rt).iter(|| async {
            let manager = session_manager::SessionManager::new().await.unwrap();
            black_box(manager.get_session("test_platform").await.unwrap())
        })
    });

    c.bench_function("cookie_storage", |b| {
        b.to_async(&rt).iter(|| async {
            let manager = session_manager::SessionManager::new().await.unwrap();
            let cookies = vec![create_test_cookie()];
            black_box(manager.store_cookies("test_platform", cookies).await.unwrap())
        })
    });

    c.bench_function("session_stats_calculation", |b| {
        b.to_async(&rt).iter(|| async {
            let manager = session_manager::SessionManager::new().await.unwrap();
            // Create some test sessions
            for i in 0..10 {
                let _ = manager.get_session(&format!("platform_{}", i)).await;
            }
            black_box(manager.get_session_stats().await)
        })
    });
}

/// Benchmark stealth browser operations
fn benchmark_stealth_browser(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    
    c.bench_function("stealth_browser_creation", |b| {
        b.to_async(&rt).iter(|| async {
            let config = stealth_browser::StealthConfig::default();
            black_box(stealth_browser::StealthBrowser::new(config).await.unwrap())
        })
    });

    c.bench_function("stealth_script_injection", |b| {
        b.to_async(&rt).iter(|| async {
            let config = stealth_browser::StealthConfig::default();
            let mut browser = stealth_browser::StealthBrowser::new(config).await.unwrap();
            let script = "console.log('stealth test');";
            black_box(browser.execute_script(script).await.unwrap())
        })
    });
}

/// Benchmark anti-bot manager integration
fn benchmark_anti_bot_integration(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    
    c.bench_function("anti_bot_manager_creation", |b| {
        b.to_async(&rt).iter(|| async {
            black_box(AntiBotManager::new().await.unwrap())
        })
    });

    c.bench_function("request_preparation", |b| {
        b.to_async(&rt).iter(|| async {
            let manager = AntiBotManager::new().await.unwrap();
            let mut request = http::Request::builder()
                .uri("https://example.com")
                .body(hyper::body::Bytes::new())
                .unwrap();
            let proxy = create_test_proxy();
            black_box(manager.prepare_request(&mut request, &proxy).await.unwrap())
        })
    });
}

/// Benchmark different proxy pool sizes
fn benchmark_proxy_pool_scaling(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let mut group = c.benchmark_group("proxy_pool_scaling");
    
    for pool_size in [10, 50, 100, 500].iter() {
        group.bench_with_input(
            BenchmarkId::new("proxy_selection", pool_size),
            pool_size,
            |b, &pool_size| {
                b.to_async(&rt).iter(|| async move {
                    let config = proxy_rotator::ProxyConfig {
                        rotation_interval: Duration::from_secs(30),
                        health_check_interval: Duration::from_secs(60),
                        max_failures: 3,
                        timeout: Duration::from_secs(10),
                    };
                    let mut rotator = proxy_rotator::ProxyRotator::new(config).await.unwrap();
                    
                    // Add test proxies
                    for i in 0..pool_size {
                        let proxy = proxy_rotator::ProxyInfo {
                            host: format!("proxy{}.example.com", i),
                            port: 8080 + i as u16,
                            proxy_type: proxy_rotator::ProxyType::Http,
                            username: Some(format!("user{}", i)),
                            password: Some(format!("pass{}", i)),
                            country: Some("US".to_string()),
                            region: Some("CA".to_string()),
                            city: Some("SF".to_string()),
                            isp: Some("Test ISP".to_string()),
                            last_used: std::time::Instant::now(),
                            success_count: 10,
                            failure_count: 1,
                            avg_response_time: Duration::from_millis(200),
                            is_healthy: true,
                        };
                        rotator.add_proxy(proxy).await.unwrap();
                    }
                    
                    black_box(rotator.get_next_proxy().await.unwrap())
                })
            },
        );
    }
    group.finish();
}

/// Helper functions for benchmarks
fn create_test_proxy() -> proxy_rotator::ProxyInfo {
    proxy_rotator::ProxyInfo {
        host: "127.0.0.1".to_string(),
        port: 8080,
        proxy_type: proxy_rotator::ProxyType::Http,
        username: Some("test_user".to_string()),
        password: Some("test_pass".to_string()),
        country: Some("US".to_string()),
        region: Some("California".to_string()),
        city: Some("San Francisco".to_string()),
        isp: Some("Test ISP".to_string()),
        last_used: std::time::Instant::now(),
        success_count: 10,
        failure_count: 1,
        avg_response_time: Duration::from_millis(200),
        is_healthy: true,
    }
}

fn create_test_cookie() -> session_manager::Cookie {
    session_manager::Cookie {
        name: "test_cookie".to_string(),
        value: "test_value".to_string(),
        domain: "example.com".to_string(),
        path: "/".to_string(),
        expires: None,
        secure: false,
        http_only: true,
        same_site: Some(session_manager::SameSite::Lax),
    }
}

criterion_group!(
    benches,
    benchmark_fingerprint_generation,
    benchmark_proxy_rotation,
    benchmark_behavior_simulation,
    benchmark_session_management,
    benchmark_stealth_browser,
    benchmark_anti_bot_integration,
    benchmark_proxy_pool_scaling
);

criterion_main!(benches);
