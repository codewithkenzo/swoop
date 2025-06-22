use anyhow::{anyhow, Result};
use std::collections::HashMap;
use tracing::{debug, info, warn};

use super::models::*;

/// Model router that selects optimal models based on various criteria
pub struct ModelRouter {
    routing_strategies: Vec<Box<dyn RoutingStrategy + Send + Sync>>,
}

impl ModelRouter {
    pub fn new() -> Self {
        Self {
            routing_strategies: vec![
                Box::new(UserTierStrategy),
                Box::new(TaskCategoryStrategy),
                Box::new(CostOptimizationStrategy),
                Box::new(PerformanceStrategy),
                Box::new(FallbackStrategy),
            ],
        }
    }

    pub async fn select_model(
        &self,
        request: &CompletionRequest,
        user_tier: &UserTier,
        registry: &ModelRegistry,
    ) -> Result<ModelInfo> {
        let mut candidates = registry.models.values().collect::<Vec<_>>();

        // Apply routing strategies in order
        for strategy in &self.routing_strategies {
            candidates = strategy.filter_models(candidates, request, user_tier, registry).await?;
            
            if candidates.is_empty() {
                warn!("No models available after applying {:?} strategy", strategy.name());
                break;
            }
            
            if candidates.len() == 1 {
                debug!("Single model selected by {:?} strategy", strategy.name());
                break;
            }
        }

        if candidates.is_empty() {
            return Err(anyhow!("No suitable models found for request"));
        }

        // Select the best model from remaining candidates
        let selected = candidates.into_iter()
            .min_by(|a, b| self.compare_models(a, b, request, user_tier))
            .unwrap();

        info!("Selected model: {} for user tier: {:?}", selected.id, user_tier);
        Ok(selected.clone())
    }

    fn compare_models(
        &self,
        a: &ModelInfo,
        b: &ModelInfo,
        request: &CompletionRequest,
        user_tier: &UserTier,
    ) -> std::cmp::Ordering {
        // Priority-based comparison
        match request.priority {
            RequestPriority::Critical | RequestPriority::High => {
                // For high priority, prefer performance over cost
                self.performance_score(a).partial_cmp(&self.performance_score(b))
                    .unwrap_or(std::cmp::Ordering::Equal)
                    .reverse()
            }
            RequestPriority::Normal | RequestPriority::Low => {
                // For normal/low priority, prefer cost efficiency
                self.cost_score(a, user_tier).partial_cmp(&self.cost_score(b, user_tier))
                    .unwrap_or(std::cmp::Ordering::Equal)
            }
        }
    }

    fn performance_score(&self, model: &ModelInfo) -> f64 {
        // Higher context length = better performance for document tasks
        let context_score = (model.context_length as f64).log10() / 6.0; // Normalize to ~0-1
        
        // Premium models generally perform better
        let pricing_score = if model.pricing.as_ref()
            .map(|p| p.prompt == "0" && p.completion == "0")
            .unwrap_or(false) {
            0.3 // Free models get lower performance score
        } else {
            0.7 // Paid models get higher performance score
        };

        context_score * 0.6 + pricing_score * 0.4
    }

    fn cost_score(&self, model: &ModelInfo, user_tier: &UserTier) -> f64 {
        match &model.pricing {
            Some(pricing) => {
                let prompt_cost = pricing.prompt.parse::<f64>().unwrap_or(0.0);
                let completion_cost = pricing.completion.parse::<f64>().unwrap_or(0.0);
                let total_cost = prompt_cost + completion_cost;
                
                // Free models get the best cost score
                if total_cost == 0.0 {
                    return 0.0;
                }
                
                // Adjust cost based on user tier
                match user_tier {
                    UserTier::Free { .. } => f64::INFINITY, // Free users can't use paid models
                    UserTier::Basic { .. } => total_cost * 1.5, // Basic users pay more weight to cost
                    UserTier::Premium { .. } => total_cost * 1.0, // Premium users pay normal cost
                    UserTier::Enterprise { .. } => total_cost * 0.8, // Enterprise gets discount
                }
            }
            None => 0.0, // No pricing info = assume free
        }
    }
}

/// Trait for routing strategies
#[async_trait::async_trait]
pub trait RoutingStrategy: std::fmt::Debug {
    fn name(&self) -> &'static str;
    
    async fn filter_models(
        &self,
        candidates: Vec<&ModelInfo>,
        request: &CompletionRequest,
        user_tier: &UserTier,
        registry: &ModelRegistry,
    ) -> Result<Vec<&ModelInfo>>;
}

/// Filter models based on user tier permissions
#[derive(Debug)]
pub struct UserTierStrategy;

#[async_trait::async_trait]
impl RoutingStrategy for UserTierStrategy {
    fn name(&self) -> &'static str {
        "UserTier"
    }

    async fn filter_models(
        &self,
        candidates: Vec<&ModelInfo>,
        _request: &CompletionRequest,
        user_tier: &UserTier,
        _registry: &ModelRegistry,
    ) -> Result<Vec<&ModelInfo>> {
        let filtered = candidates
            .into_iter()
            .filter(|model| user_tier.can_access_model(&model.id))
            .collect();

        debug!("UserTierStrategy filtered to {} models", filtered.len());
        Ok(filtered)
    }
}

/// Filter models based on task category
#[derive(Debug)]
pub struct TaskCategoryStrategy;

