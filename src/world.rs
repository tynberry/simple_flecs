use ahash::AHashMap;
use flecs_ecs_sys::*;
use std::{
    any::TypeId,
    ffi::{CStr, c_void},
    ptr::{NonNull, null_mut},
};

use crate::{
    component::{
        Component, ComponentView,
        id::{Id, IdFetcher, id},
    },
    entity::{Entity, EntityView},
    flecs::rest::Rest,
    query::QueryBuilder,
    system::SystemBuilder,
};

/// Component map, mapping local typeids to registered entity ids.
pub type ComponentMap = AHashMap<TypeId, Entity>;

/// ECS world.
#[derive(Debug)]
pub struct World {
    /// Pointer to the underlying raw world.
    pub(crate) ptr: NonNull<ecs_world_t>,
    /// Owns the pointer?
    owned: bool,
    /// Leaked component map for this current crate.
    ///
    /// Because it is leaked, it will stay in one place till the world is dropped.
    ///
    /// World must **not** be passed across (at least) dynamic linking boundary.
    pub(crate) component_map: NonNull<ComponentMap>,
    /// Owns the component map.
    map_owned: bool,
}

//------------------------------------------------------------------------------
// WORLD LIFECYCLE
//------------------------------------------------------------------------------

impl Default for World {
    fn default() -> Self {
        //leak component map
        let component_map = Box::new(AHashMap::new());
        let component_map = Box::leak(component_map);
        //compose world
        Self {
            ptr: unsafe { NonNull::new(ecs_init()).expect("could not init ecs world") },
            owned: true,
            component_map: component_map.into(),
            map_owned: true,
        }
    }
}

impl World {
    /// Creates an empty new ECS world.
    pub fn new() -> Self {
        World::default()
    }

    /// Creates a referring world from a Flecs pointer.
    ///
    /// Component map of such world is empty and all components must be re-registered.
    ///
    /// # Safety
    ///
    /// The pointer must be a valid pointer to a Flecs world.
    pub unsafe fn from_ptr(ptr: *mut ecs_world_t) -> Self {
        assert!(!ptr.is_null(), "cannot create a world from a null pointer");
        //leak component map
        let component_map = Box::new(AHashMap::new());
        let component_map = Box::leak(component_map);
        World {
            ptr: unsafe { NonNull::new_unchecked(ptr) },
            owned: false,
            component_map: component_map.into(),
            map_owned: true,
        }
    }

    /// Creats a referring world from a Flecs pointer and pointer to a component map.
    ///
    /// Component map of such world must come from the same crate, it must not be
    /// transferred across dynamic linking boundaries.
    ///
    /// # Safety
    ///
    /// The pointer must be a valid pointer to a Flecs world and the component map must be valid
    /// and from the same place.
    pub unsafe fn from_ptr_and_map(
        ptr: *mut ecs_world_t,
        component_map: *mut ComponentMap,
    ) -> Self {
        assert!(!ptr.is_null(), "cannot create a world from a null pointer");
        assert!(
            !component_map.is_null(),
            "cannot create a world from a null component map pointer"
        );
        World {
            ptr: unsafe { NonNull::new_unchecked(ptr) },
            owned: false,
            component_map: unsafe { NonNull::new_unchecked(component_map) },
            map_owned: false,
        }
    }

    /// Retrieves underlying pointer.
    #[inline]
    pub fn ptr(&self) -> *mut ecs_world_t {
        self.ptr.as_ptr()
    }
}

impl Drop for World {
    fn drop(&mut self) {
        //if panicking, we are going to be disposed off anyways
        if std::thread::panicking() {
            return;
        }
        //clear component map
        if self.map_owned {
            let component_map = unsafe { self.component_map.as_mut() };
            let component_map = unsafe { Box::from_raw(component_map) };
            drop(component_map);
        }
        //we do not own the world
        if !self.owned {
            return;
        }
        //clear ecs
        unsafe { ecs_fini(self.ptr.as_ptr()) };
    }
}

//------------------------------------------------------------------------------
// ENTITY AND COMPONENTS
//------------------------------------------------------------------------------

