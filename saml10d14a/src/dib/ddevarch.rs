#[doc = "Reader of register DDEVARCH"]
pub type R = crate::R<u32, super::DDEVARCH>;
#[doc = "Reader of field `ARCHPART`"]
pub type ARCHPART_R = crate::R<u16, u16>;
#[doc = "Reader of field `ARCHVER`"]
pub type ARCHVER_R = crate::R<u8, u8>;
#[doc = "Reader of field `REVISION`"]
pub type REVISION_R = crate::R<u8, u8>;
#[doc = "Reader of field `PRESENT`"]
pub type PRESENT_R = crate::R<bool, bool>;
#[doc = "Reader of field `ARCHITECT`"]
pub type ARCHITECT_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:11 - Architecture Part"]
    #[inline(always)]
    pub fn archpart(&self) -> ARCHPART_R {
        ARCHPART_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:15 - Architecture Version"]
    #[inline(always)]
    pub fn archver(&self) -> ARCHVER_R {
        ARCHVER_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Revision"]
    #[inline(always)]
    pub fn revision(&self) -> REVISION_R {
        REVISION_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 20 - DEVARCH Present"]
    #[inline(always)]
    pub fn present(&self) -> PRESENT_R {
        PRESENT_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bits 21:31 - Architect"]
    #[inline(always)]
    pub fn architect(&self) -> ARCHITECT_R {
        ARCHITECT_R::new(((self.bits >> 21) & 0x07ff) as u16)
    }
}
