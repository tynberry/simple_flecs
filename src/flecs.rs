//! Contains flecs traits and pre-registered components.
//!
//! Courtesy of Intra-db's Flecs-Rust bindings.
//!
//! Source: https://github.com/Indra-db/Flecs-Rust/blob/ff68122ba303107cbf32027ab800d07fcdf97468/flecs_ecs/src/core/c_types.rs#L261
use crate::{c_types::*, component::id::IdFetcher};
use flecs_ecs_sys::{self as sys, EcsDefaultChildComponent, EcsIdentifier, EcsPoly};

pub trait FlecsTrait {}

macro_rules! create_pre_registered_component {
    ($struct_name:ident, $const_name:ident) => {
        create_pre_registered_component!($struct_name, $const_name, "");
    };
    ($struct_name:ident, $const_name:ident, $doc:tt) => {
        #[derive(Debug, Default, Clone)]
        #[allow(clippy::empty_docs)]
        #[doc = $doc]
        pub struct $struct_name;

        impl crate::component::id::IdFetcher for $struct_name {
            type COMPONENT = crate::component::id::UnknownType;
            fn retrieve_id(&self, _world: &crate::world::World) -> crate::entity::Entity {
                $const_name
            }
        }
    };
}

macro_rules! impl_component_traits_binding_type_w_id {
    ($name:ident, $id:ident) => {
        impl crate::component::Component for $name {
            const IS_TAG: bool = false;
            const NEEDS_DROP: bool = false;
            const ID: Option<crate::entity::Entity> = Some($id);
        }
    };
}

// Term id flags
create_pre_registered_component!(Self_, ECS_SELF, "Match on self");
create_pre_registered_component!(Up, ECS_UP, "Match by traversing upwards");
create_pre_registered_component!(
    Trav,
    ECS_TRAV,
    "Match by traversing downwards (derived, cannot be set)"
);
create_pre_registered_component!(
    Cascade,
    ECS_CASCADE,
    "Match by traversing upwards, but iterate in breadth-first order"
);
create_pre_registered_component!(
    Desc,
    ECS_DESC,
    "Combine with Cascade to iterate hierarchy bottom to top"
);
create_pre_registered_component!(IsVariable, ECS_IS_VARIABLE, "Term id is a variable");
create_pre_registered_component!(IsEntity, ECS_IS_ENTITY, "Term id is an entity");
create_pre_registered_component!(
    IsName,
    ECS_IS_NAME,
    "Term id is a name (don't attempt to lookup as entity)"
);
create_pre_registered_component!(
    TraverseFlags,
    ECS_TRAVERSE_FLAGS,
    "all term traversal flags"
);
create_pre_registered_component!(
    TermRefFlags,
    ECS_TERM_REF_FLAGS,
    "all term reference kind flags"
);

/// Term flags discovered & set during query creation.
/// Mostly used internally to store information relevant to queries.
pub mod term_flags {
    use super::*;
    create_pre_registered_component!(MatchAny, MATCH_ANY);
    create_pre_registered_component!(MatchAnySrc, MATCH_ANY_SRC);
    create_pre_registered_component!(Transitive, TRANSITIVE);
    create_pre_registered_component!(Reflexive, REFLEXIVE);
    create_pre_registered_component!(IdInherited, ID_INHERITED);
    create_pre_registered_component!(IsTrivial, IS_TRIVIAL);
    create_pre_registered_component!(IsCacheable, IS_CACHEABLE);
    create_pre_registered_component!(IsScope, IS_SCOPE);
    create_pre_registered_component!(IsMember, IS_MEMBER);
    create_pre_registered_component!(IsToggle, IS_TOGGLE);
    create_pre_registered_component!(KeepAlive, KEEP_ALIVE);
    create_pre_registered_component!(IsSparse, IS_SPARSE);
    create_pre_registered_component!(IsUnion, IS_UNION);
    create_pre_registered_component!(IsOr, IS_OR);
}

/// Query flags discovered & set during query creation.
pub mod query_flags {
    use super::*;
    create_pre_registered_component!(
        MatchPrefab,
        ECS_QUERY_MATCH_PREFAB,
        "Query must match prefabs."
    );
    create_pre_registered_component!(
        MatchDisabled,
        ECS_QUERY_MATCH_DISABLED,
        "Query must match disabled entities."
    );
    create_pre_registered_component!(
        MatchEmptyTables,
        ECS_QUERY_MATCH_EMPTY_TABLES,
        "Query must match empty tables."
    );

    create_pre_registered_component!(
        AllowUnresolvedByName,
        ECS_QUERY_ALLOW_UNRESOLVED_BY_NAME,
        "Query may have unresolved entity identifiers."
    );
    create_pre_registered_component!(
        TableOnly,
        ECS_QUERY_TABLE_ONLY,
        "Query only returns whole tables (ignores toggle/member fields)."
    );
}

