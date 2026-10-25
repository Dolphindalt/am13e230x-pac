#[doc = "Register `SHUTDNSTORE1` reader"]
pub type R = crate::R<Shutdnstore1Spec>;
#[doc = "Register `SHUTDNSTORE1` writer"]
pub type W = crate::W<Shutdnstore1Spec>;
#[doc = "Field `DATA` reader - Shutdown storage byte 1"]
pub type DataR = crate::FieldReader;
#[doc = "Field `DATA` writer - Shutdown storage byte 1"]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 8, u8, crate::Safe>;
impl R {
    #[doc = "Bits 0:7 - Shutdown storage byte 1"]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SHUTDNSTORE1")
            .field("data", &self.data())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - Shutdown storage byte 1"]
    #[inline(always)]
    pub fn data(&mut self) -> DataW<'_, Shutdnstore1Spec> {
        DataW::new(self, 0)
    }
}
#[doc = "Shutdown storage memory (byte 1)\n\nYou can [`read`](crate::Reg::read) this register and get [`shutdnstore1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shutdnstore1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Shutdnstore1Spec;
impl crate::RegisterSpec for Shutdnstore1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`shutdnstore1::R`](R) reader structure"]
impl crate::Readable for Shutdnstore1Spec {}
#[doc = "`write(|w| ..)` method takes [`shutdnstore1::W`](W) writer structure"]
impl crate::Writable for Shutdnstore1Spec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets SHUTDNSTORE1 to value 0"]
impl crate::Resettable for Shutdnstore1Spec {}
