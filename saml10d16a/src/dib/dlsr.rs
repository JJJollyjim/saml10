#[doc = "Reader of register DLSR"]
pub type R = crate::R<u32, super::DLSR>;
#[doc = "Reader of field `SLI`"]
pub type SLI_R = crate::R<bool, bool>;
#[doc = "Reader of field `SLK`"]
pub type SLK_R = crate::R<bool, bool>;
#[doc = "Reader of field `nTT`"]
pub type NTT_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Software Lock implemented"]
    #[inline(always)]
    pub fn sli(&self) -> SLI_R {
        SLI_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Software Lock status"]
    #[inline(always)]
    pub fn slk(&self) -> SLK_R {
        SLK_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Not thirty-two bit"]
    #[inline(always)]
    pub fn n_tt(&self) -> NTT_R {
        NTT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
