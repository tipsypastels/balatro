mod jimbo;
mod misprint;
mod stencil;

pub use jimbo::*;
pub use misprint::*;
pub use stencil::*;

#[allow(unused)]
mod prelude {
    pub use crate::*;
    pub use rand::Rng;

    // TODO: Improve ergonomics a lot.
    #[cfg(test)]
    macro_rules! jokers {
        ($size:literal: $($kind:expr),*$(,)?) => {{
            let mut slate = Slate::<Joker>::new($size);

            $(
                slate.push(Joker::new($kind)).unwrap();
            )*

            slate
        }};
    }

    #[cfg(test)]
    pub(crate) use jokers;
}
