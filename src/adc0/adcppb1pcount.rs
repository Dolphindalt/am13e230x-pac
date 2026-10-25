#[doc = "Register `ADCPPB1PCOUNT` reader"]
pub type R = crate::R<Adcppb1pcountSpec>;
#[doc = "Register `ADCPPB1PCOUNT` writer"]
pub type W = crate::W<Adcppb1pcountSpec>;
#[doc = "Field `PCOUNT` reader - Post Processing Block 1 Partial Count"]
pub type PcountR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:2 - Post Processing Block 1 Partial Count"]
    #[inline(always)]
    pub fn pcount(&self) -> PcountR {
        PcountR::new((self.bits & 7) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADCPPB1PCOUNT")
            .field("pcount", &self.pcount())
            .finish()
    }
}
impl W {}
#[doc = "ADC PPB1 Partial Conversion Count Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcppb1pcount::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcppb1pcount::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adcppb1pcountSpec;
impl crate::RegisterSpec for Adcppb1pcountSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adcppb1pcount::R`](R) reader structure"]
impl crate::Readable for Adcppb1pcountSpec {}
#[doc = "`write(|w| ..)` method takes [`adcppb1pcount::W`](W) writer structure"]
impl crate::Writable for Adcppb1pcountSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets ADCPPB1PCOUNT to value 0"]
impl crate::Resettable for Adcppb1pcountSpec {}
