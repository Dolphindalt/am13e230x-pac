#[doc = "Register `SYSMEM_PAR_ADDR` reader"]
pub type R = crate::R<SysmemParAddrSpec>;
#[doc = "Register `SYSMEM_PAR_ADDR` writer"]
pub type W = crate::W<SysmemParAddrSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "SYSMEM DED address\n\nYou can [`read`](crate::Reg::read) this register and get [`sysmem_par_addr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysmem_par_addr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysmemParAddrSpec;
impl crate::RegisterSpec for SysmemParAddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sysmem_par_addr::R`](R) reader structure"]
impl crate::Readable for SysmemParAddrSpec {}
#[doc = "`write(|w| ..)` method takes [`sysmem_par_addr::W`](W) writer structure"]
impl crate::Writable for SysmemParAddrSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets SYSMEM_PAR_ADDR to value 0"]
impl crate::Resettable for SysmemParAddrSpec {}
