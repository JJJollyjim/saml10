#[doc = "Reader of register CTRLB"]
pub type R = crate::R<u32, super::CTRLB>;
#[doc = "Writer for register CTRLB"]
pub type W = crate::W<u32, super::CTRLB>;
#[doc = "Register CTRLB `reset()`'s with value 0"]
impl crate::ResetValue for super::CTRLB {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RWS`"]
pub type RWS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RWS`"]
pub struct RWS_W<'a> {
    w: &'a mut W,
}
impl<'a> RWS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 1)) | (((value as u32) & 0x0f) << 1);
        self.w
    }
}
#[doc = "Possible values of the field `SLEEPPRM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLEEPPRM_A {
    #[doc = "NVM block enters low-power mode when entering sleep.NVM block exits low-power mode upon first access."]
    WAKEONACCESS,
    #[doc = "NVM block enters low-power mode when entering sleep.NVM block exits low-power mode when exiting sleep."]
    WAKEUPINSTANT,
    #[doc = "Auto power reduction disabled."]
    DISABLED,
}
impl crate::ToBits<u8> for SLEEPPRM_A {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            SLEEPPRM_A::WAKEONACCESS => 0,
            SLEEPPRM_A::WAKEUPINSTANT => 1,
            SLEEPPRM_A::DISABLED => 3,
        }
    }
}
#[doc = "Reader of field `SLEEPPRM`"]
pub type SLEEPPRM_R = crate::R<u8, SLEEPPRM_A>;
impl SLEEPPRM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SLEEPPRM_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SLEEPPRM_A::WAKEONACCESS),
            1 => Val(SLEEPPRM_A::WAKEUPINSTANT),
            3 => Val(SLEEPPRM_A::DISABLED),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `WAKEONACCESS`"]
    #[inline(always)]
    pub fn is_wakeonaccess(&self) -> bool {
        *self == SLEEPPRM_A::WAKEONACCESS
    }
    #[doc = "Checks if the value of the field is `WAKEUPINSTANT`"]
    #[inline(always)]
    pub fn is_wakeupinstant(&self) -> bool {
        *self == SLEEPPRM_A::WAKEUPINSTANT
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SLEEPPRM_A::DISABLED
    }
}
#[doc = "Write proxy for field `SLEEPPRM`"]
pub struct SLEEPPRM_W<'a> {
    w: &'a mut W,
}
impl<'a> SLEEPPRM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SLEEPPRM_A) -> &'a mut W {
        use crate::ToBits;
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "NVM block enters low-power mode when entering sleep.NVM block exits low-power mode upon first access."]
    #[inline(always)]
    pub fn wakeonaccess(self) -> &'a mut W {
        self.variant(SLEEPPRM_A::WAKEONACCESS)
    }
    #[doc = "NVM block enters low-power mode when entering sleep.NVM block exits low-power mode when exiting sleep."]
    #[inline(always)]
    pub fn wakeupinstant(self) -> &'a mut W {
        self.variant(SLEEPPRM_A::WAKEUPINSTANT)
    }
    #[doc = "Auto power reduction disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SLEEPPRM_A::DISABLED)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `FWUP`"]
