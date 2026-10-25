#[doc = "Register `RPSTD0` reader"]
pub type R = crate::R<Rpstd0Spec>;
#[doc = "Register `RPSTD0` writer"]
pub type W = crate::W<Rpstd0Spec>;
#[doc = "Field `POSTCNT` reader - Post Count"]
pub type PostcntR = crate::FieldReader<u16>;
#[doc = "Field `POSTCNT` writer - Post Count"]
pub type PostcntW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16, crate::Safe>;
impl R {
    #[doc = "Bits 0:12 - Post Count"]
    #[inline(always)]
    pub fn postcnt(&self) -> PostcntR {
        PostcntR::new((self.bits & 0x1fff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RPSTD0")
            .field("postcnt", &self.postcnt())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:12 - Post Count"]
    #[inline(always)]
    pub fn postcnt(&mut self) -> PostcntW<'_, Rpstd0Spec> {
        PostcntW::new(self, 0)
    }
}
#[doc = "EPI Non-Blocking Read Data 0\n\nYou can [`read`](crate::Reg::read) this register and get [`rpstd0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rpstd0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rpstd0Spec;
impl crate::RegisterSpec for Rpstd0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rpstd0::R`](R) reader structure"]
impl crate::Readable for Rpstd0Spec {}
#[doc = "`write(|w| ..)` method takes [`rpstd0::W`](W) writer structure"]
impl crate::Writable for Rpstd0Spec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets RPSTD0 to value 0"]
impl crate::Resettable for Rpstd0Spec {}
