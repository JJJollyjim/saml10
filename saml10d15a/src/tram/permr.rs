#[doc = "Reader of register PERMR"]
pub type R = crate::R<u8, super::PERMR>;
#[doc = "Reader of field `DATA`"]
pub type DATA_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:2 - Permutation Scrambler Data Output"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new((self.bits & 0x07) as u8)
    }
}
