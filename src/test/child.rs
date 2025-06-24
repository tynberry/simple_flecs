use crate::{flecs::ChildOf, world::World};

#[test]
fn child_test() {
    let world = World::new();

    //create entities
    let alice = world.entity_named(c"alice");
    let bob = world.entity_named(c"bob");
    let cecil = world.entity_named(c"cecil");
    let david = world.entity_named(c"david");
    //add parent child hiearchy
    bob.add((ChildOf, alice));
    cecil.add((ChildOf, alice));
    david.add((ChildOf, cecil));

    //test lookups
    assert_eq!(world.lookup(c"alice").unwrap(), alice);
    assert_eq!(world.lookup(c"alice.bob").unwrap(), bob);
    assert_eq!(world.lookup(c"alice.cecil").unwrap(), cecil);
    assert_eq!(world.lookup(c"alice.cecil.david").unwrap(), david);
    assert_eq!(alice.lookup(c"bob").unwrap(), bob);
    assert_eq!(alice.lookup(c"cecil").unwrap(), cecil);
    assert_eq!(alice.lookup(c"cecil.david").unwrap(), david);
    //test paths
    assert_eq!(alice.path(), "alice");
    assert_eq!(bob.path(), "alice.bob");
    assert_eq!(cecil.path(), "alice.cecil");
    assert_eq!(david.path(), "alice.cecil.david");
}
