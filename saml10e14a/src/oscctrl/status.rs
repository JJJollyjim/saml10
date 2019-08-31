#[doc = "Reader of register STATUS"]
pub type R = crate::R<u32, super::STATUS>;
#[doc = "Reader of field `XOSCRDY`"]
pub type XOSCRDY_R = crate::R<bool, bool>;
#[doc = "Reader of field `XOSCFAIL`"]
pub type XOSCFAIL_R = crate::R<bool, bool>;
#[doc = "Reader of field `XOSCCKSW`"]
pub type XOSCCKSW_R = crate::R<bool, bool>;
#[doc = "Reader of field `OSC16MRDY`"]
pub type OSC16MRDY_R = crate::R<bool, bool>;
#[doc = "Reader of field `DFLLULPRDY`"]
pub type DFLLULPRDY_R = crate::R<bool, bool>;
#[doc = "Reader of field `DFLLULPLOCK`"]
pub type DFLLULPLOCK_R = crate::R<bool, bool>;
#[doc = "Reader of field `DFLLULPNOLOCK`"]
pub type DFLLULPNOLOCK_R = crate::R<bool, bool>;
#[doc = "Reader of field `DPLLLCKR`"]
pub type DPLLLCKR_R = crate::R<bool, bool>;
#[doc = "Reader of field `DPLLLCKF`"]
pub type DPLLLCKF_R = crate::R<bool, bool>;
#[doc = "Reader of field `DPLLTO`"]
pub type DPLLTO_R = crate::R<bool, bool>;
#[doc = "Reader of field `DPLLLDRTO`"]
pub type DPLLLDRTO_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - XOSC Ready"]
    #[inline(always)]
    pub fn xoscrdy(&self) -> XOSCRDY_R {
        XOSCRDY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - XOSC Clock Failure Detector"]
    #[inline(always)]
    pub fn xoscfail(&self) -> XOSCFAIL_R {
        XOSCFAIL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - XOSC Clock Switch"]
    #[inline(always)]
    pub fn xosccksw(&self) -> XOSCCKSW_R {
        XOSCCKSW_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - OSC16M Ready"]
    #[inline(always)]
    pub fn osc16mrdy(&self) -> OSC16MRDY_R {
        OSC16MRDY_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 8 - DFLLULP Ready"]
    #[inline(always)]
    pub fn dfllulprdy(&self) -> DFLLULPRDY_R {
        DFLLULPRDY_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - DFLLULP Lock"]
    #[inline(always)]
    pub fn dfllulplock(&self) -> DFLLULPLOCK_R {
        DFLLULPLOCK_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - DFLLULP No Lock"]
    #[inline(always)]
    pub fn dfllulpnolock(&self) -> DFLLULPNOLOCK_R {
        DFLLULPNOLOCK_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 16 - DPLL Lock Rise"]
    #[inline(always)]
    pub fn dplllckr(&self) -> DPLLLCKR_R {
        DPLLLCKR_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - DPLL Lock Fall"]
    #[inline(always)]
    pub fn dplllckf(&self) -> DPLLLCKF_R {
        DPLLLCKF_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - DPLL Lock Timeout"]
    #[inline(always)]
    pub fn dpllto(&self) -> DPLLTO_R {
        DPLLTO_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - DPLL Loop Divider Ratio Update Complete"]
    #[inline(always)]
    pub fn dpllldrto(&self) -> DPLLLDRTO_R {
        DPLLLDRTO_R::new(((self.bits >> 19) & 0x01) != 0)
    }
}
