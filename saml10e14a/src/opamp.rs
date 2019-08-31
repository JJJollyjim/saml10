#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control A"]
    pub ctrla: CTRLA,
    _reserved1: [u8; 1usize],
    #[doc = "0x02 - Status"]
    pub status: STATUS,
    _reserved2: [u8; 1usize],
    #[doc = "0x04 - OPAMP n Control"]
    pub opampctrl: [OPAMPCTRL; 3],
    #[doc = "0x10 - Resister Control"]
    pub resctrl: RESCTRL,
}
#[doc = "Control A\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ctrla](ctrla) module"]
pub type CTRLA = crate::Reg<u8, _CTRLA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRLA;
#[doc = "`read()` method returns [ctrla::R](ctrla::R) reader structure"]
impl crate::Readable for CTRLA {}
#[doc = "`write(|w| ..)` method takes [ctrla::W](ctrla::W) writer structure"]
impl crate::Writable for CTRLA {}
#[doc = "Control A"]
pub mod ctrla;
#[doc = "Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [status](status) module"]
pub type STATUS = crate::Reg<u8, _STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATUS;
#[doc = "`read()` method returns [status::R](status::R) reader structure"]
impl crate::Readable for STATUS {}
#[doc = "Status"]
pub mod status;
#[doc = "OPAMP n Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [opampctrl](opampctrl) module"]
pub type OPAMPCTRL = crate::Reg<u32, _OPAMPCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OPAMPCTRL;
#[doc = "`read()` method returns [opampctrl::R](opampctrl::R) reader structure"]
impl crate::Readable for OPAMPCTRL {}
#[doc = "`write(|w| ..)` method takes [opampctrl::W](opampctrl::W) writer structure"]
impl crate::Writable for OPAMPCTRL {}
#[doc = "OPAMP n Control"]
pub mod opampctrl;
#[doc = "Resister Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [resctrl](resctrl) module"]
pub type RESCTRL = crate::Reg<u8, _RESCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RESCTRL;
#[doc = "`read()` method returns [resctrl::R](resctrl::R) reader structure"]
impl crate::Readable for RESCTRL {}
#[doc = "`write(|w| ..)` method takes [resctrl::W](resctrl::W) writer structure"]
impl crate::Writable for RESCTRL {}
#[doc = "Resister Control"]
pub mod resctrl;
