#[doc = "Register `DACLVALA` reader"]
pub type R = crate::R<DaclvalaSpec>;
#[doc = "Register `DACLVALA` writer"]
pub type W = crate::W<DaclvalaSpec>;
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
        f.debug_struct("DACLVALA")
            .field("dacval", &self.dacval())
            .finish()
    }
}
impl W {}
#[doc = "CMPSS Low DAC Value Active Register\n\nYou can [`read`](crate::Reg::read) this register and get [`daclvala::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`daclvala::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DaclvalaSpec;
impl crate::RegisterSpec for DaclvalaSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`daclvala::R`](R) reader structure"]
impl crate::Readable for DaclvalaSpec {}
#[doc = "`write(|w| ..)` method takes [`daclvala::W`](W) writer structure"]
impl crate::Writable for DaclvalaSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets DACLVALA to value 0"]
impl crate::Resettable for DaclvalaSpec {}