impl World {
    /// Creates a new entity.
    pub fn entity(&self) -> EntityView<'_> {
        let edesc = Default::default();
        let entity = unsafe { ecs_entity_init(self.ptr(), &edesc as *const _) };
        EntityView {
            world: self,
            entity_id: entity,
        }
    }

    /// Creates a new named entity.
    pub fn entity_named(&self, name: &CStr) -> EntityView<'_> {
        let edesc = ecs_entity_desc_t {
            name: name.as_ptr(),
            ..Default::default()
        };
        let entity = unsafe { ecs_entity_init(self.ptr(), &edesc as *const _) };
        EntityView {
            world: self,
            entity_id: entity,
        }
    }

    /// Creates an entity view from a id.
    /// This does not create a new entity!
    #[inline]
    pub fn view(&self, id: Entity) -> EntityView<'_> {
        EntityView {
            world: self,
            entity_id: id,
        }
    }

    /// Lookup an entity by its name.
    pub fn lookup(&self, name: &CStr) -> Option<EntityView<'_>> {
        let entity = unsafe { ecs_lookup(self.ptr(), name.as_ptr()) };
        if entity == 0 {
            None
        } else {
            Some(EntityView {
                world: self,
                entity_id: entity,
            })
        }
    }

    /// Lookup an entity by its symbol.
    pub fn lookup_symbol(&self, name: &CStr) -> Option<EntityView<'_>> {
        let entity = unsafe { ecs_lookup_symbol(self.ptr(), name.as_ptr(), false, false) };
        if entity == 0 {
            None
        } else {
            Some(EntityView {
                world: self,
                entity_id: entity,
            })
        }
    }
}

impl World {
    /// Creates a new named data component or tag.
    pub fn component<T: Component>(&mut self, symbol: &CStr) -> ComponentView<'_> {
        //is it a tag
        if T::IS_TAG {
            return self.tag::<T>(symbol);
        }
        //is it already registered in flecs?
        if let Some(entity) = self.lookup_symbol(symbol) {
            let id = entity.entity_id;
            unsafe { self.component_map.as_mut() }.insert(TypeId::of::<T>(), id);
            return ComponentView {
                world: self,
                entity_id: id,
            };
        }
        //register component in flecs
        let edesc = ecs_entity_desc_t {
            name: symbol.as_ptr(),
            symbol: symbol.as_ptr(),
            use_low_id: true,
            ..Default::default()
        };
        let cdesc = ecs_component_desc_t {
            _canary: 0,
            entity: unsafe { ecs_entity_init(self.ptr(), &edesc as *const _) },
            type_: ecs_type_info_t {
                size: std::mem::size_of::<T>() as i32,
                alignment: std::mem::align_of::<T>() as i32,
                hooks: ecs_type_hooks_t {
                    dtor: if T::NEEDS_DROP {
                        Some(dtor_callback::<T>)
                    } else {
                        None
                    },
                    ..Default::default()
                },
                component: 0,
                name: symbol.as_ptr(),
            },
        };
        let id = unsafe { ecs_component_init(self.ptr(), &cdesc as *const _) };
        //check it
        assert!(id != 0, "failed to register a component");
        //remember final id
        unsafe { self.component_map.as_mut() }.insert(TypeId::of::<T>(), id);
        ComponentView {
            world: self,
            entity_id: id,
        }
    }

    /// Registers a new tag.
    pub fn tag<T: Component>(&mut self, symbol: &CStr) -> ComponentView<'_> {
        //check if it is a tag
        if !T::IS_TAG {
            panic!("tag function only registers tags");
        }
        //is it already registered in flecs?
        if let Some(entity) = self.lookup_symbol(symbol) {
            let id = entity.entity_id;
            unsafe { self.component_map.as_mut() }.insert(TypeId::of::<T>(), id);
            return ComponentView {
                world: self,
                entity_id: id,
            };
        }
        //register tag in flecs
        let edesc = ecs_entity_desc_t {
            name: symbol.as_ptr(),
            symbol: symbol.as_ptr(),
            use_low_id: true,
            ..Default::default()
        };
        let id = unsafe { ecs_entity_init(self.ptr(), &edesc as *const _) };
        //check it
        assert!(id != 0, "failed to register a tag");
        //remember final id
        unsafe { self.component_map.as_mut() }.insert(TypeId::of::<T>(), id);
        ComponentView {
            world: self,
            entity_id: id,
        }
    }

    /// Creates a component with a copy constructor derived from Clone.
    pub fn component_clone<T: Component + Clone>(&mut self, symbol: &CStr) -> ComponentView<'_> {
        //is it already registered in flecs?
        if let Some(entity) = self.lookup(symbol) {
            let id = entity.entity_id;
            unsafe { self.component_map.as_mut() }.insert(TypeId::of::<T>(), id);
            return ComponentView {
                world: self,
                entity_id: id,
            };
        }
        //register component in flecs
        let edesc = ecs_entity_desc_t {
            name: symbol.as_ptr(),
            symbol: symbol.as_ptr(),
            use_low_id: true,
            ..Default::default()
        };
        let cdesc = ecs_component_desc_t {
            _canary: 0,
            entity: unsafe { ecs_entity_init(self.ptr(), &edesc as *const _) },
            type_: ecs_type_info_t {
                size: std::mem::size_of::<T>() as i32,
                alignment: std::mem::align_of::<T>() as i32,
                hooks: ecs_type_hooks_t {
                    dtor: if T::NEEDS_DROP {
                        Some(dtor_callback::<T>)
                    } else {
                        None
                    },
                    copy: Some(copy_callback::<T>),
                    ..Default::default()
                },
                component: 0,
                name: symbol.as_ptr(),
            },
        };
        let id = unsafe { ecs_component_init(self.ptr(), &cdesc as *const _) };
        //check it
        assert!(id != 0, "failed to register a component");
        //remember final id
        unsafe { self.component_map.as_mut() }.insert(TypeId::of::<T>(), id);
        ComponentView {
            world: self,
            entity_id: id,
        }
    }
}

