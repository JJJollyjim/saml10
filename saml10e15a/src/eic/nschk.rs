#[doc = "Reader of register NSCHK"]
pub type R = crate::R<u32, super::NSCHK>;
#[doc = "Writer for register NSCHK"]
pub type W = crate::W<u32, super::NSCHK>;
#[doc = "Register NSCHK `reset()`'s with value 0"]
impl crate::ResetValue for super::NSCHK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EXTINT`"]
pub type EXTINT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EXTINT`"]
pub struct EXTINT_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTINT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `NMI`"]
pub type NMI_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NMI`"]
pub struct NMI_W<'a> {
    w: &'a mut W,
}
impl<'a> NMI_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - External Interrupt Nonsecure Check Enable"]
    #[inline(always)]
    pub fn extint(&self) -> EXTINT_R {
        EXTINT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 31 - Non-Maskable External Interrupt Nonsecure Check Enable"]
    #[inline(always)]
    pub fn nmi(&self) -> NMI_R {
        NMI_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - External Interrupt Nonsecure Check Enable"]
    #[inline(always)]
    pub fn extint(&mut self) -> EXTINT_W {
        EXTINT_W { w: self }
    }
    #[doc = "Bit 31 - Non-Maskable External Interrupt Nonsecure Check Enable"]
    #[inline(always)]
    pub fn nmi(&mut self) -> NMI_W {
        NMI_W { w: self }
    }
}
