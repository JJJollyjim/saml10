#[doc = "Reader of register EVCTRL"]
pub type R = crate::R<u32, super::EVCTRL>;
#[doc = "Writer for register EVCTRL"]
pub type W = crate::W<u32, super::EVCTRL>;
#[doc = "Register EVCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::EVCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BOD33DETEO`"]
pub type BOD33DETEO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BOD33DETEO`"]
pub struct BOD33DETEO_W<'a> {
    w: &'a mut W,
}
impl<'a> BOD33DETEO_W<'a> {
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
#[doc = "Reader of field `BOD12DETEO`"]
pub type BOD12DETEO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BOD12DETEO`"]
pub struct BOD12DETEO_W<'a> {
    w: &'a mut W,
}
impl<'a> BOD12DETEO_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bit 1 - BOD33 Detection Event Output Enable"]
    #[inline(always)]
    pub fn bod33deteo(&self) -> BOD33DETEO_R {
        BOD33DETEO_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 4 - BOD12 Detection Event Output Enable"]
    #[inline(always)]
    pub fn bod12deteo(&self) -> BOD12DETEO_R {
        BOD12DETEO_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - BOD33 Detection Event Output Enable"]
    #[inline(always)]
    pub fn bod33deteo(&mut self) -> BOD33DETEO_W {
        BOD33DETEO_W { w: self }
    }
    #[doc = "Bit 4 - BOD12 Detection Event Output Enable"]
    #[inline(always)]
    pub fn bod12deteo(&mut self) -> BOD12DETEO_W {
        BOD12DETEO_W { w: self }
    }
}
