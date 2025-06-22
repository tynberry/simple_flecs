use std::{
    ffi::{CStr, c_void},
    mem::ManuallyDrop,
    ptr::null,
};

use flecs_ecs_sys::*;

use crate::{
    component::{
        Component,
        id::{IdFetcher, TagIdFetcher, comp},
    },
    world::World,
};

/// Entity handle.
pub type Entity = ecs_entity_t;

/// Builder pattern for entity manipulation.
#[derive(Clone, Copy)]
pub struct EntityView<'a> {
    pub(crate) world: &'a World,
    pub(crate) entity_id: Entity,
}

impl<'a> EntityView<'a> {
    /// Gets entity id.
    #[inline]
    pub fn id(&self) -> Entity {
        self.entity_id
    }

    /// Gets entity name.
    #[inline]
    pub fn name(&self) -> &CStr {
        let name = unsafe { ecs_get_name(self.world.ptr(), self.entity_id) };
        unsafe { CStr::from_ptr(name) }
    }

    /// Sets entity name.
    #[inline]
    pub fn set_name(&self, name: &CStr) {
        unsafe { ecs_set_name(self.world.ptr(), self.entity_id, name.as_ptr()) };
    }

    /// Enables entity.
    #[inline]
    pub fn enable(&self) {
        unsafe { ecs_enable(self.world.ptr(), self.entity_id, true) };
    }

    /// Disables entity.
    #[inline]
    pub fn disable(&self) {
        unsafe { ecs_enable(self.world.ptr(), self.entity_id, false) };
    }

    /// Gets entity path.
    ///
    /// This allocates a new string.
    pub fn path(&self) -> String {
        let path = unsafe {
            ecs_get_path_w_sep(self.world.ptr(), 0, self.entity_id, c".".as_ptr(), null())
        };
        let cstr = unsafe { CStr::from_ptr(path) };
        let string = cstr.to_str().unwrap().to_owned();
        //free pointer
        let api = unsafe { ecs_os_get_api() };
        unsafe { (api.free_.unwrap())(path as *mut std::ffi::c_void) };
        string
    }

    /// Is the entity alive?
    pub fn is_alive(&self) -> bool {
        unsafe { ecs_is_alive(self.world.ptr(), self.entity_id) }
    }

    /// Lookups from this entity.
    pub fn lookup(&self, path: &CStr) -> Option<EntityView<'a>> {
        let entity = unsafe {
            ecs_lookup_path_w_sep(
                self.world.ptr(),
                self.entity_id,
                path.as_ptr(),
                c".".as_ptr(),
                null(),
                true,
            )
        };
        if entity == 0 {
            None
        } else {
            Some(EntityView {
                world: self.world,
                entity_id: entity,
            })
        }
    }

    /// Clear the entity.
    pub fn clear(self) -> EntityView<'a> {
        unsafe { ecs_clear(self.world.ptr(), self.entity_id) }
        self
    }

    /// Delete the entity.
    pub fn delete(self) {
        unsafe {
            ecs_delete(self.world.ptr(), self.entity_id);
        }
    }
}

impl<'a> From<EntityView<'a>> for Entity {
    fn from(value: EntityView<'a>) -> Self {
        value.entity_id
    }
}

//------------------------------------------------------------------------------
// COMPONENT MANIPULATION
//------------------------------------------------------------------------------

impl<'a> EntityView<'a> {
    /// Adds a tag to the entity.
    ///
    /// Tags are components without data.
    ///
    /// # Note
    /// Cannot add data components since they would be unitialized which is not allowed in Rust.
    pub fn add_tag(&self, id: impl TagIdFetcher) {
        let id = id.retrieve_id(self.world);
        unsafe { ecs_add_id(self.world.ptr(), self.entity_id, id) }
    }

    /// Sets a component to the entity.
    pub fn set_comp<T: Component>(&self, data: T) {
        //get id
        let comp_id = comp::<T>().retrieve_id(self.world);
        //prevent dropping, since it copies whatever is there, don't want to accidentaly deallocate
        let my_data = ManuallyDrop::new(data);
        unsafe {
            ecs_set_id(
                self.world.ptr(),
                self.entity_id,
                comp_id,
                core::mem::size_of::<T>(),
                &my_data as *const _ as *const c_void,
            );
        }
    }

    /// Gets a component from the entity.
    pub fn get<T: Component>(&self) -> Option<&T> {
        let comp_id = comp::<T>().retrieve_id(self.world);
        unsafe {
            let ptr = ecs_get_id(self.world.ptr(), self.entity_id, comp_id) as *const T;
            ptr.as_ref()
        }
    }

    /// Gets a component mutably from the entity.
    pub fn get_mut<T: Component>(&mut self) -> Option<&mut T> {
        let comp_id = comp::<T>().retrieve_id(self.world);
        unsafe {
            let ptr = ecs_get_mut_id(self.world.ptr(), self.entity_id, comp_id) as *mut T;
            ptr.as_mut()
        }
    }
}
