#[doc = "Reader of register SYNCBUSY"]
pub type R = crate::R<u32, super::SYNCBUSY>;
#[doc = "Reader of field `SWRST`"]
pub type SWRST_R = crate::R<bool, bool>;
#[doc = "Reader of field `GENCTRL0`"]
pub type GENCTRL0_R = crate::R<bool, bool>;
#[doc = "Reader of field `GENCTRL1`"]
pub type GENCTRL1_R = crate::R<bool, bool>;
#[doc = "Reader of field `GENCTRL2`"]
pub type GENCTRL2_R = crate::R<bool, bool>;
#[doc = "Reader of field `GENCTRL3`"]
pub type GENCTRL3_R = crate::R<bool, bool>;
#[doc = "Reader of field `GENCTRL4`"]
pub type GENCTRL4_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Software Reset Synchroniation Busy bit"]
    #[inline(always)]
    pub fn swrst(&self) -> SWRST_R {
        SWRST_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - Generic Clock Generator Control 0 Synchronization Busy bit"]
    #[inline(always)]
    pub fn genctrl0(&self) -> GENCTRL0_R {
        GENCTRL0_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Generic Clock Generator Control 1 Synchronization Busy bit"]
    #[inline(always)]
    pub fn genctrl1(&self) -> GENCTRL1_R {
        GENCTRL1_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Generic Clock Generator Control 2 Synchronization Busy bit"]
    #[inline(always)]
    pub fn genctrl2(&self) -> GENCTRL2_R {
        GENCTRL2_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Generic Clock Generator Control 3 Synchronization Busy bit"]
    #[inline(always)]
    pub fn genctrl3(&self) -> GENCTRL3_R {
        GENCTRL3_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Generic Clock Generator Control 4 Synchronization Busy bit"]
    #[inline(always)]
    pub fn genctrl4(&self) -> GENCTRL4_R {
        GENCTRL4_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
