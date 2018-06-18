use std::collections::HashMap;

use nphysics3d::object::BodyHandle;

use specs::prelude::*;
use specs::world::Index;

use super::{Body3d, PhysicsWorld3d};

pub struct HandleRemovalSystem3d {
    body_handles: HashMap<Index, BodyHandle>,

    body_inserted: BitSet,
    body_inserted_flag: Option<ReaderId<InsertedFlag>>,

    body_modified: BitSet,
    body_modified_flag: Option<ReaderId<ModifiedFlag>>,

    body_removed: BitSet,
    body_removed_flag: Option<ReaderId<RemovedFlag>>,
}

impl HandleRemovalSystem3d {
    pub fn new() -> Self {
        Self {
            body_handles: HashMap::new(),

            body_inserted: BitSet::new(),
            body_inserted_flag: None,

            body_modified: BitSet::new(),
            body_modified_flag: None,

            body_removed: BitSet::new(),
            body_removed_flag: None,
        }
    }
}

impl<'a> System<'a> for HandleRemovalSystem3d {
    type SystemData = (Write<'a, PhysicsWorld3d>, ReadStorage<'a, Body3d>);
    fn run(&mut self, data: Self::SystemData) {
        let (mut world, bodies) = data;

        bodies.populate_inserted(
            self.body_inserted_flag.as_mut().unwrap(),
            &mut self.body_inserted,
        );
        bodies.populate_modified(
            self.body_modified_flag.as_mut().unwrap(),
            &mut self.body_modified,
        );
        bodies.populate_removed(
            self.body_removed_flag.as_mut().unwrap(),
            &mut self.body_removed,
        );

        let handles = (&self.body_removed)
            .join()
            .map(|index| self.body_handles.get(&index))
            .filter_map(|handle| handle)
            .map(|handle| *handle)
            .collect::<Vec<BodyHandle>>();
        world.remove_bodies(handles.as_slice());

        for (_body, index) in (&bodies, &self.body_modified).join() {
            if let Some(handle) = self.body_handles.get(&index) {
                world.remove_bodies(&[*handle])
            }
        }

        for (body, index) in (&bodies, &self.body_inserted | &self.body_modified).join() {
            self.body_handles.insert(index, body.handle);
        }
    }

    fn setup(&mut self, res: &mut Resources) {
        Self::SystemData::setup(res);
        let mut bodies: WriteStorage<Body3d> = SystemData::fetch(&res);

        self.body_inserted_flag = Some(bodies.track_inserted());
        self.body_modified_flag = Some(bodies.track_modified());
        self.body_removed_flag = Some(bodies.track_removed());
    }
}
