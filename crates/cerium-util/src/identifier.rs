#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Identifier {
    namespace: String,
    path: String,
}

impl Identifier {
    pub fn new<A, B>(namespace: A, path: B) -> Self
    where
        A: Into<String>,
        B: Into<String>,
    {
        Self {
            namespace: namespace.into(),
            path: path.into(),
        }
    }

    pub fn vanilla<T>(path: T) -> Self
    where
        T: Into<String>,
    {
        Self::new("minecraft", path)
    }
}

impl std::fmt::Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.namespace, self.path)
    }
}

#[derive(Debug)]
pub struct IdentifierParseError;

impl TryFrom<String> for Identifier {
    type Error = IdentifierParseError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.split_once(":") {
            Some((namespace, path)) => Ok(Self::new(namespace, path)),
            None => Ok(Self::vanilla(value)), // all biome entries aren't prefixed with "minecraft:" ?
        }
    }
}
