#[doc = "Register `COMPDACLCTL` reader"]
pub type R = crate::R<CompdaclctlSpec>;
#[doc = "Register `COMPDACLCTL` writer"]
pub type W = crate::W<CompdaclctlSpec>;
#[doc = "Field `RAMPSOURCE` reader - Ramp Generator Source Control"]
pub type RampsourceR = crate::FieldReader;
#[doc = "Field `RAMPSOURCE` writer - Ramp Generator Source Control"]
pub type RampsourceW<'a, REG> = crate::FieldWriter<'a, REG, 4, u8, crate::Safe>;
impl R {
    #[doc = "Bits 1:4 - Ramp Generator Source Control"]
    #[inline(always)]
    pub fn rampsource(&self) -> RampsourceR {
        RampsourceR::new(((self.bits >> 1) & 0x0f) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("COMPDACLCTL")
            .field("rampsource", &self.rampsource())
            .finish()
    }
}
impl W {
    #[doc = "Bits 1:4 - Ramp Generator Source Control"]
    #[inline(always)]
    pub fn rampsource(&mut self) -> RampsourceW<'_, CompdaclctlSpec> {
        RampsourceW::new(self, 1)
    }
}
#[doc = "CMPSS Low DAC Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`compdaclctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`compdaclctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CompdaclctlSpec;
impl crate::RegisterSpec for CompdaclctlSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`compdaclctl::R`](R) reader structure"]
impl crate::Readable for CompdaclctlSpec {}
#[doc = "`write(|w| ..)` method takes [`compdaclctl::W`](W) writer structure"]
impl crate::Writable for CompdaclctlSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets COMPDACLCTL to value 0"]
impl crate::Resettable for CompdaclctlSpec {}
