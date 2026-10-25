#[doc = "Register `NMIRIS` reader"]
pub type R = crate::R<NmirisSpec>;
#[doc = "Field `WWDT0` reader - Watch Dog 0 Fault"]
pub type Wwdt0R = crate::BitReader;
#[doc = "Field `SECURITY` reader - Security Fault"]
pub type SecurityR = crate::BitReader;
#[doc = "Field `FLASHDED` reader - Flash Double Error Detect"]
pub type FlashdedR = crate::BitReader;
#[doc = "Field `SRAMPAR` reader - SRAM Parity Error Detect"]
pub type SramparR = crate::BitReader;
#[doc = "Field `TMUROMPAR` reader - TMU ROM Parity error"]
pub type TmuromparR = crate::BitReader;
#[doc = "Field `SYSMEMACC` reader - SYSMEM Access error"]
pub type SysmemaccR = crate::BitReader;
impl R {
    #[doc = "Bit 1 - Watch Dog 0 Fault"]
    #[inline(always)]
    pub fn wwdt0(&self) -> Wwdt0R {
        Wwdt0R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Security Fault"]
    #[inline(always)]
    pub fn security(&self) -> SecurityR {
        SecurityR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Flash Double Error Detect"]
    #[inline(always)]
    pub fn flashded(&self) -> FlashdedR {
        FlashdedR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SRAM Parity Error Detect"]
    #[inline(always)]
    pub fn srampar(&self) -> SramparR {
        SramparR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TMU ROM Parity error"]
    #[inline(always)]
    pub fn tmurompar(&self) -> TmuromparR {
        TmuromparR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SYSMEM Access error"]
    #[inline(always)]
    pub fn sysmemacc(&self) -> SysmemaccR {
        SysmemaccR::new(((self.bits >> 6) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NMIRIS")
            .field("security", &self.security())
            .field("sysmemacc", &self.sysmemacc())
            .field("tmurompar", &self.tmurompar())
            .field("flashded", &self.flashded())
            .field("wwdt0", &self.wwdt0())
            .field("srampar", &self.srampar())
            .finish()
    }
}
#[doc = "NMI raw interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`nmiris::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NmirisSpec;
impl crate::RegisterSpec for NmirisSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nmiris::R`](R) reader structure"]
impl crate::Readable for NmirisSpec {}
#[doc = "`reset()` method sets NMIRIS to value 0"]
impl crate::Resettable for NmirisSpec {}
