#[doc = "Register `ETPERIOD` reader"]
pub type R = crate::R<EtperiodSpec>;
#[doc = "Register `ETPERIOD` writer"]
pub type W = crate::W<EtperiodSpec>;
#[doc = "Field `ET1_PERIOD` reader - Event trigger1 PERIOD"]
pub type Et1PeriodR = crate::FieldReader;
#[doc = "Field `ET1_PERIOD` writer - Event trigger1 PERIOD"]
pub type Et1PeriodW<'a, REG> = crate::FieldWriter<'a, REG, 3, u8, crate::Safe>;
#[doc = "Field `ET2_PERIOD` reader - Event trigger2 PERIOD"]
pub type Et2PeriodR = crate::FieldReader;
#[doc = "Field `ET2_PERIOD` writer - Event trigger2 PERIOD"]
pub type Et2PeriodW<'a, REG> = crate::FieldWriter<'a, REG, 3, u8, crate::Safe>;
impl R {
    #[doc = "Bits 0:2 - Event trigger1 PERIOD"]
    #[inline(always)]
    pub fn et1_period(&self) -> Et1PeriodR {
        Et1PeriodR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:10 - Event trigger2 PERIOD"]
    #[inline(always)]
    pub fn et2_period(&self) -> Et2PeriodR {
        Et2PeriodR::new(((self.bits >> 8) & 7) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ETPERIOD")
            .field("et1_period", &self.et1_period())
            .field("et2_period", &self.et2_period())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - Event trigger1 PERIOD"]
    #[inline(always)]
    pub fn et1_period(&mut self) -> Et1PeriodW<'_, EtperiodSpec> {
        Et1PeriodW::new(self, 0)
    }
    #[doc = "Bits 8:10 - Event trigger2 PERIOD"]
    #[inline(always)]
    pub fn et2_period(&mut self) -> Et2PeriodW<'_, EtperiodSpec> {
        Et2PeriodW::new(self, 8)
    }
}
#[doc = "Event trigger period\n\nYou can [`read`](crate::Reg::read) this register and get [`etperiod::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`etperiod::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EtperiodSpec;
impl crate::RegisterSpec for EtperiodSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`etperiod::R`](R) reader structure"]
impl crate::Readable for EtperiodSpec {}
#[doc = "`write(|w| ..)` method takes [`etperiod::W`](W) writer structure"]
impl crate::Writable for EtperiodSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets ETPERIOD to value 0"]
impl crate::Resettable for EtperiodSpec {}
