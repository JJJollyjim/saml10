#[doc = "Reader of register CTRLC"]
pub type R = crate::R<u16, super::CTRLC>;
#[doc = "Writer for register CTRLC"]
pub type W = crate::W<u16, super::CTRLC>;
#[doc = "Register CTRLC `reset()`'s with value 0"]
impl crate::ResetValue for super::CTRLC {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DIFFMODE`"]
pub type DIFFMODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIFFMODE`"]
pub struct DIFFMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> DIFFMODE_W<'a> {
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
#[doc = "Reader of field `LEFTADJ`"]
pub type LEFTADJ_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LEFTADJ`"]
pub struct LEFTADJ_W<'a> {
    w: &'a mut W,
}
impl<'a> LEFTADJ_W<'a> {
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
#[doc = "Reader of field `FREERUN`"]
pub type FREERUN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FREERUN`"]
pub struct FREERUN_W<'a> {
    w: &'a mut W,
}
impl<'a> FREERUN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u16) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `CORREN`"]
pub type CORREN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CORREN`"]
pub struct CORREN_W<'a> {
    w: &'a mut W,
}
impl<'a> CORREN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u16) & 0x01) << 3);
        self.w
    }
}
#[doc = "Possible values of the field `RESSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESSEL_A {
    #[doc = "12-bit result"]
    _12BIT,
    #[doc = "For averaging mode output"]
    _16BIT,
    #[doc = "10-bit result"]
    _10BIT,
    #[doc = "8-bit result"]
    _8BIT,
}
impl crate::ToBits<u8> for RESSEL_A {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            RESSEL_A::_12BIT => 0,
            RESSEL_A::_16BIT => 1,
            RESSEL_A::_10BIT => 2,
            RESSEL_A::_8BIT => 3,
        }
    }
}
#[doc = "Reader of field `RESSEL`"]
pub type RESSEL_R = crate::R<u8, RESSEL_A>;
impl RESSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RESSEL_A {
        match self.bits {
            0 => RESSEL_A::_12BIT,
            1 => RESSEL_A::_16BIT,
            2 => RESSEL_A::_10BIT,
            3 => RESSEL_A::_8BIT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_12BIT`"]
    #[inline(always)]
    pub fn is_12bit(&self) -> bool {
        *self == RESSEL_A::_12BIT
    }
    #[doc = "Checks if the value of the field is `_16BIT`"]
    #[inline(always)]
    pub fn is_16bit(&self) -> bool {
        *self == RESSEL_A::_16BIT
    }
    #[doc = "Checks if the value of the field is `_10BIT`"]
    #[inline(always)]
    pub fn is_10bit(&self) -> bool {
        *self == RESSEL_A::_10BIT
    }
    #[doc = "Checks if the value of the field is `_8BIT`"]
    #[inline(always)]
    pub fn is_8bit(&self) -> bool {
        *self == RESSEL_A::_8BIT
    }
}
#[doc = "Write proxy for field `RESSEL`"]
pub struct RESSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RESSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RESSEL_A) -> &'a mut W {
        use crate::ToBits;
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "12-bit result"]
    #[inline(always)]
    pub fn _12bit(self) -> &'a mut W {
        self.variant(RESSEL_A::_12BIT)
    }
    #[doc = "For averaging mode output"]
    #[inline(always)]
    pub fn _16bit(self) -> &'a mut W {
        self.variant(RESSEL_A::_16BIT)
    }
    #[doc = "10-bit result"]
    #[inline(always)]
    pub fn _10bit(self) -> &'a mut W {
        self.variant(RESSEL_A::_10BIT)
    }
    #[doc = "8-bit result"]
    #[inline(always)]
    pub fn _8bit(self) -> &'a mut W {
        self.variant(RESSEL_A::_8BIT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u16) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `R2R`"]
pub type R2R_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `R2R`"]
pub struct R2R_W<'a> {
    w: &'a mut W,
}
impl<'a> R2R_W<'a> {
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
#[doc = "Possible values of the field `WINMODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WINMODE_A {
    #[doc = "No window mode (default)"]
    DISABLE,
    #[doc = "RESULT > WINLT"]
    MODE1,
    #[doc = "RESULT < WINUT"]
    MODE2,
    #[doc = "WINLT < RESULT < WINUT"]
    MODE3,
    #[doc = "!(WINLT < RESULT < WINUT)"]
    MODE4,
}
impl crate::ToBits<u8> for WINMODE_A {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            WINMODE_A::DISABLE => 0,
            WINMODE_A::MODE1 => 1,
            WINMODE_A::MODE2 => 2,
            WINMODE_A::MODE3 => 3,
            WINMODE_A::MODE4 => 4,
        }
    }
}
#[doc = "Reader of field `WINMODE`"]
pub type WINMODE_R = crate::R<u8, WINMODE_A>;
impl WINMODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, WINMODE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(WINMODE_A::DISABLE),
            1 => Val(WINMODE_A::MODE1),
            2 => Val(WINMODE_A::MODE2),
            3 => Val(WINMODE_A::MODE3),
            4 => Val(WINMODE_A::MODE4),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == WINMODE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `MODE1`"]
    #[inline(always)]
    pub fn is_mode1(&self) -> bool {
        *self == WINMODE_A::MODE1
    }
    #[doc = "Checks if the value of the field is `MODE2`"]
    #[inline(always)]
    pub fn is_mode2(&self) -> bool {
        *self == WINMODE_A::MODE2
    }
    #[doc = "Checks if the value of the field is `MODE3`"]
    #[inline(always)]
    pub fn is_mode3(&self) -> bool {
        *self == WINMODE_A::MODE3
    }
    #[doc = "Checks if the value of the field is `MODE4`"]
    #[inline(always)]
    pub fn is_mode4(&self) -> bool {
        *self == WINMODE_A::MODE4
    }
}
#[doc = "Write proxy for field `WINMODE`"]
pub struct WINMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> WINMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WINMODE_A) -> &'a mut W {
        use crate::ToBits;
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "No window mode (default)"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(WINMODE_A::DISABLE)
    }
    #[doc = "RESULT > WINLT"]
    #[inline(always)]
    pub fn mode1(self) -> &'a mut W {
        self.variant(WINMODE_A::MODE1)
    }
    #[doc = "RESULT < WINUT"]
    #[inline(always)]
    pub fn mode2(self) -> &'a mut W {
        self.variant(WINMODE_A::MODE2)
    }
    #[doc = "WINLT < RESULT < WINUT"]
    #[inline(always)]
    pub fn mode3(self) -> &'a mut W {
        self.variant(WINMODE_A::MODE3)
    }
    #[doc = "!(WINLT < RESULT < WINUT)"]
    #[inline(always)]
    pub fn mode4(self) -> &'a mut W {
        self.variant(WINMODE_A::MODE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u16) & 0x07) << 8);
        self.w
    }
}
#[doc = "Possible values of the field `DUALSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DUALSEL_A {
    #[doc = "Start event or software trigger will start a conversion on both ADCs"]
    BOTH,
    #[doc = "START event or software trigger will alternatingly start a conversion on ADC0 and ADC1"]
    INTERLEAVE,
}
impl crate::ToBits<u8> for DUALSEL_A {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            DUALSEL_A::BOTH => 0,
            DUALSEL_A::INTERLEAVE => 1,
        }
    }
}
#[doc = "Reader of field `DUALSEL`"]
pub type DUALSEL_R = crate::R<u8, DUALSEL_A>;
impl DUALSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, DUALSEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(DUALSEL_A::BOTH),
            1 => Val(DUALSEL_A::INTERLEAVE),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `BOTH`"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == DUALSEL_A::BOTH
    }
    #[doc = "Checks if the value of the field is `INTERLEAVE`"]
    #[inline(always)]
    pub fn is_interleave(&self) -> bool {
        *self == DUALSEL_A::INTERLEAVE
    }
}
#[doc = "Write proxy for field `DUALSEL`"]
pub struct DUALSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DUALSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DUALSEL_A) -> &'a mut W {
        use crate::ToBits;
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Start event or software trigger will start a conversion on both ADCs"]
    #[inline(always)]
    pub fn both(self) -> &'a mut W {
        self.variant(DUALSEL_A::BOTH)
    }
    #[doc = "START event or software trigger will alternatingly start a conversion on ADC0 and ADC1"]
    #[inline(always)]
    pub fn interleave(self) -> &'a mut W {
        self.variant(DUALSEL_A::INTERLEAVE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u16) & 0x03) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Differential Mode"]
    #[inline(always)]
    pub fn diffmode(&self) -> DIFFMODE_R {
        DIFFMODE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Left-Adjusted Result"]
    #[inline(always)]
    pub fn leftadj(&self) -> LEFTADJ_R {
        LEFTADJ_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Free Running Mode"]
    #[inline(always)]
    pub fn freerun(&self) -> FREERUN_R {
        FREERUN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Digital Correction Logic Enable"]
    #[inline(always)]
    pub fn corren(&self) -> CORREN_R {
        CORREN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - Conversion Result Resolution"]
    #[inline(always)]
    pub fn ressel(&self) -> RESSEL_R {
        RESSEL_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 7 - Rail-to-Rail mode enable"]
    #[inline(always)]
    pub fn r2r(&self) -> R2R_R {
        R2R_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:10 - Window Monitor Mode"]
    #[inline(always)]
    pub fn winmode(&self) -> WINMODE_R {
        WINMODE_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 12:13 - Dual Mode Trigger Selection"]
    #[inline(always)]
    pub fn dualsel(&self) -> DUALSEL_R {
        DUALSEL_R::new(((self.bits >> 12) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Differential Mode"]
    #[inline(always)]
    pub fn diffmode(&mut self) -> DIFFMODE_W {
        DIFFMODE_W { w: self }
    }
    #[doc = "Bit 1 - Left-Adjusted Result"]
    #[inline(always)]
    pub fn leftadj(&mut self) -> LEFTADJ_W {
        LEFTADJ_W { w: self }
    }
    #[doc = "Bit 2 - Free Running Mode"]
    #[inline(always)]
    pub fn freerun(&mut self) -> FREERUN_W {
        FREERUN_W { w: self }
    }
    #[doc = "Bit 3 - Digital Correction Logic Enable"]
    #[inline(always)]
    pub fn corren(&mut self) -> CORREN_W {
        CORREN_W { w: self }
    }
    #[doc = "Bits 4:5 - Conversion Result Resolution"]
    #[inline(always)]
    pub fn ressel(&mut self) -> RESSEL_W {
        RESSEL_W { w: self }
    }
    #[doc = "Bit 7 - Rail-to-Rail mode enable"]
    #[inline(always)]
    pub fn r2r(&mut self) -> R2R_W {
        R2R_W { w: self }
    }
    #[doc = "Bits 8:10 - Window Monitor Mode"]
    #[inline(always)]
    pub fn winmode(&mut self) -> WINMODE_W {
        WINMODE_W { w: self }
    }
    #[doc = "Bits 12:13 - Dual Mode Trigger Selection"]
    #[inline(always)]
    pub fn dualsel(&mut self) -> DUALSEL_W {
        DUALSEL_W { w: self }
    }
}
