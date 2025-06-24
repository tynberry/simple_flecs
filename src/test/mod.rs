use crate::{
    component::{Component, id::id},
    flecs::{EcsComponent, Symmetric},
    world::World,
};

#[derive(Debug, PartialEq)]
struct FooBar {
    biz: usize,
    bear: f64,
}

struct Likes;

impl Component for FooBar {}
impl Component for Likes {}

#[test]
fn basic_test() {
    //create a new world
    let mut world = World::new();

    //register components
    world.component::<FooBar>(c"FooBar");
    world.component::<Likes>(c"Likes").add_trait(Symmetric);

    //create testing entities
    {
        let alice = world.entity_named(c"alice");
        alice.set_comp(FooBar {
            biz: 17,
            bear: -8.5,
        });
        let bob = world.entity_named(c"bob");
        bob.set_comp(FooBar { biz: 8, bear: 9.2 });
        bob.add_id((id::<Likes>(), alice));
    }

    //test stuff
    {
        //test lookups
        let alice = world.lookup(c"alice").unwrap();
        let bob = world.lookup(c"bob").unwrap();
        let cecil = world.lookup(c"cecil");
        assert!(cecil.is_none(), "cecil should not exist");
        //test component lookups
        let foo_bar = world.lookup_symbol(c"FooBar").unwrap();
        let comp = unsafe { foo_bar.get::<EcsComponent>() }.unwrap();
        assert_eq!(comp.size, core::mem::size_of::<FooBar>() as i32);
        assert_eq!(comp.alignment, core::mem::align_of::<FooBar>() as i32);
        let likes = world.lookup_symbol(c"Likes").unwrap();
        let comp = unsafe { likes.get::<EcsComponent>() };
        assert!(comp.is_none(), "likes is a tag");
        //test components
        let alice_bar: &FooBar = unsafe { alice.get() }.unwrap();
        assert_eq!(
            alice_bar,
            &FooBar {
                biz: 17,
                bear: -8.5,
            }
        );
        let bob_bar: &FooBar = unsafe { bob.get() }.unwrap();
        assert_eq!(bob_bar, &FooBar { biz: 8, bear: 9.2 });
        //test (symetric) pairs
        assert!(alice.has_id((id::<Likes>(), bob)));
        assert!(bob.has_id((id::<Likes>(), alice)));
    }
}
