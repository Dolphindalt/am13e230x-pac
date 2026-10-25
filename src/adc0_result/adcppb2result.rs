#[doc = "Register `ADCPPB2RESULT` reader"]
pub type R = crate::R<Adcppb2resultSpec>;
#[doc = "Register `ADCPPB2RESULT` writer"]
pub type W = crate::W<Adcppb2resultSpec>;
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
        f.debug_struct("ADCPPB2RESULT")
            .field("ppbresult", &self.ppbresult())
            .field("sign", &self.sign())
            .finish()
    }
}
impl W {}
#[doc = "ADC Post Processing Block 2 Result Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcppb2result::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcppb2result::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adcppb2resultSpec;
impl crate::RegisterSpec for Adcppb2resultSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adcppb2result::R`](R) reader structure"]
impl crate::Readable for Adcppb2resultSpec {}
#[doc = "`write(|w| ..)` method takes [`adcppb2result::W`](W) writer structure"]
impl crate::Writable for Adcppb2resultSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets ADCPPB2RESULT to value 0"]
impl crate::Resettable for Adcppb2resultSpec {}
