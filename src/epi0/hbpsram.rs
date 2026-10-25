#[doc = "Register `HBPSRAM` reader"]
pub type R = crate::R<HbpsramSpec>;
#[doc = "Register `HBPSRAM` writer"]
pub type W = crate::W<HbpsramSpec>;
#[doc = "Field `CR` reader - PSRAM Config Register."]
pub type CrR = crate::FieldReader<u32>;
#[doc = "Field `CR` writer - PSRAM Config Register."]
pub type CrW<'a, REG> = crate::FieldWriter<'a, REG, 21, u32, crate::Safe>;
impl R {
    #[doc = "Bits 0:20 - PSRAM Config Register."]
    #[inline(always)]
    pub fn cr(&self) -> CrR {
        CrR::new(self.bits & 0x001f_ffff)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HBPSRAM").field("cr", &self.cr()).finish()
    }
}
impl W {
    #[doc = "Bits 0:20 - PSRAM Config Register."]
    #[inline(always)]
    pub fn cr(&mut self) -> CrW<'_, HbpsramSpec> {
        CrW::new(self, 0)
    }
}
#[doc = "EPI Host-Bus PSRAM\n\nYou can [`read`](crate::Reg::read) this register and get [`hbpsram::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hbpsram::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HbpsramSpec;
impl crate::RegisterSpec for HbpsramSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hbpsram::R`](R) reader structure"]
impl crate::Readable for HbpsramSpec {}
#[doc = "`write(|w| ..)` method takes [`hbpsram::W`](W) writer structure"]
impl crate::Writable for HbpsramSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets HBPSRAM to value 0"]
impl crate::Resettable for HbpsramSpec {}
