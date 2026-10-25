#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[doc = "TI_WRAPPER"]
#[doc(alias = "TI_WRAPPER")]
pub struct TiWrapper {
    processors: Processors,
    _reserved1: [u8; 0x01f0],
    msp: Msp,
}
impl TiWrapper {
    #[doc = "0x00..0x410 - PROCESSORS"]
    #[inline(always)]
    pub const fn processors(&self) -> &Processors {
        &self.processors
    }
    #[doc = "0x600..0x710 - MSP"]
    #[inline(always)]
    pub const fn msp(&self) -> &Msp {
        &self.msp
    }
}
#[doc = "PROCESSORS"]
pub use self::processors::Processors;
#[doc = r"Cluster"]
#[doc = "PROCESSORS"]
pub mod processors;
#[doc = "MSP"]
pub use self::msp::Msp;
#[doc = r"Cluster"]
#[doc = "MSP"]
pub mod msp;
