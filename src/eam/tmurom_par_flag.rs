#[doc = "Register `TMUROM_PAR_FLAG` reader"]
pub type R = crate::R<TmuromParFlagSpec>;
#[doc = "Register `TMUROM_PAR_FLAG` writer"]
pub type W = crate::W<TmuromParFlagSpec>;
#[doc = "Field `PAR` reader - Double error detect error during read"]
pub type ParR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Double error detect error during read"]
    #[inline(always)]
    pub fn par(&self) -> ParR {
        ParR::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TMUROM_PAR_FLAG")
            .field("par", &self.par())
            .finish()
    }
}
impl W {}
#[doc = "SYSMEM DED flag\n\nYou can [`read`](crate::Reg::read) this register and get [`tmurom_par_flag::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmurom_par_flag::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TmuromParFlagSpec;
impl crate::RegisterSpec for TmuromParFlagSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tmurom_par_flag::R`](R) reader structure"]
impl crate::Readable for TmuromParFlagSpec {}
#[doc = "`write(|w| ..)` method takes [`tmurom_par_flag::W`](W) writer structure"]
impl crate::Writable for TmuromParFlagSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets TMUROM_PAR_FLAG to value 0"]
impl crate::Resettable for TmuromParFlagSpec {}
