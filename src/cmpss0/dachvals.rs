#[doc = "Register `DACHVALS` reader"]
pub type R = crate::R<DachvalsSpec>;
#[doc = "Register `DACHVALS` writer"]
pub type W = crate::W<DachvalsSpec>;
#[doc = "Field `DACVAL` reader - DAC Value Control"]
pub type DacvalR = crate::FieldReader<u16>;
#[doc = "Field `DACVAL` writer - DAC Value Control"]
pub type DacvalW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16, crate::Safe>;
impl R {
    #[doc = "Bits 0:11 - DAC Value Control"]
    #[inline(always)]
    pub fn dacval(&self) -> DacvalR {
        DacvalR::new(self.bits & 0x0fff)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DACHVALS")
            .field("dacval", &self.dacval())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:11 - DAC Value Control"]
    #[inline(always)]
    pub fn dacval(&mut self) -> DacvalW<'_, DachvalsSpec> {
        DacvalW::new(self, 0)
    }
}
#[doc = "CMPSS High DAC Value Shadow Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dachvals::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dachvals::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DachvalsSpec;
impl crate::RegisterSpec for DachvalsSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`dachvals::R`](R) reader structure"]
impl crate::Readable for DachvalsSpec {}
#[doc = "`write(|w| ..)` method takes [`dachvals::W`](W) writer structure"]
impl crate::Writable for DachvalsSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets DACHVALS to value 0"]
impl crate::Resettable for DachvalsSpec {}
