#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SCS Software Lock Access Register"]
    pub dlar: DLAR,
    #[doc = "0x04 - SCS Software Lock Status Register"]
    pub dlsr: DLSR,
    #[doc = "0x08 - Debug Authentication Status Register"]
    pub dauthstatus: DAUTHSTATUS,
    #[doc = "0x0c - SCS Device Architecture Register"]
    pub ddevarch: DDEVARCH,
    _reserved4: [u8; 12usize],
    #[doc = "0x1c - SCS Device Type Register"]
    pub ddevtype: DDEVTYPE,
    #[doc = "0x20 - SCS Peripheral Identification Register 4"]
    pub dpidr4: DPIDR4,
    #[doc = "0x24 - SCS Peripheral Identification Register 5"]
    pub dpidr5: DPIDR5,
    #[doc = "0x28 - SCS Peripheral Identification Register 6"]
    pub dpidr6: DPIDR6,
    #[doc = "0x2c - SCS Peripheral Identification Register 7"]
    pub dpidr7: DPIDR7,
    #[doc = "0x30 - SCS Peripheral Identification Register 0"]
    pub dpidr0: DPIDR0,
    #[doc = "0x34 - SCS Peripheral Identification Register 1"]
    pub dpidr1: DPIDR1,
    #[doc = "0x38 - SCS Peripheral Identification Register 2"]
    pub dpidr2: DPIDR2,
    #[doc = "0x3c - SCS Peripheral Identification Register 3"]
    pub dpidr3: DPIDR3,
    #[doc = "0x40 - SCS Component Identification Register 0"]
    pub dcidr0: DCIDR0,
    #[doc = "0x44 - SCS Component Identification Register 1"]
    pub dcidr1: DCIDR1,
    #[doc = "0x48 - SCS Component Identification Register 2"]
    pub dcidr2: DCIDR2,
    #[doc = "0x4c - SCS Component Identification Register 3"]
    pub dcidr3: DCIDR3,
}
#[doc = "SCS Software Lock Access Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dlar](dlar) module"]
pub type DLAR = crate::Reg<u32, _DLAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DLAR;
#[doc = "`write(|w| ..)` method takes [dlar::W](dlar::W) writer structure"]
impl crate::Writable for DLAR {}
#[doc = "SCS Software Lock Access Register"]
pub mod dlar;
#[doc = "SCS Software Lock Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dlsr](dlsr) module"]
pub type DLSR = crate::Reg<u32, _DLSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DLSR;
#[doc = "`read()` method returns [dlsr::R](dlsr::R) reader structure"]
impl crate::Readable for DLSR {}
#[doc = "SCS Software Lock Status Register"]
pub mod dlsr;
#[doc = "Debug Authentication Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dauthstatus](dauthstatus) module"]
pub type DAUTHSTATUS = crate::Reg<u32, _DAUTHSTATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DAUTHSTATUS;
#[doc = "`read()` method returns [dauthstatus::R](dauthstatus::R) reader structure"]
impl crate::Readable for DAUTHSTATUS {}
#[doc = "Debug Authentication Status Register"]
pub mod dauthstatus;
#[doc = "SCS Device Architecture Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ddevarch](ddevarch) module"]
pub type DDEVARCH = crate::Reg<u32, _DDEVARCH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDEVARCH;
#[doc = "`read()` method returns [ddevarch::R](ddevarch::R) reader structure"]
impl crate::Readable for DDEVARCH {}
#[doc = "SCS Device Architecture Register"]
pub mod ddevarch;
#[doc = "SCS Device Type Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ddevtype](ddevtype) module"]
pub type DDEVTYPE = crate::Reg<u32, _DDEVTYPE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDEVTYPE;
#[doc = "`read()` method returns [ddevtype::R](ddevtype::R) reader structure"]
impl crate::Readable for DDEVTYPE {}
#[doc = "SCS Device Type Register"]
pub mod ddevtype;
#[doc = "SCS Peripheral Identification Register 4\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dpidr4](dpidr4) module"]
pub type DPIDR4 = crate::Reg<u32, _DPIDR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPIDR4;
#[doc = "`read()` method returns [dpidr4::R](dpidr4::R) reader structure"]
impl crate::Readable for DPIDR4 {}
#[doc = "SCS Peripheral Identification Register 4"]
pub mod dpidr4;
#[doc = "SCS Peripheral Identification Register 5\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dpidr5](dpidr5) module"]
pub type DPIDR5 = crate::Reg<u32, _DPIDR5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPIDR5;
#[doc = "`read()` method returns [dpidr5::R](dpidr5::R) reader structure"]
impl crate::Readable for DPIDR5 {}
#[doc = "SCS Peripheral Identification Register 5"]
pub mod dpidr5;
#[doc = "SCS Peripheral Identification Register 6\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dpidr6](dpidr6) module"]
pub type DPIDR6 = crate::Reg<u32, _DPIDR6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPIDR6;
#[doc = "`read()` method returns [dpidr6::R](dpidr6::R) reader structure"]
impl crate::Readable for DPIDR6 {}
#[doc = "SCS Peripheral Identification Register 6"]
pub mod dpidr6;
#[doc = "SCS Peripheral Identification Register 7\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dpidr7](dpidr7) module"]
pub type DPIDR7 = crate::Reg<u32, _DPIDR7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPIDR7;
#[doc = "`read()` method returns [dpidr7::R](dpidr7::R) reader structure"]
impl crate::Readable for DPIDR7 {}
#[doc = "SCS Peripheral Identification Register 7"]
pub mod dpidr7;
#[doc = "SCS Peripheral Identification Register 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dpidr0](dpidr0) module"]
pub type DPIDR0 = crate::Reg<u32, _DPIDR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPIDR0;
#[doc = "`read()` method returns [dpidr0::R](dpidr0::R) reader structure"]
impl crate::Readable for DPIDR0 {}
#[doc = "SCS Peripheral Identification Register 0"]
pub mod dpidr0;
#[doc = "SCS Peripheral Identification Register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dpidr1](dpidr1) module"]
pub type DPIDR1 = crate::Reg<u32, _DPIDR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPIDR1;
#[doc = "`read()` method returns [dpidr1::R](dpidr1::R) reader structure"]
impl crate::Readable for DPIDR1 {}
#[doc = "SCS Peripheral Identification Register 1"]
pub mod dpidr1;
#[doc = "SCS Peripheral Identification Register 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dpidr2](dpidr2) module"]
pub type DPIDR2 = crate::Reg<u32, _DPIDR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPIDR2;
#[doc = "`read()` method returns [dpidr2::R](dpidr2::R) reader structure"]
impl crate::Readable for DPIDR2 {}
#[doc = "SCS Peripheral Identification Register 2"]
pub mod dpidr2;
#[doc = "SCS Peripheral Identification Register 3\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dpidr3](dpidr3) module"]
pub type DPIDR3 = crate::Reg<u32, _DPIDR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPIDR3;
#[doc = "`read()` method returns [dpidr3::R](dpidr3::R) reader structure"]
impl crate::Readable for DPIDR3 {}
#[doc = "SCS Peripheral Identification Register 3"]
pub mod dpidr3;
#[doc = "SCS Component Identification Register 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dcidr0](dcidr0) module"]
pub type DCIDR0 = crate::Reg<u32, _DCIDR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCIDR0;
#[doc = "`read()` method returns [dcidr0::R](dcidr0::R) reader structure"]
impl crate::Readable for DCIDR0 {}
#[doc = "SCS Component Identification Register 0"]
pub mod dcidr0;
#[doc = "SCS Component Identification Register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dcidr1](dcidr1) module"]
pub type DCIDR1 = crate::Reg<u32, _DCIDR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCIDR1;
#[doc = "`read()` method returns [dcidr1::R](dcidr1::R) reader structure"]
impl crate::Readable for DCIDR1 {}
#[doc = "SCS Component Identification Register 1"]
pub mod dcidr1;
#[doc = "SCS Component Identification Register 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dcidr2](dcidr2) module"]
pub type DCIDR2 = crate::Reg<u32, _DCIDR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCIDR2;
#[doc = "`read()` method returns [dcidr2::R](dcidr2::R) reader structure"]
impl crate::Readable for DCIDR2 {}
#[doc = "SCS Component Identification Register 2"]
pub mod dcidr2;
#[doc = "SCS Component Identification Register 3\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dcidr3](dcidr3) module"]
pub type DCIDR3 = crate::Reg<u32, _DCIDR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCIDR3;
#[doc = "`read()` method returns [dcidr3::R](dcidr3::R) reader structure"]
impl crate::Readable for DCIDR3 {}
#[doc = "SCS Component Identification Register 3"]
pub mod dcidr3;
