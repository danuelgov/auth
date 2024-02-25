use crate::Hash;
use serde::{Deserialize, Serialize};

const ARGON2: &'static str = "Argon2";
const BCRYPT: &'static str = "Bcrypt";

#[derive(Debug, Clone, Copy)]
pub enum Hasher {
    Argon2,
    Bcrypt,
}

#[derive(Debug)]
pub enum HasherError {
    Task(tokio::task::JoinError),
    Argon2(argon2::Error),
    Bcrypt,
}

impl std::fmt::Display for Hasher {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Argon2 => write!(f, "{}", ARGON2),
            Self::Bcrypt => write!(f, "{}", BCRYPT),
        }
    }
}

impl std::str::FromStr for Hasher {
    type Err = ();

    #[inline]
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            ARGON2 => Ok(Self::Argon2),
            BCRYPT => Ok(Self::Bcrypt),
            _ => Err(()),
        }
    }
}

impl Serialize for Hasher {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        let value = match self {
            Self::Argon2 => ARGON2,
            Self::Bcrypt => BCRYPT,
        };

        value.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Hasher {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Hasher, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;

        match value.as_str() {
            ARGON2 => Ok(Hasher::Argon2),
            BCRYPT => Ok(Hasher::Bcrypt),
            _ => Err(serde::de::Error::custom(format!(
                "Invalid value: {}",
                value
            ))),
        }
    }
}

impl Hasher {
    pub async fn hash(&self, source: &[u8]) -> Result<Hash, HasherError> {
        match self {
            Hasher::Argon2 => {
                let source = source.to_owned();
                let config = argon2::Config::default();
                let handle = tokio::spawn(async move {
                    let salt: [u8; 16] = rand::random();
                    let hash = argon2::hash_encoded(&source, &salt, &config)
                        .map_err(HasherError::Argon2)?;

                    Ok(Hash::new(hash))
                });

                handle.await.map_err(HasherError::Task)?
            }
            Hasher::Bcrypt => std::todo!("Implement Bcrypt"),
        }
    }

    pub async fn verify(&self, source: &[u8], hash: &Hash) -> Result<bool, HasherError> {
        match self {
            Hasher::Argon2 => {
                let source = source.to_owned();
                let hash = hash.as_str().to_owned();
                let handle = tokio::spawn(async move {
                    let result =
                        argon2::verify_encoded(&hash, &source).map_err(HasherError::Argon2)?;

                    Ok(result)
                });

                handle.await.map_err(HasherError::Task)?
            }
            Hasher::Bcrypt => std::todo!("Implement Bcrypt"),
        }
    }
}
