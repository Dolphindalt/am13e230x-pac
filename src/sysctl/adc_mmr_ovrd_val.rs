#[doc = "Register `ADC_MMR_OVRD_VAL` reader"]
pub type R = crate::R<AdcMmrOvrdValSpec>;
#[doc = "Register `ADC_MMR_OVRD_VAL` writer"]
pub type W = crate::W<AdcMmrOvrdValSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "ADC MMR Override value register for DFT : Value of ADC enable override\n\nYou can [`read`](crate::Reg::read) this register and get [`adc_mmr_ovrd_val::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_mmr_ovrd_val::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AdcMmrOvrdValSpec;
impl crate::RegisterSpec for AdcMmrOvrdValSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adc_mmr_ovrd_val::R`](R) reader structure"]
impl crate::Readable for AdcMmrOvrdValSpec {}
#[doc = "`write(|w| ..)` method takes [`adc_mmr_ovrd_val::W`](W) writer structure"]
impl crate::Writable for AdcMmrOvrdValSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets ADC_MMR_OVRD_VAL to value 0"]
impl crate::Resettable for AdcMmrOvrdValSpec {}
