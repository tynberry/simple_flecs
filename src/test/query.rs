use crate::{
    component::{Component, id::id},
    world::World,
};

pub struct TestSingleton {
    data: usize,
}

pub struct TestData {
    data: usize,
}

pub struct TestTag;

impl Component for TestSingleton {}
impl Component for TestData {}
impl Component for TestTag {}

#[test]
fn query_test() {
    unsafe {
        flecs_ecs_sys::ecs_log_set_level(3);
    }
    let mut world = World::new();
    //register components
    world.component::<TestSingleton>(c"Singleton");
    world.component::<TestData>(c"Data");
    world.component::<TestTag>(c"Tag");
    //add singleton
    world.singleton_set(TestSingleton { data: 1 });
    //create some entities
    let e1 = world.entity_named(c"e1");
    let e2 = world.entity_named(c"e2");
    let e3 = world.entity_named(c"e3");
    let e4 = world.entity_named(c"e4");
    let e5 = world.entity_named(c"e5");
    //add some components
    e1.set_comp(TestData { data: 2 });
    e2.set_comp(TestData { data: 3 });
    e3.set_comp(TestData { data: 5 });
    e4.set_comp(TestData { data: 7 });
    e5.set_comp(TestData { data: 11 });
    //add some tags
    e1.add(id::<TestTag>());
    e3.add(id::<TestTag>());
    e5.add(id::<TestTag>());

    //simple all query
    let simple_query = world.query_expr(c"[in] Data, [out] Singleton($)").build();
    let mut query_iter = simple_query.iter();
    while query_iter.advance() {
        let data = unsafe { query_iter.get::<TestData>(0) }.unwrap();
        let mut singleton = unsafe { query_iter.get_mut::<TestSingleton>(1) }.unwrap();
        for ent in 0..query_iter.count() {
            singleton[ent].data *= data[ent].data;
        }
    }
    //assert
    assert_eq!(
        unsafe { world.singleton_get::<TestSingleton>().unwrap().data },
        2 * 3 * 5 * 7 * 11
    );
    //drop
    drop(simple_query);

    //reset
    world.singleton_set::<TestSingleton>(TestSingleton { data: 1 });
    //simple filtered query
    let filtered_query = world
        .query_expr(c"[in] Data, [out] Singleton($), Tag")
        .build();
    let mut query_iter = filtered_query.iter();
    while query_iter.advance() {
        let data = unsafe { query_iter.get::<TestData>(0) }.unwrap();
        let mut singleton = unsafe { query_iter.get_mut::<TestSingleton>(1) }.unwrap();
        for ent in 0..query_iter.count() {
            singleton[ent].data *= data[ent].data;
        }
    }
    //assert
    assert_eq!(
        unsafe { world.singleton_get::<TestSingleton>().unwrap().data },
        2 * 5 * 11
    );
    drop(filtered_query);

    //reset
    world.singleton_set::<TestSingleton>(TestSingleton { data: 1 });
    //simple negative filtered query
    let filtered_query = world
        .query_expr(c"[in] Data, [out] Singleton($), [none] !Tag")
        .build();
    let mut query_iter = filtered_query.iter();
    while query_iter.advance() {
        let data = unsafe { query_iter.get::<TestData>(0) }.unwrap();
        let mut singleton = unsafe { query_iter.get_mut::<TestSingleton>(1) }.unwrap();
        for ent in 0..query_iter.count() {
            singleton[ent].data *= data[ent].data;
        }
    }
    //assert
    assert_eq!(
        unsafe { world.singleton_get::<TestSingleton>().unwrap().data },
        7 * 3
    );
    //drop(filtered_query);
    //reset
    world.singleton_set::<TestSingleton>(TestSingleton { data: 1 });
    //complex filtered query
    let filtered_query = world
        .query_expr(c"[in] Data, [out] Singleton($), [none] ?Tag")
        .build();
    let mut query_iter = filtered_query.iter();
    while query_iter.advance() {
        let data = unsafe { query_iter.get::<TestData>(0) }.unwrap();
        let mut singleton = unsafe { query_iter.get_mut::<TestSingleton>(1) }.unwrap();
        let double_down = query_iter.has(2);
        for ent in 0..query_iter.count() {
            singleton[ent].data *= data[ent].data;
            if double_down {
                singleton[ent].data *= 17;
            }
        }
    }
    //assert
    assert_eq!(
        unsafe { world.singleton_get::<TestSingleton>().unwrap().data },
        2 * 3 * 5 * 7 * 11 * 17 * 17 * 17
    );
    drop(filtered_query);
}
