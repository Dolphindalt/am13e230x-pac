#[doc = "Register `ADCPPB1CONFIG2` reader"]
pub type R = crate::R<Adcppb1config2Spec>;
#[doc = "Register `ADCPPB1CONFIG2` writer"]
pub type W = crate::W<Adcppb1config2Spec>;
#[doc = "Field `SHIFT` reader - Post Processing Block 1 Right Shift"]
pub type ShiftR = crate::FieldReader;
#[doc = "Field `SHIFT` writer - Post Processing Block 1 Right Shift"]
pub type ShiftW<'a, REG> = crate::FieldWriter<'a, REG, 3, u8, crate::Safe>;
#[doc = "Field `SYNCINSEL` reader - Post Processing Block 1 Sync. Input Select"]
pub type SyncinselR = crate::FieldReader;
#[doc = "Field `SYNCINSEL` writer - Post Processing Block 1 Sync. Input Select"]
pub type SyncinselW<'a, REG> = crate::FieldWriter<'a, REG, 5, u8, crate::Safe>;
#[doc = "Field `SWSYNC` reader - Post Processing Block 1 Software Force Sync."]
pub type SwsyncR = crate::BitReader;
#[doc = "Field `SWSYNC` writer - Post Processing Block 1 Software Force Sync."]
pub type SwsyncW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMPSEL` reader - Post Processing Block 1 Compare Source Select"]
pub type CompselR = crate::BitReader;
#[doc = "Field `COMPSEL` writer - Post Processing Block 1 Compare Source Select"]
pub type CompselW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - Post Processing Block 1 Right Shift"]
    #[inline(always)]
    pub fn shift(&self) -> ShiftR {
        ShiftR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:8 - Post Processing Block 1 Sync. Input Select"]
    #[inline(always)]
    pub fn syncinsel(&self) -> SyncinselR {
        SyncinselR::new(((self.bits >> 4) & 0x1f) as u8)
    }
    #[doc = "Bit 11 - Post Processing Block 1 Software Force Sync."]
    #[inline(always)]
    pub fn swsync(&self) -> SwsyncR {
        SwsyncR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 15 - Post Processing Block 1 Compare Source Select"]
    #[inline(always)]
    pub fn compsel(&self) -> CompselR {
        CompselR::new(((self.bits >> 15) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADCPPB1CONFIG2")
            .field("shift", &self.shift())
            .field("syncinsel", &self.syncinsel())
            .field("swsync", &self.swsync())
            .field("compsel", &self.compsel())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - Post Processing Block 1 Right Shift"]
    #[inline(always)]
    pub fn shift(&mut self) -> ShiftW<'_, Adcppb1config2Spec> {
        ShiftW::new(self, 0)
    }
    #[doc = "Bits 4:8 - Post Processing Block 1 Sync. Input Select"]
    #[inline(always)]
    pub fn syncinsel(&mut self) -> SyncinselW<'_, Adcppb1config2Spec> {
        SyncinselW::new(self, 4)
    }
    #[doc = "Bit 11 - Post Processing Block 1 Software Force Sync."]
    #[inline(always)]
    pub fn swsync(&mut self) -> SwsyncW<'_, Adcppb1config2Spec> {
        SwsyncW::new(self, 11)
    }
    #[doc = "Bit 15 - Post Processing Block 1 Compare Source Select"]
    #[inline(always)]
    pub fn compsel(&mut self) -> CompselW<'_, Adcppb1config2Spec> {
        CompselW::new(self, 15)
    }
}
#[doc = "ADC PPB1 Sum Shift Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcppb1config2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcppb1config2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adcppb1config2Spec;
impl crate::RegisterSpec for Adcppb1config2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adcppb1config2::R`](R) reader structure"]
impl crate::Readable for Adcppb1config2Spec {}
#[doc = "`write(|w| ..)` method takes [`adcppb1config2::W`](W) writer structure"]
impl crate::Writable for Adcppb1config2Spec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets ADCPPB1CONFIG2 to value 0"]
impl crate::Resettable for Adcppb1config2Spec {}
