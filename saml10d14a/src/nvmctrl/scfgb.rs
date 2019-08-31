#[doc = "Reader of register SCFGB"]
pub type R = crate::R<u32, super::SCFGB>;
#[doc = "Writer for register SCFGB"]
pub type W = crate::W<u32, super::SCFGB>;
#[doc = "Register SCFGB `reset()`'s with value 0x03"]
impl crate::ResetValue for super::SCFGB {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x03
    }
}
#[doc = "Reader of field `BCREN`"]
pub type BCREN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BCREN`"]
pub struct BCREN_W<'a> {
    w: &'a mut W,
}
impl<'a> BCREN_W<'a> {
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
#[doc = "Reader of field `BCWEN`"]
pub type BCWEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BCWEN`"]
pub struct BCWEN_W<'a> {
    w: &'a mut W,
}
impl<'a> BCWEN_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Boot Configuration Row Read Enable"]
    #[inline(always)]
    pub fn bcren(&self) -> BCREN_R {
        BCREN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Boot Configuration Row Write Enable"]
    #[inline(always)]
    pub fn bcwen(&self) -> BCWEN_R {
        BCWEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Boot Configuration Row Read Enable"]
    #[inline(always)]
    pub fn bcren(&mut self) -> BCREN_W {
        BCREN_W { w: self }
    }
    #[doc = "Bit 1 - Boot Configuration Row Write Enable"]
    #[inline(always)]
    pub fn bcwen(&mut self) -> BCWEN_W {
        BCWEN_W { w: self }
    }
}
