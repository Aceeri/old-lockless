
pub struct KinematicController {
    
}

impl Component for KinematicController {

}

// Reverse mapping of ColliderHandle.id -> Entity
// We have Entity -> ColliderHandle by default from specs
pub struct ColliderMapping {
    mapping: HashMap<i32, Entity>,
}

pub struct ColliderMappingSystem {
    
}
