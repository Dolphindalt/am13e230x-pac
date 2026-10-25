#[doc = "Register `READFIFO4` reader"]
pub type R = crate::R<Readfifo4Spec>;
#[doc = "Register `READFIFO4` writer"]
pub type W = crate::W<Readfifo4Spec>;
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
        f.debug_struct("READFIFO4")
            .field("data", &self.data())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Reads Data."]
    #[inline(always)]
    pub fn data(&mut self) -> DataW<'_, Readfifo4Spec> {
        DataW::new(self, 0)
    }
}
#[doc = "EPI Read FIFO 4\n\nYou can [`read`](crate::Reg::read) this register and get [`readfifo4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`readfifo4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Readfifo4Spec;
impl crate::RegisterSpec for Readfifo4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`readfifo4::R`](R) reader structure"]
impl crate::Readable for Readfifo4Spec {}
#[doc = "`write(|w| ..)` method takes [`readfifo4::W`](W) writer structure"]
impl crate::Writable for Readfifo4Spec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets READFIFO4 to value 0"]
impl crate::Resettable for Readfifo4Spec {}
