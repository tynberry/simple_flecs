use crate::{
    component::{Component, id::id},
    flecs::system::System,
    world::World,
};

struct Singleton {
    a_sum: usize,
    b_sum: usize,
}

struct A {
    a: usize,
}
struct B {
    b: usize,
}

impl Component for A {}
impl Component for B {}
impl Component for Singleton {}

#[test]
fn system_test() {
    let mut world = World::new();

    //register components
    world.component::<A>(c"A");
    world.component::<B>(c"B");
    world.component::<Singleton>(c"S");

    //fill singleton
    world.singleton_set(Singleton { a_sum: 0, b_sum: 0 });

    //create entities
    let a = world.entity_named(c"a");
    let b = world.entity_named(c"b");
    let c = world.entity_named(c"c");
    let d = world.entity_named(c"d");
    let e = world.entity_named(c"e");
    let f = world.entity_named(c"f");
    //add some components
    a.set_comp(A { a: 1 });
    b.set_comp(B { b: 3 });
    c.set_comp(A { a: 7 });
    d.set_comp(B { b: 5 });
    e.set_comp(A { a: 9 });
    f.set_comp(B { b: 8 });

    //create a system
    world
        .system_expr(c"S($), A || B")
        .build_named(c"test_system", |iter| {
            let mut singleton = unsafe { iter.get::<Singleton>(0).unwrap() };
            if iter.check_id(id::<A>(), 1) {
                let data = unsafe { iter.get_from_table::<A>() }.unwrap();
                for i in 0..iter.count() {
                    singleton[i].a_sum += data[i].a;
                }
            } else if iter.check_id(id::<B>(), 1) {
                let data = unsafe { iter.get_from_table::<B>() }.unwrap();
                for i in 0..iter.count() {
                    singleton[i].b_sum += data[i].b;
                }
            } else {
                panic!("should not happen");
            }
        });

    //check existence
    let system = world.lookup(c"test_system").unwrap();
    assert!(system.has(System));

    //progress
    world.progress();
    //check results
    assert_eq!(
        unsafe { world.singleton_get::<Singleton>().unwrap().a_sum },
        1 + 7 + 9
    );
    assert_eq!(
        unsafe { world.singleton_get::<Singleton>().unwrap().b_sum },
        3 + 5 + 8
    );
}

