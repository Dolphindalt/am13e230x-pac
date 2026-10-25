#[doc = "Register `ADCPPB4RESULT` reader"]
pub type R = crate::R<Adcppb4resultSpec>;
#[doc = "Register `ADCPPB4RESULT` writer"]
pub type W = crate::W<Adcppb4resultSpec>;
#[doc = "Field `PPBRESULT` reader - ADC Post Processing Block Result"]
pub type PpbresultR = crate::FieldReader<u16>;
#[doc = "Field `SIGN` reader - Sign Extended Bits"]
pub type SignR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:12 - ADC Post Processing Block Result"]
    #[inline(always)]
    pub fn ppbresult(&self) -> PpbresultR {
        PpbresultR::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bits 13:31 - Sign Extended Bits"]
    #[inline(always)]
    pub fn sign(&self) -> SignR {
        SignR::new((self.bits >> 13) & 0x0007_ffff)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADCPPB4RESULT")
            .field("ppbresult", &self.ppbresult())
            .field("sign", &self.sign())
            .finish()
    }
}
impl W {}
#[doc = "ADC Post Processing Block 4 Result Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcppb4result::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcppb4result::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adcppb4resultSpec;
impl crate::RegisterSpec for Adcppb4resultSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adcppb4result::R`](R) reader structure"]
impl crate::Readable for Adcppb4resultSpec {}
#[doc = "`write(|w| ..)` method takes [`adcppb4result::W`](W) writer structure"]
impl crate::Writable for Adcppb4resultSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets ADCPPB4RESULT to value 0"]
impl crate::Resettable for Adcppb4resultSpec {}
