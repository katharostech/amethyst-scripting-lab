#![allow(unused_imports)]
#![allow(dead_code)]

use generic_array::{ArrayLength, GenericArray};
use specs::shred::{Accessor, AccessorCow, DynamicSystemData, Fetch, ResourceId, RunNow, System, World};
use std::collections::HashMap;
use typenum::{U1024, U256};

/// This is the interface used to add resources at runtime.
#[derive(Debug)]
pub struct ScriptingInterface {
    id_alloc: u64,
    type_map: HashMap<String, u64>,
}

/// Allows creating and removing scripted resources at runtime.
/// 
/// A resource encompasses an piece of data in the world, therefore both
/// worldwide "Resources" in the sense that Specs has defined and the component
/// data tied to a specific entity would be a resource.
impl ScriptingInterface {
    /// Creates a new `ScriptingInterface`.
    pub fn new() -> Self {
        ScriptingInterface {
            id_alloc: 1,
            type_map: HashMap::new(),
        }
    }

    pub fn resource_id<T: 'static + ArrayLength<u8>>(&self, name: &str) -> Option<ResourceId> {
        self.type_map
            .get(name)
            .cloned()
            .map(ResourceId::new_with_dynamic_id::<GenericArray<u8, T>>)
    }

    pub fn add_resource<T: 'static + Sync + Send + ArrayLength<u8>>(
        &mut self,
        name: &str,
        world: &mut World,
    ) {
        self.type_map.insert(name.into(), self.id_alloc);
        self.id_alloc += 1;

        let id = self.resource_id::<T>(name).unwrap();
        world.insert_by_id(id, GenericArray::<u8, T>::default());
    }
}

fn setup_world() -> World {
    let mut world = World::empty();

    let mut scripting_interface = ScriptingInterface::new();

    scripting_interface.add_resource::<U256>("data256-1", &mut world);
    scripting_interface.add_resource::<U256>("data256-2", &mut world);
    scripting_interface.add_resource::<U1024>("data1024-1", &mut world);

    world.insert(scripting_interface);

    world
}

pub struct ScrptingResAccessor {
    reads: Vec<ResourceId>,
}

//////////////////////////////////////

#[derive(Debug)]
pub struct Position {
    x: f32,
    y: f32,
}

impl specs::Component for Position {
    type Storage = specs::VecStorage<Self>;
}

use specs::{WorldExt, Builder};

struct BakedSystem;

impl<'a> System<'a> for BakedSystem {
    type SystemData = specs::WriteStorage<'a, Position>;

    fn run(&mut self, data: Self::SystemData) {
        use specs::Join;

        for position in data.join() {
            println!("Position: {:?}", position);
        }
    }
}

fn main() {
    let mut world = specs::World::new();
    world.register::<Position>();

    let _ball = world.create_entity().with(Position { x: 5.0, y: 10.0 }).build();

    let mut baked_system = BakedSystem;

    let mut i = 0;
    while i < 2 {
        baked_system.run_now(&mut world);
        world.maintain();
        i += 1;
        println!("------");
    }
}
