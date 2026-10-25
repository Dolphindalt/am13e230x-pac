#[doc = "Register `SYSPLLLDOCTL` reader"]
pub type R = crate::R<SyspllldoctlSpec>;
#[doc = "Field `LDOCTLLOWV` reader - LDOCTLLOWV"]
pub type LdoctllowvR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - LDOCTLLOWV"]
    #[inline(always)]
    pub fn ldoctllowv(&self) -> LdoctllowvR {
        LdoctllowvR::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SYSPLLLDOCTL")
            .field("ldoctllowv", &self.ldoctllowv())
            .finish()
    }
}
#[doc = "SYSPLLLDOCTL\n\nYou can [`read`](crate::Reg::read) this register and get [`syspllldoctl::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SyspllldoctlSpec;
impl crate::RegisterSpec for SyspllldoctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syspllldoctl::R`](R) reader structure"]
impl crate::Readable for SyspllldoctlSpec {}
#[doc = "`reset()` method sets SYSPLLLDOCTL to value 0"]
impl crate::Resettable for SyspllldoctlSpec {}
