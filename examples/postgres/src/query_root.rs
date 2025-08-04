use crate::entities::*;
use async_graphql::dynamic::{ResolverContext, Schema, SchemaError};
use chrono::Utc;
use sea_orm::{entity::ActiveValue, DatabaseConnection};
use seaography::{
    async_graphql, lazy_static, Builder, BuilderContext, GuardAction, MutationHooksInterface,
    OperationType,
};

lazy_static::lazy_static! {
    static ref CONTEXT : BuilderContext = {
        let mut context = BuilderContext::default();
        context.register_mutation_hook::<actor::ActiveModel>();
        context
    };
}

pub fn schema(
    database: DatabaseConnection,
    depth: Option<usize>,
    complexity: Option<usize>,
) -> Result<Schema, SchemaError> {
    let mut builder = Builder::new(&CONTEXT, database.clone());
    seaography::register_entities!(
        builder,
        [
            film_actor,
            rental,
            category,
            staff,
            country,
            film,
            actor,
            language,
            city,
            inventory,
            film_category,
            customer,
            store,
            payment,
            address,
        ]
    );
    builder.register_enumeration::<crate::entities::sea_orm_active_enums::MpaaRating>();
    builder
        .set_depth_limit(depth)
        .set_complexity_limit(complexity)
        .schema_builder()
        .data(database)
        .finish()
}

impl MutationHooksInterface for actor::ActiveModel {
    fn mutation(&mut self, _ctx: &ResolverContext, _action: OperationType) -> GuardAction {
        if let Some(v) = self.first_name.try_as_ref() {
            if v.is_empty() {
                return GuardAction::Block(Some("first_name must not be empty".to_string()));
            }
        }

        if let Some(v) = self.last_name.try_as_ref() {
            if v.is_empty() {
                return GuardAction::Block(Some("last_name must not be empty".to_string()));
            }
        }

        self.last_update = ActiveValue::Set(Utc::now().naive_utc());
        GuardAction::Allow
    }
}
