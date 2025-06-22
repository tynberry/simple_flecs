pub mod id;

use std::{any::Any, ffi::CStr};

use crate::{
    entity::{Entity, EntityView},
    world::World,
};
use flecs_ecs_sys::*;

/// Component for the ECS, works more as a marker.
pub trait Component: Any + Sized {
    /// Whether we need to register a Drop dtor hook.
    const NEEDS_DROP: bool = std::mem::needs_drop::<Self>();
}

/// Tag for the ECS, works more as a marker.
///
/// Tags are components which have no data on them.
/// It is an error to use this trait on a struct which has any kind of data inside it.
pub trait Tag: Any + Sized {}

/// Builder pattern for component manipulation.
#[derive(Clone, Copy)]
pub struct ComponentView<'a> {
    pub(crate) world: &'a World,
    pub(crate) entity_id: Entity,
}

impl<'a> ComponentView<'a> {
    /// Treats component as entity.
    #[inline]
    pub fn into_entity_view(self) -> EntityView<'a> {
        EntityView {
            world: self.world,
            entity_id: self.entity_id,
        }
    }

    /// Gets component id.
    #[inline]
    pub fn id(&self) -> Entity {
        self.entity_id
    }

    /// Gets component name.
    #[inline]
    pub fn name(&self) -> &CStr {
        let name = unsafe { ecs_get_name(self.world.ptr(), self.entity_id) };
        unsafe { CStr::from_ptr(name) }
    }

    /// Sets component name.
    #[inline]
    pub fn set_name(&self, name: &CStr) {
        unsafe { ecs_set_name(self.world.ptr(), self.entity_id, name.as_ptr()) };
    }

    /// Enables component.
    #[inline]
    pub fn enable(&self) {
        unsafe { ecs_enable(self.world.ptr(), self.entity_id, true) };
    }

    /// Disables component.
    #[inline]
    pub fn disable(&self) {
        unsafe { ecs_enable(self.world.ptr(), self.entity_id, false) };
    }

    /// Adds a trait.
    #[inline]
    pub fn add_trait(&self, trait_id: impl Into<Entity>) {
        unsafe { ecs_add_id(self.world.ptr(), self.entity_id, trait_id.into()) }
    }
}
