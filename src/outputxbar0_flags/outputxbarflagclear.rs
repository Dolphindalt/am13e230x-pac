#[doc = "Register `OUTPUTXBARFLAGCLEAR` reader"]
pub type R = crate::R<OutputxbarflagclearSpec>;
#[doc = "Register `OUTPUTXBARFLAGCLEAR` writer"]
pub type W = crate::W<OutputxbarflagclearSpec>;
#[doc = "Field `FLG` reader - Output Signal Latched Flag Clear"]
pub type FlgR = crate::BitReader;
#[doc = "Field `FLG` writer - Output Signal Latched Flag Clear"]
pub type FlgW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Output Signal Latched Flag Clear"]
    #[inline(always)]
    pub fn flg(&self) -> FlgR {
        FlgR::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OUTPUTXBARFLAGCLEAR")
            .field("flg", &self.flg())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Output Signal Latched Flag Clear"]
    #[inline(always)]
    pub fn flg(&mut self) -> FlgW<'_, OutputxbarflagclearSpec> {
        FlgW::new(self, 0)
    }
}
#[doc = "Output latched flag clear register\n\nYou can [`read`](crate::Reg::read) this register and get [`outputxbarflagclear::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`outputxbarflagclear::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OutputxbarflagclearSpec;
impl crate::RegisterSpec for OutputxbarflagclearSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`outputxbarflagclear::R`](R) reader structure"]
impl crate::Readable for OutputxbarflagclearSpec {}
#[doc = "`write(|w| ..)` method takes [`outputxbarflagclear::W`](W) writer structure"]
impl crate::Writable for OutputxbarflagclearSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets OUTPUTXBARFLAGCLEAR to value 0"]
impl crate::Resettable for OutputxbarflagclearSpec {}
