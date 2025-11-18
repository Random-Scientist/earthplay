macro_rules! bitflag_bits {
    {
        $( #[$attr:meta] )*
        $v:vis struct $name:ident: $t:ident bits: {
            $(
                $( #[doc = $doc:literal] )*
                $bit_name:ident: $bit:expr
            ),+ $(,)?
        }
    } => {
        ::bitflags::bitflags! {
            $(#[$attr])*
            $v struct $name: $t {
                $(
                    $( #[doc = $doc] )*
                    const $bit_name = 1 << $bit);+
                ;
            }
        }
    }
}

pub(crate) use bitflag_bits;
