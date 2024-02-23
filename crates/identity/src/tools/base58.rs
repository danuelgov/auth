#[derive(Debug)]
pub enum Base58DecodeError {
    Base58,
    Bytes,
}

const ALPHABET: &'static base58::Alphabet = base58::Alphabet::FLICKR;

impl std::fmt::Display for Base58DecodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Base58 => write!(f, "invalid base58"),
            Self::Bytes => write!(f, "invalid bytes"),
        }
    }
}

impl std::error::Error for Base58DecodeError {
    //
}

pub fn encode<S: AsRef<[u8]>>(source: S) -> String {
    base58::encode(source.as_ref())
        .with_alphabet(ALPHABET)
        .into_string()
}

pub fn decode<S: AsRef<[u8]>>(source: S) -> Result<u128, Base58DecodeError> {
    let bytes = base58::decode(source.as_ref())
        .with_alphabet(ALPHABET)
        .into_vec()
        .map_err(|_| Base58DecodeError::Base58)?;
    let bytes = bytes.try_into().map_err(|_| Base58DecodeError::Bytes)?;

    Ok(u128::from_be_bytes(bytes))
}
