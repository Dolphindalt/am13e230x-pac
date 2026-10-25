#[doc = "Register `ECCTL2` reader"]
pub type R = crate::R<Ecctl2Spec>;
#[doc = "Register `ECCTL2` writer"]
pub type W = crate::W<Ecctl2Spec>;
#[doc = "Continuous or one-shot\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ContOnesht {
    #[doc = "0: Operate in continuous mode"]
    Cont = 0,
    #[doc = "1: Operate in one-Shot mode"]
    One = 1,
}
impl From<ContOnesht> for bool {
    #[inline(always)]
    fn from(variant: ContOnesht) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CONT_ONESHT` reader - Continuous or one-shot"]
pub type ContOneshtR = crate::BitReader<ContOnesht>;
impl ContOneshtR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ContOnesht {
        match self.bits {
            false => ContOnesht::Cont,
            true => ContOnesht::One,
        }
    }
    #[doc = "Operate in continuous mode"]
    #[inline(always)]
    pub fn is_cont(&self) -> bool {
        *self == ContOnesht::Cont
    }
    #[doc = "Operate in one-Shot mode"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == ContOnesht::One
    }
}
#[doc = "Field `CONT_ONESHT` writer - Continuous or one-shot"]
pub type ContOneshtW<'a, REG> = crate::BitWriter<'a, REG, ContOnesht>;
impl<'a, REG> ContOneshtW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Operate in continuous mode"]
    #[inline(always)]
    pub fn cont(self) -> &'a mut crate::W<REG> {
        self.variant(ContOnesht::Cont)
    }
    #[doc = "Operate in one-Shot mode"]
    #[inline(always)]
    pub fn one(self) -> &'a mut crate::W<REG> {
        self.variant(ContOnesht::One)
    }
}
#[doc = "Stop value for one-shot, Wrap for continuous\n\nValue on reset: 3"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum StopWrap {
    #[doc = "0: Stop after Capture Event 1 in one-shot mode #br#Wrap after Capture Event 1 in continuous mode."]
    StopEvent1 = 0,
    #[doc = "1: Stop after Capture Event 2 in one-shot mode #br#Wrap after Capture Event 2 in continuous mode."]
    StopEvent2 = 1,
    #[doc = "2: Stop after Capture Event 3 in one-shot mode #br#Wrap after Capture Event 3 in continuous mode."]
    StopEvent3 = 2,
    #[doc = "3: Stop after Capture Event 4 in one-shot mode #br#Wrap after Capture Event 4 in continuous mode."]
    StopEvent4 = 3,
}
impl From<StopWrap> for u8 {
    #[inline(always)]
    fn from(variant: StopWrap) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for StopWrap {
    type Ux = u8;
}
impl crate::IsEnum for StopWrap {}
#[doc = "Field `STOP_WRAP` reader - Stop value for one-shot, Wrap for continuous"]
pub type StopWrapR = crate::FieldReader<StopWrap>;
impl StopWrapR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> StopWrap {
        match self.bits {
            0 => StopWrap::StopEvent1,
            1 => StopWrap::StopEvent2,
            2 => StopWrap::StopEvent3,
            3 => StopWrap::StopEvent4,
            _ => unreachable!(),
        }
    }
    #[doc = "Stop after Capture Event 1 in one-shot mode #br#Wrap after Capture Event 1 in continuous mode."]
    #[inline(always)]
    pub fn is_stop_event1(&self) -> bool {
        *self == StopWrap::StopEvent1
    }
    #[doc = "Stop after Capture Event 2 in one-shot mode #br#Wrap after Capture Event 2 in continuous mode."]
    #[inline(always)]
    pub fn is_stop_event2(&self) -> bool {
        *self == StopWrap::StopEvent2
    }
    #[doc = "Stop after Capture Event 3 in one-shot mode #br#Wrap after Capture Event 3 in continuous mode."]
    #[inline(always)]
    pub fn is_stop_event3(&self) -> bool {
        *self == StopWrap::StopEvent3
    }
    #[doc = "Stop after Capture Event 4 in one-shot mode #br#Wrap after Capture Event 4 in continuous mode."]
    #[inline(always)]
    pub fn is_stop_event4(&self) -> bool {
        *self == StopWrap::StopEvent4
    }
}
#[doc = "Field `STOP_WRAP` writer - Stop value for one-shot, Wrap for continuous"]
pub type StopWrapW<'a, REG> = crate::FieldWriter<'a, REG, 2, StopWrap, crate::Safe>;
impl<'a, REG> StopWrapW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Stop after Capture Event 1 in one-shot mode #br#Wrap after Capture Event 1 in continuous mode."]
    #[inline(always)]
    pub fn stop_event1(self) -> &'a mut crate::W<REG> {
        self.variant(StopWrap::StopEvent1)
    }
    #[doc = "Stop after Capture Event 2 in one-shot mode #br#Wrap after Capture Event 2 in continuous mode."]
    #[inline(always)]
    pub fn stop_event2(self) -> &'a mut crate::W<REG> {
        self.variant(StopWrap::StopEvent2)
    }
    #[doc = "Stop after Capture Event 3 in one-shot mode #br#Wrap after Capture Event 3 in continuous mode."]
    #[inline(always)]
    pub fn stop_event3(self) -> &'a mut crate::W<REG> {
        self.variant(StopWrap::StopEvent3)
    }
    #[doc = "Stop after Capture Event 4 in one-shot mode #br#Wrap after Capture Event 4 in continuous mode."]
    #[inline(always)]
    pub fn stop_event4(self) -> &'a mut crate::W<REG> {
        self.variant(StopWrap::StopEvent4)
    }
}
#[doc = "Field `REARM` reader - One-shot re-arm"]
pub type RearmR = crate::BitReader;
#[doc = "Field `REARM` writer - One-shot re-arm"]
pub type RearmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "TSCNT counter stop\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tsctrstop {
    #[doc = "0: TSCTR stopped"]
    Stopped = 0,
    #[doc = "1: TSCTR free-running"]
    FreeRunning = 1,
}
impl From<Tsctrstop> for bool {
    #[inline(always)]
    fn from(variant: Tsctrstop) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSCTRSTOP` reader - TSCNT counter stop"]
pub type TsctrstopR = crate::BitReader<Tsctrstop>;
impl TsctrstopR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tsctrstop {
        match self.bits {
            false => Tsctrstop::Stopped,
            true => Tsctrstop::FreeRunning,
        }
    }
    #[doc = "TSCTR stopped"]
    #[inline(always)]
    pub fn is_stopped(&self) -> bool {
        *self == Tsctrstop::Stopped
    }
    #[doc = "TSCTR free-running"]
    #[inline(always)]
    pub fn is_free_running(&self) -> bool {
        *self == Tsctrstop::FreeRunning
    }
}
#[doc = "Field `TSCTRSTOP` writer - TSCNT counter stop"]
pub type TsctrstopW<'a, REG> = crate::BitWriter<'a, REG, Tsctrstop>;
impl<'a, REG> TsctrstopW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TSCTR stopped"]
    #[inline(always)]
    pub fn stopped(self) -> &'a mut crate::W<REG> {
        self.variant(Tsctrstop::Stopped)
    }
    #[doc = "TSCTR free-running"]
    #[inline(always)]
    pub fn free_running(self) -> &'a mut crate::W<REG> {
        self.variant(Tsctrstop::FreeRunning)
    }
}
#[doc = "Counter sync-in select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SynciEn {
    #[doc = "0: Disable sync-in option"]
    DisableSyncIn = 0,
    #[doc = "1: Enable counter (TSCTR) to be loaded from CTRPHS register upon either a SYNCI signal or a S/W force event."]
    EnableCounterRegister = 1,
}
impl From<SynciEn> for bool {
    #[inline(always)]
    fn from(variant: SynciEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYNCI_EN` reader - Counter sync-in select"]
pub type SynciEnR = crate::BitReader<SynciEn>;
impl SynciEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SynciEn {
        match self.bits {
            false => SynciEn::DisableSyncIn,
            true => SynciEn::EnableCounterRegister,
        }
    }
    #[doc = "Disable sync-in option"]
    #[inline(always)]
    pub fn is_disable_sync_in(&self) -> bool {
        *self == SynciEn::DisableSyncIn
    }
    #[doc = "Enable counter (TSCTR) to be loaded from CTRPHS register upon either a SYNCI signal or a S/W force event."]
    #[inline(always)]
    pub fn is_enable_counter_register(&self) -> bool {
        *self == SynciEn::EnableCounterRegister
    }
}
#[doc = "Field `SYNCI_EN` writer - Counter sync-in select"]
pub type SynciEnW<'a, REG> = crate::BitWriter<'a, REG, SynciEn>;
impl<'a, REG> SynciEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable sync-in option"]
    #[inline(always)]
    pub fn disable_sync_in(self) -> &'a mut crate::W<REG> {
        self.variant(SynciEn::DisableSyncIn)
    }
    #[doc = "Enable counter (TSCTR) to be loaded from CTRPHS register upon either a SYNCI signal or a S/W force event."]
    #[inline(always)]
    pub fn enable_counter_register(self) -> &'a mut crate::W<REG> {
        self.variant(SynciEn::EnableCounterRegister)
    }
}
#[doc = "Sync-out mode\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SyncoSel {
    #[doc = "0: sync out signal is SWSYNC"]
    Swsync = 0,
    #[doc = "1: Select CTR = PRD event to be the sync-out signal. Note: Selection CTR = PRD is meaningful only in APWM mode"]
    CtrPrd = 1,
    #[doc = "2: Disable sync out signal"]
    Disable = 2,
}
impl From<SyncoSel> for u8 {
    #[inline(always)]
    fn from(variant: SyncoSel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SyncoSel {
    type Ux = u8;
}
impl crate::IsEnum for SyncoSel {}
#[doc = "Field `SYNCO_SEL` reader - Sync-out mode"]
pub type SyncoSelR = crate::FieldReader<SyncoSel>;
impl SyncoSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SyncoSel> {
        match self.bits {
            0 => Some(SyncoSel::Swsync),
            1 => Some(SyncoSel::CtrPrd),
            2 => Some(SyncoSel::Disable),
            _ => None,
        }
    }
    #[doc = "sync out signal is SWSYNC"]
    #[inline(always)]
    pub fn is_swsync(&self) -> bool {
        *self == SyncoSel::Swsync
    }
    #[doc = "Select CTR = PRD event to be the sync-out signal. Note: Selection CTR = PRD is meaningful only in APWM mode"]
    #[inline(always)]
    pub fn is_ctr_prd(&self) -> bool {
        *self == SyncoSel::CtrPrd
    }
    #[doc = "Disable sync out signal"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == SyncoSel::Disable
    }
}
#[doc = "Field `SYNCO_SEL` writer - Sync-out mode"]
pub type SyncoSelW<'a, REG> = crate::FieldWriter<'a, REG, 2, SyncoSel, crate::Safe>;
impl<'a, REG> SyncoSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "sync out signal is SWSYNC"]
    #[inline(always)]
    pub fn swsync(self) -> &'a mut crate::W<REG> {
        self.variant(SyncoSel::Swsync)
    }
    #[doc = "Select CTR = PRD event to be the sync-out signal. Note: Selection CTR = PRD is meaningful only in APWM mode"]
    #[inline(always)]
    pub fn ctr_prd(self) -> &'a mut crate::W<REG> {
        self.variant(SyncoSel::CtrPrd)
    }
    #[doc = "Disable sync out signal"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(SyncoSel::Disable)
    }
}
#[doc = "Field `SWSYNC` reader - SW forced counter sync"]
pub type SwsyncR = crate::BitReader;
#[doc = "Field `SWSYNC` writer - SW forced counter sync"]
pub type SwsyncW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "CAP/APWM operating mode select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CapApwm {
    #[doc = "0: ECAP module operates in capture mode"]
    Capture = 0,
    #[doc = "1: ECAP module operates in APWM mode"]
    Apwm = 1,
}
impl From<CapApwm> for bool {
    #[inline(always)]
    fn from(variant: CapApwm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAP_APWM` reader - CAP/APWM operating mode select"]
pub type CapApwmR = crate::BitReader<CapApwm>;
impl CapApwmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CapApwm {
        match self.bits {
            false => CapApwm::Capture,
            true => CapApwm::Apwm,
        }
    }
    #[doc = "ECAP module operates in capture mode"]
    #[inline(always)]
    pub fn is_capture(&self) -> bool {
        *self == CapApwm::Capture
    }
    #[doc = "ECAP module operates in APWM mode"]
    #[inline(always)]
    pub fn is_apwm(&self) -> bool {
        *self == CapApwm::Apwm
    }
}
#[doc = "Field `CAP_APWM` writer - CAP/APWM operating mode select"]
pub type CapApwmW<'a, REG> = crate::BitWriter<'a, REG, CapApwm>;
impl<'a, REG> CapApwmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ECAP module operates in capture mode"]
    #[inline(always)]
    pub fn capture(self) -> &'a mut crate::W<REG> {
        self.variant(CapApwm::Capture)
    }
    #[doc = "ECAP module operates in APWM mode"]
    #[inline(always)]
    pub fn apwm(self) -> &'a mut crate::W<REG> {
        self.variant(CapApwm::Apwm)
    }
}
#[doc = "APWM output polarity select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Apwmpol {
    #[doc = "0: Output is active high (Compare value defines high time)"]
    High = 0,
    #[doc = "1: Output is active low (Compare value defines low time)"]
    Low = 1,
}
impl From<Apwmpol> for bool {
    #[inline(always)]
    fn from(variant: Apwmpol) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `APWMPOL` reader - APWM output polarity select"]
pub type ApwmpolR = crate::BitReader<Apwmpol>;
impl ApwmpolR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Apwmpol {
        match self.bits {
            false => Apwmpol::High,
            true => Apwmpol::Low,
        }
    }
    #[doc = "Output is active high (Compare value defines high time)"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Apwmpol::High
    }
    #[doc = "Output is active low (Compare value defines low time)"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Apwmpol::Low
    }
}
#[doc = "Field `APWMPOL` writer - APWM output polarity select"]
pub type ApwmpolW<'a, REG> = crate::BitWriter<'a, REG, Apwmpol>;
impl<'a, REG> ApwmpolW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output is active high (Compare value defines high time)"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Apwmpol::High)
    }
    #[doc = "Output is active low (Compare value defines low time)"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Apwmpol::Low)
    }
}
#[doc = "Field `CTRFILTRESET` reader - Reset event filter, modulus counter, and interrupt flags."]
pub type CtrfiltresetR = crate::BitReader;
#[doc = "Field `CTRFILTRESET` writer - Reset event filter, modulus counter, and interrupt flags."]
pub type CtrfiltresetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAEVTSEL` reader - DMA event select"]
pub type DmaevtselR = crate::FieldReader;
#[doc = "Field `DMAEVTSEL` writer - DMA event select"]
pub type DmaevtselW<'a, REG> = crate::FieldWriter<'a, REG, 2, u8, crate::Safe>;
#[doc = "Field `MODCNTRSTS` reader - modulo counter status"]
pub type ModcntrstsR = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Continuous or one-shot"]
    #[inline(always)]
    pub fn cont_onesht(&self) -> ContOneshtR {
        ContOneshtR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Stop value for one-shot, Wrap for continuous"]
    #[inline(always)]
    pub fn stop_wrap(&self) -> StopWrapR {
        StopWrapR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - One-shot re-arm"]
    #[inline(always)]
    pub fn rearm(&self) -> RearmR {
        RearmR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TSCNT counter stop"]
    #[inline(always)]
    pub fn tsctrstop(&self) -> TsctrstopR {
        TsctrstopR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Counter sync-in select"]
    #[inline(always)]
    pub fn synci_en(&self) -> SynciEnR {
        SynciEnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Sync-out mode"]
    #[inline(always)]
    pub fn synco_sel(&self) -> SyncoSelR {
        SyncoSelR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - SW forced counter sync"]
    #[inline(always)]
    pub fn swsync(&self) -> SwsyncR {
        SwsyncR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CAP/APWM operating mode select"]
    #[inline(always)]
    pub fn cap_apwm(&self) -> CapApwmR {
        CapApwmR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - APWM output polarity select"]
    #[inline(always)]
    pub fn apwmpol(&self) -> ApwmpolR {
        ApwmpolR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Reset event filter, modulus counter, and interrupt flags."]
    #[inline(always)]
    pub fn ctrfiltreset(&self) -> CtrfiltresetR {
        CtrfiltresetR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - DMA event select"]
    #[inline(always)]
    pub fn dmaevtsel(&self) -> DmaevtselR {
        DmaevtselR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - modulo counter status"]
    #[inline(always)]
    pub fn modcntrsts(&self) -> ModcntrstsR {
        ModcntrstsR::new(((self.bits >> 14) & 3) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ECCTL2")
            .field("cont_onesht", &self.cont_onesht())
            .field("stop_wrap", &self.stop_wrap())
            .field("rearm", &self.rearm())
            .field("tsctrstop", &self.tsctrstop())
            .field("synci_en", &self.synci_en())
            .field("synco_sel", &self.synco_sel())
            .field("swsync", &self.swsync())
            .field("cap_apwm", &self.cap_apwm())
            .field("apwmpol", &self.apwmpol())
            .field("ctrfiltreset", &self.ctrfiltreset())
            .field("dmaevtsel", &self.dmaevtsel())
            .field("modcntrsts", &self.modcntrsts())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Continuous or one-shot"]
    #[inline(always)]
    pub fn cont_onesht(&mut self) -> ContOneshtW<'_, Ecctl2Spec> {
        ContOneshtW::new(self, 0)
    }
    #[doc = "Bits 1:2 - Stop value for one-shot, Wrap for continuous"]
    #[inline(always)]
    pub fn stop_wrap(&mut self) -> StopWrapW<'_, Ecctl2Spec> {
        StopWrapW::new(self, 1)
    }
    #[doc = "Bit 3 - One-shot re-arm"]
    #[inline(always)]
    pub fn rearm(&mut self) -> RearmW<'_, Ecctl2Spec> {
        RearmW::new(self, 3)
    }
    #[doc = "Bit 4 - TSCNT counter stop"]
    #[inline(always)]
    pub fn tsctrstop(&mut self) -> TsctrstopW<'_, Ecctl2Spec> {
        TsctrstopW::new(self, 4)
    }
    #[doc = "Bit 5 - Counter sync-in select"]
    #[inline(always)]
    pub fn synci_en(&mut self) -> SynciEnW<'_, Ecctl2Spec> {
        SynciEnW::new(self, 5)
    }
    #[doc = "Bits 6:7 - Sync-out mode"]
    #[inline(always)]
    pub fn synco_sel(&mut self) -> SyncoSelW<'_, Ecctl2Spec> {
        SyncoSelW::new(self, 6)
    }
    #[doc = "Bit 8 - SW forced counter sync"]
    #[inline(always)]
    pub fn swsync(&mut self) -> SwsyncW<'_, Ecctl2Spec> {
        SwsyncW::new(self, 8)
    }
    #[doc = "Bit 9 - CAP/APWM operating mode select"]
    #[inline(always)]
    pub fn cap_apwm(&mut self) -> CapApwmW<'_, Ecctl2Spec> {
        CapApwmW::new(self, 9)
    }
    #[doc = "Bit 10 - APWM output polarity select"]
    #[inline(always)]
    pub fn apwmpol(&mut self) -> ApwmpolW<'_, Ecctl2Spec> {
        ApwmpolW::new(self, 10)
    }
    #[doc = "Bit 11 - Reset event filter, modulus counter, and interrupt flags."]
    #[inline(always)]
    pub fn ctrfiltreset(&mut self) -> CtrfiltresetW<'_, Ecctl2Spec> {
        CtrfiltresetW::new(self, 11)
    }
    #[doc = "Bits 12:13 - DMA event select"]
    #[inline(always)]
    pub fn dmaevtsel(&mut self) -> DmaevtselW<'_, Ecctl2Spec> {
        DmaevtselW::new(self, 12)
    }
}
#[doc = "Capture Control Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`ecctl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ecctl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ecctl2Spec;
impl crate::RegisterSpec for Ecctl2Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ecctl2::R`](R) reader structure"]
impl crate::Readable for Ecctl2Spec {}
#[doc = "`write(|w| ..)` method takes [`ecctl2::W`](W) writer structure"]
impl crate::Writable for Ecctl2Spec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets ECCTL2 to value 0x06"]
impl crate::Resettable for Ecctl2Spec {
    const RESET_VALUE: u16 = 0x06;
}
