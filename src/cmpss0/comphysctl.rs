#[doc = "Register `COMPHYSCTL` reader"]
pub type R = crate::R<ComphysctlSpec>;
#[doc = "Register `COMPHYSCTL` writer"]
pub type W = crate::W<ComphysctlSpec>;
#[doc = "Field `COMPHYS` reader - Comparator Hysteresis Trim"]
pub type ComphysR = crate::FieldReader;
#[doc = "Field `COMPHYS` writer - Comparator Hysteresis Trim"]
pub type ComphysW<'a, REG> = crate::FieldWriter<'a, REG, 4, u8, crate::Safe>;
impl R {
    #[doc = "Bits 0:3 - Comparator Hysteresis Trim"]
    #[inline(always)]
    pub fn comphys(&self) -> ComphysR {
        ComphysR::new((self.bits & 0x0f) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("COMPHYSCTL")
            .field("comphys", &self.comphys())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - Comparator Hysteresis Trim"]
    #[inline(always)]
    pub fn comphys(&mut self) -> ComphysW<'_, ComphysctlSpec> {
        ComphysW::new(self, 0)
    }
}
#[doc = "CMPSS Comparator Hysteresis Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`comphysctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comphysctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ComphysctlSpec;
impl crate::RegisterSpec for ComphysctlSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`comphysctl::R`](R) reader structure"]
impl crate::Readable for ComphysctlSpec {}
#[doc = "`write(|w| ..)` method takes [`comphysctl::W`](W) writer structure"]
impl crate::Writable for ComphysctlSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets COMPHYSCTL to value 0"]
impl crate::Resettable for ComphysctlSpec {}
