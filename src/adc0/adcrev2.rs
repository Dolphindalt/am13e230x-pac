#[doc = "Register `ADCREV2` reader"]
pub type R = crate::R<Adcrev2Spec>;
#[doc = "Register `ADCREV2` writer"]
pub type W = crate::W<Adcrev2Spec>;
#[doc = "Field `WRAPPERTYPE` reader - ADC Wrapper Type"]
pub type WrappertypeR = crate::FieldReader;
#[doc = "Field `WRAPPERREV` reader - ADC Wrapper Revision"]
pub type WrapperrevR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:7 - ADC Wrapper Type"]
    #[inline(always)]
    pub fn wrappertype(&self) -> WrappertypeR {
        WrappertypeR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - ADC Wrapper Revision"]
    #[inline(always)]
    pub fn wrapperrev(&self) -> WrapperrevR {
        WrapperrevR::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADCREV2")
            .field("wrappertype", &self.wrappertype())
            .field("wrapperrev", &self.wrapperrev())
            .finish()
    }
}
impl W {}
#[doc = "ADC Wrapper Revision Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcrev2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcrev2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adcrev2Spec;
impl crate::RegisterSpec for Adcrev2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adcrev2::R`](R) reader structure"]
impl crate::Readable for Adcrev2Spec {}
#[doc = "`write(|w| ..)` method takes [`adcrev2::W`](W) writer structure"]
impl crate::Writable for Adcrev2Spec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets ADCREV2 to value 0x06"]
impl crate::Resettable for Adcrev2Spec {
    const RESET_VALUE: u32 = 0x06;
}
