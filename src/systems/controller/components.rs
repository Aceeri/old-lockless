
/// Player tag component.
/// 
/// Entities tagged with this component will be controlled by controller systems.
pub struct Player {
    entity: Entity, 
}

impl Component for Player {
    type Storage = DenseVecStorage<Self>;
}

