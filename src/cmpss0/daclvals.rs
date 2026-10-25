#[doc = "Register `DACLVALS` reader"]
pub type R = crate::R<DaclvalsSpec>;
#[doc = "Register `DACLVALS` writer"]
pub type W = crate::W<DaclvalsSpec>;
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
        f.debug_struct("DACLVALS")
            .field("dacval", &self.dacval())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:11 - DAC Value Control"]
    #[inline(always)]
    pub fn dacval(&mut self) -> DacvalW<'_, DaclvalsSpec> {
        DacvalW::new(self, 0)
    }
}
#[doc = "CMPSS Low DAC Value Shadow Register\n\nYou can [`read`](crate::Reg::read) this register and get [`daclvals::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`daclvals::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DaclvalsSpec;
impl crate::RegisterSpec for DaclvalsSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`daclvals::R`](R) reader structure"]
impl crate::Readable for DaclvalsSpec {}
#[doc = "`write(|w| ..)` method takes [`daclvals::W`](W) writer structure"]
impl crate::Writable for DaclvalsSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets DACLVALS to value 0"]
impl crate::Resettable for DaclvalsSpec {}
