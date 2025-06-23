use std::any::TypeId;

use crate::{
    entity::{Entity, EntityView},
    world::World,
};

use super::{Component, ComponentView};

/// Fetch component id from the world.
#[derive(Clone, Copy)]
pub struct Id<T> {
    __m: std::marker::PhantomData<T>,
}

/// Use id of the component.
#[inline]
pub fn id<T: Component>() -> Id<T> {
    Id::<T> {
        __m: std::marker::PhantomData,
    }
}

/// Trait for things that can be turned into an id.
///
/// Common examples is [Id<T>] which finds the registered id of `T` inside the world
/// the operation is being called in.
pub trait IdFetcher {
    /// Retrieves identifier from the world.
    fn retrieve_id(&self, world: &World) -> Entity;
}

//Id fetching capabilities for Entity
impl IdFetcher for Entity {
    fn retrieve_id(&self, _world: &World) -> Entity {
        *self
    }
}

//Id fetching capabilities for EntityView
impl<'a> IdFetcher for EntityView<'a> {
    fn retrieve_id(&self, _world: &World) -> Entity {
        self.entity_id
    }
}

//Id fetching capabilities for ComponentView
impl<'a> IdFetcher for ComponentView<'a> {
    fn retrieve_id(&self, _world: &World) -> Entity {
        self.entity_id
    }
}

//Id fetching capabilities id structus
//component
impl<T: Component> IdFetcher for Id<T> {
    fn retrieve_id(&self, world: &World) -> Entity {
        let Some(id) = world.component_map.get(&TypeId::of::<T>()) else {
            panic!(
                "component {:?} not implemented",
                core::any::type_name::<T>()
            )
        };
        *id
    }
}
