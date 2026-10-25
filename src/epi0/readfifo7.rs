#[doc = "Register `READFIFO7` reader"]
pub type R = crate::R<Readfifo7Spec>;
#[doc = "Register `READFIFO7` writer"]
pub type W = crate::W<Readfifo7Spec>;
#[doc = "Field `DATA` reader - Reads Data."]
pub type DataR = crate::FieldReader<u32>;
#[doc = "Field `DATA` writer - Reads Data."]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32, crate::Safe>;
impl R {
    #[doc = "Bits 0:31 - Reads Data."]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new(self.bits)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("READFIFO7")
            .field("data", &self.data())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Reads Data."]
    #[inline(always)]
    pub fn data(&mut self) -> DataW<'_, Readfifo7Spec> {
        DataW::new(self, 0)
    }
}
#[doc = "EPI Read FIFO 7\n\nYou can [`read`](crate::Reg::read) this register and get [`readfifo7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`readfifo7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Readfifo7Spec;
impl crate::RegisterSpec for Readfifo7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`readfifo7::R`](R) reader structure"]
impl crate::Readable for Readfifo7Spec {}
#[doc = "`write(|w| ..)` method takes [`readfifo7::W`](W) writer structure"]
impl crate::Writable for Readfifo7Spec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets READFIFO7 to value 0"]
impl crate::Resettable for Readfifo7Spec {}
