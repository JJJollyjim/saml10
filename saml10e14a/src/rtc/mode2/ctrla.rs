#[doc = "Reader of register CTRLA"]
pub type R = crate::R<u16, super::CTRLA>;
#[doc = "Writer for register CTRLA"]
pub type W = crate::W<u16, super::CTRLA>;
#[doc = "Register CTRLA `reset()`'s with value 0"]
impl crate::ResetValue for super::CTRLA {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SWRST`"]
pub type SWRST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SWRST`"]
pub struct SWRST_W<'a> {
    w: &'a mut W,
}
impl<'a> SWRST_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u16) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `ENABLE`"]
pub type ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENABLE`"]
pub struct ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u16) & 0x01) << 1);
        self.w
    }
}
#[doc = "Possible values of the field `MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODE_A {
    #[doc = "Mode 0: 32-bit Counter"]
    COUNT32,
    #[doc = "Mode 1: 16-bit Counter"]
    COUNT16,
    #[doc = "Mode 2: Clock/Calendar"]
    CLOCK,
}
impl crate::ToBits<u8> for MODE_A {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            MODE_A::COUNT32 => 0,
            MODE_A::COUNT16 => 1,
            MODE_A::CLOCK => 2,
        }
    }
}
#[doc = "Reader of field `MODE`"]
pub type MODE_R = crate::R<u8, MODE_A>;
impl MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, MODE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(MODE_A::COUNT32),
            1 => Val(MODE_A::COUNT16),
            2 => Val(MODE_A::CLOCK),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `COUNT32`"]
    #[inline(always)]
    pub fn is_count32(&self) -> bool {
        *self == MODE_A::COUNT32
    }
    #[doc = "Checks if the value of the field is `COUNT16`"]
    #[inline(always)]
    pub fn is_count16(&self) -> bool {
        *self == MODE_A::COUNT16
    }
    #[doc = "Checks if the value of the field is `CLOCK`"]
    #[inline(always)]
    pub fn is_clock(&self) -> bool {
        *self == MODE_A::CLOCK
    }
}
#[doc = "Write proxy for field `MODE`"]
pub struct MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODE_A) -> &'a mut W {
        use crate::ToBits;
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Mode 0: 32-bit Counter"]
    #[inline(always)]
    pub fn count32(self) -> &'a mut W {
        self.variant(MODE_A::COUNT32)
    }
    #[doc = "Mode 1: 16-bit Counter"]
    #[inline(always)]
    pub fn count16(self) -> &'a mut W {
        self.variant(MODE_A::COUNT16)
    }
    #[doc = "Mode 2: Clock/Calendar"]
    #[inline(always)]
    pub fn clock(self) -> &'a mut W {
        self.variant(MODE_A::CLOCK)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u16) & 0x03) << 2);
        self.w
    }
}
#[doc = "Reader of field `CLKREP`"]
pub type CLKREP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLKREP`"]
pub struct CLKREP_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKREP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u16) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `MATCHCLR`"]
pub type MATCHCLR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MATCHCLR`"]
pub struct MATCHCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> MATCHCLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u16) & 0x01) << 7);
        self.w
    }
}
#[doc = "Possible values of the field `PRESCALER`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRESCALER_A {
    #[doc = "CLK_RTC_CNT = GCLK_RTC/1"]
    OFF,
    #[doc = "CLK_RTC_CNT = GCLK_RTC/1"]
    DIV1,
    #[doc = "CLK_RTC_CNT = GCLK_RTC/2"]
    DIV2,
    #[doc = "CLK_RTC_CNT = GCLK_RTC/4"]
    DIV4,
    #[doc = "CLK_RTC_CNT = GCLK_RTC/8"]
    DIV8,
    #[doc = "CLK_RTC_CNT = GCLK_RTC/16"]
    DIV16,
    #[doc = "CLK_RTC_CNT = GCLK_RTC/32"]
    DIV32,
    #[doc = "CLK_RTC_CNT = GCLK_RTC/64"]
    DIV64,
    #[doc = "CLK_RTC_CNT = GCLK_RTC/128"]
    DIV128,
    #[doc = "CLK_RTC_CNT = GCLK_RTC/256"]
    DIV256,
    #[doc = "CLK_RTC_CNT = GCLK_RTC/512"]
    DIV512,
    #[doc = "CLK_RTC_CNT = GCLK_RTC/1024"]
    DIV1024,
}
impl crate::ToBits<u8> for PRESCALER_A {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            PRESCALER_A::OFF => 0,
            PRESCALER_A::DIV1 => 1,
            PRESCALER_A::DIV2 => 2,
            PRESCALER_A::DIV4 => 3,
            PRESCALER_A::DIV8 => 4,
            PRESCALER_A::DIV16 => 5,
            PRESCALER_A::DIV32 => 6,
            PRESCALER_A::DIV64 => 7,
            PRESCALER_A::DIV128 => 8,
            PRESCALER_A::DIV256 => 9,
            PRESCALER_A::DIV512 => 10,
            PRESCALER_A::DIV1024 => 11,
        }
    }
}
#[doc = "Reader of field `PRESCALER`"]
pub type PRESCALER_R = crate::R<u8, PRESCALER_A>;
impl PRESCALER_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PRESCALER_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PRESCALER_A::OFF),
            1 => Val(PRESCALER_A::DIV1),
            2 => Val(PRESCALER_A::DIV2),
            3 => Val(PRESCALER_A::DIV4),
            4 => Val(PRESCALER_A::DIV8),
            5 => Val(PRESCALER_A::DIV16),
            6 => Val(PRESCALER_A::DIV32),
            7 => Val(PRESCALER_A::DIV64),
            8 => Val(PRESCALER_A::DIV128),
            9 => Val(PRESCALER_A::DIV256),
            10 => Val(PRESCALER_A::DIV512),
            11 => Val(PRESCALER_A::DIV1024),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == PRESCALER_A::OFF
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == PRESCALER_A::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PRESCALER_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == PRESCALER_A::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == PRESCALER_A::DIV8
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == PRESCALER_A::DIV16
    }
    #[doc = "Checks if the value of the field is `DIV32`"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == PRESCALER_A::DIV32
    }
    #[doc = "Checks if the value of the field is `DIV64`"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == PRESCALER_A::DIV64
    }
    #[doc = "Checks if the value of the field is `DIV128`"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == PRESCALER_A::DIV128
    }
    #[doc = "Checks if the value of the field is `DIV256`"]
    #[inline(always)]
    pub fn is_div256(&self) -> bool {
        *self == PRESCALER_A::DIV256
    }
    #[doc = "Checks if the value of the field is `DIV512`"]
    #[inline(always)]
    pub fn is_div512(&self) -> bool {
        *self == PRESCALER_A::DIV512
    }
    #[doc = "Checks if the value of the field is `DIV1024`"]
    #[inline(always)]
    pub fn is_div1024(&self) -> bool {
        *self == PRESCALER_A::DIV1024
    }
}
#[doc = "Write proxy for field `PRESCALER`"]
pub struct PRESCALER_W<'a> {
    w: &'a mut W,
}
impl<'a> PRESCALER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PRESCALER_A) -> &'a mut W {
        use crate::ToBits;
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "CLK_RTC_CNT = GCLK_RTC/1"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(PRESCALER_A::OFF)
    }
    #[doc = "CLK_RTC_CNT = GCLK_RTC/1"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(PRESCALER_A::DIV1)
    }
    #[doc = "CLK_RTC_CNT = GCLK_RTC/2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(PRESCALER_A::DIV2)
    }
    #[doc = "CLK_RTC_CNT = GCLK_RTC/4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(PRESCALER_A::DIV4)
    }
    #[doc = "CLK_RTC_CNT = GCLK_RTC/8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(PRESCALER_A::DIV8)
    }
    #[doc = "CLK_RTC_CNT = GCLK_RTC/16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(PRESCALER_A::DIV16)
    }
    #[doc = "CLK_RTC_CNT = GCLK_RTC/32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut W {
        self.variant(PRESCALER_A::DIV32)
    }
    #[doc = "CLK_RTC_CNT = GCLK_RTC/64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut W {
        self.variant(PRESCALER_A::DIV64)
    }
    #[doc = "CLK_RTC_CNT = GCLK_RTC/128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut W {
        self.variant(PRESCALER_A::DIV128)
    }
    #[doc = "CLK_RTC_CNT = GCLK_RTC/256"]
    #[inline(always)]
    pub fn div256(self) -> &'a mut W {
        self.variant(PRESCALER_A::DIV256)
    }
    #[doc = "CLK_RTC_CNT = GCLK_RTC/512"]
    #[inline(always)]
    pub fn div512(self) -> &'a mut W {
        self.variant(PRESCALER_A::DIV512)
    }
    #[doc = "CLK_RTC_CNT = GCLK_RTC/1024"]
    #[inline(always)]
    pub fn div1024(self) -> &'a mut W {
        self.variant(PRESCALER_A::DIV1024)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u16) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `GPTRST`"]
