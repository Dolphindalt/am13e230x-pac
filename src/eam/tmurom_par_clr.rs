#[doc = "Register `TMUROM_PAR_CLR` reader"]
pub type R = crate::R<TmuromParClrSpec>;
#[doc = "Register `TMUROM_PAR_CLR` writer"]
pub type W = crate::W<TmuromParClrSpec>;
#[doc = "Field `PAR` reader - Double error detect error"]
pub type ParR = crate::BitReader;
#[doc = "Field `PAR` writer - Double error detect error"]
pub type ParW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Double error detect error"]
    #[inline(always)]
    pub fn par(&self) -> ParR {
        ParR::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TMUROM_PAR_CLR")
            .field("par", &self.par())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Double error detect error"]
    #[inline(always)]
    pub fn par(&mut self) -> ParW<'_, TmuromParClrSpec> {
        ParW::new(self, 0)
    }
}
#[doc = "SYSMEM DED clear\n\nYou can [`read`](crate::Reg::read) this register and get [`tmurom_par_clr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmurom_par_clr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TmuromParClrSpec;
impl crate::RegisterSpec for TmuromParClrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tmurom_par_clr::R`](R) reader structure"]
impl crate::Readable for TmuromParClrSpec {}
#[doc = "`write(|w| ..)` method takes [`tmurom_par_clr::W`](W) writer structure"]
impl crate::Writable for TmuromParClrSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets TMUROM_PAR_CLR to value 0"]
impl crate::Resettable for TmuromParClrSpec {}
