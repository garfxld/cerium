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

    pub fn vanilla<S>(path: S) -> Self
    where
        S: Into<String>,
    {
        Self::new("minecraft", path)
    }

    pub fn of<S>(key: S) -> Self
    where
        S: Into<String>,
    {
        let key: String = key.into();
        if let Some((namespace, path)) = key.split_once(":") {
            Self::new(namespace, path)
        } else {
            Self::vanilla(key)
        }
    }
}

impl std::fmt::Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.namespace, self.path)
    }
}