#[async_trait::async_trait]
impl RoutingStrategy for TaskCategoryStrategy {
    fn name(&self) -> &'static str {
        "TaskCategory"
    }

    async fn filter_models(
        &self,
        candidates: Vec<&ModelInfo>,
        request: &CompletionRequest,
        _user_tier: &UserTier,
        registry: &ModelRegistry,
    ) -> Result<Vec<&ModelInfo>> {
        // Get models specialized for this task category
        let specialized_models = registry.get_models_for_category(&request.task_category);
        
        if specialized_models.is_empty() {
            // If no specialized models, fall back to general models
            let general_models = registry.get_models_for_category(&TaskCategory::General);
            let filtered = candidates
                .into_iter()
                .filter(|model| general_models.iter().any(|gm| gm.id == model.id))
                .collect();
            
            debug!("TaskCategoryStrategy: No specialized models, using {} general models", filtered.len());
            return Ok(filtered);
        }

        let filtered = candidates
            .into_iter()
            .filter(|model| specialized_models.iter().any(|sm| sm.id == model.id))
            .collect();

        debug!("TaskCategoryStrategy filtered to {} specialized models for {:?}", 
               filtered.len(), request.task_category);
        Ok(filtered)
    }
}

/// Filter models based on cost optimization
#[derive(Debug)]
pub struct CostOptimizationStrategy;

#[async_trait::async_trait]
impl RoutingStrategy for CostOptimizationStrategy {
    fn name(&self) -> &'static str {
        "CostOptimization"
    }

    async fn filter_models(
        &self,
        candidates: Vec<&ModelInfo>,
        request: &CompletionRequest,
        user_tier: &UserTier,
        _registry: &ModelRegistry,
    ) -> Result<Vec<&ModelInfo>> {
        let cost_limit = user_tier.get_cost_limit();
        
        // For free tier or low priority requests, prefer free models
        if cost_limit == 0.0 || request.priority <= RequestPriority::Low {
            let free_models: Vec<&ModelInfo> = candidates
                .into_iter()
                .filter(|model| {
                    model.pricing.as_ref()
                        .map(|p| p.prompt == "0" && p.completion == "0")
                        .unwrap_or(true)
                })
                .collect();
            
            if !free_models.is_empty() {
                debug!("CostOptimizationStrategy: Using {} free models", free_models.len());
                return Ok(free_models);
            }
        }

        // For paid tiers, filter by cost efficiency
        let mut cost_efficient: Vec<(&ModelInfo, f64)> = candidates
            .into_iter()
            .filter_map(|model| {
                let cost = model.pricing.as_ref()
                    .and_then(|p| {
                        let prompt_cost = p.prompt.parse::<f64>().ok()?;
                        let completion_cost = p.completion.parse::<f64>().ok()?;
                        Some(prompt_cost + completion_cost)
                    })
                    .unwrap_or(0.0);
                
                if cost <= cost_limit {
                    Some((model, cost))
                } else {
                    None
                }
            })
            .collect();

        // Sort by cost (ascending)
        cost_efficient.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
        
        let filtered = cost_efficient.into_iter().map(|(model, _)| model).collect();
        debug!("CostOptimizationStrategy filtered to {} cost-efficient models", filtered.len());
        Ok(filtered)
    }
}

/// Filter models based on performance requirements
#[derive(Debug)]
pub struct PerformanceStrategy;

#[async_trait::async_trait]
impl RoutingStrategy for PerformanceStrategy {
    fn name(&self) -> &'static str {
        "Performance"
    }

    async fn filter_models(
        &self,
        candidates: Vec<&ModelInfo>,
        request: &CompletionRequest,
        _user_tier: &UserTier,
        _registry: &ModelRegistry,
    ) -> Result<Vec<&ModelInfo>> {
        // For high priority requests, filter by context length and capabilities
        if request.priority >= RequestPriority::High {
            let min_context_length = if request.document_context.len() > 5 {
                32000 // Long documents need high context
            } else {
                8000  // Normal context requirement
            };

            let filtered = candidates
                .into_iter()
                .filter(|model| model.context_length >= min_context_length)
                .collect();

            debug!("PerformanceStrategy: High priority filtered to {} high-context models", filtered.len());
            return Ok(filtered);
        }

        // For normal/low priority, no additional filtering
        debug!("PerformanceStrategy: Normal priority, no filtering applied");
        Ok(candidates)
    }
}

/// Fallback strategy to ensure we always have models available
#[derive(Debug)]
pub struct FallbackStrategy;

#[async_trait::async_trait]
impl RoutingStrategy for FallbackStrategy {
    fn name(&self) -> &'static str {
        "Fallback"
    }

    async fn filter_models(
        &self,
        candidates: Vec<&ModelInfo>,
        _request: &CompletionRequest,
        user_tier: &UserTier,
        registry: &ModelRegistry,
    ) -> Result<Vec<&ModelInfo>> {
        if !candidates.is_empty() {
            return Ok(candidates);
        }

        // If no candidates remain, try to provide fallback options
        warn!("FallbackStrategy activated - no models available from previous filters");

        // Try free models first
        let free_models = registry.get_free_models();
        let accessible_free: Vec<&ModelInfo> = free_models
            .into_iter()
            .filter(|model| user_tier.can_access_model(&model.id))
            .collect();

        if !accessible_free.is_empty() {
            debug!("FallbackStrategy: Using {} accessible free models", accessible_free.len());
            return Ok(accessible_free);
        }

        // Last resort: any model the user can access
        let all_accessible: Vec<&ModelInfo> = registry.models.values()
            .filter(|model| user_tier.can_access_model(&model.id))
            .collect();

        debug!("FallbackStrategy: Last resort - {} accessible models", all_accessible.len());
        Ok(all_accessible)
    }
}

impl Default for ModelRouter {
    fn default() -> Self {
        Self::new()
    }
} 