pub mod id_flags {
    use super::*;
    create_pre_registered_component!(Pair, ECS_PAIR, "Indicates that the id is a pair.");
    create_pre_registered_component!(
        AutoOverride,
        ECS_AUTO_OVERRIDE,
        "Automatically override component when it is inherited"
    );
    create_pre_registered_component!(
        Toggle,
        ECS_TOGGLE,
        "Adds bitset to storage which allows component to be enabled/disabled"
    );
    create_pre_registered_component!(
        And,
        ECS_AND,
        "Include all components from entity to which AND is applied"
    );
}

// Builtin component ids
pub type EcsComponent = flecs_ecs_sys::EcsComponent;
pub type Identifier = flecs_ecs_sys::EcsIdentifier;
pub type Poly = flecs_ecs_sys::EcsPoly;
pub type DefaultChildComponent = flecs_ecs_sys::EcsDefaultChildComponent;

// component
impl IdFetcher for flecs_ecs_sys::EcsComponent {
    type COMPONENT = EcsComponent;
    fn retrieve_id(&self, _world: &crate::world::World) -> crate::entity::Entity {
        ECS_COMPONENT
    }
}

impl crate::component::Component for EcsComponent {
    const NEEDS_DROP: bool = false;
    const IS_TAG: bool = false;
    const ID: Option<crate::entity::Entity> = Some(ECS_COMPONENT);
}

//identifier
impl IdFetcher for flecs_ecs_sys::EcsIdentifier {
    type COMPONENT = EcsIdentifier;
    fn retrieve_id(&self, _world: &crate::world::World) -> crate::entity::Entity {
        ECS_IDENTIFIER
    }
}

impl crate::component::Component for Identifier {
    const NEEDS_DROP: bool = false;
    const IS_TAG: bool = false;
    const ID: Option<crate::entity::Entity> = Some(ECS_IDENTIFIER);
}

//poly
impl IdFetcher for flecs_ecs_sys::EcsPoly {
    type COMPONENT = EcsPoly;
    fn retrieve_id(&self, _world: &crate::world::World) -> crate::entity::Entity {
        ECS_POLY
    }
}

impl crate::component::Component for Poly {
    const NEEDS_DROP: bool = false;
    const IS_TAG: bool = false;
    const ID: Option<crate::entity::Entity> = Some(ECS_POLY);
}

//default child component
impl IdFetcher for flecs_ecs_sys::EcsDefaultChildComponent {
    type COMPONENT = EcsDefaultChildComponent;
    fn retrieve_id(&self, _world: &crate::world::World) -> crate::entity::Entity {
        ECS_DEFAULT_CHILD_COMPONENT
    }
}

impl crate::component::Component for DefaultChildComponent {
    const NEEDS_DROP: bool = false;
    const IS_TAG: bool = false;
    const ID: Option<crate::entity::Entity> = Some(ECS_DEFAULT_CHILD_COMPONENT);
}

// Poly target components
create_pre_registered_component!(Query, ECS_QUERY);
create_pre_registered_component!(Observer, ECS_OBSERVER);

// Core scopes & entities
create_pre_registered_component!(EcsWorld, ECS_WORLD);
create_pre_registered_component!(Flecs, ECS_FLECS);
create_pre_registered_component!(FlecsCore, ECS_FLECS_CORE);
create_pre_registered_component!(FlecsInternals, ECS_FLECS_INTERNALS);
create_pre_registered_component!(Module, ECS_MODULE);
create_pre_registered_component!(Private, ECS_PRIVATE);
create_pre_registered_component!(Prefab, ECS_PREFAB);
create_pre_registered_component!(Disabled, ECS_DISABLED);
create_pre_registered_component!(NotQueryable, ECS_NOT_QUERYABLE);
create_pre_registered_component!(SlotOf, ECS_SLOT_OF);
create_pre_registered_component!(Flag, ECS_FLAG);
create_pre_registered_component!(Monitor, ECS_MONITOR);
create_pre_registered_component!(Empty, ECS_EMPTY);

