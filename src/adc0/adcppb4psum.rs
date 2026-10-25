#[doc = "Register `ADCPPB4PSUM` reader"]
pub type R = crate::R<Adcppb4psumSpec>;
#[doc = "Register `ADCPPB4PSUM` writer"]
pub type W = crate::W<Adcppb4psumSpec>;
#[doc = "Field `PSUM` reader - Post Processing Block 4 Oversampling Partial Sum"]
pub type PsumR = crate::FieldReader<u16>;
#[doc = "Field `SIGN` reader - Sign Extended Bits"]
pub type SignR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Post Processing Block 4 Oversampling Partial Sum"]
    #[inline(always)]
    pub fn psum(&self) -> PsumR {
        PsumR::new((self.bits & 0xffff) as u16)
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
        f.debug_struct("ADCPPB4PSUM")
            .field("psum", &self.psum())
            .field("sign", &self.sign())
            .finish()
    }
}
impl W {}
#[doc = "ADC PPB4 Partial Sum Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcppb4psum::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcppb4psum::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adcppb4psumSpec;
impl crate::RegisterSpec for Adcppb4psumSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adcppb4psum::R`](R) reader structure"]
impl crate::Readable for Adcppb4psumSpec {}
#[doc = "`write(|w| ..)` method takes [`adcppb4psum::W`](W) writer structure"]
impl crate::Writable for Adcppb4psumSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets ADCPPB4PSUM to value 0"]
impl crate::Resettable for Adcppb4psumSpec {}
