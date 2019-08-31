#[doc = "Reader of register STATUS"]
pub type R = crate::R<u8, super::STATUS>;
#[doc = "Reader of field `RAMINV`"]
pub type RAMINV_R = crate::R<bool, bool>;
#[doc = "Reader of field `DRP`"]
pub type DRP_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - RAM Inversion Bit"]
    #[inline(always)]
    pub fn raminv(&self) -> RAMINV_R {
        RAMINV_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Data Remanence Prevention Ongoing"]
    #[inline(always)]
    pub fn drp(&self) -> DRP_R {
        DRP_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
