#[doc = "Register `ADCPPB4COUNT` reader"]
pub type R = crate::R<Adcppb4countSpec>;
#[doc = "Register `ADCPPB4COUNT` writer"]
pub type W = crate::W<Adcppb4countSpec>;
#[doc = "Field `COUNT` reader - Post Processing Block 4 Final Count"]
pub type CountR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Post Processing Block 4 Final Count"]
    #[inline(always)]
    pub fn count(&self) -> CountR {
        CountR::new((self.bits & 0x0f) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADCPPB4COUNT")
            .field("count", &self.count())
            .finish()
    }
}
impl W {}
#[doc = "ADC PPB4 Final Conversion Count Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcppb4count::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcppb4count::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adcppb4countSpec;
impl crate::RegisterSpec for Adcppb4countSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`adcppb4count::R`](R) reader structure"]
impl crate::Readable for Adcppb4countSpec {}
#[doc = "`write(|w| ..)` method takes [`adcppb4count::W`](W) writer structure"]
impl crate::Writable for Adcppb4countSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets ADCPPB4COUNT to value 0"]
impl crate::Resettable for Adcppb4countSpec {}
