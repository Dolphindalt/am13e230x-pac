#[doc = "Register `TMUROM_PAR_TYPE` reader"]
pub type R = crate::R<TmuromParTypeSpec>;
#[doc = "Register `TMUROM_PAR_TYPE` writer"]
pub type W = crate::W<TmuromParTypeSpec>;
#[doc = "Field `TYPE` reader - Master ID"]
pub type TypeR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:2 - Master ID"]
    #[inline(always)]
    pub fn type_(&self) -> TypeR {
        TypeR::new((self.bits & 7) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TMUROM_PAR_TYPE")
            .field("type_", &self.type_())
            .finish()
    }
}
impl W {}
#[doc = "SYSMEM DED master id\n\nYou can [`read`](crate::Reg::read) this register and get [`tmurom_par_type::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmurom_par_type::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TmuromParTypeSpec;
impl crate::RegisterSpec for TmuromParTypeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tmurom_par_type::R`](R) reader structure"]
impl crate::Readable for TmuromParTypeSpec {}
#[doc = "`write(|w| ..)` method takes [`tmurom_par_type::W`](W) writer structure"]
impl crate::Writable for TmuromParTypeSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets TMUROM_PAR_TYPE to value 0"]
impl crate::Resettable for TmuromParTypeSpec {}
