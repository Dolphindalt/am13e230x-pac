#[doc = "Register `RPSTD1` reader"]
pub type R = crate::R<Rpstd1Spec>;
#[doc = "Register `RPSTD1` writer"]
pub type W = crate::W<Rpstd1Spec>;
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
        f.debug_struct("RPSTD1")
            .field("postcnt", &self.postcnt())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:12 - Post Count"]
    #[inline(always)]
    pub fn postcnt(&mut self) -> PostcntW<'_, Rpstd1Spec> {
        PostcntW::new(self, 0)
    }
}
#[doc = "EPI Non-Blocking Read Data 1\n\nYou can [`read`](crate::Reg::read) this register and get [`rpstd1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rpstd1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rpstd1Spec;
impl crate::RegisterSpec for Rpstd1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rpstd1::R`](R) reader structure"]
impl crate::Readable for Rpstd1Spec {}
#[doc = "`write(|w| ..)` method takes [`rpstd1::W`](W) writer structure"]
impl crate::Writable for Rpstd1Spec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets RPSTD1 to value 0"]
impl crate::Resettable for Rpstd1Spec {}
