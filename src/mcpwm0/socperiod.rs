#[doc = "Register `SOCPERIOD` reader"]
pub type R = crate::R<SocperiodSpec>;
#[doc = "Register `SOCPERIOD` writer"]
pub type W = crate::W<SocperiodSpec>;
#[doc = "Field `SOCA_PERIOD` reader - Start of conversion (SOC) A PERIOD"]
pub type SocaPeriodR = crate::FieldReader;
#[doc = "Field `SOCA_PERIOD` writer - Start of conversion (SOC) A PERIOD"]
pub type SocaPeriodW<'a, REG> = crate::FieldWriter<'a, REG, 3, u8, crate::Safe>;
#[doc = "Field `SOCB_PERIOD` reader - Start of conversion (SOC) B PERIOD"]
pub type SocbPeriodR = crate::FieldReader;
#[doc = "Field `SOCB_PERIOD` writer - Start of conversion (SOC) B PERIOD"]
pub type SocbPeriodW<'a, REG> = crate::FieldWriter<'a, REG, 3, u8, crate::Safe>;
#[doc = "Field `SOCC_PERIOD` reader - Start of conversion (SOC) C PERIOD"]
pub type SoccPeriodR = crate::FieldReader;
#[doc = "Field `SOCC_PERIOD` writer - Start of conversion (SOC) C PERIOD"]
pub type SoccPeriodW<'a, REG> = crate::FieldWriter<'a, REG, 3, u8, crate::Safe>;
#[doc = "Field `SOCD_PERIOD` reader - Start of conversion (SOC) D PERIOD"]
pub type SocdPeriodR = crate::FieldReader;
#[doc = "Field `SOCD_PERIOD` writer - Start of conversion (SOC) D PERIOD"]
pub type SocdPeriodW<'a, REG> = crate::FieldWriter<'a, REG, 3, u8, crate::Safe>;
impl R {
    #[doc = "Bits 0:2 - Start of conversion (SOC) A PERIOD"]
    #[inline(always)]
    pub fn soca_period(&self) -> SocaPeriodR {
        SocaPeriodR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:10 - Start of conversion (SOC) B PERIOD"]
    #[inline(always)]
    pub fn socb_period(&self) -> SocbPeriodR {
        SocbPeriodR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 16:18 - Start of conversion (SOC) C PERIOD"]
    #[inline(always)]
    pub fn socc_period(&self) -> SoccPeriodR {
        SoccPeriodR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 24:26 - Start of conversion (SOC) D PERIOD"]
    #[inline(always)]
    pub fn socd_period(&self) -> SocdPeriodR {
        SocdPeriodR::new(((self.bits >> 24) & 7) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SOCPERIOD")
            .field("soca_period", &self.soca_period())
            .field("socb_period", &self.socb_period())
            .field("socc_period", &self.socc_period())
            .field("socd_period", &self.socd_period())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - Start of conversion (SOC) A PERIOD"]
    #[inline(always)]
    pub fn soca_period(&mut self) -> SocaPeriodW<'_, SocperiodSpec> {
        SocaPeriodW::new(self, 0)
    }
    #[doc = "Bits 8:10 - Start of conversion (SOC) B PERIOD"]
    #[inline(always)]
    pub fn socb_period(&mut self) -> SocbPeriodW<'_, SocperiodSpec> {
        SocbPeriodW::new(self, 8)
    }
    #[doc = "Bits 16:18 - Start of conversion (SOC) C PERIOD"]
    #[inline(always)]
    pub fn socc_period(&mut self) -> SoccPeriodW<'_, SocperiodSpec> {
        SoccPeriodW::new(self, 16)
    }
    #[doc = "Bits 24:26 - Start of conversion (SOC) D PERIOD"]
    #[inline(always)]
    pub fn socd_period(&mut self) -> SocdPeriodW<'_, SocperiodSpec> {
        SocdPeriodW::new(self, 24)
    }
}
#[doc = "Start of conversion period\n\nYou can [`read`](crate::Reg::read) this register and get [`socperiod::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`socperiod::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SocperiodSpec;
impl crate::RegisterSpec for SocperiodSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`socperiod::R`](R) reader structure"]
impl crate::Readable for SocperiodSpec {}
#[doc = "`write(|w| ..)` method takes [`socperiod::W`](W) writer structure"]
impl crate::Writable for SocperiodSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets SOCPERIOD to value 0"]
impl crate::Resettable for SocperiodSpec {}
