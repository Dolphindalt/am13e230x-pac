#[doc = "Register `LINCNT` reader"]
pub type R = crate::R<LincntSpec>;
#[doc = "Register `LINCNT` writer"]
pub type W = crate::W<LincntSpec>;
#[doc = "Field `VALUE` reader - 16 bit up counter clocked by the functional clock of the UART."]
pub type ValueR = crate::FieldReader<u16>;
#[doc = "Field `VALUE` writer - 16 bit up counter clocked by the functional clock of the UART."]
pub type ValueW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16, crate::Safe>;
impl R {
    #[doc = "Bits 0:15 - 16 bit up counter clocked by the functional clock of the UART."]
    #[inline(always)]
    pub fn value(&self) -> ValueR {
        ValueR::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LINCNT")
            .field("value", &self.value())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - 16 bit up counter clocked by the functional clock of the UART."]
    #[inline(always)]
    pub fn value(&mut self) -> ValueW<'_, LincntSpec> {
        ValueW::new(self, 0)
    }
}
#[doc = "UART LIN Mode Counter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lincnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lincnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LincntSpec;
impl crate::RegisterSpec for LincntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lincnt::R`](R) reader structure"]
impl crate::Readable for LincntSpec {}
#[doc = "`write(|w| ..)` method takes [`lincnt::W`](W) writer structure"]
impl crate::Writable for LincntSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets LINCNT to value 0"]
impl crate::Resettable for LincntSpec {}
