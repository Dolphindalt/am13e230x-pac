#[doc = "Register `SHUTDNSTORE2` reader"]
pub type R = crate::R<Shutdnstore2Spec>;
#[doc = "Register `SHUTDNSTORE2` writer"]
pub type W = crate::W<Shutdnstore2Spec>;
#[doc = "Field `DATA` reader - Shutdown storage byte 2"]
pub type DataR = crate::FieldReader;
#[doc = "Field `DATA` writer - Shutdown storage byte 2"]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 8, u8, crate::Safe>;
impl R {
    #[doc = "Bits 0:7 - Shutdown storage byte 2"]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SHUTDNSTORE2")
            .field("data", &self.data())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - Shutdown storage byte 2"]
    #[inline(always)]
    pub fn data(&mut self) -> DataW<'_, Shutdnstore2Spec> {
        DataW::new(self, 0)
    }
}
#[doc = "Shutdown storage memory (byte 2)\n\nYou can [`read`](crate::Reg::read) this register and get [`shutdnstore2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shutdnstore2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Shutdnstore2Spec;
impl crate::RegisterSpec for Shutdnstore2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`shutdnstore2::R`](R) reader structure"]
impl crate::Readable for Shutdnstore2Spec {}
#[doc = "`write(|w| ..)` method takes [`shutdnstore2::W`](W) writer structure"]
impl crate::Writable for Shutdnstore2Spec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets SHUTDNSTORE2 to value 0"]
impl crate::Resettable for Shutdnstore2Spec {}