unsafe extern "C" fn copy_callback<T: Component + Clone>(
    dst: *mut c_void,
    src: *const c_void,
    count: i32,
    _t_i: *const ecs_type_info_t,
) {
    for i in 0..count {
        let src = unsafe { src.offset(i as isize) } as *mut T;
        let dst = unsafe { dst.offset(i as isize) } as *mut T;
        unsafe { std::ptr::write(dst, (*src).clone()) };
    }
}

unsafe extern "C" fn dtor_callback<T: Component>(
    ptr: *mut c_void,
    count: i32,
    _type_info: *const ecs_type_info_t,
) {
    for i in 0..count {
        let ptr = unsafe { ptr.offset(i as isize) };
        let thing = ptr as *mut T;
        unsafe { thing.drop_in_place() };
    }
}

//------------------------------------------------------------------------------
// SINGLETON
//------------------------------------------------------------------------------

impl World {
    /// Gives you access to an entity for a singleton.
    ///
    /// The same as [Self::component], but does not register it.
    pub fn singleton<'a, T: Component>(&'a self) -> EntityView<'a> {
        let id = id::<T>().retrieve_id(self);
        self.view(id)
    }

    /// Gets you an access to the singleton.
    ///
    /// Same as `self.singleton::<T>().get::<T>()`
    ///
    /// # Safety
    ///
    /// Same as `get` from EntityView.
    pub unsafe fn singleton_get<T: Component>(&self) -> Option<&T> {
        unsafe { self.singleton::<T>().get::<T>() }
    }

    /// Gets you a mutable access to the singleton.
    ///
    /// Same as `self.singleton::<T>().get::<T>()`
    ///
    /// # Safety
    ///
    /// Same as `get` from EntityView.
    pub unsafe fn singleton_get_mut<T: Component>(&self) -> Option<&mut T> {
        unsafe { self.singleton::<T>().get_mut::<T>() }
    }

    // Adds a tag singleton to the world.
    //
    // Tags are components without data.
    //
    // # Note
    // Cannot add data components since they would be unitialized which is not allowed in Rust.
    // singleton's don't make much sense as tags
    //pub fn singleton_add<I: IdFetcher>(&self, id: I)
    //where
    //    I::COMPONENT: Component,
    //{
    //    const {
    //        if !<I::COMPONENT as Component>::IS_TAG {
    //            panic!("cannot add a data component as a singleton")
    //        }
    //    }
    //    let id = id.retrieve_id(self);
    //    self.singleton::<I::COMPONENT>().add(id)
    //}

    /// Sets data component a singleton.
    pub fn singleton_set<T: Component>(&self, data: T) {
        self.singleton::<T>().set_comp(data);
    }

    /// Removes a singleton.
    pub fn singleton_remove<T: Component>(&self, id: Id<T>) {
        self.singleton::<T>().remove(id);
    }

    /// Checks whether a singleton is set.
    ///
    /// Assumes the relevant component is registered.
    pub fn singleton_exists<T: Component>(&self, id: Id<T>) -> bool {
        self.singleton::<T>().has(id)
    }
}

