pub mod bubble_bucket;
pub mod detect_cycles;
pub mod frontends;
mod size_expander;
pub use size_expander::size_expander;
mod time_on_air;
pub use time_on_air::{Bandwidth, LoRa};
pub mod gui;
pub mod rf;
pub mod wakan;

pub type Number = ordered_f32::OrderedF32;

#[macro_export]
macro_rules! impl_ops_via_intermediary {
    ($T:ty, $Intermediary:ty) => {
        use std::ops::{
            Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign,
        };

        impl<T> Add<T> for $T
        where
            T: Into<$Intermediary>,
            $T: Into<$Intermediary>,
            $Intermediary: Add<Output = $Intermediary>,
            $Intermediary: Into<$T>,
        {
            type Output = $T;

            fn add(self, rhs: T) -> Self::Output {
                (Into::<$Intermediary>::into(self) + rhs.into()).into()
            }
        }

        impl<T> Add<T> for &$T
        where
            T: Into<$Intermediary>,
            $T: Into<$Intermediary>,
            $Intermediary: Add<Output = $Intermediary>,
            $Intermediary: Into<$T>,
        {
            type Output = $T;

            fn add(self, rhs: T) -> Self::Output {
                (Into::<$Intermediary>::into(self) + rhs.into()).into()
            }
        }

        impl<T> AddAssign<T> for $T
        where
            T: Into<$Intermediary>,
            $T: Into<$Intermediary>,
            $Intermediary: Add<Output = $Intermediary>,
            $Intermediary: Into<$T>,
        {
            fn add_assign(&mut self, rhs: T) {
                let x = (Into::<$Intermediary>::into(self.clone()) + rhs.into()).into();
                let _ = std::mem::replace(self, x);
            }
        }

        impl<T> Div<T> for $T
        where
            T: Into<$Intermediary>,
            $T: Into<$Intermediary>,
            $Intermediary: Div<Output = $Intermediary>,
            $Intermediary: Into<$T>,
        {
            type Output = $T;

            fn div(self, rhs: T) -> Self::Output {
                (Into::<$Intermediary>::into(self) / rhs.into()).into()
            }
        }

        impl<T> Div<T> for &$T
        where
            T: Into<$Intermediary>,
            $T: Into<$Intermediary>,
            $Intermediary: Div<Output = $Intermediary>,
            $Intermediary: Into<$T>,
        {
            type Output = $T;

            fn div(self, rhs: T) -> Self::Output {
                (Into::<$Intermediary>::into(self) / rhs.into()).into()
            }
        }

        impl<T> DivAssign<T> for $T
        where
            T: Into<$Intermediary>,
            $T: Into<$Intermediary>,
            $Intermediary: Div<Output = $Intermediary>,
            $Intermediary: Into<$T>,
        {
            fn div_assign(&mut self, rhs: T) {
                let x = (Into::<$Intermediary>::into(self.clone()) / rhs.into()).into();
                let _ = std::mem::replace(self, x);
            }
        }

        impl<T> Mul<T> for $T
        where
            T: Into<$Intermediary>,
            $T: Into<$Intermediary>,
            $Intermediary: Mul<Output = $Intermediary>,
            $Intermediary: Into<$T>,
        {
            type Output = $T;

            fn mul(self, rhs: T) -> Self::Output {
                (Into::<$Intermediary>::into(self) * rhs.into()).into()
            }
        }

        impl<T> Mul<T> for &$T
        where
            T: Into<$Intermediary>,
            $T: Into<$Intermediary>,
            $Intermediary: Mul<Output = $Intermediary>,
            $Intermediary: Into<$T>,
        {
            type Output = $T;

            fn mul(self, rhs: T) -> Self::Output {
                (Into::<$Intermediary>::into(self) * rhs.into()).into()
            }
        }

        impl<T> MulAssign<T> for $T
        where
            T: Into<$Intermediary>,
            $T: Into<$Intermediary>,
            $Intermediary: Mul<Output = $Intermediary>,
            $Intermediary: Into<$T>,
        {
            fn mul_assign(&mut self, rhs: T) {
                let x = (Into::<$Intermediary>::into(self.clone()) * rhs.into()).into();
                let _ = std::mem::replace(self, x);
            }
        }

        impl Neg for $T {
            type Output = $T;

            fn neg(self) -> Self::Output {
                Into::<$Intermediary>::into(self).neg().into()
            }
        }

        impl Neg for &$T {
            type Output = $T;

            fn neg(self) -> Self::Output {
                Into::<$Intermediary>::into(self).neg().into()
            }
        }

        impl<T> Rem<T> for $T
        where
            T: Into<$Intermediary>,
            $T: Into<$Intermediary>,
            $Intermediary: Rem<Output = $Intermediary>,
            $Intermediary: Into<$T>,
        {
            type Output = $T;

            fn rem(self, rhs: T) -> Self::Output {
                (Into::<$Intermediary>::into(self) % rhs.into()).into()
            }
        }

        impl<T> Rem<T> for &$T
        where
            T: Into<$Intermediary>,
            $T: Into<$Intermediary>,
            $Intermediary: Rem<Output = $Intermediary>,
            $Intermediary: Into<$T>,
        {
            type Output = $T;

            fn rem(self, rhs: T) -> Self::Output {
                (Into::<$Intermediary>::into(self) % rhs.into()).into()
            }
        }

        impl<T> RemAssign<T> for $T
        where
            T: Into<$Intermediary>,
            $T: Into<$Intermediary>,
            $Intermediary: Rem<Output = $Intermediary>,
            $Intermediary: Into<$T>,
        {
            fn rem_assign(&mut self, rhs: T) {
                let x = (Into::<$Intermediary>::into(self.clone()) % rhs.into()).into();
                let _ = std::mem::replace(self, x);
            }
        }

        impl<T> Sub<T> for $T
        where
            T: Into<$Intermediary>,
            $T: Into<$Intermediary>,
            $Intermediary: Sub<Output = $Intermediary>,
            $Intermediary: Into<$T>,
        {
            type Output = $T;

            fn sub(self, rhs: T) -> Self::Output {
                (Into::<$Intermediary>::into(self) - rhs.into()).into()
            }
        }

        impl<T> Sub<T> for &$T
        where
            T: Into<$Intermediary>,
            $T: Into<$Intermediary>,
            $Intermediary: Sub<Output = $Intermediary>,
            $Intermediary: Into<$T>,
        {
            type Output = $T;

            fn sub(self, rhs: T) -> Self::Output {
                (Into::<$Intermediary>::into(self) - rhs.into()).into()
            }
        }

        impl<T> SubAssign<T> for $T
        where
            T: Into<$Intermediary>,
            $T: Into<$Intermediary>,
            $Intermediary: Sub<Output = $Intermediary>,
            $Intermediary: Into<$T>,
        {
            fn sub_assign(&mut self, rhs: T) {
                let x = (Into::<$Intermediary>::into(self.clone()) - rhs.into()).into();
                let _ = std::mem::replace(self, x);
            }
        }
    };
}
