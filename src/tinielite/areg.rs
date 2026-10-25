#[doc = "Register `AREG[%s]` reader"]
pub type R = crate::R<AregSpec>;
#[doc = "Register `AREG[%s]` writer"]
pub type W = crate::W<AregSpec>;
#[doc = "Field `DATA` reader - Data word."]
pub type DataR = crate::FieldReader<u16>;
#[doc = "Field `DATA` writer - Data word."]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Data word."]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AREG").field("data", &self.data()).finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - Data word."]
    #[inline(always)]
    pub fn data(&mut self) -> DataW<'_, AregSpec> {
        DataW::new(self, 0)
    }
}
#[doc = "Address registers.\n\nYou can [`read`](crate::Reg::read) this register and get [`areg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`areg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AregSpec;
impl crate::RegisterSpec for AregSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`areg::R`](R) reader structure"]
impl crate::Readable for AregSpec {}
#[doc = "`write(|w| ..)` method takes [`areg::W`](W) writer structure"]
impl crate::Writable for AregSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AREG[%s] to value 0"]
impl crate::Resettable for AregSpec {}
