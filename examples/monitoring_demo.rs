/*!
 * Advanced Monitoring Demo
 * 
 * Demonstrates Phase 2 monitoring features:
 * - Prometheus metrics collection
 * - Health check endpoints (/health, /ready, /metrics, /stats)
 * - Real-time performance monitoring
 * - Production-ready observability
 */

use std::time::Duration;
use tokio::time::sleep;
use tracing::{info, warn};
use tracing_subscriber::EnvFilter;

use swoop::{
    error::Result,
    monitoring::MonitoringSystem,
    server::{CrawlServer, ServerConfig},
};

#[tokio::main]
async fn main() -> Result<()> {
    // Setup logging
    let filter = EnvFilter::from_default_env()
        .add_directive("monitoring_demo=info".parse().unwrap())
        .add_directive("swoop=info".parse().unwrap());

    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_target(false)
        .init();

    info!("🎯 === CRAWL4AI ADVANCED MONITORING DEMO ===");
    
    // Initialize monitoring system
    let monitoring = MonitoringSystem::new()?;
    info!("✅ Monitoring system initialized");

    // Simulate some metrics
    simulate_workload(&monitoring).await;

    // Start HTTP server with monitoring endpoints
    let config = ServerConfig {
        bind_addr: "0.0.0.0:8080".parse().unwrap(),
        ..Default::default()
    };

    info!("🚀 Starting server with monitoring endpoints:");
    info!("   📊 Health: http://localhost:8080/health");
    info!("   🔥 Ready:  http://localhost:8080/ready");
    info!("   📈 Metrics: http://localhost:8080/metrics");
    info!("   📋 Stats:  http://localhost:8080/stats");

    // Start background metrics simulation
    let monitoring_clone = monitoring.clone();
    tokio::spawn(async move {
        background_metrics_simulation(monitoring_clone).await;
    });

    // Start server
    let server = CrawlServer::new(config)?;
    server.start().await?;

    Ok(())
}

/// Simulate initial workload to populate metrics
async fn simulate_workload(monitoring: &MonitoringSystem) -> Result<()> {
    info!("🔧 Simulating initial workload...");

    // Simulate successful crawls
    for i in 1..=5 {
        monitoring.record_crawl_request().await;
        
        let duration = Duration::from_millis(100 + (i * 50));
        monitoring.record_successful_crawl(duration).await;
        
        info!("✅ Simulated successful crawl {} ({}ms)", i, duration.as_millis());
        sleep(Duration::from_millis(200)).await;
    }

    // Simulate some failures
    for i in 1..=2 {
        monitoring.record_crawl_request().await;
        
        let duration = Duration::from_millis(500);
        monitoring.record_failed_crawl(duration).await;
        
        warn!("❌ Simulated failed crawl {} ({}ms)", i, duration.as_millis());
        sleep(Duration::from_millis(200)).await;
    }

    // Simulate parser operations
    for i in 1..=3 {
        let duration = Duration::from_millis(50 + (i * 20));
        monitoring.record_parser_operation(duration).await;
        
        info!("🔍 Simulated parser operation {} ({}ms)", i, duration.as_millis());
        sleep(Duration::from_millis(100)).await;
    }

    // Simulate storage operations
    for i in 1..=4 {
        let duration = Duration::from_millis(25 + (i * 15));
        monitoring.record_storage_operation(duration).await;
        
        info!("💾 Simulated storage operation {} ({}ms)", i, duration.as_millis());
        sleep(Duration::from_millis(100)).await;
    }

    // Simulate rate limiting
    monitoring.record_rate_limited().await;
    monitoring.record_rate_limited().await;
    warn!("⏰ Simulated rate limiting events");

    // Update system metrics
    monitoring.update_active_connections(15).await;
    monitoring.update_memory_usage(125_000_000).await; // 125MB
    
    info!("📊 Initial workload simulation complete");
    
    Ok(())
}

/// Background task to continuously generate metrics
async fn background_metrics_simulation(monitoring: MonitoringSystem) {
    info!("🔄 Starting background metrics simulation...");
    
    let mut counter = 0;
    
    loop {
        sleep(Duration::from_secs(10)).await;
        counter += 1;
        
        // Simulate periodic activity
        monitoring.record_crawl_request().await;
        
        let success = counter % 4 != 0; // 75% success rate
        let duration = Duration::from_millis(80 + (counter % 5) * 40);
        
        if success {
            monitoring.record_successful_crawl(duration).await;
            info!("🔄 Background: successful crawl ({}ms)", duration.as_millis());
        } else {
            monitoring.record_failed_crawl(duration).await;
            warn!("🔄 Background: failed crawl ({}ms)", duration.as_millis());
        }
        
        // Periodic parser and storage operations
        if counter % 2 == 0 {
            let parse_duration = Duration::from_millis(30 + (counter % 3) * 20);
            monitoring.record_parser_operation(parse_duration).await;
            
            let storage_duration = Duration::from_millis(20 + (counter % 4) * 10);
            monitoring.record_storage_operation(storage_duration).await;
        }
        
        // Simulate rate limiting occasionally
        if counter % 7 == 0 {
            monitoring.record_rate_limited().await;
            warn!("🔄 Background: rate limiting event");
        }
        
        // Update system metrics with some variation
        let connections = 10 + (counter % 10);
        let memory = 120_000_000 + (counter % 5) * 10_000_000; // 120-160MB
        
        monitoring.update_active_connections(connections).await;
        monitoring.update_memory_usage(memory).await;
        
        // Print current stats periodically
        if counter % 3 == 0 {
            let stats = monitoring.get_stats().await;
            info!("📈 Current stats: {} requests, {} successful, {} failed, {} connections", 
                  stats.requests_total, stats.successful_crawls, stats.failed_crawls, stats.active_connections);
        }
    }
} 