//------------------------------------------------------------------------------
// QUERY & SYSTEMS
//------------------------------------------------------------------------------

impl World {
    /// Creates an empty query builder.
    pub fn query<'a>(&'a self) -> QueryBuilder<'a> {
        //create an empty descriptor
        let desc = ecs_query_desc_t::default();
        //create a builder
        QueryBuilder {
            inner: desc,
            expr: None,
            world: self,
        }
    }

    /// Creates a query builder from an expression.
    pub fn query_expr<'a>(&'a self, expr: &CStr) -> QueryBuilder<'a> {
        //create an empty descriptor
        let desc = ecs_query_desc_t::default();
        //create a builder
        let builder = QueryBuilder {
            inner: desc,
            expr: None,
            world: self,
        };
        builder.expression(expr)
    }

    /// Creates an empty system builder.
    pub fn system<'a>(&'a self) -> SystemBuilder<'a> {
        //create an empty descriptor
        let desc = ecs_system_desc_t::default();
        //create a builder
        SystemBuilder {
            kind: 0,
            inner: desc,
            expr: None,
            world: self,
        }
    }

    /// Creates a system builder from an expression.
    pub fn system_expr<'a>(&'a self, expr: &CStr) -> SystemBuilder<'a> {
        //create an empty descriptor
        let desc = ecs_system_desc_t::default();
        //create a builder
        let builder = SystemBuilder {
            inner: desc,
            kind: 0,
            expr: None,
            world: self,
        };
        builder.expression(expr)
    }
}

//------------------------------------------------------------------------------
// PROGRESSING AND META STUFF
//------------------------------------------------------------------------------

impl World {
    /// Begins a deferred mode.
    ///
    /// Every operation will be postponed until `defer_end` is called.
    #[inline]
    pub fn defer_begin(&self) {
        unsafe {
            ecs_defer_begin(self.ptr());
        }
    }

    /// Ends a deferred mode.
    #[inline]
    pub fn defer_end(&self) {
        unsafe {
            ecs_defer_end(self.ptr());
        }
    }

    /// Suspends deferred mode.
    #[inline]
    pub fn defer_suspend(&self) {
        unsafe {
            ecs_defer_suspend(self.ptr());
        }
    }

    /// Resumes deferred mode.
    #[inline]
    pub fn defer_resume(&self) {
        unsafe {
            ecs_defer_resume(self.ptr());
        }
    }

    /// Progresses the world.
    ///
    /// Calls every system.
    #[inline]
    pub fn progress(&self) {
        unsafe {
            ecs_progress(self.ptr(), 0.0);
        }
    }

    /// Progresses the world with a specified delta time.
    ///
    /// Calls every system.
    #[inline]
    pub fn progress_deltatime(&self, dt: f32) {
        unsafe {
            ecs_progress(self.ptr(), dt);
        }
    }

    /// Imports and enabled REST api, allows you to connect using flecs explorer.
    pub fn explorer(&mut self) {
        //import flecs stats
        unsafe {
            ecs_import(self.ptr(), Some(FlecsStatsImport), c"FlecsStat".as_ptr());
        }
        //set rest server singleton
        self.singleton_set::<Rest>(Rest {
            port: 27750,
            ipaddr: null_mut(),
            impl_: null_mut(),
        })
    }
}
