use super::GuardAction;
use async_graphql::dynamic::ResolverContext;
use sea_orm::{entity::ActiveModelTrait, Condition};
use std::{any::Any, ops::Deref};

pub struct LifecycleHooks(pub(crate) Box<dyn LifecycleHooksInterface>);

impl Deref for LifecycleHooks {
    type Target = dyn LifecycleHooksInterface;

    fn deref(&self) -> &Self::Target {
        &*self.0
    }
}

impl Default for LifecycleHooks {
    fn default() -> Self {
        Self(Box::new(DefaultLifecycleHook))
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum OperationType {
    Read,
    Create,
    Update,
    Delete,
}

impl LifecycleHooks {
    pub fn new<T: LifecycleHooksInterface + 'static>(t: T) -> Self {
        Self(Box::new(t))
    }
}

pub trait LifecycleHooksInterface: Send + Sync {
    fn entity_guard(
        &self,
        _ctx: &ResolverContext,
        _entity: &str,
        _action: OperationType,
    ) -> GuardAction {
        GuardAction::Allow
    }

    fn field_guard(
        &self,
        _ctx: &ResolverContext,
        _entity: &str,
        _field: &str,
        _action: OperationType,
    ) -> GuardAction {
        GuardAction::Allow
    }

    fn entity_filter(
        &self,
        _ctx: &ResolverContext,
        _entity: &str,
        _action: OperationType,
    ) -> Option<Condition> {
        None
    }
}

pub struct DefaultLifecycleHook;

impl LifecycleHooksInterface for DefaultLifecycleHook {}

pub struct DynamicMutationHooks(pub(crate) Box<dyn DynamicMutationHooksInterface>);

impl DynamicMutationHooks {
    pub fn new<T: DynamicMutationHooksInterface + 'static>(t: T) -> Self {
        Self(Box::new(t))
    }
}

impl Deref for DynamicMutationHooks {
    type Target = dyn DynamicMutationHooksInterface;

    fn deref(&self) -> &Self::Target {
        &*self.0
    }
}

pub trait DynamicMutationHooksInterface: Send + Sync {
    fn mutation(
        &self,
        _ctx: &ResolverContext,
        _model: &mut dyn Any,
        _action: OperationType,
    ) -> GuardAction;
}

impl<A> DynamicMutationHooksInterface for A
where
    A: MutationHooksInterface,
{
    fn mutation(
        &self,
        ctx: &ResolverContext,
        model: &mut dyn Any,
        action: OperationType,
    ) -> GuardAction {
        model.downcast_mut::<A>().unwrap().mutation(ctx, action)
    }
}

pub trait MutationHooksInterface: ActiveModelTrait + Any + Send + Sync {
    fn mutation(&mut self, _ctx: &ResolverContext, _action: OperationType) -> GuardAction;
}