pub type GPTRST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPTRST`"]
pub struct GPTRST_W<'a> {
    w: &'a mut W,
}
impl<'a> GPTRST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u16) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `CLOCKSYNC`"]
pub type CLOCKSYNC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLOCKSYNC`"]
pub struct CLOCKSYNC_W<'a> {
    w: &'a mut W,
}
impl<'a> CLOCKSYNC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u16) & 0x01) << 15);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Software Reset"]
    #[inline(always)]
    pub fn swrst(&self) -> SWRST_R {
        SWRST_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enable"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:3 - Operating Mode"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 6 - Clock Representation"]
    #[inline(always)]
    pub fn clkrep(&self) -> CLKREP_R {
        CLKREP_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Clear on Match"]
    #[inline(always)]
    pub fn matchclr(&self) -> MATCHCLR_R {
        MATCHCLR_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:11 - Prescaler"]
    #[inline(always)]
    pub fn prescaler(&self) -> PRESCALER_R {
        PRESCALER_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 14 - GP Registers Reset On Tamper Enable"]
    #[inline(always)]
    pub fn gptrst(&self) -> GPTRST_R {
        GPTRST_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Clock Read Synchronization Enable"]
    #[inline(always)]
    pub fn clocksync(&self) -> CLOCKSYNC_R {
        CLOCKSYNC_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Software Reset"]
    #[inline(always)]
    pub fn swrst(&mut self) -> SWRST_W {
        SWRST_W { w: self }
    }
    #[doc = "Bit 1 - Enable"]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W {
        ENABLE_W { w: self }
    }
    #[doc = "Bits 2:3 - Operating Mode"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W { w: self }
    }
    #[doc = "Bit 6 - Clock Representation"]
    #[inline(always)]
    pub fn clkrep(&mut self) -> CLKREP_W {
        CLKREP_W { w: self }
    }
    #[doc = "Bit 7 - Clear on Match"]
    #[inline(always)]
    pub fn matchclr(&mut self) -> MATCHCLR_W {
        MATCHCLR_W { w: self }
    }
    #[doc = "Bits 8:11 - Prescaler"]
    #[inline(always)]
    pub fn prescaler(&mut self) -> PRESCALER_W {
        PRESCALER_W { w: self }
    }
    #[doc = "Bit 14 - GP Registers Reset On Tamper Enable"]
    #[inline(always)]
    pub fn gptrst(&mut self) -> GPTRST_W {
        GPTRST_W { w: self }
    }
    #[doc = "Bit 15 - Clock Read Synchronization Enable"]
    #[inline(always)]
    pub fn clocksync(&mut self) -> CLOCKSYNC_W {
        CLOCKSYNC_W { w: self }
    }
}
