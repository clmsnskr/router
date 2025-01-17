//! Calls out to nodejs query planner

use std::sync::Arc;

use crate::prelude::graphql::*;
use async_trait::async_trait;
use router_bridge::plan;

/// A query planner that calls out to the nodejs router-bridge query planner.
///
/// No caching is performed. To cache, wrap in a [`CachingQueryPlanner`].
#[derive(Debug)]
pub struct RouterBridgeQueryPlanner {
    schema: Arc<Schema>,
}

impl RouterBridgeQueryPlanner {
    /// Create a new router-bridge query planner
    pub fn new(schema: Arc<Schema>) -> Self {
        Self { schema }
    }
}

#[async_trait]
impl QueryPlanner for RouterBridgeQueryPlanner {
    async fn get(
        &self,
        query: String,
        operation: Option<String>,
        options: QueryPlanOptions,
    ) -> Result<QueryPlan, QueryPlannerError> {
        let context = plan::OperationalContext {
            schema: self.schema.as_str().to_string(),
            query,
            operation_name: operation.unwrap_or_default(),
        };

        tokio::task::spawn_blocking(|| {
            plan::plan(context, options.into())
                .map_err(|e| QueryPlannerError::PlanningErrors(Arc::new(e)))
        })
        .await?
    }
}

impl From<QueryPlanOptions> for plan::QueryPlanOptions {
    fn from(_: QueryPlanOptions) -> Self {
        plan::QueryPlanOptions::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_env_log::test;

    #[test(tokio::test)]
    async fn test_plan() {
        let planner = RouterBridgeQueryPlanner::new(Arc::new(
            include_str!("testdata/schema.graphql").parse().unwrap(),
        ));
        let result = planner
            .get(
                include_str!("testdata/query.graphql").into(),
                None,
                QueryPlanOptions::default(),
            )
            .await;
        assert_eq!(
            QueryPlan {
                node: Some(PlanNode::Fetch(FetchNode {
                    service_name: "accounts".into(),
                    requires: None,
                    variable_usages: vec![],
                    operation: "{me{name{first last}}}".into()
                }))
            },
            result.unwrap()
        );
    }

    #[test(tokio::test)]
    async fn test_plan_error() {
        let planner = RouterBridgeQueryPlanner::new(Arc::new("".parse().unwrap()));
        let result = planner
            .get("".into(), None, QueryPlanOptions::default())
            .await;

        assert_eq!(
            "Query planning had errors: Planning errors: UNKNOWN: Syntax Error: Unexpected <EOF>.",
            result.unwrap_err().to_string()
        );
    }
}
