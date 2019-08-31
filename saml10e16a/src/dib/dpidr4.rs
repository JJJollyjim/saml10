#[doc = "Reader of register DPIDR4"]
pub type R = crate::R<u32, super::DPIDR4>;
#[doc = "Reader of field `DES_2`"]
pub type DES_2_R = crate::R<u8, u8>;
#[doc = "Reader of field `SIZE`"]
pub type SIZE_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - JEP106 continuation code"]
    #[inline(always)]
    pub fn des_2(&self) -> DES_2_R {
        DES_2_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - 4KB count"]
    #[inline(always)]
    pub fn size(&self) -> SIZE_R {
        SIZE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
