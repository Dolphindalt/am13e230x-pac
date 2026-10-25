#[doc = "Register `LC[%s]` reader"]
pub type R = crate::R<LcSpec>;
#[doc = "Register `LC[%s]` writer"]
pub type W = crate::W<LcSpec>;
#[doc = "Field `CNT` reader - Loop count."]
pub type CntR = crate::FieldReader<u16>;
#[doc = "Field `CNT` writer - Loop count."]
pub type CntW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Loop count."]
    #[inline(always)]
    pub fn cnt(&self) -> CntR {
        CntR::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LC").field("cnt", &self.cnt()).finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - Loop count."]
    #[inline(always)]
    pub fn cnt(&mut self) -> CntW<'_, LcSpec> {
        CntW::new(self, 0)
    }
}
#[doc = "Loop count registers.\n\nYou can [`read`](crate::Reg::read) this register and get [`lc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LcSpec;
impl crate::RegisterSpec for LcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lc::R`](R) reader structure"]
impl crate::Readable for LcSpec {}
#[doc = "`write(|w| ..)` method takes [`lc::W`](W) writer structure"]
impl crate::Writable for LcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LC[%s] to value 0"]
impl crate::Resettable for LcSpec {}
