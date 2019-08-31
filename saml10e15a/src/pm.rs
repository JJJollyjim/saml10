#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 1usize],
    #[doc = "0x01 - Sleep Configuration"]
    pub sleepcfg: SLEEPCFG,
    #[doc = "0x02 - Performance Level Configuration"]
    pub plcfg: PLCFG,
    #[doc = "0x03 - Power Configuration"]
    pub pwcfg: PWCFG,
    #[doc = "0x04 - Interrupt Enable Clear"]
    pub intenclr: INTENCLR,
    #[doc = "0x05 - Interrupt Enable Set"]
    pub intenset: INTENSET,
    #[doc = "0x06 - Interrupt Flag Status and Clear"]
    pub intflag: INTFLAG,
    _reserved6: [u8; 1usize],
    #[doc = "0x08 - Standby Configuration"]
    pub stdbycfg: STDBYCFG,
}
#[doc = "Sleep Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sleepcfg](sleepcfg) module"]
pub type SLEEPCFG = crate::Reg<u8, _SLEEPCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SLEEPCFG;
#[doc = "`read()` method returns [sleepcfg::R](sleepcfg::R) reader structure"]
impl crate::Readable for SLEEPCFG {}
#[doc = "`write(|w| ..)` method takes [sleepcfg::W](sleepcfg::W) writer structure"]
impl crate::Writable for SLEEPCFG {}
#[doc = "Sleep Configuration"]
pub mod sleepcfg;
#[doc = "Performance Level Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [plcfg](plcfg) module"]
pub type PLCFG = crate::Reg<u8, _PLCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PLCFG;
#[doc = "`read()` method returns [plcfg::R](plcfg::R) reader structure"]
impl crate::Readable for PLCFG {}
#[doc = "`write(|w| ..)` method takes [plcfg::W](plcfg::W) writer structure"]
impl crate::Writable for PLCFG {}
#[doc = "Performance Level Configuration"]
pub mod plcfg;
#[doc = "Power Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pwcfg](pwcfg) module"]
pub type PWCFG = crate::Reg<u8, _PWCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWCFG;
#[doc = "`read()` method returns [pwcfg::R](pwcfg::R) reader structure"]
impl crate::Readable for PWCFG {}
#[doc = "`write(|w| ..)` method takes [pwcfg::W](pwcfg::W) writer structure"]
impl crate::Writable for PWCFG {}
#[doc = "Power Configuration"]
pub mod pwcfg;
#[doc = "Interrupt Enable Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [intenclr](intenclr) module"]
pub type INTENCLR = crate::Reg<u8, _INTENCLR>;
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
pub type INTENSET = crate::Reg<u8, _INTENSET>;
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
pub type INTFLAG = crate::Reg<u8, _INTFLAG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTFLAG;
#[doc = "`read()` method returns [intflag::R](intflag::R) reader structure"]
impl crate::Readable for INTFLAG {}
#[doc = "`write(|w| ..)` method takes [intflag::W](intflag::W) writer structure"]
impl crate::Writable for INTFLAG {}
#[doc = "Interrupt Flag Status and Clear"]
pub mod intflag;
#[doc = "Standby Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [stdbycfg](stdbycfg) module"]
pub type STDBYCFG = crate::Reg<u16, _STDBYCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STDBYCFG;
#[doc = "`read()` method returns [stdbycfg::R](stdbycfg::R) reader structure"]
impl crate::Readable for STDBYCFG {}
#[doc = "`write(|w| ..)` method takes [stdbycfg::W](stdbycfg::W) writer structure"]
impl crate::Writable for STDBYCFG {}
#[doc = "Standby Configuration"]
pub mod stdbycfg;
