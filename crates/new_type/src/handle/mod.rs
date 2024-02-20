#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Handle(String);

#[derive(Debug)]
pub enum HandleError {
    MissingPrefix,
    TooShort,
    TooLong,
}

impl std::ops::Deref for Handle {
    type Target = str;

    #[inline]
    fn deref(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for Handle {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "@{}", self.0)
    }
}

impl serde::Serialize for Handle {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.to_string().serialize(serializer)
    }
}

impl<'de> serde::Deserialize<'de> for Handle {
    fn deserialize<D>(deserializer: D) -> Result<Handle, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let source = String::deserialize(deserializer)?;
        let handle = source.parse().map_err(serde::de::Error::custom)?;

        Ok(handle)
    }
}

impl std::str::FromStr for Handle {
    type Err = HandleError;

    fn from_str(source: &str) -> Result<Self, Self::Err> {
        match source.strip_prefix("@") {
            Some(handle) if handle.len() < 3 => Err(HandleError::TooShort),
            Some(handle) if handle.len() > 20 => Err(HandleError::TooLong),
            Some(handle) => Ok(Handle(handle.to_owned())),
            None => Err(HandleError::MissingPrefix),
        }
    }
}

impl std::fmt::Display for HandleError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::MissingPrefix => write!(f, "missing @ prefix"),
            Self::TooShort => write!(f, "handle is too short"),
            Self::TooLong => write!(f, "handle is too long"),
        }
    }
}

impl std::error::Error for HandleError {
    //
}

impl Handle {
    pub fn new() -> Self {
        let handle: [u8; 16] = rand::random();
        let handle = base58::encode(handle)
            .with_alphabet(base58::Alphabet::FLICKR)
            .into_string();

        Self(handle)
    }

    #[inline]
    pub fn new_unchecked<S: Into<String>>(source: S) -> Self {
        Self(source.into())
    }

    #[inline]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
