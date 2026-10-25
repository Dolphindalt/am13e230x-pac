#[doc = "Register `CLKCTL` reader"]
pub type R = crate::R<ClkctlSpec>;
#[doc = "Register `CLKCTL` writer"]
pub type W = crate::W<ClkctlSpec>;
#[doc = "MCAN clock stop request bit\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stopreq {
    #[doc = "0: Disable MCAN-SS clock stop request"]
    Disable = 0,
    #[doc = "1: Enable MCAN-SS clock stop request"]
    Enable = 1,
}
impl From<Stopreq> for bool {
    #[inline(always)]
    fn from(variant: Stopreq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STOPREQ` reader - MCAN clock stop request bit"]
pub type StopreqR = crate::BitReader<Stopreq>;
impl StopreqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Stopreq {
        match self.bits {
            false => Stopreq::Disable,
            true => Stopreq::Enable,
        }
    }
    #[doc = "Disable MCAN-SS clock stop request"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Stopreq::Disable
    }
    #[doc = "Enable MCAN-SS clock stop request"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Stopreq::Enable
    }
}
#[doc = "Field `STOPREQ` writer - MCAN clock stop request bit"]
pub type StopreqW<'a, REG> = crate::BitWriter<'a, REG, Stopreq>;
impl<'a, REG> StopreqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable MCAN-SS clock stop request"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Stopreq::Disable)
    }
    #[doc = "Enable MCAN-SS clock stop request"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Stopreq::Enable)
    }
}
#[doc = "MCAN IP clock stop wakeup interrupt enable bit\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WakeupIntEn {
    #[doc = "0: Disable MCAN IP clock stop wakeup interrupt"]
    Disable = 0,
    #[doc = "1: Enable MCAN IP clock stop wakeup interrupt"]
    Enable = 1,
}
impl From<WakeupIntEn> for bool {
    #[inline(always)]
    fn from(variant: WakeupIntEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WAKEUP_INT_EN` reader - MCAN IP clock stop wakeup interrupt enable bit"]
pub type WakeupIntEnR = crate::BitReader<WakeupIntEn>;
impl WakeupIntEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WakeupIntEn {
        match self.bits {
            false => WakeupIntEn::Disable,
            true => WakeupIntEn::Enable,
        }
    }
    #[doc = "Disable MCAN IP clock stop wakeup interrupt"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == WakeupIntEn::Disable
    }
    #[doc = "Enable MCAN IP clock stop wakeup interrupt"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == WakeupIntEn::Enable
    }
}
#[doc = "Field `WAKEUP_INT_EN` writer - MCAN IP clock stop wakeup interrupt enable bit"]
pub type WakeupIntEnW<'a, REG> = crate::BitWriter<'a, REG, WakeupIntEn>;
impl<'a, REG> WakeupIntEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable MCAN IP clock stop wakeup interrupt"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(WakeupIntEn::Disable)
    }
    #[doc = "Enable MCAN IP clock stop wakeup interrupt"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(WakeupIntEn::Enable)
    }
}
#[doc = "Clock stop wake-up on RXD, glitch filter enable.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WkupGltfltEn {
    #[doc = "0: Disable glitch filter enable on RXD input when MCAN is in clock stop mode (waiting for event on RXD input for clock stop wakeup)."]
    Disable = 0,
    #[doc = "1: Enable glitch filter enable on RXD input when MCAN is in clock stop mode (waiting for event on RXD input for clock stop wakeup)."]
    Enable = 1,
}
impl From<WkupGltfltEn> for bool {
    #[inline(always)]
    fn from(variant: WkupGltfltEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUP_GLTFLT_EN` reader - Clock stop wake-up on RXD, glitch filter enable."]
pub type WkupGltfltEnR = crate::BitReader<WkupGltfltEn>;
impl WkupGltfltEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WkupGltfltEn {
        match self.bits {
            false => WkupGltfltEn::Disable,
            true => WkupGltfltEn::Enable,
        }
    }
    #[doc = "Disable glitch filter enable on RXD input when MCAN is in clock stop mode (waiting for event on RXD input for clock stop wakeup)."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == WkupGltfltEn::Disable
    }
    #[doc = "Enable glitch filter enable on RXD input when MCAN is in clock stop mode (waiting for event on RXD input for clock stop wakeup)."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == WkupGltfltEn::Enable
    }
}
#[doc = "Field `WKUP_GLTFLT_EN` writer - Clock stop wake-up on RXD, glitch filter enable."]
pub type WkupGltfltEnW<'a, REG> = crate::BitWriter<'a, REG, WkupGltfltEn>;
impl<'a, REG> WkupGltfltEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable glitch filter enable on RXD input when MCAN is in clock stop mode (waiting for event on RXD input for clock stop wakeup)."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(WkupGltfltEn::Disable)
    }
    #[doc = "Enable glitch filter enable on RXD input when MCAN is in clock stop mode (waiting for event on RXD input for clock stop wakeup)."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(WkupGltfltEn::Enable)
    }
}
impl R {
    #[doc = "Bit 0 - MCAN clock stop request bit"]
    #[inline(always)]
    pub fn stopreq(&self) -> StopreqR {
        StopreqR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - MCAN IP clock stop wakeup interrupt enable bit"]
    #[inline(always)]
    pub fn wakeup_int_en(&self) -> WakeupIntEnR {
        WakeupIntEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - Clock stop wake-up on RXD, glitch filter enable."]
    #[inline(always)]
    pub fn wkup_gltflt_en(&self) -> WkupGltfltEnR {
        WkupGltfltEnR::new(((self.bits >> 8) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLKCTL")
            .field("stopreq", &self.stopreq())
            .field("wakeup_int_en", &self.wakeup_int_en())
            .field("wkup_gltflt_en", &self.wkup_gltflt_en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - MCAN clock stop request bit"]
    #[inline(always)]
    pub fn stopreq(&mut self) -> StopreqW<'_, ClkctlSpec> {
        StopreqW::new(self, 0)
    }
    #[doc = "Bit 4 - MCAN IP clock stop wakeup interrupt enable bit"]
    #[inline(always)]
    pub fn wakeup_int_en(&mut self) -> WakeupIntEnW<'_, ClkctlSpec> {
        WakeupIntEnW::new(self, 4)
    }
    #[doc = "Bit 8 - Clock stop wake-up on RXD, glitch filter enable."]
    #[inline(always)]
    pub fn wkup_gltflt_en(&mut self) -> WkupGltfltEnW<'_, ClkctlSpec> {
        WkupGltfltEnW::new(self, 8)
    }
}
#[doc = "MCAN-SS clock stop control register\n\nYou can [`read`](crate::Reg::read) this register and get [`clkctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkctlSpec;
impl crate::RegisterSpec for ClkctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clkctl::R`](R) reader structure"]
impl crate::Readable for ClkctlSpec {}
#[doc = "`write(|w| ..)` method takes [`clkctl::W`](W) writer structure"]
impl crate::Writable for ClkctlSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CLKCTL to value 0"]
impl crate::Resettable for ClkctlSpec {}
