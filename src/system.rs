use std::{
    ffi::{CStr, CString, c_void},
    ptr::NonNull,
};

use flecs_ecs_sys::*;

use crate::{
    c_types::{
        ECS_QUERY_MATCH_DISABLED, ECS_QUERY_MATCH_EMPTY_TABLES, ECS_QUERY_MATCH_PREFAB,
        QueryCacheKind,
    },
    component::{
        Component,
        id::{IdFetcher, id},
    },
    entity::Entity,
    query::{
        callbacks::OrderByFunc,
        iter::{Iter, MaybeOwnedIter},
    },
    world::World,
};

/// Binding context for systems.
#[repr(C)]
pub struct CallbackContext<F: Fn(Iter<true>)> {
    func: F,
}

/// Builder for creating queries. Allows you to set certain flags and the components to request.
#[derive(Debug)]
pub struct SystemBuilder<'a> {
    pub(crate) inner: ecs_system_desc_t,
    pub(crate) expr: Option<CString>,
    pub(crate) kind: Entity,
    pub(crate) world: &'a World,
}

impl<'a> SystemBuilder<'a> {
    /// Sets an expression as the base for the query.
    ///
    /// This allocates a string, due to lifetimes.
    pub fn expression(mut self, expr: &CStr) -> Self {
        self.expr = Some(expr.to_owned());
        self.inner.query.expr = self.expr.as_ref().unwrap().as_ptr();
        self
    }

    /// Sets query's cache kind.
    pub fn set_cache(mut self, kind: QueryCacheKind) -> Self {
        self.inner.query.cache_kind = kind as u32;
        self
    }

    /// Sets query to match prefabs.
    pub fn match_prefabs(mut self) -> Self {
        self.inner.query.flags |= ECS_QUERY_MATCH_PREFAB as u32;
        self
    }

    /// Sets query to match disabled.
    pub fn match_disabled(mut self) -> Self {
        self.inner.query.flags |= ECS_QUERY_MATCH_DISABLED as u32;
        self
    }

    /// Sets query to match empty tables.
    pub fn match_empty_tables(mut self) -> Self {
        self.inner.query.flags |= ECS_QUERY_MATCH_EMPTY_TABLES as u32;
        self
    }

    /// Orders output by a component.
    pub fn order_by<T: Component, F: OrderByFunc<T>>(mut self, callback: F) -> Self {
        self.inner.query.order_by = id::<T>().retrieve_id(self.world);
        self.inner.query.order_by_callback = unsafe {
            // SAFETY:
            // Components are sized, so references are thin, though it is an evil hack.
            let extern_fun = std::mem::transmute::<
                extern "C" fn(u64, &T, u64, &T) -> i32,
                unsafe extern "C" fn(u64, *const c_void, u64, *const c_void) -> i32,
            >(callback.to_extern());
            Some(extern_fun)
        };
        self
    }

    /// Sets system kind.
    pub fn kind(mut self, id: impl IdFetcher) -> Self {
        self.kind = id.retrieve_id(self.world);
        self
    }

    /// Finishes this system with default callback.
    pub fn build<F>(mut self, callback: F)
    where
        F: Fn(Iter<true>) + 'static,
    {
        //set callback
        self.inner.callback = Some(system_callback::<F>);
        self.inner.callback_ctx =
            Box::leak(Box::new(CallbackContext { func: callback })) as *mut _ as *mut c_void;
        self.inner.query.binding_ctx = self.world.component_map.as_ptr() as *mut c_void;
    }
}

unsafe extern "C" fn system_callback<F: Fn(Iter<true>) + 'static>(iter: *mut ecs_iter_t) {
    //retrieve the function
    let context = unsafe { (*iter).callback_ctx as *mut CallbackContext<F> };
    let context = unsafe { context.as_ref().unwrap() };
    //create iterator
    let iter = Iter::<true> {
        iter: MaybeOwnedIter::Ptr(NonNull::new(iter).unwrap()),
    };
    //call callback
    (context.func)(iter);
}
