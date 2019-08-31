#[doc = "Reader of register DFLLULPSYNCBUSY"]
pub type R = crate::R<u32, super::DFLLULPSYNCBUSY>;
#[doc = "Reader of field `ENABLE`"]
pub type ENABLE_R = crate::R<bool, bool>;
#[doc = "Reader of field `TUNE`"]
pub type TUNE_R = crate::R<bool, bool>;
#[doc = "Reader of field `DELAY`"]
pub type DELAY_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 1 - Enable Bit Synchronization Busy"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Tune Bit Synchronization Busy"]
    #[inline(always)]
    pub fn tune(&self) -> TUNE_R {
        TUNE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Delay Register Synchronization Busy"]
    #[inline(always)]
    pub fn delay(&self) -> DELAY_R {
        DELAY_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
