#[doc = "Reader of register TAMPCTRLB"]
pub type R = crate::R<u32, super::TAMPCTRLB>;
#[doc = "Writer for register TAMPCTRLB"]
pub type W = crate::W<u32, super::TAMPCTRLB>;
#[doc = "Register TAMPCTRLB `reset()`'s with value 0"]
impl crate::ResetValue for super::TAMPCTRLB {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ALSI0`"]
pub type ALSI0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ALSI0`"]
pub struct ALSI0_W<'a> {
    w: &'a mut W,
}
impl<'a> ALSI0_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `ALSI1`"]
pub type ALSI1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ALSI1`"]
pub struct ALSI1_W<'a> {
    w: &'a mut W,
}
impl<'a> ALSI1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `ALSI2`"]
pub type ALSI2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ALSI2`"]
pub struct ALSI2_W<'a> {
    w: &'a mut W,
}
impl<'a> ALSI2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `ALSI3`"]
pub type ALSI3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ALSI3`"]
pub struct ALSI3_W<'a> {
    w: &'a mut W,
}
impl<'a> ALSI3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Active Layer Select Internal 0"]
    #[inline(always)]
    pub fn alsi0(&self) -> ALSI0_R {
        ALSI0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Active Layer Select Internal 1"]
    #[inline(always)]
    pub fn alsi1(&self) -> ALSI1_R {
        ALSI1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Active Layer Select Internal 2"]
    #[inline(always)]
    pub fn alsi2(&self) -> ALSI2_R {
        ALSI2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Active Layer Select Internal 3"]
    #[inline(always)]
    pub fn alsi3(&self) -> ALSI3_R {
        ALSI3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Active Layer Select Internal 0"]
    #[inline(always)]
    pub fn alsi0(&mut self) -> ALSI0_W {
        ALSI0_W { w: self }
    }
    #[doc = "Bit 1 - Active Layer Select Internal 1"]
    #[inline(always)]
    pub fn alsi1(&mut self) -> ALSI1_W {
        ALSI1_W { w: self }
    }
    #[doc = "Bit 2 - Active Layer Select Internal 2"]
    #[inline(always)]
    pub fn alsi2(&mut self) -> ALSI2_W {
        ALSI2_W { w: self }
    }
    #[doc = "Bit 3 - Active Layer Select Internal 3"]
    #[inline(always)]
    pub fn alsi3(&mut self) -> ALSI3_W {
        ALSI3_W { w: self }
    }
}
