#[doc = "Register `OUTPUTXBARFLAG` reader"]
pub type R = crate::R<OutputxbarflagSpec>;
#[doc = "Register `OUTPUTXBARFLAG` writer"]
pub type W = crate::W<OutputxbarflagSpec>;
#[doc = "Field `FLG` reader - Output Signal Latched Flag Status"]
pub type FlgR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Output Signal Latched Flag Status"]
    #[inline(always)]
    pub fn flg(&self) -> FlgR {
        FlgR::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OUTPUTXBARFLAG")
            .field("flg", &self.flg())
            .finish()
    }
}
impl W {}
#[doc = "Output latched flag register\n\nYou can [`read`](crate::Reg::read) this register and get [`outputxbarflag::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`outputxbarflag::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OutputxbarflagSpec;
impl crate::RegisterSpec for OutputxbarflagSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`outputxbarflag::R`](R) reader structure"]
impl crate::Readable for OutputxbarflagSpec {}
#[doc = "`write(|w| ..)` method takes [`outputxbarflag::W`](W) writer structure"]
impl crate::Writable for OutputxbarflagSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets OUTPUTXBARFLAG to value 0"]
impl crate::Resettable for OutputxbarflagSpec {}
