#[doc = "MODE2_ALARM Alarm n Value\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [alarm](alarm) module"]
pub type ALARM = crate::Reg<u32, _ALARM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ALARM;
#[doc = "`read()` method returns [alarm::R](alarm::R) reader structure"]
impl crate::Readable for ALARM {}
#[doc = "`write(|w| ..)` method takes [alarm::W](alarm::W) writer structure"]
impl crate::Writable for ALARM {}
#[doc = "MODE2_ALARM Alarm n Value"]
pub mod alarm;
#[doc = "MODE2_ALARM Alarm n Mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mask](mask) module"]
pub type MASK = crate::Reg<u8, _MASK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MASK;
#[doc = "`read()` method returns [mask::R](mask::R) reader structure"]
impl crate::Readable for MASK {}
#[doc = "`write(|w| ..)` method takes [mask::W](mask::W) writer structure"]
impl crate::Writable for MASK {}
#[doc = "MODE2_ALARM Alarm n Mask"]
pub mod mask;
