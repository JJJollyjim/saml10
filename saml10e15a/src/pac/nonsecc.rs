#[doc = "Reader of register NONSECC"]
pub type R = crate::R<u32, super::NONSECC>;
#[doc = "Reader of field `EVSYS_`"]
pub type EVSYS__R = crate::R<bool, bool>;
#[doc = "Reader of field `SERCOM0_`"]
pub type SERCOM0__R = crate::R<bool, bool>;
#[doc = "Reader of field `SERCOM1_`"]
pub type SERCOM1__R = crate::R<bool, bool>;
#[doc = "Reader of field `SERCOM2_`"]
pub type SERCOM2__R = crate::R<bool, bool>;
#[doc = "Reader of field `TC0_`"]
pub type TC0__R = crate::R<bool, bool>;
#[doc = "Reader of field `TC1_`"]
pub type TC1__R = crate::R<bool, bool>;
#[doc = "Reader of field `TC2_`"]
pub type TC2__R = crate::R<bool, bool>;
#[doc = "Reader of field `ADC_`"]
pub type ADC__R = crate::R<bool, bool>;
#[doc = "Reader of field `DAC_`"]
pub type DAC__R = crate::R<bool, bool>;
#[doc = "Reader of field `PTC_`"]
pub type PTC__R = crate::R<bool, bool>;
#[doc = "Reader of field `TRNG_`"]
pub type TRNG__R = crate::R<bool, bool>;
#[doc = "Reader of field `CCL_`"]
pub type CCL__R = crate::R<bool, bool>;
#[doc = "Reader of field `OPAMP_`"]
pub type OPAMP__R = crate::R<bool, bool>;
#[doc = "Reader of field `TRAM_`"]
pub type TRAM__R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - EVSYS Non-Secure"]
    #[inline(always)]
    pub fn evsys_(&self) -> EVSYS__R {
        EVSYS__R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - SERCOM0 Non-Secure"]
    #[inline(always)]
    pub fn sercom0_(&self) -> SERCOM0__R {
        SERCOM0__R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - SERCOM1 Non-Secure"]
    #[inline(always)]
    pub fn sercom1_(&self) -> SERCOM1__R {
        SERCOM1__R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - SERCOM2 Non-Secure"]
    #[inline(always)]
    pub fn sercom2_(&self) -> SERCOM2__R {
        SERCOM2__R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - TC0 Non-Secure"]
    #[inline(always)]
    pub fn tc0_(&self) -> TC0__R {
        TC0__R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - TC1 Non-Secure"]
    #[inline(always)]
    pub fn tc1_(&self) -> TC1__R {
        TC1__R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - TC2 Non-Secure"]
    #[inline(always)]
    pub fn tc2_(&self) -> TC2__R {
        TC2__R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - ADC Non-Secure"]
    #[inline(always)]
    pub fn adc_(&self) -> ADC__R {
        ADC__R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - DAC Non-Secure"]
    #[inline(always)]
    pub fn dac_(&self) -> DAC__R {
        DAC__R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - PTC Non-Secure"]
    #[inline(always)]
    pub fn ptc_(&self) -> PTC__R {
        PTC__R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - TRNG Non-Secure"]
    #[inline(always)]
    pub fn trng_(&self) -> TRNG__R {
        TRNG__R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - CCL Non-Secure"]
    #[inline(always)]
    pub fn ccl_(&self) -> CCL__R {
        CCL__R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - OPAMP Non-Secure"]
    #[inline(always)]
    pub fn opamp_(&self) -> OPAMP__R {
        OPAMP__R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - TRAM Non-Secure"]
    #[inline(always)]
    pub fn tram_(&self) -> TRAM__R {
        TRAM__R::new(((self.bits >> 13) & 0x01) != 0)
    }
}
