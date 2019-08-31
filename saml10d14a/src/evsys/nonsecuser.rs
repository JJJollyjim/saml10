#[doc = "Reader of register NONSECUSER[%s]"]
pub type R = crate::R<u32, super::NONSECUSER>;
#[doc = "Writer for register NONSECUSER[%s]"]
pub type W = crate::W<u32, super::NONSECUSER>;
#[doc = "Register NONSECUSER[%s] `reset()`'s with value 0"]
impl crate::ResetValue for super::NONSECUSER {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `USER0`"]
pub type USER0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USER0`"]
pub struct USER0_W<'a> {
    w: &'a mut W,
}
impl<'a> USER0_W<'a> {
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
#[doc = "Reader of field `USER1`"]
pub type USER1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USER1`"]
pub struct USER1_W<'a> {
    w: &'a mut W,
}
impl<'a> USER1_W<'a> {
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
#[doc = "Reader of field `USER2`"]
pub type USER2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USER2`"]
pub struct USER2_W<'a> {
    w: &'a mut W,
}
impl<'a> USER2_W<'a> {
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
#[doc = "Reader of field `USER3`"]
pub type USER3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USER3`"]
pub struct USER3_W<'a> {
    w: &'a mut W,
}
impl<'a> USER3_W<'a> {
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
#[doc = "Reader of field `USER4`"]
pub type USER4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USER4`"]
pub struct USER4_W<'a> {
    w: &'a mut W,
}
impl<'a> USER4_W<'a> {
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
#[doc = "Reader of field `USER5`"]
pub type USER5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USER5`"]
pub struct USER5_W<'a> {
    w: &'a mut W,
}
impl<'a> USER5_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `USER6`"]
pub type USER6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USER6`"]
pub struct USER6_W<'a> {
    w: &'a mut W,
}
impl<'a> USER6_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `USER7`"]
pub type USER7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USER7`"]
pub struct USER7_W<'a> {
    w: &'a mut W,
}
impl<'a> USER7_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `USER8`"]
pub type USER8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USER8`"]
pub struct USER8_W<'a> {
    w: &'a mut W,
}
impl<'a> USER8_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `USER9`"]
pub type USER9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USER9`"]
pub struct USER9_W<'a> {
    w: &'a mut W,
}
impl<'a> USER9_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `USER10`"]
pub type USER10_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USER10`"]
pub struct USER10_W<'a> {
    w: &'a mut W,
}
impl<'a> USER10_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `USER11`"]
pub type USER11_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USER11`"]
pub struct USER11_W<'a> {
    w: &'a mut W,
}
impl<'a> USER11_W<'a> {
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
#[doc = "Reader of field `USER12`"]
pub type USER12_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USER12`"]
pub struct USER12_W<'a> {
    w: &'a mut W,
}
impl<'a> USER12_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `USER13`"]
pub type USER13_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USER13`"]
pub struct USER13_W<'a> {
    w: &'a mut W,
}
impl<'a> USER13_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `USER14`"]
pub type USER14_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USER14`"]
pub struct USER14_W<'a> {
    w: &'a mut W,
}
impl<'a> USER14_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `USER15`"]
pub type USER15_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USER15`"]
pub struct USER15_W<'a> {
    w: &'a mut W,
}
impl<'a> USER15_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `USER16`"]
pub type USER16_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USER16`"]
pub struct USER16_W<'a> {
    w: &'a mut W,
}
impl<'a> USER16_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `USER17`"]
pub type USER17_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USER17`"]
pub struct USER17_W<'a> {
    w: &'a mut W,
}
impl<'a> USER17_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `USER18`"]
pub type USER18_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USER18`"]
pub struct USER18_W<'a> {
    w: &'a mut W,
}
impl<'a> USER18_W<'a> {
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
#[doc = "Reader of field `USER19`"]
pub type USER19_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USER19`"]
pub struct USER19_W<'a> {
    w: &'a mut W,
}
impl<'a> USER19_W<'a> {
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
#[doc = "Reader of field `USER20`"]
pub type USER20_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USER20`"]
pub struct USER20_W<'a> {
    w: &'a mut W,
}
impl<'a> USER20_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Reader of field `USER21`"]
pub type USER21_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USER21`"]
pub struct USER21_W<'a> {
    w: &'a mut W,
}
impl<'a> USER21_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Reader of field `USER22`"]
pub type USER22_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USER22`"]
pub struct USER22_W<'a> {
    w: &'a mut W,
}
impl<'a> USER22_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Non-Secure for User 0"]
    #[inline(always)]
    pub fn user0(&self) -> USER0_R {
        USER0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Non-Secure for User 1"]
    #[inline(always)]
    pub fn user1(&self) -> USER1_R {
        USER1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Non-Secure for User 2"]
    #[inline(always)]
    pub fn user2(&self) -> USER2_R {
        USER2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Non-Secure for User 3"]
    #[inline(always)]
    pub fn user3(&self) -> USER3_R {
        USER3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Non-Secure for User 4"]
    #[inline(always)]
    pub fn user4(&self) -> USER4_R {
        USER4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Non-Secure for User 5"]
    #[inline(always)]
    pub fn user5(&self) -> USER5_R {
        USER5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Non-Secure for User 6"]
    #[inline(always)]
    pub fn user6(&self) -> USER6_R {
        USER6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Non-Secure for User 7"]
    #[inline(always)]
    pub fn user7(&self) -> USER7_R {
        USER7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Non-Secure for User 8"]
    #[inline(always)]
    pub fn user8(&self) -> USER8_R {
        USER8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Non-Secure for User 9"]
    #[inline(always)]
    pub fn user9(&self) -> USER9_R {
        USER9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Non-Secure for User 10"]
    #[inline(always)]
    pub fn user10(&self) -> USER10_R {
        USER10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Non-Secure for User 11"]
    #[inline(always)]
    pub fn user11(&self) -> USER11_R {
        USER11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Non-Secure for User 12"]
    #[inline(always)]
    pub fn user12(&self) -> USER12_R {
        USER12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Non-Secure for User 13"]
    #[inline(always)]
    pub fn user13(&self) -> USER13_R {
        USER13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Non-Secure for User 14"]
    #[inline(always)]
    pub fn user14(&self) -> USER14_R {
        USER14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Non-Secure for User 15"]
    #[inline(always)]
    pub fn user15(&self) -> USER15_R {
        USER15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Non-Secure for User 16"]
    #[inline(always)]
    pub fn user16(&self) -> USER16_R {
        USER16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Non-Secure for User 17"]
    #[inline(always)]
    pub fn user17(&self) -> USER17_R {
        USER17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Non-Secure for User 18"]
    #[inline(always)]
    pub fn user18(&self) -> USER18_R {
        USER18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Non-Secure for User 19"]
    #[inline(always)]
    pub fn user19(&self) -> USER19_R {
        USER19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Non-Secure for User 20"]
    #[inline(always)]
    pub fn user20(&self) -> USER20_R {
        USER20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Non-Secure for User 21"]
    #[inline(always)]
    pub fn user21(&self) -> USER21_R {
        USER21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Non-Secure for User 22"]
    #[inline(always)]
    pub fn user22(&self) -> USER22_R {
        USER22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Non-Secure for User 0"]
    #[inline(always)]
    pub fn user0(&mut self) -> USER0_W {
        USER0_W { w: self }
    }
    #[doc = "Bit 1 - Non-Secure for User 1"]
    #[inline(always)]
    pub fn user1(&mut self) -> USER1_W {
        USER1_W { w: self }
    }
    #[doc = "Bit 2 - Non-Secure for User 2"]
    #[inline(always)]
    pub fn user2(&mut self) -> USER2_W {
        USER2_W { w: self }
    }
    #[doc = "Bit 3 - Non-Secure for User 3"]
    #[inline(always)]
    pub fn user3(&mut self) -> USER3_W {
        USER3_W { w: self }
    }
    #[doc = "Bit 4 - Non-Secure for User 4"]
    #[inline(always)]
    pub fn user4(&mut self) -> USER4_W {
        USER4_W { w: self }
    }
    #[doc = "Bit 5 - Non-Secure for User 5"]
    #[inline(always)]
    pub fn user5(&mut self) -> USER5_W {
        USER5_W { w: self }
    }
    #[doc = "Bit 6 - Non-Secure for User 6"]
    #[inline(always)]
    pub fn user6(&mut self) -> USER6_W {
        USER6_W { w: self }
    }
    #[doc = "Bit 7 - Non-Secure for User 7"]
    #[inline(always)]
    pub fn user7(&mut self) -> USER7_W {
        USER7_W { w: self }
    }
    #[doc = "Bit 8 - Non-Secure for User 8"]
    #[inline(always)]
    pub fn user8(&mut self) -> USER8_W {
        USER8_W { w: self }
    }
    #[doc = "Bit 9 - Non-Secure for User 9"]
    #[inline(always)]
    pub fn user9(&mut self) -> USER9_W {
        USER9_W { w: self }
    }
    #[doc = "Bit 10 - Non-Secure for User 10"]
    #[inline(always)]
    pub fn user10(&mut self) -> USER10_W {
        USER10_W { w: self }
    }
    #[doc = "Bit 11 - Non-Secure for User 11"]
    #[inline(always)]
    pub fn user11(&mut self) -> USER11_W {
        USER11_W { w: self }
    }
    #[doc = "Bit 12 - Non-Secure for User 12"]
    #[inline(always)]
    pub fn user12(&mut self) -> USER12_W {
        USER12_W { w: self }
    }
    #[doc = "Bit 13 - Non-Secure for User 13"]
    #[inline(always)]
    pub fn user13(&mut self) -> USER13_W {
        USER13_W { w: self }
    }
    #[doc = "Bit 14 - Non-Secure for User 14"]
    #[inline(always)]
    pub fn user14(&mut self) -> USER14_W {
        USER14_W { w: self }
    }
    #[doc = "Bit 15 - Non-Secure for User 15"]
    #[inline(always)]
    pub fn user15(&mut self) -> USER15_W {
        USER15_W { w: self }
    }
    #[doc = "Bit 16 - Non-Secure for User 16"]
    #[inline(always)]
    pub fn user16(&mut self) -> USER16_W {
        USER16_W { w: self }
    }
    #[doc = "Bit 17 - Non-Secure for User 17"]
    #[inline(always)]
    pub fn user17(&mut self) -> USER17_W {
        USER17_W { w: self }
    }
    #[doc = "Bit 18 - Non-Secure for User 18"]
    #[inline(always)]
    pub fn user18(&mut self) -> USER18_W {
        USER18_W { w: self }
    }
    #[doc = "Bit 19 - Non-Secure for User 19"]
    #[inline(always)]
    pub fn user19(&mut self) -> USER19_W {
        USER19_W { w: self }
    }
    #[doc = "Bit 20 - Non-Secure for User 20"]
    #[inline(always)]
    pub fn user20(&mut self) -> USER20_W {
        USER20_W { w: self }
    }
    #[doc = "Bit 21 - Non-Secure for User 21"]
    #[inline(always)]
    pub fn user21(&mut self) -> USER21_W {
        USER21_W { w: self }
    }
    #[doc = "Bit 22 - Non-Secure for User 22"]
    #[inline(always)]
    pub fn user22(&mut self) -> USER22_W {
        USER22_W { w: self }
    }
}
