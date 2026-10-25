#[doc = "Register `CTL1` reader"]
pub type R = crate::R<Ctl1Spec>;
#[doc = "Register `CTL1` writer"]
pub type W = crate::W<Ctl1Spec>;
#[doc = "Field `ENABLE` reader - SPI enable"]
pub type EnableR = crate::BitReader;
#[doc = "Field `ENABLE` writer - SPI enable"]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Loop back mode enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lbm {
    #[doc = "0: Disable loopback mode"]
    Disable = 0,
    #[doc = "1: Enable loopback mode"]
    Enable = 1,
}
impl From<Lbm> for bool {
    #[inline(always)]
    fn from(variant: Lbm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LBM` reader - Loop back mode enable"]
pub type LbmR = crate::BitReader<Lbm>;
impl LbmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lbm {
        match self.bits {
            false => Lbm::Disable,
            true => Lbm::Enable,
        }
    }
    #[doc = "Disable loopback mode"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Lbm::Disable
    }
    #[doc = "Enable loopback mode"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Lbm::Enable
    }
}
#[doc = "Field `LBM` writer - Loop back mode enable"]
pub type LbmW<'a, REG> = crate::BitWriter<'a, REG, Lbm>;
impl<'a, REG> LbmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable loopback mode"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Lbm::Disable)
    }
    #[doc = "Enable loopback mode"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Lbm::Enable)
    }
}
#[doc = "Controller or peripheral mode select\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cp {
    #[doc = "0: Select Peripheral mode"]
    Disable = 0,
    #[doc = "1: Select Controller Mode"]
    Enable = 1,
}
impl From<Cp> for bool {
    #[inline(always)]
    fn from(variant: Cp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CP` reader - Controller or peripheral mode select"]
pub type CpR = crate::BitReader<Cp>;
impl CpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cp {
        match self.bits {
            false => Cp::Disable,
            true => Cp::Enable,
        }
    }
    #[doc = "Select Peripheral mode"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Cp::Disable
    }
    #[doc = "Select Controller Mode"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Cp::Enable
    }
}
#[doc = "Field `CP` writer - Controller or peripheral mode select"]
pub type CpW<'a, REG> = crate::BitWriter<'a, REG, Cp>;
impl<'a, REG> CpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Select Peripheral mode"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Cp::Disable)
    }
    #[doc = "Select Controller Mode"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Cp::Enable)
    }
}
#[doc = "Peripheral-mode: Data output disabled\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pod {
    #[doc = "0: SPI can drive the POCI output in peripheral mode."]
    Disable = 0,
    #[doc = "1: SPI cannot drive the POCI output in peripheral mode."]
    Enable = 1,
}
impl From<Pod> for bool {
    #[inline(always)]
    fn from(variant: Pod) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POD` reader - Peripheral-mode: Data output disabled"]
pub type PodR = crate::BitReader<Pod>;
impl PodR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pod {
        match self.bits {
            false => Pod::Disable,
            true => Pod::Enable,
        }
    }
    #[doc = "SPI can drive the POCI output in peripheral mode."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Pod::Disable
    }
    #[doc = "SPI cannot drive the POCI output in peripheral mode."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Pod::Enable
    }
}
#[doc = "Field `POD` writer - Peripheral-mode: Data output disabled"]
pub type PodW<'a, REG> = crate::BitWriter<'a, REG, Pod>;
impl<'a, REG> PodW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPI can drive the POCI output in peripheral mode."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Pod::Disable)
    }
    #[doc = "SPI cannot drive the POCI output in peripheral mode."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Pod::Enable)
    }
}
#[doc = "MSB first select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Msb {
    #[doc = "0: LSB first"]
    Disable = 0,
    #[doc = "1: MSB first"]
    Enable = 1,
}
impl From<Msb> for bool {
    #[inline(always)]
    fn from(variant: Msb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSB` reader - MSB first select"]
pub type MsbR = crate::BitReader<Msb>;
impl MsbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Msb {
        match self.bits {
            false => Msb::Disable,
            true => Msb::Enable,
        }
    }
    #[doc = "LSB first"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Msb::Disable
    }
    #[doc = "MSB first"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Msb::Enable
    }
}
#[doc = "Field `MSB` writer - MSB first select"]
pub type MsbW<'a, REG> = crate::BitWriter<'a, REG, Msb>;
impl<'a, REG> MsbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LSB first"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Msb::Disable)
    }
    #[doc = "MSB first"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Msb::Enable)
    }
}
#[doc = "Parity Receive Enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pren {
    #[doc = "0: Disable Parity receive function"]
    Disable = 0,
    #[doc = "1: Enable Parity receive function"]
    Enable = 1,
}
impl From<Pren> for bool {
    #[inline(always)]
    fn from(variant: Pren) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PREN` reader - Parity Receive Enable"]
pub type PrenR = crate::BitReader<Pren>;
impl PrenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pren {
        match self.bits {
            false => Pren::Disable,
            true => Pren::Enable,
        }
    }
    #[doc = "Disable Parity receive function"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Pren::Disable
    }
    #[doc = "Enable Parity receive function"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Pren::Enable
    }
}
#[doc = "Field `PREN` writer - Parity Receive Enable"]
pub type PrenW<'a, REG> = crate::BitWriter<'a, REG, Pren>;
impl<'a, REG> PrenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Parity receive function"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Pren::Disable)
    }
    #[doc = "Enable Parity receive function"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Pren::Enable)
    }
}
#[doc = "Even Parity Select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pes {
    #[doc = "0: Odd Parity mode"]
    Disable = 0,
    #[doc = "1: Even Parity mode"]
    Enable = 1,
}
impl From<Pes> for bool {
    #[inline(always)]
    fn from(variant: Pes) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PES` reader - Even Parity Select"]
pub type PesR = crate::BitReader<Pes>;
impl PesR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pes {
        match self.bits {
            false => Pes::Disable,
            true => Pes::Enable,
        }
    }
    #[doc = "Odd Parity mode"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Pes::Disable
    }
    #[doc = "Even Parity mode"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Pes::Enable
    }
}
#[doc = "Field `PES` writer - Even Parity Select"]
pub type PesW<'a, REG> = crate::BitWriter<'a, REG, Pes>;
impl<'a, REG> PesW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Odd Parity mode"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Pes::Disable)
    }
    #[doc = "Even Parity mode"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Pes::Enable)
    }
}
#[doc = "Parity Transmit Enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pten {
    #[doc = "0: Parity transmission is disabled"]
    Disable = 0,
    #[doc = "1: Parity transmission is enabled"]
    Enable = 1,
}
impl From<Pten> for bool {
    #[inline(always)]
    fn from(variant: Pten) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTEN` reader - Parity Transmit Enable"]
pub type PtenR = crate::BitReader<Pten>;
impl PtenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pten {
        match self.bits {
            false => Pten::Disable,
            true => Pten::Enable,
        }
    }
    #[doc = "Parity transmission is disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Pten::Disable
    }
    #[doc = "Parity transmission is enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Pten::Enable
    }
}
#[doc = "Field `PTEN` writer - Parity Transmit Enable"]
pub type PtenW<'a, REG> = crate::BitWriter<'a, REG, Pten>;
impl<'a, REG> PtenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Parity transmission is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Pten::Disable)
    }
    #[doc = "Parity transmission is enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Pten::Enable)
    }
}
#[doc = "Suspend external communication\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Suspend {
    #[doc = "0: Functional mode resumed"]
    Disable = 0,
    #[doc = "1: External communication suspended"]
    Enable = 1,
}
impl From<Suspend> for bool {
    #[inline(always)]
    fn from(variant: Suspend) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SUSPEND` reader - Suspend external communication"]
pub type SuspendR = crate::BitReader<Suspend>;
impl SuspendR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Suspend {
        match self.bits {
            false => Suspend::Disable,
            true => Suspend::Enable,
        }
    }
    #[doc = "Functional mode resumed"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Suspend::Disable
    }
    #[doc = "External communication suspended"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Suspend::Enable
    }
}
#[doc = "Field `SUSPEND` writer - Suspend external communication"]
pub type SuspendW<'a, REG> = crate::BitWriter<'a, REG, Suspend>;
impl<'a, REG> SuspendW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Functional mode resumed"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Suspend::Disable)
    }
    #[doc = "External communication suspended"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Suspend::Enable)
    }
}
impl R {
    #[doc = "Bit 0 - SPI enable"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Loop back mode enable"]
    #[inline(always)]
    pub fn lbm(&self) -> LbmR {
        LbmR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Controller or peripheral mode select"]
    #[inline(always)]
    pub fn cp(&self) -> CpR {
        CpR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Peripheral-mode: Data output disabled"]
    #[inline(always)]
    pub fn pod(&self) -> PodR {
        PodR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - MSB first select"]
    #[inline(always)]
    pub fn msb(&self) -> MsbR {
        MsbR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Parity Receive Enable"]
    #[inline(always)]
    pub fn pren(&self) -> PrenR {
        PrenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Even Parity Select"]
    #[inline(always)]
    pub fn pes(&self) -> PesR {
        PesR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Parity Transmit Enable"]
    #[inline(always)]
    pub fn pten(&self) -> PtenR {
        PtenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Suspend external communication"]
    #[inline(always)]
    pub fn suspend(&self) -> SuspendR {
        SuspendR::new(((self.bits >> 9) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTL1")
            .field("lbm", &self.lbm())
            .field("cp", &self.cp())
            .field("pod", &self.pod())
            .field("msb", &self.msb())
            .field("pren", &self.pren())
            .field("pes", &self.pes())
            .field("enable", &self.enable())
            .field("pten", &self.pten())
            .field("suspend", &self.suspend())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - SPI enable"]
    #[inline(always)]
    pub fn enable(&mut self) -> EnableW<'_, Ctl1Spec> {
        EnableW::new(self, 0)
    }
    #[doc = "Bit 1 - Loop back mode enable"]
    #[inline(always)]
    pub fn lbm(&mut self) -> LbmW<'_, Ctl1Spec> {
        LbmW::new(self, 1)
    }
    #[doc = "Bit 2 - Controller or peripheral mode select"]
    #[inline(always)]
    pub fn cp(&mut self) -> CpW<'_, Ctl1Spec> {
        CpW::new(self, 2)
    }
    #[doc = "Bit 3 - Peripheral-mode: Data output disabled"]
    #[inline(always)]
    pub fn pod(&mut self) -> PodW<'_, Ctl1Spec> {
        PodW::new(self, 3)
    }
    #[doc = "Bit 4 - MSB first select"]
    #[inline(always)]
    pub fn msb(&mut self) -> MsbW<'_, Ctl1Spec> {
        MsbW::new(self, 4)
    }
    #[doc = "Bit 5 - Parity Receive Enable"]
    #[inline(always)]
    pub fn pren(&mut self) -> PrenW<'_, Ctl1Spec> {
        PrenW::new(self, 5)
    }
    #[doc = "Bit 6 - Even Parity Select"]
    #[inline(always)]
    pub fn pes(&mut self) -> PesW<'_, Ctl1Spec> {
        PesW::new(self, 6)
    }
    #[doc = "Bit 8 - Parity Transmit Enable"]
    #[inline(always)]
    pub fn pten(&mut self) -> PtenW<'_, Ctl1Spec> {
        PtenW::new(self, 8)
    }
    #[doc = "Bit 9 - Suspend external communication"]
    #[inline(always)]
    pub fn suspend(&mut self) -> SuspendW<'_, Ctl1Spec> {
        SuspendW::new(self, 9)
    }
}
#[doc = "SPI control register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ctl1Spec;
impl crate::RegisterSpec for Ctl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl1::R`](R) reader structure"]
impl crate::Readable for Ctl1Spec {}
#[doc = "`write(|w| ..)` method takes [`ctl1::W`](W) writer structure"]
impl crate::Writable for Ctl1Spec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CTL1 to value 0x04"]
impl crate::Resettable for Ctl1Spec {
    const RESET_VALUE: u32 = 0x04;
}
