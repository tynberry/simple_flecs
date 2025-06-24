use std::{
    ffi::{CStr, c_void},
    mem::ManuallyDrop,
    ptr::null,
};

use flecs_ecs_sys::*;

use crate::{
    component::{
        Component,
        id::{IdFetcher, id},
    },
    world::World,
};

/// Entity handle.
pub type Entity = ecs_entity_t;

/// Builder pattern for entity manipulation.
#[derive(Debug, Clone, Copy)]
pub struct EntityView<'a> {
    pub(crate) world: &'a World,
    pub(crate) entity_id: Entity,
}

impl<'a> PartialEq for EntityView<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.entity_id == other.entity_id
    }
}

impl<'a> Eq for EntityView<'a> {}

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

    /// Is the entity valid?
    pub fn is_valid(&self) -> bool {
        unsafe { ecs_is_valid(self.world.ptr(), self.entity_id) }
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
    /// Adds a tag from an id to the entity.
    ///
    /// Tags are components without data.
    ///
    /// # Note
    /// Cannot add data components since they would be unitialized which is not allowed in Rust.
    /// TODO: Add a check against that.
    pub fn add(&self, id: impl IdFetcher) {
        let id = id.retrieve_id(self.world);
        unsafe { ecs_add_id(self.world.ptr(), self.entity_id, id) }
    }

    /// Sets a component to an entity.
    pub fn set_comp<T: Component>(&self, data: T) {
        //get id
        let comp_id = id::<T>().retrieve_id(self.world);
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

    /// Sets a component pair to an entity, where the first is a component and the second is a
    /// tag.
    pub fn set_first<F: Component>(&self, first: F, second: impl IdFetcher) {
        //get ids
        let first_id = id::<F>().retrieve_id(self.world);
        let second_id = second.retrieve_id(self.world);
        let pair_id = unsafe { ecs_make_pair(first_id, second_id) };
        //prevent dropping, since it copies whatever is there, don't want to accidentaly deallocate
        let my_data = ManuallyDrop::new(first);
        unsafe {
            ecs_set_id(
                self.world.ptr(),
                self.entity_id,
                pair_id,
                core::mem::size_of::<F>(),
                &my_data as *const _ as *const c_void,
            );
        }
    }

    /// Sets a component pair to an entity, where the second is a component and the first is a
    /// tag.
    pub fn set_second<F: Component>(&self, first: impl IdFetcher, second: F) {
        //get ids
        let first_id = first.retrieve_id(self.world);
        let second_id = id::<F>().retrieve_id(self.world);
        let pair_id = unsafe { ecs_make_pair(first_id, second_id) };
        //prevent dropping, since it copies whatever is there, don't want to accidentaly deallocate
        let my_data = ManuallyDrop::new(second);
        unsafe {
            ecs_set_id(
                self.world.ptr(),
                self.entity_id,
                pair_id,
                core::mem::size_of::<F>(),
                &my_data as *const _ as *const c_void,
            );
        }
    }

    /// Gets a component from the entity.
    ///
    /// # Safety
    ///
    /// You can invalidate the reference by performing any world mutating action.
    pub unsafe fn get<T: Component>(&self) -> Option<&'a T> {
        let comp_id = id::<T>().retrieve_id(self.world);
        unsafe {
            let ptr = ecs_get_id(self.world.ptr(), self.entity_id, comp_id) as *const T;
            ptr.as_ref()
        }
    }

    /// Gets a data pair where first is the data from the entity.
    ///
    /// # Safety
    ///
    /// You can invalidate the reference by performing any world mutating action.
    pub unsafe fn get_first<T: Component>(&self, second: impl IdFetcher) -> Option<&'a T> {
        //get ids
        let first_id = id::<T>().retrieve_id(self.world);
        let second_id = second.retrieve_id(self.world);
        let pair_id = unsafe { ecs_make_pair(first_id, second_id) };
        unsafe {
            let ptr = ecs_get_id(self.world.ptr(), self.entity_id, pair_id) as *const T;
            ptr.as_ref()
        }
    }

    /// Gets a data pair where second is the data from the entity.
    ///
    /// # Safety
    ///
    /// You can invalidate the reference by performing any world mutating action.
    pub unsafe fn get_second<T: Component>(&self, first: impl IdFetcher) -> Option<&'a T> {
        //get ids
        let second_id = id::<T>().retrieve_id(self.world);
        let first_id = first.retrieve_id(self.world);
        let pair_id = unsafe { ecs_make_pair(first_id, second_id) };
        unsafe {
            let ptr = ecs_get_id(self.world.ptr(), self.entity_id, pair_id) as *const T;
            ptr.as_ref()
        }
    }

    /// Gets a component mutably from the entity.
    ///
    /// # Safety
    ///
    /// You can invalidate the reference by performing any world mutating action.
    pub unsafe fn get_mut<T: Component>(&mut self) -> Option<&'a mut T> {
        let comp_id = id::<T>().retrieve_id(self.world);
        unsafe {
            let ptr = ecs_get_mut_id(self.world.ptr(), self.entity_id, comp_id) as *mut T;
            ptr.as_mut()
        }
    }

    /// Gets a data pair mutably where first is the data from the entity.
    ///
    /// # Safety
    ///
    /// You can invalidate the reference by performing any world mutating action.
    pub unsafe fn get_first_mut<T: Component>(&self, second: impl IdFetcher) -> Option<&'a mut T> {
        //get ids
        let first_id = id::<T>().retrieve_id(self.world);
        let second_id = second.retrieve_id(self.world);
        let pair_id = unsafe { ecs_make_pair(first_id, second_id) };
        unsafe {
            let ptr = ecs_get_mut_id(self.world.ptr(), self.entity_id, pair_id) as *mut T;
            ptr.as_mut()
        }
    }

    /// Gets a data pair mutably where second is the data from the entity.
    ///
    /// # Safety
    ///
    /// You can invalidate the reference by performing any world mutating action.
    pub unsafe fn get_second_mut<T: Component>(&self, first: impl IdFetcher) -> Option<&'a mut T> {
        //get ids
        let second_id = id::<T>().retrieve_id(self.world);
        let first_id = first.retrieve_id(self.world);
        let pair_id = unsafe { ecs_make_pair(first_id, second_id) };
        unsafe {
            let ptr = ecs_get_mut_id(self.world.ptr(), self.entity_id, pair_id) as *mut T;
            ptr.as_mut()
        }
    }

    /// Checks whether entity has a component or pair.
    pub fn has(&self, id: impl IdFetcher) -> bool {
        let id = id.retrieve_id(self.world);
        unsafe { ecs_has_id(self.world.ptr(), self.entity_id, id) }
    }

    /// Removes a component or pair.
    pub fn remove(&self, id: impl IdFetcher) {
        let id = id.retrieve_id(self.world);
        unsafe {
            ecs_remove_id(self.world.ptr(), self.entity_id, id);
        }
    }

    /// Enables/Disables component/pair of the entity.
    pub fn enable_comp(&self, id: impl IdFetcher, state: bool) {
        let id = id.retrieve_id(self.world);
        unsafe {
            ecs_enable_id(self.world.ptr(), self.entity_id, id, state);
        }
    }

    /// Checks whether component/pair is enabled.
    pub fn is_enabled_comp(&self, id: impl IdFetcher) -> bool {
        let id = id.retrieve_id(self.world);
        unsafe { ecs_is_enabled_id(self.world.ptr(), self.entity_id, id) }
    }
}

//------------------------------------------------------------------------------
// MISC
//------------------------------------------------------------------------------

impl<'a> EntityView<'a> {
    /// Returns a debug string of the entity's archetype.
    pub fn archetype_str(&self) -> String {
        //get archetype
        let typ = unsafe { ecs_get_type(self.world.ptr(), self.entity_id) };
        //turn it into string
        let type_str = unsafe { ecs_type_str(self.world.ptr(), typ) };
        let c_str = unsafe { CStr::from_ptr(type_str) };
        let type_owned_str = c_str.to_str().unwrap().to_owned();
        //free pointer
        let api = unsafe { ecs_os_get_api() };
        unsafe { (api.free_.unwrap())(type_str as *mut std::ffi::c_void) };
        type_owned_str
    }
}
