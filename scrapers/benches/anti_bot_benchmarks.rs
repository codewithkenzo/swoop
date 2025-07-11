//! Performance benchmarks for anti-bot evasion systems
//! 
//! These benchmarks measure the performance impact of various
//! anti-bot techniques to ensure they don't significantly
//! slow down scraping operations.

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use scrapers::anti_bot::*;
use std::time::Duration;
use tokio::runtime::Runtime;

/// Benchmark fingerprint generation performance
fn benchmark_fingerprint_generation(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    
    c.bench_function("fingerprint_profile_generation", |b| {
        rt.block_on(async {
            let manager = fingerprint_manager::FingerprintManager::new().await.unwrap();
            b.iter(|| {
                rt.block_on(async {
                    black_box(manager.generate_fingerprint_profile().await);
                })
            });
        });
    });
}

/// Benchmark proxy rotation performance
fn benchmark_proxy_rotation(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    
    c.bench_function("proxy_selection", |b| {
        rt.block_on(async {
            let rotator = proxy_rotator::ProxyRotator::new().await.unwrap();
            b.iter(|| {
                rt.block_on(async {
                    black_box(rotator.get_current_proxy("test").await.unwrap());
                })
            });
        });
    });
}

/// Benchmark behavioral simulation performance
fn benchmark_behavior_simulation(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    
    c.bench_function("mouse_movement_generation", |b| {
        rt.block_on(async {
            let engine = behavior_engine::BehaviorEngine::new().await.unwrap();
            b.iter(|| {
                rt.block_on(async {
                    black_box(engine.simulate_mouse_movement(
                        (0.0, 0.0),
                        (500.0, 300.0)
                    ).await);
                })
            });
        });
    });

    c.bench_function("typing_simulation", |b| {
        rt.block_on(async {
            let engine = behavior_engine::BehaviorEngine::new().await.unwrap();
            let text = "Hello, this is a test typing simulation!";
            b.iter(|| {
                rt.block_on(async {
                    black_box(engine.simulate_typing(text).await);
                })
            });
        });
    });
}

criterion_group!(
    benches,
    benchmark_fingerprint_generation,
    benchmark_proxy_rotation,
    benchmark_behavior_simulation,
);

criterion_main!(benches);
