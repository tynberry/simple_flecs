pub mod iter;

use std::{
    ffi::{CStr, CString, c_void},
    ptr::NonNull,
};

use flecs_ecs_sys::*;
use iter::Iter;

use crate::{
    c_types::{
        ECS_QUERY_MATCH_DISABLED, ECS_QUERY_MATCH_EMPTY_TABLES, ECS_QUERY_MATCH_PREFAB,
        QueryCacheKind,
    },
    entity::Entity,
    world::{ComponentMap, World},
};

/// Wrapper around a query in an ECS world.
///
/// The query must not out live the world it was created in.
pub struct Query {
    world_ptr: NonNull<ecs_world_t>,
    component_map: NonNull<ComponentMap>,
    query: NonNull<ecs_query_t>,
    entity_id: Option<Entity>,
}

impl Drop for Query {
    fn drop(&mut self) {
        if std::thread::panicking() {
            return;
        }
        //drop the query
        match self.entity_id {
            Some(_) => {
                //do nothing, the query is on an entity
            }
            None => unsafe { ecs_query_fini(self.query.as_ptr()) },
        }
    }
}

impl Query {
    /// Begin query iteration.
    pub fn iter(&self) -> Iter {
        let mut iter = unsafe { ecs_query_iter(self.world_ptr.as_ptr(), self.query.as_ptr()) };
        iter.binding_ctx = self.component_map.as_ptr() as *mut c_void;
        Iter { iter }
    }
}

/// Builder for creating queries. Allows you to set certain flags and the components to request.
#[derive(Debug)]
pub struct QueryBuilder<'a> {
    pub(crate) inner: ecs_query_desc_t,
    pub(crate) expr: Option<CString>,
    pub(crate) world: &'a World,
}

impl<'a> QueryBuilder<'a> {
    /// Sets an expression as the base for the query.
    ///
    /// This allocates a string, due to lifetimes.
    pub fn expression(mut self, expr: &CStr) -> Self {
        self.expr = Some(expr.to_owned());
        self.inner.expr = self.expr.as_ref().unwrap().as_ptr();
        self
    }

    /// Sets query's cache kind.
    pub fn set_cache(mut self, kind: QueryCacheKind) -> Self {
        self.inner.cache_kind = kind as u32;
        self
    }

    /// Sets query to match prefabs.
    pub fn match_prefabs(mut self) -> Self {
        self.inner.flags |= ECS_QUERY_MATCH_PREFAB as u32;
        self
    }

    /// Sets query to match disabled.
    pub fn match_disabled(mut self) -> Self {
        self.inner.flags |= ECS_QUERY_MATCH_DISABLED as u32;
        self
    }

    /// Sets query to match empty tables.
    pub fn match_empty_tables(mut self) -> Self {
        self.inner.flags |= ECS_QUERY_MATCH_EMPTY_TABLES as u32;
        self
    }

    /// Builds the query.
    pub fn build(self) -> Query {
        let query_ptr = unsafe { ecs_query_init(self.world.ptr(), &self.inner as *const _) };
        Query {
            world_ptr: self.world.ptr,
            query: NonNull::new(query_ptr).unwrap(),
            component_map: self.world.component_map,
            entity_id: None,
        }
    }

    /// Builds the query to an associated entity.
    pub fn build_with_entity(self) -> Query {
        let entity_id = Some(self.world.entity().id());
        let query_ptr = unsafe { ecs_query_init(self.world.ptr(), &self.inner as *const _) };
        Query {
            world_ptr: self.world.ptr,
            query: NonNull::new(query_ptr).unwrap(),
            component_map: self.world.component_map,
            entity_id,
        }
    }

    /// Builds the query to an associated named entity.
    pub fn buiild_with_entity_named(self, name: &CStr) -> Query {
        let entity_id = Some(self.world.entity_named(name).id());
        let query_ptr = unsafe { ecs_query_init(self.world.ptr(), &self.inner as *const _) };
        Query {
            world_ptr: self.world.ptr,
            query: NonNull::new(query_ptr).unwrap(),
            component_map: self.world.component_map,
            entity_id,
        }
    }
}
