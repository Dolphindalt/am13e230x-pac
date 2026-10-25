#[doc = "Register `READFIFO3` reader"]
pub type R = crate::R<Readfifo3Spec>;
#[doc = "Register `READFIFO3` writer"]
pub type W = crate::W<Readfifo3Spec>;
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
        f.debug_struct("READFIFO3")
            .field("data", &self.data())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Reads Data."]
    #[inline(always)]
    pub fn data(&mut self) -> DataW<'_, Readfifo3Spec> {
        DataW::new(self, 0)
    }
}
#[doc = "EPI Read FIFO 3\n\nYou can [`read`](crate::Reg::read) this register and get [`readfifo3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`readfifo3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Readfifo3Spec;
impl crate::RegisterSpec for Readfifo3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`readfifo3::R`](R) reader structure"]
impl crate::Readable for Readfifo3Spec {}
#[doc = "`write(|w| ..)` method takes [`readfifo3::W`](W) writer structure"]
impl crate::Writable for Readfifo3Spec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets READFIFO3 to value 0"]
impl crate::Resettable for Readfifo3Spec {}
