mod authorization;
mod basic;
mod bearer;
mod credential;

pub use authorization::*;
pub use basic::*;
pub use bearer::*;
pub use credential::*;

use rocket::{
    http::{HeaderMap, Status},
    request::{FromRequest, Outcome},
    serde::Deserialize,
    Request,
};

pub trait HeaderName: Sized + Send {
    const NAME: &'static str;
}

#[derive(Debug, PartialEq, Eq)]
pub struct Header<T> {
    value: T,
    raw: String,
}

impl<T> std::ops::Deref for Header<T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &T {
        &self.value
    }
}

#[rocket::async_trait]
impl<'r, T> FromRequest<'r> for Header<T>
where
    T: HeaderName + std::str::FromStr,
{
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        match request.headers().try_into() {
            Ok(header) => Outcome::Success(header),
            Err(_) => Outcome::Forward(Status::BadRequest),
        }
    }
}

impl<T> TryFrom<&HeaderMap<'_>> for Header<T>
where
    T: HeaderName + std::str::FromStr,
{
    type Error = ();

    fn try_from(headers: &HeaderMap<'_>) -> Result<Self, Self::Error> {
        let Some(raw) = headers.get(T::NAME).last() else {
            return Err(());
        };
        let Ok(value) = T::from_str(&raw) else {
            return Err(());
        };

        Ok(Header {
            value,
            raw: raw.to_owned(),
        })
    }
}

impl<T> Header<T> {
    #[inline]
    pub fn value(&self) -> &T {
        &self.value
    }

    #[inline]
    pub fn raw(&self) -> &str {
        &self.raw
    }
}

#[cfg(test)]
mod tests {
    use rocket::http::{Header, HeaderMap};

    #[derive(Debug, PartialEq, Eq)]
    struct CustomHeader {
        value: String,
    }

    impl std::str::FromStr for CustomHeader {
        type Err = ();

        fn from_str(source: &str) -> Result<Self, Self::Err> {
            Ok(CustomHeader {
                value: source.to_string(),
            })
        }
    }

    impl super::HeaderName for CustomHeader {
        const NAME: &'static str = "X-Custom-Header";
    }

    #[test]
    fn uncased_key() {
        let headers = {
            let mut headers = HeaderMap::new();
            headers.add(Header::new("x-custom-header", "value"));
            headers
        };
        let header = super::Header::<CustomHeader>::try_from(&headers);

        assert_eq!(
            header,
            Ok(super::Header {
                value: CustomHeader {
                    value: "value".to_string()
                },
                raw: "value".to_string()
            })
        );
    }
}
