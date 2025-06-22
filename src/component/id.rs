use std::any::TypeId;

use crate::{
    entity::{Entity, EntityView},
    world::World,
};

use super::{Component, ComponentView, Tag};

/// Fetch component id from the world.
#[derive(Clone, Copy)]
pub struct CompId<T> {
    __m: std::marker::PhantomData<T>,
}

/// Fetch tag id from the world.
#[derive(Clone, Copy)]
pub struct TagId<T> {
    __m: std::marker::PhantomData<T>,
}

/// Use id of the component.
#[inline]
pub fn comp<T: Component>() -> CompId<T> {
    CompId::<T> {
        __m: std::marker::PhantomData,
    }
}

/// Use id of the tag.
#[inline]
pub fn tag<T: Tag>() -> TagId<T> {
    TagId::<T> {
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

/// Trait for things that can be turned into an id representing a component.
///
/// Common examples is [Id<T>] which finds the registered id of `T` inside the world
/// the operation is being called in.
pub trait ComponentIdFetcher: IdFetcher {}

/// Trait for things that can be turned into an id representing a tag.
///
/// Common examples is [Id<T>] which finds the registered id of `T` inside the world
/// the operation is being called in.
pub trait TagIdFetcher: IdFetcher {}

//Id fetching capabilities for Entity
impl IdFetcher for Entity {
    fn retrieve_id(&self, _world: &World) -> Entity {
        *self
    }
}

impl ComponentIdFetcher for Entity {}
impl TagIdFetcher for Entity {}

//Id fetching capabilities for EntityView
impl<'a> IdFetcher for EntityView<'a> {
    fn retrieve_id(&self, _world: &World) -> Entity {
        self.entity_id
    }
}

impl<'a> ComponentIdFetcher for EntityView<'a> {}
impl<'a> TagIdFetcher for EntityView<'a> {}

//Id fetching capabilities for ComponentView
impl<'a> IdFetcher for ComponentView<'a> {
    fn retrieve_id(&self, _world: &World) -> Entity {
        self.entity_id
    }
}

impl<'a> ComponentIdFetcher for ComponentView<'a> {}
impl<'a> TagIdFetcher for ComponentView<'a> {}

//Id fetching capabilities id structus
//component
impl<T: Component> IdFetcher for CompId<T> {
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

impl<T: Component> ComponentIdFetcher for CompId<T> {}

//tag
impl<T: Tag> IdFetcher for TagId<T> {
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

impl<T: Tag> TagIdFetcher for TagId<T> {}
