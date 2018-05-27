
pub struct MeshMap {
    meshes: HashMap<String, Handle<::amethyst::renderer::Mesh>>,
}

pub struct MeshIdent {
    ident: String,
}

impl Component for MeshIdent {
    type Storage = FlaggedStorage<Self, DenseVecStorage<Self>>;
}

pub struct MaterialMap {
    materials: HashMap<String, Handle<::amethyst::renderer::Material>>,
}

pub struct MaterialIdent {
    ident: String,
}

impl Component for MaterialIdent {
    type Storage = FlaggedStorage<Self, DenseVecStorage<Self>>;
}