// Component traits
create_pre_registered_component!(Wildcard, ECS_WILDCARD, "Match all entities");
create_pre_registered_component!(Any, ECS_ANY, "Match at most one entity");
create_pre_registered_component!(This_, ECS_THIS);
create_pre_registered_component!(Variable, ECS_VARIABLE);
// Shortcut as EcsVariable is typically used as source for singleton terms
create_pre_registered_component!(Singleton, ECS_VARIABLE);
create_pre_registered_component!(
    Transitive,
    ECS_TRANSITIVE,
    "Component trait. Relationship is marked as transitive."
);
create_pre_registered_component!(
    Reflexive,
    ECS_REFLEXIVE,
    "Component trait. Relationship is marked as reflexive."
);
create_pre_registered_component!(
    Symmetric,
    ECS_SYMMETRIC,
    "Component trait. Relationship is marked as symmetric."
);
create_pre_registered_component!(
    Final,
    ECS_FINAL,
    "Component trait. This component cannot be used in an [`IsA`] relationship."
);
create_pre_registered_component!(
    PairIsTag,
    ECS_PAIR_IS_TAG,
    "Component trait. A relationship can be marked with PairIsTag in which case
     a pair with the relationship will never contain data."
);
create_pre_registered_component!(
    Exclusive,
    ECS_EXCLUSIVE,
    "Component trait. Enforces that an entity can only have a single instance of a relationship."
);
create_pre_registered_component!(
    Acyclic,
    ECS_ACYCLIC,
    "Component trait. Indicates that the relationship cannot contain cycles."
);
create_pre_registered_component!(
    Traversable,
    ECS_TRAVERSABLE,
    "Component trait. This relationship can be traversed automatically by queries, e.g. using [`Up`]."
);
create_pre_registered_component!(
    With,
    ECS_WITH,
    "Component trait. Indicates that this relationship must always come together with another component."
);
create_pre_registered_component!(
    OneOf,
    ECS_ONE_OF,
    "Component trait. Enforces that the target of the relationship is a child of a specified entity."
);
create_pre_registered_component!(
    CanToggle,
    ECS_CAN_TOGGLE,
    "Component trait. Allows a component to be toggled."
);
create_pre_registered_component!(
    Trait,
    ECS_TRAIT,
    "Component trait. Marks an entity as a trait."
);
create_pre_registered_component!(
    Relationship,
    ECS_RELATIONSHIP,
    "Component trait. Enforces that an entity can only be used as a relationship."
);
create_pre_registered_component!(
    Target,
    ECS_TARGET,
    "Component trait. Enforces that an entity can only be used as the target of a relationship."
);

// OnInstantiate traits
create_pre_registered_component!(
    OnInstantiate,
    ECS_ON_INSTANTIATE,
    "Component trait. Configures behavior of components when an entity is instantiated from another entity. \
    Used as a pair with one of [`Override`], [`Inherit`], or [`DontInherit`]."
);
create_pre_registered_component!(
    Override,
    ECS_OVERRIDE,
    "The default behavior. Inherited components are copied to the instance."
);
create_pre_registered_component!(
    Inherit,
    ECS_INHERIT,
    "Inherited components are not copied to the instance. \
    Operations such as `get` and `has`, and queries will automatically lookup inheritable components \
    by following the [`IsA`] relationship."
);
create_pre_registered_component!(
    DontInherit,
    ECS_DONT_INHERIT,
    "Components with the [`DontInherit`] trait are not inherited from a base entity \
    (the [`IsA`] target) on instantiation."
);

// OnDelete/OnDeleteTarget traits
create_pre_registered_component!(OnDelete, ECS_ON_DELETE);
create_pre_registered_component!(OnDeleteTarget, ECS_ON_DELETE_TARGET);
create_pre_registered_component!(Remove, ECS_REMOVE);
create_pre_registered_component!(Delete, ECS_DELETE);
create_pre_registered_component!(Panic, ECS_PANIC);

// Builtin relationships
create_pre_registered_component!(
    ChildOf,
    ECS_CHILD_OF,
    "Builtin relationship. Allows for the creation of entity hierarchies."
);
create_pre_registered_component!(
    IsA,
    ECS_IS_A,
    "Builtin relationship. Used to express that one entity is equivalent to another."
);
create_pre_registered_component!(
    DependsOn,
    ECS_DEPENDS_ON,
    "Builtin relationship. Used to determine the execution order of systems."
);

// Identifier tags
create_pre_registered_component!(Name, ECS_NAME);
create_pre_registered_component!(Symbol, ECS_SYMBOL);
create_pre_registered_component!(Alias, ECS_ALIAS);

// Events
create_pre_registered_component!(
    OnAdd,
    ECS_ON_ADD,
    "Event. Invoked whenever a component, tag or pair is added to an entity."
);
create_pre_registered_component!(
    OnRemove,
    ECS_ON_REMOVE,
    "Event. Invoked whenever a component, tag or pair is removed from an entity."
);
create_pre_registered_component!(
    OnSet,
    ECS_ON_SET,
    "Event. Invoked whenever a component is assigned a new value."
);
create_pre_registered_component!(OnTableCreate, ECS_ON_TABLE_CREATE);
create_pre_registered_component!(OnTableDelete, ECS_ON_TABLE_DELETE);

// System
pub mod system {
    use super::*;
    pub type TickSource = sys::EcsTickSource;
    impl_component_traits_binding_type_w_id!(TickSource, ECS_TICK_SOURCE);

