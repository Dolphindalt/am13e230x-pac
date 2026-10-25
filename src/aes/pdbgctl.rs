#[doc = "Register `PDBGCTL` reader"]
pub type R = crate::R<PdbgctlSpec>;
#[doc = "Field `FREE` reader - Free run control"]
pub type FreeR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Free run control"]
    #[inline(always)]
    pub fn free(&self) -> FreeR {
        FreeR::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PDBGCTL")
            .field("free", &self.free())
            .finish()
    }
}
#[doc = "Peripheral Debug Control\n\nYou can [`read`](crate::Reg::read) this register and get [`pdbgctl::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PdbgctlSpec;
impl crate::RegisterSpec for PdbgctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pdbgctl::R`](R) reader structure"]
impl crate::Readable for PdbgctlSpec {}
#[doc = "`reset()` method sets PDBGCTL to value 0"]
impl crate::Resettable for PdbgctlSpec {}
