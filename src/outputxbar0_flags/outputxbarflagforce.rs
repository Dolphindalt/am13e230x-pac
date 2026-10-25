#[doc = "Register `OUTPUTXBARFLAGFORCE` reader"]
pub type R = crate::R<OutputxbarflagforceSpec>;
#[doc = "Register `OUTPUTXBARFLAGFORCE` writer"]
pub type W = crate::W<OutputxbarflagforceSpec>;
#[doc = "Field `FLG` reader - Output Signal Latched Flag Force"]
pub type FlgR = crate::BitReader;
#[doc = "Field `FLG` writer - Output Signal Latched Flag Force"]
pub type FlgW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Output Signal Latched Flag Force"]
    #[inline(always)]
    pub fn flg(&self) -> FlgR {
        FlgR::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OUTPUTXBARFLAGFORCE")
            .field("flg", &self.flg())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Output Signal Latched Flag Force"]
    #[inline(always)]
    pub fn flg(&mut self) -> FlgW<'_, OutputxbarflagforceSpec> {
        FlgW::new(self, 0)
    }
}
#[doc = "Output latched flag Force register\n\nYou can [`read`](crate::Reg::read) this register and get [`outputxbarflagforce::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`outputxbarflagforce::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OutputxbarflagforceSpec;
impl crate::RegisterSpec for OutputxbarflagforceSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`outputxbarflagforce::R`](R) reader structure"]
impl crate::Readable for OutputxbarflagforceSpec {}
#[doc = "`write(|w| ..)` method takes [`outputxbarflagforce::W`](W) writer structure"]
impl crate::Writable for OutputxbarflagforceSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets OUTPUTXBARFLAGFORCE to value 0"]
impl crate::Resettable for OutputxbarflagforceSpec {}
