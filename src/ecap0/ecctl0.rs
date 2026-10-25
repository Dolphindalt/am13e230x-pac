#[doc = "Register `ECCTL0` reader"]
pub type R = crate::R<Ecctl0Spec>;
#[doc = "Register `ECCTL0` writer"]
pub type W = crate::W<Ecctl0Spec>;
#[doc = "Field `INPUTSEL` reader - INPUT source select"]
pub type InputselR = crate::FieldReader;
#[doc = "Field `INPUTSEL` writer - INPUT source select"]
pub type InputselW<'a, REG> = crate::FieldWriter<'a, REG, 7, u8, crate::Safe>;
impl R {
    #[doc = "Bits 0:6 - INPUT source select"]
    #[inline(always)]
    pub fn inputsel(&self) -> InputselR {
        InputselR::new((self.bits & 0x7f) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ECCTL0")
            .field("inputsel", &self.inputsel())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:6 - INPUT source select"]
    #[inline(always)]
    pub fn inputsel(&mut self) -> InputselW<'_, Ecctl0Spec> {
        InputselW::new(self, 0)
    }
}
#[doc = "Capture Control Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ecctl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ecctl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ecctl0Spec;
impl crate::RegisterSpec for Ecctl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ecctl0::R`](R) reader structure"]
impl crate::Readable for Ecctl0Spec {}
#[doc = "`write(|w| ..)` method takes [`ecctl0::W`](W) writer structure"]
impl crate::Writable for Ecctl0Spec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets ECCTL0 to value 0x7f"]
impl crate::Resettable for Ecctl0Spec {
    const RESET_VALUE: u32 = 0x7f;
}
