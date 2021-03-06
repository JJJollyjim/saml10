#[doc = "Reader of register NONSEC"]
pub type R = crate::R<u32, super::NONSEC>;
#[doc = "Writer for register NONSEC"]
pub type W = crate::W<u32, super::NONSEC>;
#[doc = "Register NONSEC `reset()`'s with value 0x01"]
impl crate::ResetValue for super::NONSEC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
    }
}
#[doc = "Reader of field `WRITE`"]
pub type WRITE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WRITE`"]
pub struct WRITE_W<'a> {
    w: &'a mut W,
}
impl<'a> WRITE_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Non-secure APB alias write enable, non-secure AHB writes to non-secure regions enable"]
    #[inline(always)]
    pub fn write(&self) -> WRITE_R {
        WRITE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Non-secure APB alias write enable, non-secure AHB writes to non-secure regions enable"]
    #[inline(always)]
    pub fn write(&mut self) -> WRITE_W {
        WRITE_W { w: self }
    }
}
