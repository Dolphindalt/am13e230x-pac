#[doc = "Register `ADCPPB3SUM` reader"]
pub type R = crate::R<Adcppb3sumSpec>;
#[doc = "Register `ADCPPB3SUM` writer"]
pub type W = crate::W<Adcppb3sumSpec>;
#[doc = "Field `SUM` reader - Post Processing Block 3 Oversampling Sum"]
pub type SumR = crate::FieldReader<u16>;
#[doc = "Field `SIGN` reader - Sign Extended Bits"]
pub type SignR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Post Processing Block 3 Oversampling Sum"]
    #[inline(always)]
    pub fn sum(&self) -> SumR {
        SumR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Sign Extended Bits"]
    #[inline(always)]
    pub fn sign(&self) -> SignR {
        SignR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADCPPB3SUM")
            .field("sum", &self.sum())
            .field("sign", &self.sign())
            .finish()
    }
}
impl W {}
#[doc = "ADC PPB 3 Final Sum Result Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcppb3sum::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcppb3sum::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adcppb3sumSpec;
impl crate::RegisterSpec for Adcppb3sumSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adcppb3sum::R`](R) reader structure"]
impl crate::Readable for Adcppb3sumSpec {}
#[doc = "`write(|w| ..)` method takes [`adcppb3sum::W`](W) writer structure"]
impl crate::Writable for Adcppb3sumSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets ADCPPB3SUM to value 0"]
impl crate::Resettable for Adcppb3sumSpec {}
