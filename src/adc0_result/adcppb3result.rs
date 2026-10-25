#[doc = "Register `ADCPPB3RESULT` reader"]
pub type R = crate::R<Adcppb3resultSpec>;
#[doc = "Register `ADCPPB3RESULT` writer"]
pub type W = crate::W<Adcppb3resultSpec>;
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
        f.debug_struct("ADCPPB3RESULT")
            .field("ppbresult", &self.ppbresult())
            .field("sign", &self.sign())
            .finish()
    }
}
impl W {}
#[doc = "ADC Post Processing Block 3 Result Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcppb3result::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcppb3result::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adcppb3resultSpec;
impl crate::RegisterSpec for Adcppb3resultSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adcppb3result::R`](R) reader structure"]
impl crate::Readable for Adcppb3resultSpec {}
#[doc = "`write(|w| ..)` method takes [`adcppb3result::W`](W) writer structure"]
impl crate::Writable for Adcppb3resultSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets ADCPPB3RESULT to value 0"]
impl crate::Resettable for Adcppb3resultSpec {}
