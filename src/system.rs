use std::{
    ffi::{CStr, CString, c_void},
    ptr::NonNull,
};

use flecs_ecs_sys::*;

use crate::{
    c_types::{ECS_QUERY_MATCH_DISABLED, ECS_QUERY_MATCH_EMPTY_TABLES, ECS_QUERY_MATCH_PREFAB},
    component::{
        Component,
        id::{IdFetcher, id},
    },
    entity::Entity,
    flecs::{DependsOn, pipeline::OnUpdate},
    query::{
        callbacks::OrderByFunc,
        iter::{Iter, MaybeOwnedIter},
    },
    world::{ComponentMap, World},
};

/// Binding context for systems.
#[repr(C)]
pub struct CallbackContext<F: Fn(Iter<true>)> {
    /// This field must always be first!
    component_map: *mut ComponentMap,
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

    /// Sets system's rate.
    ///
    /// If the rate is set to 2, the system will run every second frame.
    /// If set to 3, it will run every third frame, and so on.
    pub fn rate(mut self, rate: u32) -> Self {
        self.inner.rate = rate as i32;
        self
    }

    /// Sets system's interval.
    pub fn interval(mut self, interval: f32) -> Self {
        self.inner.interval = interval;
        self
    }

    /// Finishes this system with default callback.
    pub fn build<F>(mut self, callback: F)
    where
        F: Fn(Iter<true>) + 'static,
    {
        //set callback
        self.inner.callback = Some(system_callback::<F>);
        self.inner.callback_ctx = Box::leak(Box::new(CallbackContext {
            component_map: self.world.component_map.as_ptr(),
            func: callback,
        })) as *mut _ as *mut c_void;
        self.inner.callback_ctx_free = Some(callback_ctx_free::<F>);
        //creates an entity
        let entity = self.world.entity();
        //adds a kind if any
        if self.kind != 0 {
            entity.add((DependsOn, self.kind));
        }
        //sets the entity
        self.inner.entity = entity.id();
        //creates the system
        unsafe {
            ecs_system_init(self.world.ptr(), &self.inner as *const _);
        }
    }

    /// Finishes this system with default callback.
    pub fn build_named<F>(mut self, name: &CStr, callback: F)
    where
        F: Fn(Iter<true>) + 'static,
    {
        //set callback
        self.inner.callback = Some(system_callback::<F>);
        self.inner.callback_ctx = Box::leak(Box::new(CallbackContext {
            component_map: self.world.component_map.as_ptr(),
            func: callback,
        })) as *mut _ as *mut c_void;
        self.inner.callback_ctx_free = Some(callback_ctx_free::<F>);
        //creates an entity
        let entity = self.world.entity_named(name);
        //adds a kind if any
        if self.kind != 0 {
            entity.add((DependsOn, self.kind));
        } else {
            entity.add((DependsOn, OnUpdate));
        }
        //sets the entity
        self.inner.entity = entity.id();
        //creates the system
        unsafe {
            ecs_system_init(self.world.ptr(), &self.inner as *const _);
        }
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

unsafe extern "C" fn callback_ctx_free<F: Fn(Iter<true>) + 'static>(ctx: *mut c_void) {
    // SAFETY:
    // ctx is a pointer to CallbackContext<F>, so we can safely cast it back.
    let _ = unsafe { Box::from_raw(ctx as *mut CallbackContext<F>) };
}
