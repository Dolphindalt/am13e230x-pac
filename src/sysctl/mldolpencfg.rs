#[doc = "Register `MLDOLPENCFG` reader"]
pub type R = crate::R<MldolpencfgSpec>;
#[doc = "Register `MLDOLPENCFG` writer"]
pub type W = crate::W<MldolpencfgSpec>;
#[doc = "Field `CVLODIS` reader - Control to disable lowering the core voltage for STOP and STANDBY"]
pub type CvlodisR = crate::BitReader;
#[doc = "Field `CVLODIS` writer - Control to disable lowering the core voltage for STOP and STANDBY"]
pub type CvlodisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Control to disable lowering the core voltage for STOP and STANDBY"]
    #[inline(always)]
    pub fn cvlodis(&self) -> CvlodisR {
        CvlodisR::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MLDOLPENCFG")
            .field("cvlodis", &self.cvlodis())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Control to disable lowering the core voltage for STOP and STANDBY"]
    #[inline(always)]
    pub fn cvlodis(&mut self) -> CvlodisW<'_, MldolpencfgSpec> {
        CvlodisW::new(self, 0)
    }
}
#[doc = "LDO Configuration Control\n\nYou can [`read`](crate::Reg::read) this register and get [`mldolpencfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mldolpencfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MldolpencfgSpec;
impl crate::RegisterSpec for MldolpencfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mldolpencfg::R`](R) reader structure"]
impl crate::Readable for MldolpencfgSpec {}
#[doc = "`write(|w| ..)` method takes [`mldolpencfg::W`](W) writer structure"]
impl crate::Writable for MldolpencfgSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets MLDOLPENCFG to value 0"]
impl crate::Resettable for MldolpencfgSpec {}
