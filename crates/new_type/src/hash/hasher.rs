use crate::{Hash, Salt};

const ARGON2: &'static str = "Argon2";

#[derive(Debug, Clone, Copy)]
pub enum Hasher {
    Argon2,
}

#[derive(Debug)]
pub enum HasherError {
    Task(tokio::task::JoinError),
    Argon2(argon2::Error),
}

impl std::fmt::Display for Hasher {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Argon2 => write!(f, "{}", ARGON2),
        }
    }
}

impl std::str::FromStr for Hasher {
    type Err = ();

    #[inline]
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            ARGON2 => Ok(Self::Argon2),
            _ => Err(()),
        }
    }
}

impl Hasher {
    pub async fn hash(&self, source: &[u8], salt: &Salt) -> Result<Hash, HasherError> {
        match self {
            Hasher::Argon2 => {
                let source = source.to_owned();
                let config = argon2::Config::default();
                let salt = salt.as_bytes();
                let handle = tokio::spawn(async move {
                    let hash = argon2::hash_encoded(&source, &salt, &config)
                        .map_err(HasherError::Argon2)?;

                    Ok(Hash::new(hash))
                });

                handle.await.map_err(HasherError::Task)?
            }
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
        }
    }
}
