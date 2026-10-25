#[doc = "Register `CC_45[%s]` reader"]
pub type R = crate::R<Cc45Spec>;
#[doc = "Register `CC_45[%s]` writer"]
pub type W = crate::W<Cc45Spec>;
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
        f.debug_struct("CC_45")
            .field("ccval", &self.ccval())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Capture or compare value"]
    #[inline(always)]
    pub fn ccval(&mut self) -> CcvalW<'_, Cc45Spec> {
        CcvalW::new(self, 0)
    }
}
#[doc = "The CC_45 register are a registers which can be used as compare to the current CTR to create an events CC4U, CC4D, CC5U and CC5D.\n\nYou can [`read`](crate::Reg::read) this register and get [`cc_45::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cc_45::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cc45Spec;
impl crate::RegisterSpec for Cc45Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cc_45::R`](R) reader structure"]
impl crate::Readable for Cc45Spec {}
#[doc = "`write(|w| ..)` method takes [`cc_45::W`](W) writer structure"]
impl crate::Writable for Cc45Spec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CC_45[%s] to value 0"]
impl crate::Resettable for Cc45Spec {}
