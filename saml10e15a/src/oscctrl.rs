#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Event Control"]
    pub evctrl: EVCTRL,
    _reserved1: [u8; 3usize],
    #[doc = "0x04 - Interrupt Enable Clear"]
    pub intenclr: INTENCLR,
    #[doc = "0x08 - Interrupt Enable Set"]
    pub intenset: INTENSET,
    #[doc = "0x0c - Interrupt Flag Status and Clear"]
    pub intflag: INTFLAG,
    #[doc = "0x10 - Status"]
    pub status: STATUS,
    #[doc = "0x14 - External Multipurpose Crystal Oscillator (XOSC) Control"]
    pub xoscctrl: XOSCCTRL,
    #[doc = "0x16 - Clock Failure Detector Prescaler"]
    pub cfdpresc: CFDPRESC,
    _reserved7: [u8; 1usize],
    #[doc = "0x18 - 16MHz Internal Oscillator (OSC16M) Control"]
    pub osc16mctrl: OSC16MCTRL,
    _reserved8: [u8; 3usize],
    #[doc = "0x1c - DFLLULP Control"]
    pub dfllulpctrl: DFLLULPCTRL,
    #[doc = "0x1e - DFLLULP Dither Control"]
    pub dfllulpdither: DFLLULPDITHER,
    #[doc = "0x1f - DFLLULP Read Request"]
    pub dfllulprreq: DFLLULPRREQ,
    #[doc = "0x20 - DFLLULP Delay Value"]
    pub dfllulpdly: DFLLULPDLY,
    #[doc = "0x24 - DFLLULP Target Ratio"]
    pub dfllulpratio: DFLLULPRATIO,
    #[doc = "0x28 - DFLLULP Synchronization Busy"]
    pub dfllulpsyncbusy: DFLLULPSYNCBUSY,
    #[doc = "0x2c - DPLL Control A"]
    pub dpllctrla: DPLLCTRLA,
    _reserved15: [u8; 3usize],
    #[doc = "0x30 - DPLL Ratio Control"]
    pub dpllratio: DPLLRATIO,
    #[doc = "0x34 - DPLL Control B"]
    pub dpllctrlb: DPLLCTRLB,
    #[doc = "0x38 - DPLL Prescaler"]
    pub dpllpresc: DPLLPRESC,
    _reserved18: [u8; 3usize],
    #[doc = "0x3c - DPLL Synchronization Busy"]
    pub dpllsyncbusy: DPLLSYNCBUSY,
    _reserved19: [u8; 3usize],
    #[doc = "0x40 - DPLL Status"]
    pub dpllstatus: DPLLSTATUS,
}
#[doc = "Event Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [evctrl](evctrl) module"]
pub type EVCTRL = crate::Reg<u8, _EVCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVCTRL;
#[doc = "`read()` method returns [evctrl::R](evctrl::R) reader structure"]
impl crate::Readable for EVCTRL {}
#[doc = "`write(|w| ..)` method takes [evctrl::W](evctrl::W) writer structure"]
impl crate::Writable for EVCTRL {}
#[doc = "Event Control"]
pub mod evctrl;
#[doc = "Interrupt Enable Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [intenclr](intenclr) module"]
pub type INTENCLR = crate::Reg<u32, _INTENCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTENCLR;
#[doc = "`read()` method returns [intenclr::R](intenclr::R) reader structure"]
impl crate::Readable for INTENCLR {}
#[doc = "`write(|w| ..)` method takes [intenclr::W](intenclr::W) writer structure"]
impl crate::Writable for INTENCLR {}
#[doc = "Interrupt Enable Clear"]
pub mod intenclr;
#[doc = "Interrupt Enable Set\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [intenset](intenset) module"]
pub type INTENSET = crate::Reg<u32, _INTENSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTENSET;
#[doc = "`read()` method returns [intenset::R](intenset::R) reader structure"]
impl crate::Readable for INTENSET {}
#[doc = "`write(|w| ..)` method takes [intenset::W](intenset::W) writer structure"]
impl crate::Writable for INTENSET {}
#[doc = "Interrupt Enable Set"]
pub mod intenset;
#[doc = "Interrupt Flag Status and Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [intflag](intflag) module"]
pub type INTFLAG = crate::Reg<u32, _INTFLAG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTFLAG;
#[doc = "`read()` method returns [intflag::R](intflag::R) reader structure"]
impl crate::Readable for INTFLAG {}
#[doc = "`write(|w| ..)` method takes [intflag::W](intflag::W) writer structure"]
impl crate::Writable for INTFLAG {}
#[doc = "Interrupt Flag Status and Clear"]
pub mod intflag;
#[doc = "Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [status](status) module"]
pub type STATUS = crate::Reg<u32, _STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATUS;
#[doc = "`read()` method returns [status::R](status::R) reader structure"]
impl crate::Readable for STATUS {}
#[doc = "Status"]
pub mod status;
#[doc = "External Multipurpose Crystal Oscillator (XOSC) Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [xoscctrl](xoscctrl) module"]
pub type XOSCCTRL = crate::Reg<u16, _XOSCCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _XOSCCTRL;
#[doc = "`read()` method returns [xoscctrl::R](xoscctrl::R) reader structure"]
impl crate::Readable for XOSCCTRL {}
#[doc = "`write(|w| ..)` method takes [xoscctrl::W](xoscctrl::W) writer structure"]
impl crate::Writable for XOSCCTRL {}
#[doc = "External Multipurpose Crystal Oscillator (XOSC) Control"]
pub mod xoscctrl;
#[doc = "Clock Failure Detector Prescaler\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cfdpresc](cfdpresc) module"]
pub type CFDPRESC = crate::Reg<u8, _CFDPRESC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFDPRESC;
#[doc = "`read()` method returns [cfdpresc::R](cfdpresc::R) reader structure"]
impl crate::Readable for CFDPRESC {}
#[doc = "`write(|w| ..)` method takes [cfdpresc::W](cfdpresc::W) writer structure"]
impl crate::Writable for CFDPRESC {}
#[doc = "Clock Failure Detector Prescaler"]
pub mod cfdpresc;
#[doc = "16MHz Internal Oscillator (OSC16M) Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [osc16mctrl](osc16mctrl) module"]
pub type OSC16MCTRL = crate::Reg<u8, _OSC16MCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OSC16MCTRL;
#[doc = "`read()` method returns [osc16mctrl::R](osc16mctrl::R) reader structure"]
impl crate::Readable for OSC16MCTRL {}
#[doc = "`write(|w| ..)` method takes [osc16mctrl::W](osc16mctrl::W) writer structure"]
impl crate::Writable for OSC16MCTRL {}
#[doc = "16MHz Internal Oscillator (OSC16M) Control"]
pub mod osc16mctrl;
#[doc = "DFLLULP Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dfllulpctrl](dfllulpctrl) module"]
pub type DFLLULPCTRL = crate::Reg<u16, _DFLLULPCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFLLULPCTRL;
#[doc = "`read()` method returns [dfllulpctrl::R](dfllulpctrl::R) reader structure"]
impl crate::Readable for DFLLULPCTRL {}
#[doc = "`write(|w| ..)` method takes [dfllulpctrl::W](dfllulpctrl::W) writer structure"]
impl crate::Writable for DFLLULPCTRL {}
#[doc = "DFLLULP Control"]
pub mod dfllulpctrl;
#[doc = "DFLLULP Dither Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dfllulpdither](dfllulpdither) module"]
pub type DFLLULPDITHER = crate::Reg<u8, _DFLLULPDITHER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFLLULPDITHER;
#[doc = "`read()` method returns [dfllulpdither::R](dfllulpdither::R) reader structure"]
impl crate::Readable for DFLLULPDITHER {}
#[doc = "`write(|w| ..)` method takes [dfllulpdither::W](dfllulpdither::W) writer structure"]
impl crate::Writable for DFLLULPDITHER {}
#[doc = "DFLLULP Dither Control"]
pub mod dfllulpdither;
#[doc = "DFLLULP Read Request\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dfllulprreq](dfllulprreq) module"]
pub type DFLLULPRREQ = crate::Reg<u8, _DFLLULPRREQ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFLLULPRREQ;
#[doc = "`read()` method returns [dfllulprreq::R](dfllulprreq::R) reader structure"]
impl crate::Readable for DFLLULPRREQ {}
#[doc = "`write(|w| ..)` method takes [dfllulprreq::W](dfllulprreq::W) writer structure"]
impl crate::Writable for DFLLULPRREQ {}
#[doc = "DFLLULP Read Request"]
pub mod dfllulprreq;
#[doc = "DFLLULP Delay Value\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dfllulpdly](dfllulpdly) module"]
pub type DFLLULPDLY = crate::Reg<u32, _DFLLULPDLY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFLLULPDLY;
#[doc = "`read()` method returns [dfllulpdly::R](dfllulpdly::R) reader structure"]
impl crate::Readable for DFLLULPDLY {}
#[doc = "`write(|w| ..)` method takes [dfllulpdly::W](dfllulpdly::W) writer structure"]
impl crate::Writable for DFLLULPDLY {}
#[doc = "DFLLULP Delay Value"]
pub mod dfllulpdly;
#[doc = "DFLLULP Target Ratio\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dfllulpratio](dfllulpratio) module"]
pub type DFLLULPRATIO = crate::Reg<u32, _DFLLULPRATIO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFLLULPRATIO;
#[doc = "`read()` method returns [dfllulpratio::R](dfllulpratio::R) reader structure"]
impl crate::Readable for DFLLULPRATIO {}
#[doc = "`write(|w| ..)` method takes [dfllulpratio::W](dfllulpratio::W) writer structure"]
impl crate::Writable for DFLLULPRATIO {}
#[doc = "DFLLULP Target Ratio"]
pub mod dfllulpratio;
#[doc = "DFLLULP Synchronization Busy\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dfllulpsyncbusy](dfllulpsyncbusy) module"]
pub type DFLLULPSYNCBUSY = crate::Reg<u32, _DFLLULPSYNCBUSY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFLLULPSYNCBUSY;
#[doc = "`read()` method returns [dfllulpsyncbusy::R](dfllulpsyncbusy::R) reader structure"]
impl crate::Readable for DFLLULPSYNCBUSY {}
#[doc = "DFLLULP Synchronization Busy"]
pub mod dfllulpsyncbusy;
#[doc = "DPLL Control A\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dpllctrla](dpllctrla) module"]
pub type DPLLCTRLA = crate::Reg<u8, _DPLLCTRLA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPLLCTRLA;
#[doc = "`read()` method returns [dpllctrla::R](dpllctrla::R) reader structure"]
impl crate::Readable for DPLLCTRLA {}
#[doc = "`write(|w| ..)` method takes [dpllctrla::W](dpllctrla::W) writer structure"]
impl crate::Writable for DPLLCTRLA {}
#[doc = "DPLL Control A"]
pub mod dpllctrla;
#[doc = "DPLL Ratio Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dpllratio](dpllratio) module"]
pub type DPLLRATIO = crate::Reg<u32, _DPLLRATIO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPLLRATIO;
#[doc = "`read()` method returns [dpllratio::R](dpllratio::R) reader structure"]
impl crate::Readable for DPLLRATIO {}
#[doc = "`write(|w| ..)` method takes [dpllratio::W](dpllratio::W) writer structure"]
impl crate::Writable for DPLLRATIO {}
#[doc = "DPLL Ratio Control"]
pub mod dpllratio;
#[doc = "DPLL Control B\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dpllctrlb](dpllctrlb) module"]
pub type DPLLCTRLB = crate::Reg<u32, _DPLLCTRLB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPLLCTRLB;
#[doc = "`read()` method returns [dpllctrlb::R](dpllctrlb::R) reader structure"]
impl crate::Readable for DPLLCTRLB {}
#[doc = "`write(|w| ..)` method takes [dpllctrlb::W](dpllctrlb::W) writer structure"]
impl crate::Writable for DPLLCTRLB {}
#[doc = "DPLL Control B"]
pub mod dpllctrlb;
#[doc = "DPLL Prescaler\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dpllpresc](dpllpresc) module"]
pub type DPLLPRESC = crate::Reg<u8, _DPLLPRESC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPLLPRESC;
#[doc = "`read()` method returns [dpllpresc::R](dpllpresc::R) reader structure"]
impl crate::Readable for DPLLPRESC {}
#[doc = "`write(|w| ..)` method takes [dpllpresc::W](dpllpresc::W) writer structure"]
impl crate::Writable for DPLLPRESC {}
#[doc = "DPLL Prescaler"]
pub mod dpllpresc;
#[doc = "DPLL Synchronization Busy\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dpllsyncbusy](dpllsyncbusy) module"]
pub type DPLLSYNCBUSY = crate::Reg<u8, _DPLLSYNCBUSY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPLLSYNCBUSY;
#[doc = "`read()` method returns [dpllsyncbusy::R](dpllsyncbusy::R) reader structure"]
impl crate::Readable for DPLLSYNCBUSY {}
#[doc = "DPLL Synchronization Busy"]
pub mod dpllsyncbusy;
#[doc = "DPLL Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dpllstatus](dpllstatus) module"]
pub type DPLLSTATUS = crate::Reg<u8, _DPLLSTATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPLLSTATUS;
#[doc = "`read()` method returns [dpllstatus::R](dpllstatus::R) reader structure"]
impl crate::Readable for DPLLSTATUS {}
#[doc = "DPLL Status"]
pub mod dpllstatus;
