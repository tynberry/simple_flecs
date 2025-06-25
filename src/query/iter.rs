use std::{
    ffi::CStr,
    marker::PhantomData,
    ops::{Deref, Index, IndexMut},
    ptr::NonNull,
};

use flecs_ecs_sys::*;

use crate::{
    component::{
        Component,
        id::{IdFetcher, id},
    },
    entity::Entity,
    world::{ComponentMap, World},
};

/// Field of an component inside an iterator.
pub struct Field<'a, T: Component> {
    cache_field: NonNull<T>,
    length: usize,
    is_on_self: bool,
    __m: PhantomData<&'a ()>,
}

impl<'a, T: Component> Index<usize> for Field<'a, T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        //check bounds
        if index > self.length {
            panic!(
                "index out of bounds: the length is {}, but the index is {}",
                self.length, index
            );
        }
        //find thing
        if self.is_on_self {
            unsafe { self.cache_field.offset(index as isize).as_ref() }
        } else {
            unsafe { self.cache_field.as_ref() }
        }
    }
}

impl<'a, T: Component> IndexMut<usize> for Field<'a, T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        //check bounds
        if index > self.length {
            panic!(
                "index out of bounds: the length is {}, but the index is {}",
                self.length, index
            );
        }
        //find thing
        if self.is_on_self {
            unsafe { self.cache_field.offset(index as isize).as_mut() }
        } else {
            unsafe { self.cache_field.as_mut() }
        }
    }
}

/// Owned or pointer to sys iterator.
#[allow(clippy::large_enum_variant)]
pub(crate) enum MaybeOwnedIter {
    Owned(ecs_iter_t),
    Ptr(NonNull<ecs_iter_t>),
}

impl MaybeOwnedIter {
    /// Converts the iterator to a pointer.
    pub fn as_ptr(&self) -> *mut ecs_iter_t {
        match self {
            MaybeOwnedIter::Owned(iter) => iter as *const _ as *mut _,
            MaybeOwnedIter::Ptr(ptr) => ptr.as_ptr(),
        }
    }
}

impl Deref for MaybeOwnedIter {
    type Target = ecs_iter_t;

