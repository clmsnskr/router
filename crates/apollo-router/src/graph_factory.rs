use crate::configuration::Configuration;
use crate::http_service_registry::HttpServiceRegistry;
use apollo_router_core::prelude::{graphql::*, *};
use async_trait::async_trait;
#[cfg(test)]
use mockall::{automock, predicate::*};
use std::sync::Arc;

/// Factory for creating graphs.
///
/// This trait enables us to test that `StateMachine` correctly recreates the FederatedGraph when
/// necessary e.g. when schema changes.
#[cfg_attr(test, automock)]
#[async_trait]
pub(crate) trait GraphFactory<F>
where
    F: graphql::Fetcher,
{
    async fn create(&self, configuration: &Configuration, schema: Arc<graphql::Schema>) -> F;
}

#[derive(Default)]
pub(crate) struct FederatedGraphFactory;

#[async_trait]
impl GraphFactory<graphql::FederatedGraph> for FederatedGraphFactory {
    async fn create(
        &self,
        configuration: &Configuration,
        schema: Arc<graphql::Schema>,
    ) -> graphql::FederatedGraph {
        let service_registry = HttpServiceRegistry::new(configuration);
        let configuration = configuration.clone();
        tokio::task::spawn_blocking(move || {
            let query_planner =
                graphql::RouterBridgeQueryPlanner::new(Arc::clone(&schema)).with_caching();
            let extensions = configuration.load_wasm_modules().unwrap();

            graphql::FederatedGraph::new(
                Arc::new(query_planner),
                Arc::new(service_registry),
                schema,
                extensions,
            )
        })
        .await
        .expect("FederatedGraph::new() is infallible; qed")
    }
}
