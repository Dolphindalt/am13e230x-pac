#[doc = "Register `COMPDACHCTL` reader"]
pub type R = crate::R<CompdachctlSpec>;
#[doc = "Register `COMPDACHCTL` writer"]
pub type W = crate::W<CompdachctlSpec>;
#[doc = "Field `RAMPSOURCE` reader - Ramp Generator Source Control"]
pub type RampsourceR = crate::FieldReader;
#[doc = "Field `RAMPSOURCE` writer - Ramp Generator Source Control"]
pub type RampsourceW<'a, REG> = crate::FieldWriter<'a, REG, 4, u8, crate::Safe>;
#[doc = "Field `SWLOADSEL` reader - Software Load Select"]
pub type SwloadselR = crate::BitReader;
#[doc = "Field `SWLOADSEL` writer - Software Load Select"]
pub type SwloadselW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 1:4 - Ramp Generator Source Control"]
    #[inline(always)]
    pub fn rampsource(&self) -> RampsourceR {
        RampsourceR::new(((self.bits >> 1) & 0x0f) as u8)
    }
    #[doc = "Bit 7 - Software Load Select"]
    #[inline(always)]
    pub fn swloadsel(&self) -> SwloadselR {
        SwloadselR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("COMPDACHCTL")
            .field("rampsource", &self.rampsource())
            .field("swloadsel", &self.swloadsel())
            .finish()
    }
}
impl W {
    #[doc = "Bits 1:4 - Ramp Generator Source Control"]
    #[inline(always)]
    pub fn rampsource(&mut self) -> RampsourceW<'_, CompdachctlSpec> {
        RampsourceW::new(self, 1)
    }
    #[doc = "Bit 7 - Software Load Select"]
    #[inline(always)]
    pub fn swloadsel(&mut self) -> SwloadselW<'_, CompdachctlSpec> {
        SwloadselW::new(self, 7)
    }
}
#[doc = "CMPSS High DAC Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`compdachctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`compdachctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CompdachctlSpec;
impl crate::RegisterSpec for CompdachctlSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`compdachctl::R`](R) reader structure"]
impl crate::Readable for CompdachctlSpec {}
#[doc = "`write(|w| ..)` method takes [`compdachctl::W`](W) writer structure"]
impl crate::Writable for CompdachctlSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets COMPDACHCTL to value 0"]
impl crate::Resettable for CompdachctlSpec {}
