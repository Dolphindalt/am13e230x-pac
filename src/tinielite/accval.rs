#[doc = "Register `ACCVAL[%s]` reader"]
pub type R = crate::R<AccvalSpec>;
#[doc = "Register `ACCVAL[%s]` writer"]
pub type W = crate::W<AccvalSpec>;
#[doc = "Field `DATA` reader - Data word."]
pub type DataR = crate::FieldReader<u32>;
#[doc = "Field `DATA` writer - Data word."]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 18, u32>;
impl R {
    #[doc = "Bits 0:17 - Data word."]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new(self.bits & 0x0003_ffff)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ACCVAL")
            .field("data", &self.data())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:17 - Data word."]
    #[inline(always)]
    pub fn data(&mut self) -> DataW<'_, AccvalSpec> {
        DataW::new(self, 0)
    }
}
#[doc = "Register file data.\n\nYou can [`read`](crate::Reg::read) this register and get [`accval::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`accval::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AccvalSpec;
impl crate::RegisterSpec for AccvalSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`accval::R`](R) reader structure"]
impl crate::Readable for AccvalSpec {}
#[doc = "`write(|w| ..)` method takes [`accval::W`](W) writer structure"]
impl crate::Writable for AccvalSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ACCVAL[%s] to value 0"]
impl crate::Resettable for AccvalSpec {}
