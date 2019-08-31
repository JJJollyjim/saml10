#[doc = "Reader of register DFLLULPRREQ"]
pub type R = crate::R<u8, super::DFLLULPRREQ>;
#[doc = "Writer for register DFLLULPRREQ"]
pub type W = crate::W<u8, super::DFLLULPRREQ>;
#[doc = "Register DFLLULPRREQ `reset()`'s with value 0"]
impl crate::ResetValue for super::DFLLULPRREQ {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RREQ`"]
pub type RREQ_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RREQ`"]
pub struct RREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> RREQ_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u8) & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bit 7 - Read Request"]
    #[inline(always)]
    pub fn rreq(&self) -> RREQ_R {
        RREQ_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - Read Request"]
    #[inline(always)]
    pub fn rreq(&mut self) -> RREQ_W {
        RREQ_W { w: self }
    }
}
