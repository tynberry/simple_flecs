use crate::{
    component::{Component, id::id},
    world::World,
};

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Foo {
    foo_one: usize,
}
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Bar {
    bar_one: usize,
}
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Baz {
    baz_one: usize,
}
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Qwak {
    qwak_one: usize,
}

impl Component for Foo {}
impl Component for Bar {}
impl Component for Baz {}
impl Component for Qwak {}

#[test]
fn singleton_test() {
    let mut world = World::new();
    //register components
    world.component::<Foo>(c"Foo");
    world.component::<Bar>(c"Bar");
    world.component::<Baz>(c"Baz");
    world.component::<Qwak>(c"Qwak");
    //singletons
    world.singleton_set(Foo { foo_one: 1 });
    world.singleton_set(Bar { bar_one: 10 });
    world.singleton_set(Baz { baz_one: 100 });
    world.singleton_set(Qwak { qwak_one: 1000 });
    //check, gets
    unsafe {
        assert_eq!(world.singleton_get::<Foo>().unwrap(), &Foo { foo_one: 1 });
        assert_eq!(world.singleton_get::<Bar>().unwrap(), &Bar { bar_one: 10 });
        assert_eq!(world.singleton_get::<Baz>().unwrap(), &Baz { baz_one: 100 });
        assert_eq!(
            world.singleton_get::<Qwak>().unwrap(),
            &Qwak { qwak_one: 1000 }
        );
    }
    //check existence
    assert!(world.singleton_exists(id::<Foo>()));
    assert!(world.singleton_exists(id::<Bar>()));
    assert!(world.singleton_exists(id::<Baz>()));
    assert!(world.singleton_exists(id::<Qwak>()));
    //remove
    world.singleton_remove(id::<Foo>());
    world.singleton_remove(id::<Bar>());
    world.singleton_remove(id::<Baz>());
    world.singleton_remove(id::<Qwak>());
    //check existence
    assert!(!world.singleton_exists(id::<Foo>()));
    assert!(!world.singleton_exists(id::<Bar>()));
    assert!(!world.singleton_exists(id::<Baz>()));
    assert!(!world.singleton_exists(id::<Qwak>()));
}
