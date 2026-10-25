#[doc = "Register `CC_01[%s]` reader"]
pub type R = crate::R<Cc01Spec>;
#[doc = "Register `CC_01[%s]` writer"]
pub type W = crate::W<Cc01Spec>;
#[doc = "Field `CCVAL` reader - Capture or compare value"]
pub type CcvalR = crate::FieldReader<u32>;
#[doc = "Field `CCVAL` writer - Capture or compare value"]
pub type CcvalW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32, crate::Safe>;
impl R {
    #[doc = "Bits 0:31 - Capture or compare value"]
    #[inline(always)]
    pub fn ccval(&self) -> CcvalR {
        CcvalR::new(self.bits)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CC_01")
            .field("ccval", &self.ccval())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Capture or compare value"]
    #[inline(always)]
    pub fn ccval(&mut self) -> CcvalW<'_, Cc01Spec> {
        CcvalW::new(self, 0)
    }
}
#[doc = "Capture or Compare Register 0/1\n\nYou can [`read`](crate::Reg::read) this register and get [`cc_01::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cc_01::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cc01Spec;
impl crate::RegisterSpec for Cc01Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cc_01::R`](R) reader structure"]
impl crate::Readable for Cc01Spec {}
#[doc = "`write(|w| ..)` method takes [`cc_01::W`](W) writer structure"]
impl crate::Writable for Cc01Spec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CC_01[%s] to value 0"]
impl crate::Resettable for Cc01Spec {}
