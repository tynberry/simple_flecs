use std::{
    marker::PhantomData,
    ops::{Index, IndexMut},
    ptr::NonNull,
};

use flecs_ecs_sys::*;

use crate::{
    component::{Component, id::IdFetcher},
    world::{ComponentMap, World},
};

/// Field of an component inside an iterator.
pub struct Field<'a, T: Component> {
    cache_field: NonNull<T>,
    is_on_self: bool,
    __m: PhantomData<&'a ()>,
}

impl<'a, T: Component> Index<usize> for Field<'a, T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        if self.is_on_self {
            unsafe { self.cache_field.offset(index as isize).as_ref() }
        } else {
            unsafe { self.cache_field.as_ref() }
        }
    }
}

/// Field of an component inside an iterator.
/// Allows component mutation.
pub struct FieldMut<'a, T: Component> {
    cache_field: NonNull<T>,
    is_on_self: bool,
    __m: PhantomData<&'a ()>,
}

impl<'a, T: Component> Index<usize> for FieldMut<'a, T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        if self.is_on_self {
            unsafe { self.cache_field.offset(index as isize).as_ref() }
        } else {
            unsafe { self.cache_field.as_ref() }
        }
    }
}

impl<'a, T: Component> IndexMut<usize> for FieldMut<'a, T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if self.is_on_self {
            unsafe { self.cache_field.offset(index as isize).as_mut() }
        } else {
            unsafe { self.cache_field.as_mut() }
        }
    }
}

/// Table iterator for a query or system.
///
/// This iterator must not outlive the query, system it was created in.
///
/// When creating the iterator, it is expected that the binding context is set to the component
/// map pointer.
///
/// Iter must not be passed between threads and dynamic linking boundaries.
pub struct Iter {
    pub(crate) iter: ecs_iter_t,
}

impl Iter {
    /// Jumps to the next table in the iterator.
    #[inline]
    pub fn advance(&mut self) -> bool {
        unsafe { ecs_query_next(&mut self.iter as *mut _) }
    }

    /// Checks the id of a term.
    pub fn check_id(&self, id: impl IdFetcher, index: i8) -> bool {
        // SAFETY:
        // Unless mutliple threads are accessing the same iterator, this is safe.
        let it_ptr = &self.iter as *const ecs_iter_t as *mut _;
        //check id
        let world_ref = unsafe {
            World::from_ptr_and_map(self.iter.world, self.iter.binding_ctx as *mut ComponentMap)
        };
        let component_id = id.retrieve_id(&world_ref);
        let field_id = unsafe { ecs_field_id(it_ptr, index) };
        component_id != field_id
    }

    /// Retrieves a component field from the current table in the iterator.
    ///
    /// # Safety
    ///
    /// The Component must be correct.
    pub unsafe fn get<'a, T: Component>(&'a self, index: i8) -> Option<Field<'a, T>> {
        const {
            if T::IS_TAG {
                panic!("cannot get a field of tags");
            }
        }
        // SAFETY:
        // Unless mutliple threads are accessing the same iterator, this is safe.
        let it_ptr = &self.iter as *const ecs_iter_t as *mut _;
        //check if the field is set
        if unsafe { !ecs_field_is_set(it_ptr, index) } {
            return None;
        }
        //gain access to the field
        let cache_field = unsafe { ecs_field_w_size(it_ptr, core::mem::size_of::<T>(), index) };
        let is_on_self = unsafe { ecs_field_is_self(it_ptr, index) };
        Some(Field {
            cache_field: NonNull::new(cache_field as *mut T).unwrap(),
            is_on_self,
            __m: PhantomData,
        })
    }

    /// Retrieves a component field mutably from the current table in the iterator.
    ///
    /// # Safety
    ///
    /// The Component must be correct.
    pub unsafe fn get_mut<'a, T: Component>(&'a self, index: i8) -> Option<FieldMut<'a, T>> {
        const {
            if T::IS_TAG {
                panic!("cannot get a field of tags");
            }
        }
        // SAFETY:
        // Unless mutliple threads are accessing the same iterator, this is safe.
        let it_ptr = &self.iter as *const ecs_iter_t as *mut _;
        //check if the field is set
        if unsafe { !ecs_field_is_set(it_ptr, index) } {
            return None;
        }
        //gain access to the field
        let cache_field = unsafe { ecs_field_w_size(it_ptr, core::mem::size_of::<T>(), index) };
        let is_on_self = unsafe { ecs_field_is_self(it_ptr, index) };
        Some(FieldMut {
            cache_field: NonNull::new(cache_field as *mut T).unwrap(),
            is_on_self,
            __m: PhantomData,
        })
    }
}