    create_pre_registered_component!(System, ECS_SYSTEM);
}

pub mod timer {
    use super::*;

    pub type Timer = sys::EcsTimer;
    impl_component_traits_binding_type_w_id!(Timer, ECS_TIMER);

    pub type RateFilter = sys::EcsRateFilter;
    impl_component_traits_binding_type_w_id!(RateFilter, ECS_RATE_FILTER);
}

create_pre_registered_component!(
    Sparse,
    ECS_SPARSE,
    "Component trait. Configures a component to use sparse storage."
);
create_pre_registered_component!(
    Union,
    ECS_UNION,
    "Component trait. Similar to [`Exclusive`] but combines \
    different relationship targets in a single table."
);

// Builtin predicate for comparing entity ids
create_pre_registered_component!(PredEq, ECS_PRED_EQ);
create_pre_registered_component!(PredMatch, ECS_PRED_MATCH);
create_pre_registered_component!(PredLookup, ECS_PRED_LOOKUP);

// builtin marker entities for query scopes
create_pre_registered_component!(ScopeOpen, ECS_SCOPE_OPEN);
create_pre_registered_component!(ScopeClose, ECS_SCOPE_CLOSE);

// Pipeline
pub mod pipeline {
    use super::*;
    create_pre_registered_component!(Pipeline, ECS_PIPELINE);
    create_pre_registered_component!(OnStart, ECS_ON_START);
    //create_pre_registered_component!(PreFrame, ECS_PRE_FRAME); //not meant to be exposed, internal only
    create_pre_registered_component!(OnLoad, ECS_ON_LOAD);
    create_pre_registered_component!(PostLoad, ECS_POST_LOAD);
    create_pre_registered_component!(PreUpdate, ECS_PRE_UPDATE);
    create_pre_registered_component!(OnUpdate, ECS_ON_UPDATE);
    create_pre_registered_component!(OnValidate, ECS_ON_VALIDATE);
    create_pre_registered_component!(PostUpdate, ECS_POST_UPDATE);
    create_pre_registered_component!(PreStore, ECS_PRE_STORE);
    create_pre_registered_component!(OnStore, ECS_ON_STORE);
    //create_pre_registered_component!(PostFrame, ECS_POST_FRAME); //not meant to be exposed, internal only
    create_pre_registered_component!(Phase, ECS_PHASE);
}

pub mod meta {
    use super::*;
    // Meta primitive components (don't use low ids to save id space)
    create_pre_registered_component!(Bool, ECS_BOOL_T);
    create_pre_registered_component!(Char, ECS_CHAR_T);
    create_pre_registered_component!(Byte, ECS_BYTE_T);
    create_pre_registered_component!(U8, ECS_U8_T);
    create_pre_registered_component!(U16, ECS_U16_T);
    create_pre_registered_component!(U32, ECS_U32_T);
    create_pre_registered_component!(U64, ECS_U64_T);
    create_pre_registered_component!(UPtr, ECS_UPTR_T);
    create_pre_registered_component!(I8, ECS_I8_T);
    create_pre_registered_component!(I16, ECS_I16_T);
    create_pre_registered_component!(I32, ECS_I32_T);
    create_pre_registered_component!(I64, ECS_I64_T);
    create_pre_registered_component!(IPtr, ECS_IPTR_T);
    create_pre_registered_component!(F32, ECS_F32_T);
    create_pre_registered_component!(F64, ECS_F64_T);
    create_pre_registered_component!(String, ECS_STRING_T);
    create_pre_registered_component!(Entity, ECS_ENTITY_T);
    create_pre_registered_component!(Constant, ECS_CONSTANT);
    create_pre_registered_component!(Quantity, ECS_QUANTITY);
    create_pre_registered_component!(EcsOpaque, ECS_OPAQUE);

    // Meta type components
    pub type Type = sys::EcsType;
    pub type TypeSerializer = sys::EcsTypeSerializer;
    pub type Primitive = sys::EcsPrimitive;
    pub type EcsEnum = sys::EcsEnum;
    pub type Bitmask = sys::EcsBitmask;
    pub type Member = sys::EcsMember;
    pub type MemberRanges = sys::EcsMemberRanges;
    pub type EcsStruct = sys::EcsStruct;
    pub type Array = sys::EcsArray;
    pub type Vector = sys::EcsVector;
    pub type Unit = sys::EcsUnit;
    pub type UnitPrefix = sys::EcsUnitPrefix;

