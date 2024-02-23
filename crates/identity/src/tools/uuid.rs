use uuid::Uuid;

#[inline]
pub fn new_v4() -> u128 {
    Uuid::new_v4().as_u128()
}

#[inline]
pub fn new_v7() -> u128 {
    Uuid::now_v7().as_u128()
}

#[inline]
pub const fn is_v4(id: u128) -> bool {
    matches!(
        Uuid::from_u128(id).get_version(),
        Some(uuid::Version::Random)
    )
}

#[inline]
pub const fn is_v7(id: u128) -> bool {
    matches!(
        Uuid::from_u128(id).get_version(),
        Some(uuid::Version::SortRand)
    )
}