    fn deref(&self) -> &Self::Target {
        match self {
            MaybeOwnedIter::Owned(iter) => iter,
            MaybeOwnedIter::Ptr(ptr) => unsafe { ptr.as_ref() },
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
///
/// SYSTEM = true, means that iterator was produced by a system.
/// SYSTEM = false, means the iterator comes from a query.
pub struct Iter<const SYSTEM: bool> {
    pub(crate) iter: MaybeOwnedIter,
}

impl<const SYSTEM: bool> Iter<SYSTEM> {
    /// Jumps to the next table in the iterator.
    #[inline]
    pub fn advance(&mut self) -> bool {
        if SYSTEM {
            unsafe { ecs_iter_next(self.iter.as_ptr()) }
        } else {
            unsafe { ecs_query_next(self.iter.as_ptr()) }
        }
    }

    /// Accesses relevant world.
    ///
    /// Creates a non owning reference, dropping it does not drop neither the world nor the
    /// component map.
    #[inline]
    pub fn world(&self) -> World {
        unsafe {
            //if SYSTEM {
            //    let binding = self.iter.binding_ctx as *mut *mut ComponentMap;
            //    World::from_ptr_and_map(self.iter.world, *binding)
            //} else {
            World::from_ptr_and_map(self.iter.world, self.iter.binding_ctx as *mut ComponentMap)
            //}
        }
    }

    /// Checks the id of a term.
    pub fn check_id(&self, id: impl IdFetcher, index: i8) -> bool {
        // SAFETY:
        // Unless mutliple threads are accessing the same iterator, this is safe.
        let it_ptr = self.iter.as_ptr();
        //check id
        let world_ref = unsafe {
            World::from_ptr_and_map(self.iter.world, self.iter.binding_ctx as *mut ComponentMap)
        };
        let component_id = id.retrieve_id(&world_ref);
        let field_id = unsafe { ecs_field_id(it_ptr, index) };
        component_id == field_id
    }

    /// Checks if the iterator has a certain term.
    pub fn has(&self, index: i8) -> bool {
        // SAFETY:
        // Unless mutliple threads are accessing the same iterator, this is safe.
        let it_ptr = self.iter.as_ptr();
        unsafe { ecs_field_is_set(it_ptr, index) }
    }

    /// Returns a source entity of a term, if it returns None it means that the component is stored
    /// inside a field array.
    pub fn source(&self, index: i8) -> Option<Entity> {
        // SAFETY:
        // Unless mutliple threads are accessing the same iterator, this is safe.
        let it_ptr = self.iter.as_ptr();
        let src = unsafe { ecs_field_src(it_ptr, index) };
        if src == 0 { None } else { Some(src) }
    }

    /// Sets a variable to an id.
    pub fn set_var(&self, variable: &CStr, id: impl IdFetcher) {
        //retrieve variable id
        let var_loc = unsafe { ecs_query_find_var(self.iter.query, variable.as_ptr()) };
        //retrieve entity id
        let world_ref = unsafe {
            World::from_ptr_and_map(self.iter.world, self.iter.binding_ctx as *mut ComponentMap)
        };
        let id = id.retrieve_id(&world_ref);
        // SAFETY:
        // Unless mutliple threads are accessing the same iterator, this is safe.
        let it_ptr = self.iter.as_ptr();
        unsafe {
            ecs_iter_set_var(it_ptr, var_loc, id);
        }
    }

    /// Get count of entities in the current table.
    #[inline]
    pub fn count(&self) -> usize {
        self.iter.count as usize
    }

    /// Get entity from the index.
    pub fn entity(&self, index: usize) -> Option<Entity> {
        //check bounds
        if index > self.iter.count as usize {
            return None;
        }
        Some(unsafe { *self.iter.entities.add(index) })
    }

    /// Retrieves a component field from the current table in the iterator.
    ///
    /// # Safety
    ///
    /// The Component must be correct. The term queries must not be sparse or use the Or operator.
    ///
    /// If the component is borrowed read-only (\[in\] access modifier) or write-only (\[out\]),
    /// you must use the returned Field with such considerations.
    pub unsafe fn get<'a, T: Component>(&'a self, index: i8) -> Option<Field<'a, T>> {
        const {
            if T::IS_TAG {
                panic!("cannot get a field of tags");
            }
        }
        // SAFETY:
        // Unless mutliple threads are accessing the same iterator, this is safe.
        let it_ptr = self.iter.as_ptr();
        //check if the field is set
        if unsafe { !ecs_field_is_set(it_ptr, index) } {
            return None;
        }
        //gain access to the field
        let cache_field = unsafe { ecs_field_w_size(it_ptr, core::mem::size_of::<T>(), index) };
        let is_on_self = unsafe { ecs_field_is_self(it_ptr, index) };
        Some(Field {
            cache_field: NonNull::new(cache_field as *mut T).unwrap(),
            length: self.iter.count as usize,
            is_on_self,
            __m: PhantomData,
        })
    }

    /// Retrieves a component from the table according to a component id for a certain entity.
    ///
    /// Used to access Or operator fields.
    ///
    /// # Safety
    ///
    /// The Component must be correct. The term queries must not be sparse (todo, check
    /// information).
    ///
    /// If the component is borrowed read-only (\[in\] access modifier) or write-only (\[out\]),
    /// you must use the returned Field with such considerations.
    pub unsafe fn get_from_table<T: Component>(&self) -> Option<&mut [T]> {
        //check id
        let world_ref = unsafe {
            World::from_ptr_and_map(self.iter.world, self.iter.binding_ctx as *mut ComponentMap)
        };
        let component_id = id::<T>().retrieve_id(&world_ref);
        //get data
        let ptr = unsafe {
            ecs_table_get_id(
                self.iter.world,
                self.iter.table,
                component_id,
                self.iter.offset,
            )
        };
        if ptr.is_null() {
            return None;
        }
        unsafe {
            Some(core::slice::from_raw_parts_mut(
                ptr as *mut T,
                self.iter.count as usize,
            ))
        }
    }
}
