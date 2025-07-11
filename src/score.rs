use paste::paste;
use std::ops::{Add, AddAssign, Mul, MulAssign};

// TODO: Consider making the u64 private, `ChipsAllowMul` is toothless otherwise.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Chips(pub u64);

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Mult(pub u64);

macro_rules! op {
    ($trait:ident::$method:ident for $newtype:ident) => {
        impl $trait for $newtype {
            type Output = $newtype;

            fn $method(self, rhs: $newtype) -> $newtype {
                $newtype(self.0.$method(rhs.0))
            }
        }

        impl $trait<u64> for $newtype {
            type Output = $newtype;

            fn $method(self, rhs: u64) -> $newtype {
                $newtype(self.0.$method(rhs))
            }
        }

        paste! {
            impl [<$trait Assign>] for $newtype {
                fn [<$method _assign>](&mut self, rhs: Self) {
                    *self = Self(self.0.$method(rhs.0))
                }
            }

            impl [<$trait Assign>]<u64> for $newtype {
                fn [<$method _assign>](&mut self, rhs: u64) {
                    *self = Self(self.0.$method(rhs))
                }
            }
        }
    };
}

op!(Add::add for Chips);
op!(Add::add for Mult);
op!(Mul::mul for Mult);

// needed for some calculations like calculating planet scaling
// but wrapped in a separate newtype so it can't be done by accident
// since multiplying chips is contrary to how chips typically work
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct ChipsAllowMul(u64);

op!(Mul::mul for ChipsAllowMul);

impl ChipsAllowMul {
    pub(crate) const fn new(chips: Chips) -> Self {
        Self(chips.0)
    }

    pub(crate) const fn finish(self) -> Chips {
        Chips(self.0)
    }
}

#[derive(Debug)]
pub struct ScoreBuilder {
    pub chips: Chips,
    pub mult: Mult,
}
