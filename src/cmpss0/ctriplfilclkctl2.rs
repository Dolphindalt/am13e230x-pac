#[doc = "Register `CTRIPLFILCLKCTL2` reader"]
pub type R = crate::R<Ctriplfilclkctl2Spec>;
#[doc = "Register `CTRIPLFILCLKCTL2` writer"]
pub type W = crate::W<Ctriplfilclkctl2Spec>;
#[doc = "Field `CLKPRESCALEU` reader - Sample Clock Prescale Upper Bits"]
pub type ClkprescaleuR = crate::FieldReader;
#[doc = "Field `CLKPRESCALEU` writer - Sample Clock Prescale Upper Bits"]
pub type ClkprescaleuW<'a, REG> = crate::FieldWriter<'a, REG, 8, u8, crate::Safe>;
impl R {
    #[doc = "Bits 0:7 - Sample Clock Prescale Upper Bits"]
    #[inline(always)]
    pub fn clkprescaleu(&self) -> ClkprescaleuR {
        ClkprescaleuR::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRIPLFILCLKCTL2")
            .field("clkprescaleu", &self.clkprescaleu())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - Sample Clock Prescale Upper Bits"]
    #[inline(always)]
    pub fn clkprescaleu(&mut self) -> ClkprescaleuW<'_, Ctriplfilclkctl2Spec> {
        ClkprescaleuW::new(self, 0)
    }
}
#[doc = "CTRIPL Filter Clock Control Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`ctriplfilclkctl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctriplfilclkctl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ctriplfilclkctl2Spec;
impl crate::RegisterSpec for Ctriplfilclkctl2Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ctriplfilclkctl2::R`](R) reader structure"]
impl crate::Readable for Ctriplfilclkctl2Spec {}
#[doc = "`write(|w| ..)` method takes [`ctriplfilclkctl2::W`](W) writer structure"]
impl crate::Writable for Ctriplfilclkctl2Spec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CTRIPLFILCLKCTL2 to value 0"]
impl crate::Resettable for Ctriplfilclkctl2Spec {}
