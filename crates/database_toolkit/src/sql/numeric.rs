pub trait Numeric: std::fmt::Display {
    //
}

macro_rules! impl_numeric {
    ($($t:ty),*) => {
        $(
            impl Numeric for $t {
                //
            }
        )*
    };
}

impl_numeric!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);
