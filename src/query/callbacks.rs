use crate::{component::Component, entity::Entity};

/// Trait implemented by functions suitable to be used to order query results.
pub trait OrderByFunc<T: Component> {
    fn to_extern(self) -> extern "C" fn(Entity, &T, Entity, &T) -> i32;
}

impl<T: Component> OrderByFunc<T> for extern "C" fn(Entity, &T, Entity, &T) -> i32 {
    fn to_extern(self) -> extern "C" fn(Entity, &T, Entity, &T) -> i32 {
        self
    }
}