    impl_component_traits_binding_type_w_id!(Type, ECS_META_TYPE);
    impl_component_traits_binding_type_w_id!(TypeSerializer, ECS_META_TYPE_SERIALIZER);
    impl_component_traits_binding_type_w_id!(Primitive, ECS_PRIMITIVE);
    impl_component_traits_binding_type_w_id!(EcsEnum, ECS_ENUM);
    impl_component_traits_binding_type_w_id!(Bitmask, ECS_BITMASK);
    impl_component_traits_binding_type_w_id!(Member, ECS_MEMBER);
    impl_component_traits_binding_type_w_id!(MemberRanges, ECS_MEMBER_RANGES);
    impl_component_traits_binding_type_w_id!(EcsStruct, ECS_STRUCT);
    impl_component_traits_binding_type_w_id!(Array, ECS_ARRAY);
    impl_component_traits_binding_type_w_id!(Vector, ECS_VECTOR);
    impl_component_traits_binding_type_w_id!(Unit, ECS_UNIT);
    impl_component_traits_binding_type_w_id!(UnitPrefix, ECS_UNIT_PREFIX);
}

// Doc module components
pub mod doc {
    use super::*;
    create_pre_registered_component!(Description, ECS_DOC_DESCRIPTION);
    create_pre_registered_component!(Brief, ECS_DOC_BRIEF);
    create_pre_registered_component!(Detail, ECS_DOC_DETAIL);
    create_pre_registered_component!(Link, ECS_DOC_LINK);
    create_pre_registered_component!(Color, ECS_DOC_COLOR);
    create_pre_registered_component!(UUID, ECS_DOC_UUID);
}

pub mod rest {
    use crate::c_types::ECS_REST;

    // REST module components
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct Rest {
        #[doc = "< Port of server (optional, default = 27750)"]
        pub port: u16,
        #[doc = "< Interface address (optional, default = 0.0.0.0)"]
        pub ipaddr: *mut ::core::ffi::c_char,
        pub impl_: *mut ::core::ffi::c_void,
    }

    impl Default for Rest {
        fn default() -> Self {
            Self {
                port: Default::default(),
                ipaddr: core::ptr::null_mut::<core::ffi::c_char>(),
                impl_: core::ptr::null_mut::<core::ffi::c_void>(),
            }
        }
    }

    impl_component_traits_binding_type_w_id!(Rest, ECS_REST);
    unsafe impl Send for Rest {}
    unsafe impl Sync for Rest {}
}

// default component for event API
#[cfg(test)]
mod tests {

    use crate::component::Component;

    use super::*;

    macro_rules! wassert_eq {
        ($world: expr, $left:expr, $right:expr $(,)?) => {
            let _xx_left_side = ($left).retrieve_id(&$world);
            assert_eq!(_xx_left_side, $right);
        };
        ($world: expr, $left:expr, $right:expr , $($arg:tt)+) => {
            let _xx_left_side = ($left).retrieve_id(&$world);
            assert_eq!(_xx_left_side, $right);
        };
    }