pub type FWUP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FWUP`"]
pub struct FWUP_W<'a> {
    w: &'a mut W,
}
impl<'a> FWUP_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Possible values of the field `READMODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum READMODE_A {
    #[doc = "The NVM Controller (cache system) does not insert wait states on a cache miss. Gives the best system performance."]
    NO_MISS_PENALTY,
    #[doc = "Reduces power consumption of the cache system, but inserts a wait state each time there is a cache miss. This mode may not be relevant if CPU performance is required, as the application will be stalled and may lead to increase run time."]
    LOW_POWER,
    #[doc = "The cache system ensures that a cache hit or miss takes the same amount of time, determined by the number of programmed flash wait states. This mode can be used for real-time applications that require deterministic execution timings."]
    DETERMINISTIC,
}
impl crate::ToBits<u8> for READMODE_A {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            READMODE_A::NO_MISS_PENALTY => 0,
            READMODE_A::LOW_POWER => 1,
            READMODE_A::DETERMINISTIC => 2,
        }
    }
}
#[doc = "Reader of field `READMODE`"]
pub type READMODE_R = crate::R<u8, READMODE_A>;
impl READMODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, READMODE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(READMODE_A::NO_MISS_PENALTY),
            1 => Val(READMODE_A::LOW_POWER),
            2 => Val(READMODE_A::DETERMINISTIC),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NO_MISS_PENALTY`"]
    #[inline(always)]
    pub fn is_no_miss_penalty(&self) -> bool {
        *self == READMODE_A::NO_MISS_PENALTY
    }
    #[doc = "Checks if the value of the field is `LOW_POWER`"]
    #[inline(always)]
    pub fn is_low_power(&self) -> bool {
        *self == READMODE_A::LOW_POWER
    }
    #[doc = "Checks if the value of the field is `DETERMINISTIC`"]
    #[inline(always)]
    pub fn is_deterministic(&self) -> bool {
        *self == READMODE_A::DETERMINISTIC
    }
}
#[doc = "Write proxy for field `READMODE`"]
pub struct READMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> READMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: READMODE_A) -> &'a mut W {
        use crate::ToBits;
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "The NVM Controller (cache system) does not insert wait states on a cache miss. Gives the best system performance."]
    #[inline(always)]
    pub fn no_miss_penalty(self) -> &'a mut W {
        self.variant(READMODE_A::NO_MISS_PENALTY)
    }
    #[doc = "Reduces power consumption of the cache system, but inserts a wait state each time there is a cache miss. This mode may not be relevant if CPU performance is required, as the application will be stalled and may lead to increase run time."]
    #[inline(always)]
    pub fn low_power(self) -> &'a mut W {
        self.variant(READMODE_A::LOW_POWER)
    }
    #[doc = "The cache system ensures that a cache hit or miss takes the same amount of time, determined by the number of programmed flash wait states. This mode can be used for real-time applications that require deterministic execution timings."]
    #[inline(always)]
    pub fn deterministic(self) -> &'a mut W {
        self.variant(READMODE_A::DETERMINISTIC)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Reader of field `CACHEDIS`"]
pub type CACHEDIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CACHEDIS`"]
pub struct CACHEDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> CACHEDIS_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `QWEN`"]
pub type QWEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `QWEN`"]
pub struct QWEN_W<'a> {
    w: &'a mut W,
}
impl<'a> QWEN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
impl R {
    #[doc = "Bits 1:4 - NVM Read Wait States"]
    #[inline(always)]
    pub fn rws(&self) -> RWS_R {
        RWS_R::new(((self.bits >> 1) & 0x0f) as u8)
    }
    #[doc = "Bits 8:9 - Power Reduction Mode during Sleep"]
    #[inline(always)]
    pub fn sleepprm(&self) -> SLEEPPRM_R {
        SLEEPPRM_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bit 11 - fast wake-up"]
    #[inline(always)]
    pub fn fwup(&self) -> FWUP_R {
        FWUP_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 16:17 - NVMCTRL Read Mode"]
    #[inline(always)]
    pub fn readmode(&self) -> READMODE_R {
        READMODE_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bit 18 - Cache Disable"]
    #[inline(always)]
    pub fn cachedis(&self) -> CACHEDIS_R {
        CACHEDIS_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Quick Write Enable"]
    #[inline(always)]
    pub fn qwen(&self) -> QWEN_R {
        QWEN_R::new(((self.bits >> 19) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 1:4 - NVM Read Wait States"]
    #[inline(always)]
    pub fn rws(&mut self) -> RWS_W {
        RWS_W { w: self }
    }
    #[doc = "Bits 8:9 - Power Reduction Mode during Sleep"]
    #[inline(always)]
    pub fn sleepprm(&mut self) -> SLEEPPRM_W {
        SLEEPPRM_W { w: self }
    }
    #[doc = "Bit 11 - fast wake-up"]
    #[inline(always)]
    pub fn fwup(&mut self) -> FWUP_W {
        FWUP_W { w: self }
    }
    #[doc = "Bits 16:17 - NVMCTRL Read Mode"]
    #[inline(always)]
    pub fn readmode(&mut self) -> READMODE_W {
        READMODE_W { w: self }
    }
    #[doc = "Bit 18 - Cache Disable"]
    #[inline(always)]
    pub fn cachedis(&mut self) -> CACHEDIS_W {
        CACHEDIS_W { w: self }
    }
    #[doc = "Bit 19 - Quick Write Enable"]
    #[inline(always)]
    pub fn qwen(&mut self) -> QWEN_W {
        QWEN_W { w: self }
    }
}
