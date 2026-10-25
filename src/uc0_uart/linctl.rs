#[doc = "Register `LINCTL` reader"]
pub type R = crate::R<LinctlSpec>;
#[doc = "Register `LINCTL` writer"]
pub type W = crate::W<LinctlSpec>;
#[doc = "LIN Counter Enable. LIN counter will only count when enabled.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctrena {
    #[doc = "0: Counter disabled"]
    Disable = 0,
    #[doc = "1: Counter enabled"]
    Enable = 1,
}
impl From<Ctrena> for bool {
    #[inline(always)]
    fn from(variant: Ctrena) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTRENA` reader - LIN Counter Enable. LIN counter will only count when enabled."]
pub type CtrenaR = crate::BitReader<Ctrena>;
impl CtrenaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctrena {
        match self.bits {
            false => Ctrena::Disable,
            true => Ctrena::Enable,
        }
    }
    #[doc = "Counter disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Ctrena::Disable
    }
    #[doc = "Counter enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Ctrena::Enable
    }
}
#[doc = "Field `CTRENA` writer - LIN Counter Enable. LIN counter will only count when enabled."]
pub type CtrenaW<'a, REG> = crate::BitWriter<'a, REG, Ctrena>;
impl<'a, REG> CtrenaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Ctrena::Disable)
    }
    #[doc = "Counter enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Ctrena::Enable)
    }
}
#[doc = "Zero on negative Edge of RXD. When enabled the counter is set to 0 and starts counting on a negative edge of RXD\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Zerone {
    #[doc = "0: Zero on negative edge disabled"]
    Disable = 0,
    #[doc = "1: Zero on negative edge enabled"]
    Enable = 1,
}
impl From<Zerone> for bool {
    #[inline(always)]
    fn from(variant: Zerone) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ZERONE` reader - Zero on negative Edge of RXD. When enabled the counter is set to 0 and starts counting on a negative edge of RXD"]
pub type ZeroneR = crate::BitReader<Zerone>;
impl ZeroneR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Zerone {
        match self.bits {
            false => Zerone::Disable,
            true => Zerone::Enable,
        }
    }
    #[doc = "Zero on negative edge disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Zerone::Disable
    }
    #[doc = "Zero on negative edge enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Zerone::Enable
    }
}
#[doc = "Field `ZERONE` writer - Zero on negative Edge of RXD. When enabled the counter is set to 0 and starts counting on a negative edge of RXD"]
pub type ZeroneW<'a, REG> = crate::BitWriter<'a, REG, Zerone>;
impl<'a, REG> ZeroneW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Zero on negative edge disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Zerone::Disable)
    }
    #[doc = "Zero on negative edge enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Zerone::Enable)
    }
}
#[doc = "Count while low Signal on RXD When counter is enabled and the signal on RXD is low, the counter increments.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cntrxlow {
    #[doc = "0: Count while low Signal on UARTxRXD disabled"]
    Disable = 0,
    #[doc = "1: Count while low Signal on UARTxRXD enabled"]
    Enable = 1,
}
impl From<Cntrxlow> for bool {
    #[inline(always)]
    fn from(variant: Cntrxlow) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CNTRXLOW` reader - Count while low Signal on RXD When counter is enabled and the signal on RXD is low, the counter increments."]
pub type CntrxlowR = crate::BitReader<Cntrxlow>;
impl CntrxlowR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cntrxlow {
        match self.bits {
            false => Cntrxlow::Disable,
            true => Cntrxlow::Enable,
        }
    }
    #[doc = "Count while low Signal on UARTxRXD disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Cntrxlow::Disable
    }
    #[doc = "Count while low Signal on UARTxRXD enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Cntrxlow::Enable
    }
}
#[doc = "Field `CNTRXLOW` writer - Count while low Signal on RXD When counter is enabled and the signal on RXD is low, the counter increments."]
pub type CntrxlowW<'a, REG> = crate::BitWriter<'a, REG, Cntrxlow>;
impl<'a, REG> CntrxlowW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Count while low Signal on UARTxRXD disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Cntrxlow::Disable)
    }
    #[doc = "Count while low Signal on UARTxRXD enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Cntrxlow::Enable)
    }
}
#[doc = "Capture Counter on negative RXD Edge. When enabled the counter value is captured to LINC0 register on each negative RXD edge. A LINC0 interrupt is triggered when enabled.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Linc0cap {
    #[doc = "0: Capture counter on negative UARTxRXD edge disabled"]
    Disable = 0,
    #[doc = "1: Capture counter on negative UARTxRXD edge enabled"]
    Enable = 1,
}
impl From<Linc0cap> for bool {
    #[inline(always)]
    fn from(variant: Linc0cap) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LINC0CAP` reader - Capture Counter on negative RXD Edge. When enabled the counter value is captured to LINC0 register on each negative RXD edge. A LINC0 interrupt is triggered when enabled."]
pub type Linc0capR = crate::BitReader<Linc0cap>;
impl Linc0capR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Linc0cap {
        match self.bits {
            false => Linc0cap::Disable,
            true => Linc0cap::Enable,
        }
    }
    #[doc = "Capture counter on negative UARTxRXD edge disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Linc0cap::Disable
    }
    #[doc = "Capture counter on negative UARTxRXD edge enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Linc0cap::Enable
    }
}
#[doc = "Field `LINC0CAP` writer - Capture Counter on negative RXD Edge. When enabled the counter value is captured to LINC0 register on each negative RXD edge. A LINC0 interrupt is triggered when enabled."]
pub type Linc0capW<'a, REG> = crate::BitWriter<'a, REG, Linc0cap>;
impl<'a, REG> Linc0capW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Capture counter on negative UARTxRXD edge disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Linc0cap::Disable)
    }
    #[doc = "Capture counter on negative UARTxRXD edge enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Linc0cap::Enable)
    }
}
#[doc = "Capture Counter on positive RXD Edge. When enabled the counter value is captured to LINC1 register on each positive RXD edge. A LINC1 interrupt is triggered when enabled.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Linc1cap {
    #[doc = "0: Capture counter on positive UARTxRXD edge disabled"]
    Disable = 0,
    #[doc = "1: Capture counter on positive UARTxRXD edge enabled"]
    Enable = 1,
}
impl From<Linc1cap> for bool {
    #[inline(always)]
    fn from(variant: Linc1cap) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LINC1CAP` reader - Capture Counter on positive RXD Edge. When enabled the counter value is captured to LINC1 register on each positive RXD edge. A LINC1 interrupt is triggered when enabled."]
pub type Linc1capR = crate::BitReader<Linc1cap>;
impl Linc1capR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Linc1cap {
        match self.bits {
            false => Linc1cap::Disable,
            true => Linc1cap::Enable,
        }
    }
    #[doc = "Capture counter on positive UARTxRXD edge disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Linc1cap::Disable
    }
    #[doc = "Capture counter on positive UARTxRXD edge enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Linc1cap::Enable
    }
}
#[doc = "Field `LINC1CAP` writer - Capture Counter on positive RXD Edge. When enabled the counter value is captured to LINC1 register on each positive RXD edge. A LINC1 interrupt is triggered when enabled."]
pub type Linc1capW<'a, REG> = crate::BitWriter<'a, REG, Linc1cap>;
impl<'a, REG> Linc1capW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Capture counter on positive UARTxRXD edge disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Linc1cap::Disable)
    }
    #[doc = "Capture counter on positive UARTxRXD edge enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Linc1cap::Enable)
    }
}
#[doc = "Counter Compare Match Mode When this bit is set to 1 a counter compare match with LINC0 register triggers an LINC0 interrupt when enabled.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Linc0Match {
    #[doc = "0: Counter compare match mode disabled (capture mode enabled)"]
    Disable = 0,
    #[doc = "1: Counter compare match enabled (capture mode disabled)"]
    Enable = 1,
}
impl From<Linc0Match> for bool {
    #[inline(always)]
    fn from(variant: Linc0Match) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LINC0_MATCH` reader - Counter Compare Match Mode When this bit is set to 1 a counter compare match with LINC0 register triggers an LINC0 interrupt when enabled."]
pub type Linc0MatchR = crate::BitReader<Linc0Match>;
impl Linc0MatchR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Linc0Match {
        match self.bits {
            false => Linc0Match::Disable,
            true => Linc0Match::Enable,
        }
    }
    #[doc = "Counter compare match mode disabled (capture mode enabled)"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Linc0Match::Disable
    }
    #[doc = "Counter compare match enabled (capture mode disabled)"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Linc0Match::Enable
    }
}
#[doc = "Field `LINC0_MATCH` writer - Counter Compare Match Mode When this bit is set to 1 a counter compare match with LINC0 register triggers an LINC0 interrupt when enabled."]
pub type Linc0MatchW<'a, REG> = crate::BitWriter<'a, REG, Linc0Match>;
impl<'a, REG> Linc0MatchW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter compare match mode disabled (capture mode enabled)"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Linc0Match::Disable)
    }
    #[doc = "Counter compare match enabled (capture mode disabled)"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Linc0Match::Enable)
    }
}
#[doc = "Enable FRAME ERROR\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnFrmErr {
    #[doc = "0: Disable Frame Error Generation If STOP bit of '0' is received, then, FRAME ERROR is not generated"]
    Disable = 0,
    #[doc = "1: Enable Frame Error Generation If STOP bit of '0' is received, then, FRAME ERROR is generated"]
    Enable = 1,
}
impl From<EnFrmErr> for bool {
    #[inline(always)]
    fn from(variant: EnFrmErr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EN_FRM_ERR` reader - Enable FRAME ERROR"]
pub type EnFrmErrR = crate::BitReader<EnFrmErr>;
impl EnFrmErrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnFrmErr {
        match self.bits {
            false => EnFrmErr::Disable,
            true => EnFrmErr::Enable,
        }
    }
    #[doc = "Disable Frame Error Generation If STOP bit of '0' is received, then, FRAME ERROR is not generated"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == EnFrmErr::Disable
    }
    #[doc = "Enable Frame Error Generation If STOP bit of '0' is received, then, FRAME ERROR is generated"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == EnFrmErr::Enable
    }
}
#[doc = "Field `EN_FRM_ERR` writer - Enable FRAME ERROR"]
pub type EnFrmErrW<'a, REG> = crate::BitWriter<'a, REG, EnFrmErr>;
impl<'a, REG> EnFrmErrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Frame Error Generation If STOP bit of '0' is received, then, FRAME ERROR is not generated"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(EnFrmErr::Disable)
    }
    #[doc = "Enable Frame Error Generation If STOP bit of '0' is received, then, FRAME ERROR is generated"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(EnFrmErr::Enable)
    }
}
impl R {
    #[doc = "Bit 0 - LIN Counter Enable. LIN counter will only count when enabled."]
    #[inline(always)]
    pub fn ctrena(&self) -> CtrenaR {
        CtrenaR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Zero on negative Edge of RXD. When enabled the counter is set to 0 and starts counting on a negative edge of RXD"]
    #[inline(always)]
    pub fn zerone(&self) -> ZeroneR {
        ZeroneR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Count while low Signal on RXD When counter is enabled and the signal on RXD is low, the counter increments."]
    #[inline(always)]
    pub fn cntrxlow(&self) -> CntrxlowR {
        CntrxlowR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Capture Counter on negative RXD Edge. When enabled the counter value is captured to LINC0 register on each negative RXD edge. A LINC0 interrupt is triggered when enabled."]
    #[inline(always)]
    pub fn linc0cap(&self) -> Linc0capR {
        Linc0capR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Capture Counter on positive RXD Edge. When enabled the counter value is captured to LINC1 register on each positive RXD edge. A LINC1 interrupt is triggered when enabled."]
    #[inline(always)]
    pub fn linc1cap(&self) -> Linc1capR {
        Linc1capR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Counter Compare Match Mode When this bit is set to 1 a counter compare match with LINC0 register triggers an LINC0 interrupt when enabled."]
    #[inline(always)]
    pub fn linc0_match(&self) -> Linc0MatchR {
        Linc0MatchR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable FRAME ERROR"]
    #[inline(always)]
    pub fn en_frm_err(&self) -> EnFrmErrR {
        EnFrmErrR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LINCTL")
            .field("ctrena", &self.ctrena())
            .field("zerone", &self.zerone())
            .field("cntrxlow", &self.cntrxlow())
            .field("linc0cap", &self.linc0cap())
            .field("linc1cap", &self.linc1cap())
            .field("linc0_match", &self.linc0_match())
            .field("en_frm_err", &self.en_frm_err())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - LIN Counter Enable. LIN counter will only count when enabled."]
    #[inline(always)]
    pub fn ctrena(&mut self) -> CtrenaW<'_, LinctlSpec> {
        CtrenaW::new(self, 0)
    }
    #[doc = "Bit 1 - Zero on negative Edge of RXD. When enabled the counter is set to 0 and starts counting on a negative edge of RXD"]
    #[inline(always)]
    pub fn zerone(&mut self) -> ZeroneW<'_, LinctlSpec> {
        ZeroneW::new(self, 1)
    }
    #[doc = "Bit 2 - Count while low Signal on RXD When counter is enabled and the signal on RXD is low, the counter increments."]
    #[inline(always)]
    pub fn cntrxlow(&mut self) -> CntrxlowW<'_, LinctlSpec> {
        CntrxlowW::new(self, 2)
    }
    #[doc = "Bit 4 - Capture Counter on negative RXD Edge. When enabled the counter value is captured to LINC0 register on each negative RXD edge. A LINC0 interrupt is triggered when enabled."]
    #[inline(always)]
    pub fn linc0cap(&mut self) -> Linc0capW<'_, LinctlSpec> {
        Linc0capW::new(self, 4)
    }
    #[doc = "Bit 5 - Capture Counter on positive RXD Edge. When enabled the counter value is captured to LINC1 register on each positive RXD edge. A LINC1 interrupt is triggered when enabled."]
    #[inline(always)]
    pub fn linc1cap(&mut self) -> Linc1capW<'_, LinctlSpec> {
        Linc1capW::new(self, 5)
    }
    #[doc = "Bit 6 - Counter Compare Match Mode When this bit is set to 1 a counter compare match with LINC0 register triggers an LINC0 interrupt when enabled."]
    #[inline(always)]
    pub fn linc0_match(&mut self) -> Linc0MatchW<'_, LinctlSpec> {
        Linc0MatchW::new(self, 6)
    }
    #[doc = "Bit 7 - Enable FRAME ERROR"]
    #[inline(always)]
    pub fn en_frm_err(&mut self) -> EnFrmErrW<'_, LinctlSpec> {
        EnFrmErrW::new(self, 7)
    }
}
#[doc = "UART LIN Mode Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`linctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`linctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LinctlSpec;
impl crate::RegisterSpec for LinctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`linctl::R`](R) reader structure"]
impl crate::Readable for LinctlSpec {}
#[doc = "`write(|w| ..)` method takes [`linctl::W`](W) writer structure"]
impl crate::Writable for LinctlSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets LINCTL to value 0"]
impl crate::Resettable for LinctlSpec {}