    #[test]
    fn test_c_vs_rust_ids() {
        let world = crate::world::World::new();

        unsafe {
            let _xx_left_side = (Self_).retrieve_id(&world);
            assert_eq!(_xx_left_side, (sys::EcsSelf as u64));
            wassert_eq!(world, Up, sys::EcsUp, "EcsUp (C) != Up (Rust)");
            wassert_eq!(world, Trav, sys::EcsTrav, "EcsTrav (C) != Trav (Rust)");
            wassert_eq!(
                world,
                Cascade,
                sys::EcsCascade,
                "EcsCascade (C) != Cascade (Rust)"
            );
            wassert_eq!(world, Desc, sys::EcsDesc, "EcsDesc (C) != Desc (Rust)");
            wassert_eq!(
                world,
                IsVariable,
                sys::EcsIsVariable,
                "EcsIsVariable (C) != IsVariable (Rust)"
            );
            wassert_eq!(
                world,
                IsEntity,
                sys::EcsIsEntity,
                "EcsIsEntity (C) != IsEntity (Rust)"
            );
            wassert_eq!(
                world,
                IsName,
                sys::EcsIsName,
                "EcsIsName (C) != IsName (Rust)"
            );
            wassert_eq!(
                world,
                TraverseFlags,
                sys::EcsTraverseFlags as u64,
                "EcsTraverseFlags (C) != TraverseFlags (Rust)"
            );
            wassert_eq!(
                world,
                TermRefFlags,
                sys::EcsTermRefFlags as u64,
                "EcsTermRefFlags (C) != TermRefFlags (Rust)"
            );

            // Term flags
            wassert_eq!(
                world,
                term_flags::MatchAny,
                sys::EcsTermMatchAny as u64,
                "EcsTermMatchAny (C) != MatchAny (Rust)"
            );
            wassert_eq!(
                world,
                term_flags::MatchAnySrc,
                sys::EcsTermMatchAnySrc as u64,
                "EcsTermMatchAnySrc (C) != MatchAnySrc (Rust)"
            );
            wassert_eq!(
                world,
                term_flags::Transitive,
                sys::EcsTermTransitive as u64,
                "EcsTermTransitive (C) != Transitive (Rust)"
            );
            wassert_eq!(
                world,
                term_flags::Reflexive,
                sys::EcsTermReflexive as u64,
                "EcsTermReflexive (C) != Reflexive (Rust)"
            );
            wassert_eq!(
                world,
                term_flags::IdInherited,
                sys::EcsTermIdInherited as u64,
                "EcsTermIdInherited (C) != IdInherited (Rust)"
            );
            wassert_eq!(
                world,
                term_flags::IsTrivial,
                sys::EcsTermIsTrivial as u64,
                "EcsTermIsTrivial (C) != IsTrivial (Rust)"
            );
            wassert_eq!(
                world,
                term_flags::IsCacheable,
                sys::EcsTermIsCacheable as u64,
                "EcsTermIsCacheable (C) != IsCacheable (Rust)"
            );
            wassert_eq!(
                world,
                term_flags::IsScope,
                sys::EcsTermIsScope as u64,
                "EcsTermIsScope (C) != IsScope (Rust)"
            );
            wassert_eq!(
                world,
                term_flags::IsMember,
                sys::EcsTermIsMember as u64,
                "EcsTermIsMember (C) != IsMember (Rust)"
            );
            wassert_eq!(
                world,
                term_flags::IsToggle,
                sys::EcsTermIsToggle as u64,
                "EcsTermIsToggle (C) != IsToggle (Rust)"
            );
            wassert_eq!(
                world,
                term_flags::KeepAlive,
                sys::EcsTermKeepAlive as u64,
                "EcsTermKeepAlive (C) != KeepAlive (Rust)"
            );
            wassert_eq!(
                world,
                term_flags::IsSparse,
                sys::EcsTermIsSparse as u64,
                "EcsTermIsSparse (C) != IsSparse (Rust)"
            );
            wassert_eq!(
                world,
                term_flags::IsUnion,
                sys::EcsTermIsUnion as u64,
                "EcsTermIsUnion (C) != IsUnion (Rust)"
            );
            wassert_eq!(
                world,
                term_flags::IsOr,
                sys::EcsTermIsOr as u64,
                "EcsTermIsOr (C) != IsOr (Rust)"
            );

            // Query flags
            wassert_eq!(
                world,
                query_flags::MatchPrefab,
                sys::EcsQueryMatchPrefab as u64,
                "EcsQueryMatchPrefab (C) != MatchPrefab (Rust)"
            );
            wassert_eq!(
                world,
                query_flags::MatchDisabled,
                sys::EcsQueryMatchDisabled as u64,
                "EcsQueryMatchDisabled (C) != MatchDisabled (Rust)"
            );
            wassert_eq!(
                world,
                query_flags::MatchEmptyTables,
                sys::EcsQueryMatchEmptyTables as u64,
                "EcsQueryMatchEmptyTables (C) != MatchEmptyTables (Rust)"
            );
            wassert_eq!(
                world,
                query_flags::AllowUnresolvedByName,
                sys::EcsQueryAllowUnresolvedByName as u64,
                "EcsQueryAllowUnresolvedByName (C) != AllowUnresolvedByName (Rust)"
            );
            wassert_eq!(
                world,
                query_flags::TableOnly,
                sys::EcsQueryTableOnly as u64,
                "EcsQueryTableOnly (C) != TableOnly (Rust)"
            );

            assert_eq!(EcsComponent::ID.unwrap(), sys::FLECS_IDEcsComponentID_);
            assert_eq!(Identifier::ID.unwrap(), sys::FLECS_IDEcsIdentifierID_);
            assert_eq!(Poly::ID.unwrap(), sys::FLECS_IDEcsPolyID_);
            assert_eq!(
                DefaultChildComponent::ID.unwrap(),
                sys::FLECS_IDEcsDefaultChildComponentID_
            );

            // Poly target components
            wassert_eq!(world, Query, sys::EcsQuery);
            wassert_eq!(world, Observer, sys::EcsObserver);

            // Core scontities
            wassert_eq!(world, EcsWorld, sys::EcsWorld);
            wassert_eq!(world, Flecs, sys::EcsFlecs);
            wassert_eq!(world, FlecsCore, sys::EcsFlecsCore);
            //wassert_eq::FlecsInternals, sys::EcsFlecsInternals);
            wassert_eq!(world, Module, sys::EcsModule);
            wassert_eq!(world, Private, sys::EcsPrivate);
            wassert_eq!(world, Prefab, sys::EcsPrefab);
            wassert_eq!(world, Disabled, sys::EcsDisabled);
            wassert_eq!(world, NotQueryable, sys::EcsNotQueryable);
            wassert_eq!(world, SlotOf, sys::EcsSlotOf);
            //wassert_eq::Flag, sys::EcsFlag);
            wassert_eq!(world, Monitor, sys::EcsMonitor);
            wassert_eq!(world, Empty, sys::EcsEmpty);

            // Componens
            wassert_eq!(world, Wildcard, sys::EcsWildcard);
            wassert_eq!(world, Any, sys::EcsAny);
            wassert_eq!(world, This_, sys::EcsThis);
            wassert_eq!(world, Variable, sys::EcsVariable);
            wassert_eq!(world, Singleton, sys::EcsVariable);
            wassert_eq!(world, Transitive, sys::EcsTransitive);
            wassert_eq!(world, Reflexive, sys::EcsReflexive);
            wassert_eq!(world, Symmetric, sys::EcsSymmetric);
            wassert_eq!(world, Final, sys::EcsFinal);
            wassert_eq!(world, PairIsTag, sys::EcsPairIsTag);
            wassert_eq!(world, Exclusive, sys::EcsExclusive);
            wassert_eq!(world, Acyclic, sys::EcsAcyclic);
            wassert_eq!(world, Traversable, sys::EcsTraversable);
            wassert_eq!(world, With, sys::EcsWith);
            wassert_eq!(world, OneOf, sys::EcsOneOf);
            wassert_eq!(world, CanToggle, sys::EcsCanToggle);
            wassert_eq!(world, Trait, sys::EcsTrait);
            wassert_eq!(world, Relationship, sys::EcsRelationship);
            wassert_eq!(world, Target, sys::EcsTarget);

            // OnInstanraits
            wassert_eq!(world, OnInstantiate, sys::EcsOnInstantiate);
            wassert_eq!(world, Override, sys::EcsOverride);
            wassert_eq!(world, Inherit, sys::EcsInherit);
            wassert_eq!(world, DontInherit, sys::EcsDontInherit);

            // OnDeleteteTarget traits
            wassert_eq!(world, OnDelete, sys::EcsOnDelete);
            wassert_eq!(world, OnDeleteTarget, sys::EcsOnDeleteTarget);
            wassert_eq!(world, Remove, sys::EcsRemove);
            wassert_eq!(world, Delete, sys::EcsDelete);
            wassert_eq!(world, Panic, sys::EcsPanic);

            // Builtin nships
            wassert_eq!(world, ChildOf, sys::EcsChildOf);
            wassert_eq!(world, IsA, sys::EcsIsA);
            wassert_eq!(world, DependsOn, sys::EcsDependsOn);

            // Identifi
            wassert_eq!(world, Name, sys::EcsName);
            wassert_eq!(world, Symbol, sys::EcsSymbol);
            wassert_eq!(world, Alias, sys::EcsAlias);

            // Events
            wassert_eq!(world, OnAdd, sys::EcsOnAdd);
            wassert_eq!(world, OnRemove, sys::EcsOnRemove);
            wassert_eq!(world, OnSet, sys::EcsOnSet);
            wassert_eq!(world, OnTableCreate, sys::EcsOnTableCreate);
            wassert_eq!(world, OnTableDelete, sys::EcsOnTableDelete);

            // System
            {
                assert_eq!(
                    system::TickSource::ID.unwrap(),
                    sys::FLECS_IDEcsTickSourceID_
                );
                wassert_eq!(world, system::System, sys::EcsSystem);
            }

            // Timer
            {
                assert_eq!(timer::Timer::ID.unwrap(), sys::FLECS_IDEcsTimerID_);
                assert_eq!(
                    timer::RateFilter::ID.unwrap(),
                    sys::FLECS_IDEcsRateFilterID_
                );
            }

            wassert_eq!(
                world,
                Sparse,
                sys::EcsSparse,
                "EcsSparse (C) != Sparse (Rust)",
            );
            wassert_eq!(world, Union, sys::EcsUnion, "EcsUnion (C) != Union (Rust)");

            // Builtin predicate for comparing entity ids
            wassert_eq!(world, PredEq, sys::EcsPredEq);
            wassert_eq!(world, PredMatch, sys::EcsPredMatch);
            wassert_eq!(world, PredLookup, sys::EcsPredLookup);

            // builtin marker entities for query scopes
            wassert_eq!(world, ScopeOpen, sys::EcsScopeOpen);
            wassert_eq!(world, ScopeClose, sys::EcsScopeClose);

            // Pipeline
            {
                wassert_eq!(world, pipeline::Pipeline, sys::FLECS_IDEcsPipelineID_);
                wassert_eq!(world, pipeline::OnStart, sys::EcsOnStart);
                wassert_eq!(world, pipeline::OnLoad, sys::EcsOnLoad);
                wassert_eq!(world, pipeline::PostLoad, sys::EcsPostLoad);
                wassert_eq!(world, pipeline::PreUpdate, sys::EcsPreUpdate);
                wassert_eq!(world, pipeline::OnUpdate, sys::EcsOnUpdate);
                wassert_eq!(world, pipeline::OnValidate, sys::EcsOnValidate);
                wassert_eq!(world, pipeline::PostUpdate, sys::EcsPostUpdate);
                wassert_eq!(world, pipeline::PreStore, sys::EcsPreStore);
                wassert_eq!(world, pipeline::OnStore, sys::EcsOnStore);
                wassert_eq!(world, pipeline::Phase, sys::EcsPhase);
            }

            // Meta
            {
                wassert_eq!(world, meta::Bool, sys::FLECS_IDecs_bool_tID_);
                wassert_eq!(world, meta::Char, sys::FLECS_IDecs_char_tID_);
                wassert_eq!(world, meta::Byte, sys::FLECS_IDecs_byte_tID_);
                wassert_eq!(world, meta::U8, sys::FLECS_IDecs_u8_tID_);
                wassert_eq!(world, meta::U16, sys::FLECS_IDecs_u16_tID_);
                wassert_eq!(world, meta::U32, sys::FLECS_IDecs_u32_tID_);
                wassert_eq!(world, meta::U64, sys::FLECS_IDecs_u64_tID_);
                wassert_eq!(world, meta::UPtr, sys::FLECS_IDecs_uptr_tID_);
                wassert_eq!(world, meta::I8, sys::FLECS_IDecs_i8_tID_);
                wassert_eq!(world, meta::I16, sys::FLECS_IDecs_i16_tID_);
                wassert_eq!(world, meta::I32, sys::FLECS_IDecs_i32_tID_);
                wassert_eq!(world, meta::I64, sys::FLECS_IDecs_i64_tID_);
                wassert_eq!(world, meta::IPtr, sys::FLECS_IDecs_iptr_tID_);
                wassert_eq!(world, meta::F32, sys::FLECS_IDecs_f32_tID_);
                wassert_eq!(world, meta::F64, sys::FLECS_IDecs_f64_tID_);
                wassert_eq!(world, meta::String, sys::FLECS_IDecs_string_tID_);
                wassert_eq!(world, meta::Entity, sys::FLECS_IDecs_entity_tID_);
                wassert_eq!(world, meta::Constant, sys::EcsConstant);
                wassert_eq!(world, meta::Quantity, sys::EcsQuantity);
                wassert_eq!(world, meta::EcsOpaque, sys::FLECS_IDEcsOpaqueID_);

                assert_eq!(meta::Type::ID.unwrap(), sys::FLECS_IDEcsTypeID_);
                assert_eq!(
                    meta::TypeSerializer::ID.unwrap(),
                    sys::FLECS_IDEcsTypeSerializerID_
                );
                assert_eq!(meta::Primitive::ID.unwrap(), sys::FLECS_IDEcsPrimitiveID_);
                assert_eq!(meta::EcsEnum::ID.unwrap(), sys::FLECS_IDEcsEnumID_);
                assert_eq!(meta::Bitmask::ID.unwrap(), sys::FLECS_IDEcsBitmaskID_);
                assert_eq!(meta::Member::ID.unwrap(), sys::FLECS_IDEcsMemberID_);
                assert_eq!(
                    meta::MemberRanges::ID.unwrap(),
                    sys::FLECS_IDEcsMemberRangesID_
                );
                assert_eq!(meta::EcsStruct::ID.unwrap(), sys::FLECS_IDEcsStructID_);
                assert_eq!(meta::Array::ID.unwrap(), sys::FLECS_IDEcsArrayID_);
                assert_eq!(meta::Vector::ID.unwrap(), sys::FLECS_IDEcsVectorID_);
                assert_eq!(meta::Unit::ID.unwrap(), sys::FLECS_IDEcsUnitID_);
                assert_eq!(meta::UnitPrefix::ID.unwrap(), sys::FLECS_IDEcsUnitPrefixID_);
            }

            // Doc
            {
                wassert_eq!(world, doc::Description, sys::FLECS_IDEcsDocDescriptionID_);
                wassert_eq!(world, doc::Brief, sys::EcsDocBrief);
                wassert_eq!(world, doc::Detail, sys::EcsDocDetail);
                wassert_eq!(world, doc::Link, sys::EcsDocLink);
                wassert_eq!(world, doc::Color, sys::EcsDocColor);
                wassert_eq!(world, doc::UUID, sys::EcsDocUuid);
            }

            // Rest
            {
                assert_eq!(rest::Rest::ID.unwrap(), sys::FLECS_IDEcsRestID_);
            }
        }
    }
}
