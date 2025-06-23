use ahash::AHashMap;
use flecs_ecs_sys::*;
use std::{
    any::TypeId,
    ffi::{CStr, c_void},
    ptr::NonNull,
};

use crate::{
    component::{Component, ComponentView},
    entity::{Entity, EntityView},
};

/// ECS world.
pub struct World {
    /// Pointer to the underlying raw world.
    ptr: NonNull<ecs_world_t>,
    /// Component map for this current crate.
    ///
    /// World must **not** be passed across (at least) dynamic linking boundary.
    pub(crate) component_map: AHashMap<TypeId, Entity>,
}

//------------------------------------------------------------------------------
// WORLD LIFECYCLE
//------------------------------------------------------------------------------

impl Default for World {
    fn default() -> Self {
        Self {
            ptr: unsafe { NonNull::new(ecs_init()).expect("could not init ecs world") },
            component_map: Default::default(),
        }
    }
}

impl World {
    /// Creates an empty new ECS world.
    pub fn new() -> Self {
        World::default()
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
    /// Creates a new named component.
    pub fn component<T: Component>(&mut self, symbol: &CStr) -> ComponentView<'_> {
        //is it already registered in flecs?
        if let Some(entity) = self.lookup_symbol(symbol) {
            let id = entity.entity_id;
            self.component_map.insert(TypeId::of::<T>(), id);
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
        self.component_map.insert(TypeId::of::<T>(), id);
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
            self.component_map.insert(TypeId::of::<T>(), id);
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
        self.component_map.insert(TypeId::of::<T>(), id);
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
