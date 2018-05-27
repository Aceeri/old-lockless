
pub trait Ident {
    fn identifier(&self) -> String;
}

pub trait IdentMap {
    type Handle: Component;
    fn ident_value(&self, ident: String) -> Mapped;
}

impl IdentMap for HashMap<String, Handle<Mesh>> {
    type Handle = Handle<Mesh>;
}

pub struct IdentMapperSystem<I, Map>;
impl<'a, I, H, Map> System<'a> for IdentMapperSystem<Ident, Mapped, Map<Mapped>>
where
    I: Ident,
    Map: IdentMap,
{
    type SystemData = (
        ReadResources<'a, C>,
        Fetch<'a, Map>,
    );
}
