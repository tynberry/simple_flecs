use std::any::TypeId;

use flecs_ecs_sys::ecs_make_pair;

use crate::{
    entity::{Entity, EntityView},
    world::World,
};

use super::{Component, ComponentView, traits::ComponentOrPair};

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

/// Type when the type is unknown.
pub struct UnknownType;
impl Component for UnknownType {
    const ID: Option<Entity> = Some(0);
}

/// Trait for things that can be turned into an id.
///
/// Common examples is [Id<T>] which finds the registered id of `T` inside the world
/// the operation is being called in.
pub trait IdFetcher {
    /// Component of pair the Id represents.
    ///
    /// Is UnknownType if the type is.. unknown.
    type COMPONENT: ComponentOrPair;
    /// Retrieves identifier from the world.
    fn retrieve_id(&self, world: &World) -> Entity;
}

//Id fetching capabilities for Entity
impl IdFetcher for Entity {
    type COMPONENT = UnknownType;
    fn retrieve_id(&self, _world: &World) -> Entity {
        *self
    }
}

//Id fetching capabilities for EntityView
impl<'a> IdFetcher for EntityView<'a> {
    type COMPONENT = UnknownType;
    fn retrieve_id(&self, _world: &World) -> Entity {
        self.entity_id
    }
}

//Id fetching capabilities for ComponentView
impl<'a> IdFetcher for ComponentView<'a> {
    type COMPONENT = UnknownType;
    fn retrieve_id(&self, _world: &World) -> Entity {
        self.entity_id
    }
}

//Id fetching capabilities id structus
//component
impl<T: Component> IdFetcher for Id<T> {
    type COMPONENT = T;
    fn retrieve_id(&self, world: &World) -> Entity {
        //check if component has const id
        if let Some(id) = T::ID {
            return id;
        }
        //retrieve dynamicaly
        let Some(id) = world.component_map.get(&TypeId::of::<T>()) else {
            panic!(
                "component {:?} not implemented",
                core::any::type_name::<T>()
            )
        };
        *id
    }
}

//Id fetching for pairs
impl<L, R> IdFetcher for (L, R)
where
    L: IdFetcher,
    R: IdFetcher,
    L::COMPONENT: Component,
    R::COMPONENT: Component,
{
    type COMPONENT = (L::COMPONENT, R::COMPONENT);
    fn retrieve_id(&self, world: &World) -> Entity {
        let left = self.0.retrieve_id(world);
        let right = self.1.retrieve_id(world);
        unsafe { ecs_make_pair(left, right) }
    }
}
