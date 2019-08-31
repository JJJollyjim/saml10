#[doc = "Reader of register BUSYCH"]
pub type R = crate::R<u32, super::BUSYCH>;
#[doc = "Reader of field `BUSYCH0`"]
pub type BUSYCH0_R = crate::R<bool, bool>;
#[doc = "Reader of field `BUSYCH1`"]
pub type BUSYCH1_R = crate::R<bool, bool>;
#[doc = "Reader of field `BUSYCH2`"]
pub type BUSYCH2_R = crate::R<bool, bool>;
#[doc = "Reader of field `BUSYCH3`"]
pub type BUSYCH3_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Busy Channel 0"]
    #[inline(always)]
    pub fn busych0(&self) -> BUSYCH0_R {
        BUSYCH0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Busy Channel 1"]
    #[inline(always)]
    pub fn busych1(&self) -> BUSYCH1_R {
        BUSYCH1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Busy Channel 2"]
    #[inline(always)]
    pub fn busych2(&self) -> BUSYCH2_R {
        BUSYCH2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Busy Channel 3"]
    #[inline(always)]
    pub fn busych3(&self) -> BUSYCH3_R {
        BUSYCH3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
