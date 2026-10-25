#[doc = "Register `ADCSEQFRCGBSEL` reader"]
pub type R = crate::R<AdcseqfrcgbselSpec>;
#[doc = "Register `ADCSEQFRCGBSEL` writer"]
pub type W = crate::W<AdcseqfrcgbselSpec>;
#[doc = "Field `ADCA` reader - Generate synchronous SW trigger for ADCA"]
pub type AdcaR = crate::BitReader;
#[doc = "Field `ADCA` writer - Generate synchronous SW trigger for ADCA"]
pub type AdcaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADCB` reader - Generate synchronous SW trigger for ADCB"]
pub type AdcbR = crate::BitReader;
#[doc = "Field `ADCB` writer - Generate synchronous SW trigger for ADCB"]
pub type AdcbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADCC` reader - Generate synchronous SW trigger for ADCC"]
pub type AdccR = crate::BitReader;
#[doc = "Field `ADCC` writer - Generate synchronous SW trigger for ADCC"]
pub type AdccW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Generate synchronous SW trigger for ADCA"]
    #[inline(always)]
    pub fn adca(&self) -> AdcaR {
        AdcaR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Generate synchronous SW trigger for ADCB"]
    #[inline(always)]
    pub fn adcb(&self) -> AdcbR {
        AdcbR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Generate synchronous SW trigger for ADCC"]
    #[inline(always)]
    pub fn adcc(&self) -> AdccR {
        AdccR::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADCSEQFRCGBSEL")
            .field("adcb", &self.adcb())
            .field("adcc", &self.adcc())
            .field("adca", &self.adca())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Generate synchronous SW trigger for ADCA"]
    #[inline(always)]
    pub fn adca(&mut self) -> AdcaW<'_, AdcseqfrcgbselSpec> {
        AdcaW::new(self, 0)
    }
    #[doc = "Bit 1 - Generate synchronous SW trigger for ADCB"]
    #[inline(always)]
    pub fn adcb(&mut self) -> AdcbW<'_, AdcseqfrcgbselSpec> {
        AdcbW::new(self, 1)
    }
    #[doc = "Bit 2 - Generate synchronous SW trigger for ADCC"]
    #[inline(always)]
    pub fn adcc(&mut self) -> AdccW<'_, AdcseqfrcgbselSpec> {
        AdccW::new(self, 2)
    }
}
#[doc = "ADC Global Sequence Force Select\n\nYou can [`read`](crate::Reg::read) this register and get [`adcseqfrcgbsel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcseqfrcgbsel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AdcseqfrcgbselSpec;
impl crate::RegisterSpec for AdcseqfrcgbselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adcseqfrcgbsel::R`](R) reader structure"]
impl crate::Readable for AdcseqfrcgbselSpec {}
#[doc = "`write(|w| ..)` method takes [`adcseqfrcgbsel::W`](W) writer structure"]
impl crate::Writable for AdcseqfrcgbselSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets ADCSEQFRCGBSEL to value 0"]
impl crate::Resettable for AdcseqfrcgbselSpec {}
