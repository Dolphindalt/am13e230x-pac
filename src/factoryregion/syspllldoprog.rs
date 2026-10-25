#[doc = "Register `SYSPLLLDOPROG` reader"]
pub type R = crate::R<SyspllldoprogSpec>;
#[doc = "Field `LDOVOUTPROGLOWV` reader - LDOVOUTPROGLOWV"]
pub type LdovoutproglowvR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:2 - LDOVOUTPROGLOWV"]
    #[inline(always)]
    pub fn ldovoutproglowv(&self) -> LdovoutproglowvR {
        LdovoutproglowvR::new((self.bits & 7) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SYSPLLLDOPROG")
            .field("ldovoutproglowv", &self.ldovoutproglowv())
            .finish()
    }
}
#[doc = "SYSPLLLDOPROG\n\nYou can [`read`](crate::Reg::read) this register and get [`syspllldoprog::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SyspllldoprogSpec;
impl crate::RegisterSpec for SyspllldoprogSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syspllldoprog::R`](R) reader structure"]
impl crate::Readable for SyspllldoprogSpec {}
#[doc = "`reset()` method sets SYSPLLLDOPROG to value 0"]
impl crate::Resettable for SyspllldoprogSpec {}
