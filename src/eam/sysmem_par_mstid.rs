#[doc = "Register `SYSMEM_PAR_MSTID` reader"]
pub type R = crate::R<SysmemParMstidSpec>;
#[doc = "Register `SYSMEM_PAR_MSTID` writer"]
pub type W = crate::W<SysmemParMstidSpec>;
#[doc = "Field `MSTID` reader - Master ID"]
pub type MstidR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Master ID"]
    #[inline(always)]
    pub fn mstid(&self) -> MstidR {
        MstidR::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SYSMEM_PAR_MSTID")
            .field("mstid", &self.mstid())
            .finish()
    }
}
impl W {}
#[doc = "SYSMEM DED master id\n\nYou can [`read`](crate::Reg::read) this register and get [`sysmem_par_mstid::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysmem_par_mstid::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysmemParMstidSpec;
impl crate::RegisterSpec for SysmemParMstidSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sysmem_par_mstid::R`](R) reader structure"]
impl crate::Readable for SysmemParMstidSpec {}
#[doc = "`write(|w| ..)` method takes [`sysmem_par_mstid::W`](W) writer structure"]
impl crate::Writable for SysmemParMstidSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets SYSMEM_PAR_MSTID to value 0"]
impl crate::Resettable for SysmemParMstidSpec {}
