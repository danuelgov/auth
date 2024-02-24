use url::Url;

#[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ImageUrl(Url);

impl std::fmt::Display for ImageUrl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl std::fmt::Debug for ImageUrl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ImageUrl(\"{}\")", self.as_str())
    }
}

impl std::ops::Deref for ImageUrl {
    type Target = Url;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::str::FromStr for ImageUrl {
    type Err = url::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let url = Url::from_str(s)?;

        Ok(ImageUrl(url))
    }
}
