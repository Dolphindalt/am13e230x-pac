#[doc = "Register `ADC_MMR_OVRD_CTL` reader"]
pub type R = crate::R<AdcMmrOvrdCtlSpec>;
#[doc = "Register `ADC_MMR_OVRD_CTL` writer"]
pub type W = crate::W<AdcMmrOvrdCtlSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "ADC MMR Override control register for DFT: Control ADC enable override\n\nYou can [`read`](crate::Reg::read) this register and get [`adc_mmr_ovrd_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_mmr_ovrd_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AdcMmrOvrdCtlSpec;
impl crate::RegisterSpec for AdcMmrOvrdCtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adc_mmr_ovrd_ctl::R`](R) reader structure"]
impl crate::Readable for AdcMmrOvrdCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`adc_mmr_ovrd_ctl::W`](W) writer structure"]
impl crate::Writable for AdcMmrOvrdCtlSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets ADC_MMR_OVRD_CTL to value 0"]
impl crate::Resettable for AdcMmrOvrdCtlSpec {}
