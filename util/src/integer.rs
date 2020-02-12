pub trait Integer : Sized
    + Copy
    + std::ops::Add<Self, Output=Self>
    + std::ops::Sub<Self, Output=Self>
    + std::ops::Mul<Self, Output=Self>
    + std::ops::Div<Self, Output=Self>
    + std::ops::Rem<Self, Output=Self>
    + std::ops::AddAssign<Self>
    + std::ops::SubAssign<Self>
    + std::ops::MulAssign<Self>
    + std::ops::DivAssign<Self>
    + std::ops::RemAssign<Self>
    + std::cmp::Eq
{
    fn zero() -> Self;
}

macro_rules! impl_integer {
    ($t:ty) => { impl Integer for $t {
        fn zero() -> Self {
            0
        }
    } };
    ($first:ty, $($rest:ty),+) => {
        impl_integer!($first);
        impl_integer!($($rest),+);
    };
}

impl_integer!(u8, u16, u32, u64, usize,i8, i16, i32, i64, isize);


macro_rules! impl_marker_trait {
    ($trait:ty, $t:ty) => { impl $trait for $t { } };
    ($trait:ty, $first:ty, $($rest:ty),+) => {
        impl_marker_trait!($trait, $first);
        impl_marker_trait!($trait, $($rest),+);
    };
}

pub trait SignedInteger : Integer { }
pub trait UnsignedInteger : Integer { }

impl_marker_trait!(SignedInteger, i8, i16, i32, i64, isize);
impl_marker_trait!(UnsignedInteger, u8, u16, u32, u64, usize);