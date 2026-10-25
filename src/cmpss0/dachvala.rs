#[doc = "Register `DACHVALA` reader"]
pub type R = crate::R<DachvalaSpec>;
#[doc = "Register `DACHVALA` writer"]
pub type W = crate::W<DachvalaSpec>;
#[doc = "Field `DACVAL` reader - DAC Value Control"]
pub type DacvalR = crate::FieldReader<u16>;
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
        f.debug_struct("DACHVALA")
            .field("dacval", &self.dacval())
            .finish()
    }
}
impl W {}
#[doc = "CMPSS High DAC Value Active Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dachvala::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dachvala::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DachvalaSpec;
impl crate::RegisterSpec for DachvalaSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`dachvala::R`](R) reader structure"]
impl crate::Readable for DachvalaSpec {}
#[doc = "`write(|w| ..)` method takes [`dachvala::W`](W) writer structure"]
impl crate::Writable for DachvalaSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets DACHVALA to value 0"]
impl crate::Resettable for DachvalaSpec {}
