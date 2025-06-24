use crate::{
    component::{Component, id::id},
    world::World,
};

static mut DROP_COUNT: usize = 0;
static mut DROPPED: usize = 0;

#[derive(Debug)]
struct Dropper {
    cool_data: usize,
}

impl Component for Dropper {}

impl Drop for Dropper {
    fn drop(&mut self) {
        unsafe {
            DROP_COUNT += 1;
            DROPPED ^= self.cool_data;
        }
    }
}

#[test]
fn drop_test() {
    //create a world
    let mut world = World::new();
    //register component
    world.component::<Dropper>(c"Dropper");
    //create entity with said component
    {
        let alice = world.entity_named(c"alice");
        alice.set_comp::<Dropper>(Dropper { cool_data: 189 })
    };
    //remove said component
    {
        let alice = world.lookup(c"alice").unwrap();
        alice.remove(id::<Dropper>());
    }
    //check
    unsafe {
        let drop_count = DROP_COUNT;
        let dropped = DROPPED;
        assert_eq!(drop_count, 1);
        assert_eq!(dropped, 189);
    }
}
