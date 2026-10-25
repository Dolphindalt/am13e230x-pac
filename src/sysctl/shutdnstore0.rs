#[doc = "Register `SHUTDNSTORE0` reader"]
pub type R = crate::R<Shutdnstore0Spec>;
#[doc = "Register `SHUTDNSTORE0` writer"]
pub type W = crate::W<Shutdnstore0Spec>;
#[doc = "Field `DATA` reader - Shutdown storage byte 0"]
pub type DataR = crate::FieldReader;
#[doc = "Field `DATA` writer - Shutdown storage byte 0"]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 8, u8, crate::Safe>;
impl R {
    #[doc = "Bits 0:7 - Shutdown storage byte 0"]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SHUTDNSTORE0")
            .field("data", &self.data())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - Shutdown storage byte 0"]
    #[inline(always)]
    pub fn data(&mut self) -> DataW<'_, Shutdnstore0Spec> {
        DataW::new(self, 0)
    }
}
#[doc = "Shutdown storage memory (byte 0)\n\nYou can [`read`](crate::Reg::read) this register and get [`shutdnstore0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shutdnstore0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Shutdnstore0Spec;
impl crate::RegisterSpec for Shutdnstore0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`shutdnstore0::R`](R) reader structure"]
impl crate::Readable for Shutdnstore0Spec {}
#[doc = "`write(|w| ..)` method takes [`shutdnstore0::W`](W) writer structure"]
impl crate::Writable for Shutdnstore0Spec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets SHUTDNSTORE0 to value 0"]
impl crate::Resettable for Shutdnstore0Spec {}
