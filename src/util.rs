use core::num::NonZeroU8;

#[derive(Debug, Clone, Copy)]
pub struct NonValueU8<const FORBIDDEN: u8>(NonZeroU8);
impl<const FORBIDDEN: u8> NonValueU8<FORBIDDEN> {
    #[allow(unsafe_code)]
    pub const unsafe fn new_unchecked(v: u8) -> Self {
        Self(unsafe { NonZeroU8::new_unchecked(v ^ FORBIDDEN) })
    }
    pub const fn get(self) -> u8 {
        self.get() ^ FORBIDDEN
    }
}
