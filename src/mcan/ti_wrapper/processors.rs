#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[doc = "PROCESSORS"]
#[doc(alias = "PROCESSORS")]
pub struct Processors {
    regs: Regs,
    _reserved1: [u8; 0x01d4],
    ecc_regs: EccRegs,
}
impl Processors {
    #[doc = "0x00..0x2c - REGS"]
    #[inline(always)]
    pub const fn regs(&self) -> &Regs {
        &self.regs
    }
    #[doc = "0x200..0x410 - ECC_REGS"]
    #[inline(always)]
    pub const fn ecc_regs(&self) -> &EccRegs {
        &self.ecc_regs
    }
}
#[doc = "REGS"]
pub use self::regs::Regs;
#[doc = r"Cluster"]
#[doc = "REGS"]
pub mod regs;
#[doc = "ECC_REGS"]
pub use self::ecc_regs::EccRegs;
#[doc = r"Cluster"]
#[doc = "ECC_REGS"]
pub mod ecc_regs;
