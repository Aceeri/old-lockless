
#[derive(Debug)]
pub enum Error {
    Amethyst(::amethyst::Error),
    Rayon(::rayon::ThreadPoolBuildError),
    Generic(Box<Error>),
    Custom(String),
}

