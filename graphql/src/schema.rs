use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use crate::objects::{Query, };

pub struct GQLSchema;
pub type ServiceSchema = Schema<Query, EmptyMutation, EmptySubscription>;

impl GQLSchema {
    pub fn new() -> ServiceSchema {
        Schema::build(
            Query::default(),
            EmptyMutation,
            EmptySubscription
        ).finish()
    }
}