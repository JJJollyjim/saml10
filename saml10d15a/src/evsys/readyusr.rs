#[doc = "Reader of register READYUSR"]
pub type R = crate::R<u32, super::READYUSR>;
#[doc = "Reader of field `READYUSR0`"]
pub type READYUSR0_R = crate::R<bool, bool>;
#[doc = "Reader of field `READYUSR1`"]
pub type READYUSR1_R = crate::R<bool, bool>;
#[doc = "Reader of field `READYUSR2`"]
pub type READYUSR2_R = crate::R<bool, bool>;
#[doc = "Reader of field `READYUSR3`"]
pub type READYUSR3_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Ready User for Channel 0"]
    #[inline(always)]
    pub fn readyusr0(&self) -> READYUSR0_R {
        READYUSR0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Ready User for Channel 1"]
    #[inline(always)]
    pub fn readyusr1(&self) -> READYUSR1_R {
        READYUSR1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Ready User for Channel 2"]
    #[inline(always)]
    pub fn readyusr2(&self) -> READYUSR2_R {
        READYUSR2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Ready User for Channel 3"]
    #[inline(always)]
    pub fn readyusr3(&self) -> READYUSR3_R {
        READYUSR3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
