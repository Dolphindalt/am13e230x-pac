#[doc = "Register `TSNSSCTL` reader"]
pub type R = crate::R<TsnssctlSpec>;
#[doc = "Register `TSNSSCTL` writer"]
pub type W = crate::W<TsnssctlSpec>;
#[doc = "Field `ENABLE` reader - Temperature Sensor Enable"]
pub type EnableR = crate::BitReader;
#[doc = "Field `ENABLE` writer - Temperature Sensor Enable"]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Temperature Sensor Enable"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TSNSSCTL")
            .field("enable", &self.enable())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Temperature Sensor Enable"]
    #[inline(always)]
    pub fn enable(&mut self) -> EnableW<'_, TsnssctlSpec> {
        EnableW::new(self, 0)
    }
}
#[doc = "Temperature Sensor Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tsnssctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tsnssctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TsnssctlSpec;
impl crate::RegisterSpec for TsnssctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tsnssctl::R`](R) reader structure"]
impl crate::Readable for TsnssctlSpec {}
#[doc = "`write(|w| ..)` method takes [`tsnssctl::W`](W) writer structure"]
impl crate::Writable for TsnssctlSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets TSNSSCTL to value 0"]
impl crate::Resettable for TsnssctlSpec {}
