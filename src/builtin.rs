use crate::entity::Entity;

macro_rules! builtin_entity {
    ($name:ident) => {
        pub struct $name;

        impl From<$name> for Entity {
            fn from(_: $name) -> Entity {
                unsafe { ::flecs_ecs_sys::$name }
            }
        }
    };
}

// Builtin components
builtin_entity!(EcsPrefab);

// Component traits
builtin_entity!(EcsSparse);

// Pipeline phases
builtin_entity!(EcsOnLoad);
