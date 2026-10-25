#[doc = "Register `STAT` reader"]
pub type R = crate::R<StatSpec>;
#[doc = "Register `STAT` writer"]
pub type W = crate::W<StatSpec>;
#[doc = "Register Active\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Active {
    #[doc = "0: If NBRBUSY is set, the EPIRPSTD0 register is active.If the NBRBUSY bit is clear, then neither EPIRPSTDx register is active."]
    Std0 = 0,
    #[doc = "1: The EPIRPSTD1 register is active."]
    Std1 = 1,
}
impl From<Active> for bool {
    #[inline(always)]
    fn from(variant: Active) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACTIVE` reader - Register Active"]
pub type ActiveR = crate::BitReader<Active>;
impl ActiveR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Active {
        match self.bits {
            false => Active::Std0,
            true => Active::Std1,
        }
    }
    #[doc = "If NBRBUSY is set, the EPIRPSTD0 register is active.If the NBRBUSY bit is clear, then neither EPIRPSTDx register is active."]
    #[inline(always)]
    pub fn is_std0(&self) -> bool {
        *self == Active::Std0
    }
    #[doc = "The EPIRPSTD1 register is active."]
    #[inline(always)]
    pub fn is_std1(&self) -> bool {
        *self == Active::Std1
    }
}
#[doc = "Field `ACTIVE` writer - Register Active"]
pub type ActiveW<'a, REG> = crate::BitWriter<'a, REG, Active>;
impl<'a, REG> ActiveW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "If NBRBUSY is set, the EPIRPSTD0 register is active.If the NBRBUSY bit is clear, then neither EPIRPSTDx register is active."]
    #[inline(always)]
    pub fn std0(self) -> &'a mut crate::W<REG> {
        self.variant(Active::Std0)
    }
    #[doc = "The EPIRPSTD1 register is active."]
    #[inline(always)]
    pub fn std1(self) -> &'a mut crate::W<REG> {
        self.variant(Active::Std1)
    }
}
#[doc = "Non-Blocking Read Busy\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Nbrbusy {
    #[doc = "0: The external interface is not performing a non-blocking read."]
    Free = 0,
    #[doc = "1: The external interface is performing a non-blocking read, or if the non-blocking read is paused due to a write."]
    Busy = 1,
}
impl From<Nbrbusy> for bool {
    #[inline(always)]
    fn from(variant: Nbrbusy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NBRBUSY` reader - Non-Blocking Read Busy"]
pub type NbrbusyR = crate::BitReader<Nbrbusy>;
impl NbrbusyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Nbrbusy {
        match self.bits {
            false => Nbrbusy::Free,
            true => Nbrbusy::Busy,
        }
    }
    #[doc = "The external interface is not performing a non-blocking read."]
    #[inline(always)]
    pub fn is_free(&self) -> bool {
        *self == Nbrbusy::Free
    }
    #[doc = "The external interface is performing a non-blocking read, or if the non-blocking read is paused due to a write."]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == Nbrbusy::Busy
    }
}
#[doc = "Field `NBRBUSY` writer - Non-Blocking Read Busy"]
pub type NbrbusyW<'a, REG> = crate::BitWriter<'a, REG, Nbrbusy>;
impl<'a, REG> NbrbusyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The external interface is not performing a non-blocking read."]
    #[inline(always)]
    pub fn free(self) -> &'a mut crate::W<REG> {
        self.variant(Nbrbusy::Free)
    }
    #[doc = "The external interface is performing a non-blocking read, or if the non-blocking read is paused due to a write."]
    #[inline(always)]
    pub fn busy(self) -> &'a mut crate::W<REG> {
        self.variant(Nbrbusy::Busy)
    }
}
#[doc = "Write Busy\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wbusy {
    #[doc = "0: The external interface is not performing a write."]
    Free = 0,
    #[doc = "1: The external interface is performing a write."]
    Busy = 1,
}
impl From<Wbusy> for bool {
    #[inline(always)]
    fn from(variant: Wbusy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WBUSY` reader - Write Busy"]
pub type WbusyR = crate::BitReader<Wbusy>;
impl WbusyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wbusy {
        match self.bits {
            false => Wbusy::Free,
            true => Wbusy::Busy,
        }
    }
    #[doc = "The external interface is not performing a write."]
    #[inline(always)]
    pub fn is_free(&self) -> bool {
        *self == Wbusy::Free
    }
    #[doc = "The external interface is performing a write."]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == Wbusy::Busy
    }
}
#[doc = "Field `WBUSY` writer - Write Busy"]
pub type WbusyW<'a, REG> = crate::BitWriter<'a, REG, Wbusy>;
impl<'a, REG> WbusyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The external interface is not performing a write."]
    #[inline(always)]
    pub fn free(self) -> &'a mut crate::W<REG> {
        self.variant(Wbusy::Free)
    }
    #[doc = "The external interface is performing a write."]
    #[inline(always)]
    pub fn busy(self) -> &'a mut crate::W<REG> {
        self.variant(Wbusy::Busy)
    }
}
#[doc = "Initialization Sequence\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Initseq {
    #[doc = "0: The SDRAM interface is not in the wakeup period."]
    Okay = 0,
    #[doc = "1: The SDRAM interface is running through the wakeup period (greater than 100 microseconds).If an attempt is made to read or write the SDRAM during this period, the access is held off until the wakeup period is complete."]
    Busy = 1,
}
impl From<Initseq> for bool {
    #[inline(always)]
    fn from(variant: Initseq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INITSEQ` reader - Initialization Sequence"]
pub type InitseqR = crate::BitReader<Initseq>;
impl InitseqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Initseq {
        match self.bits {
            false => Initseq::Okay,
            true => Initseq::Busy,
        }
    }
    #[doc = "The SDRAM interface is not in the wakeup period."]
    #[inline(always)]
    pub fn is_okay(&self) -> bool {
        *self == Initseq::Okay
    }
    #[doc = "The SDRAM interface is running through the wakeup period (greater than 100 microseconds).If an attempt is made to read or write the SDRAM during this period, the access is held off until the wakeup period is complete."]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == Initseq::Busy
    }
}
#[doc = "Field `INITSEQ` writer - Initialization Sequence"]
pub type InitseqW<'a, REG> = crate::BitWriter<'a, REG, Initseq>;
impl<'a, REG> InitseqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The SDRAM interface is not in the wakeup period."]
    #[inline(always)]
    pub fn okay(self) -> &'a mut crate::W<REG> {
        self.variant(Initseq::Okay)
    }
    #[doc = "The SDRAM interface is running through the wakeup period (greater than 100 microseconds).If an attempt is made to read or write the SDRAM during this period, the access is held off until the wakeup period is complete."]
    #[inline(always)]
    pub fn busy(self) -> &'a mut crate::W<REG> {
        self.variant(Initseq::Busy)
    }
}
#[doc = "External FIFO Empty\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Xfempty {
    #[doc = "0: The XFIFO is not signaling as empty."]
    Okay = 0,
    #[doc = "1: The XFIFO is signaling as empty (the FIFO empty signal is high). Attempts to read in this case are stalled until the XFIFO empty signal goes low or the counter times out as specified by the MAXWAIT field."]
    Empty = 1,
}
impl From<Xfempty> for bool {
    #[inline(always)]
    fn from(variant: Xfempty) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `XFEMPTY` reader - External FIFO Empty"]
pub type XfemptyR = crate::BitReader<Xfempty>;
impl XfemptyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Xfempty {
        match self.bits {
            false => Xfempty::Okay,
            true => Xfempty::Empty,
        }
    }
    #[doc = "The XFIFO is not signaling as empty."]
    #[inline(always)]
    pub fn is_okay(&self) -> bool {
        *self == Xfempty::Okay
    }
    #[doc = "The XFIFO is signaling as empty (the FIFO empty signal is high). Attempts to read in this case are stalled until the XFIFO empty signal goes low or the counter times out as specified by the MAXWAIT field."]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == Xfempty::Empty
    }
}
#[doc = "Field `XFEMPTY` writer - External FIFO Empty"]
pub type XfemptyW<'a, REG> = crate::BitWriter<'a, REG, Xfempty>;
impl<'a, REG> XfemptyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The XFIFO is not signaling as empty."]
    #[inline(always)]
    pub fn okay(self) -> &'a mut crate::W<REG> {
        self.variant(Xfempty::Okay)
    }
    #[doc = "The XFIFO is signaling as empty (the FIFO empty signal is high). Attempts to read in this case are stalled until the XFIFO empty signal goes low or the counter times out as specified by the MAXWAIT field."]
    #[inline(always)]
    pub fn empty(self) -> &'a mut crate::W<REG> {
        self.variant(Xfempty::Empty)
    }
}
#[doc = "External FIFO Full\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Xffull {
    #[doc = "0: The XFIFO is not signaling as full."]
    Okay = 0,
    #[doc = "1: The XFIFO is signaling as full (the FIFO full signal is high). Attempts to write in this case are stalled until the XFIFO full signal goes low or the counter times out as specified by the MAXWAIT field."]
    Full = 1,
}
impl From<Xffull> for bool {
    #[inline(always)]
    fn from(variant: Xffull) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `XFFULL` reader - External FIFO Full"]
pub type XffullR = crate::BitReader<Xffull>;
impl XffullR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Xffull {
        match self.bits {
            false => Xffull::Okay,
            true => Xffull::Full,
        }
    }
    #[doc = "The XFIFO is not signaling as full."]
    #[inline(always)]
    pub fn is_okay(&self) -> bool {
        *self == Xffull::Okay
    }
    #[doc = "The XFIFO is signaling as full (the FIFO full signal is high). Attempts to write in this case are stalled until the XFIFO full signal goes low or the counter times out as specified by the MAXWAIT field."]
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == Xffull::Full
    }
}
#[doc = "Field `XFFULL` writer - External FIFO Full"]
pub type XffullW<'a, REG> = crate::BitWriter<'a, REG, Xffull>;
impl<'a, REG> XffullW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The XFIFO is not signaling as full."]
    #[inline(always)]
    pub fn okay(self) -> &'a mut crate::W<REG> {
        self.variant(Xffull::Okay)
    }
    #[doc = "The XFIFO is signaling as full (the FIFO full signal is high). Attempts to write in this case are stalled until the XFIFO full signal goes low or the counter times out as specified by the MAXWAIT field."]
    #[inline(always)]
    pub fn full(self) -> &'a mut crate::W<REG> {
        self.variant(Xffull::Full)
    }
}
#[doc = "Clock Enable Low\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Celow {
    #[doc = "0: The external device is not gating the clock."]
    Off = 0,
    #[doc = "1: The external device is gating the clock (iRDY is low). Attempts to read or write in this situation are stalled until the clock is enabled or the counter times out as specified by the MAXWAIT field."]
    On = 1,
}
impl From<Celow> for bool {
    #[inline(always)]
    fn from(variant: Celow) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CELOW` reader - Clock Enable Low"]
pub type CelowR = crate::BitReader<Celow>;
impl CelowR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Celow {
        match self.bits {
            false => Celow::Off,
            true => Celow::On,
        }
    }
    #[doc = "The external device is not gating the clock."]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Celow::Off
    }
    #[doc = "The external device is gating the clock (iRDY is low). Attempts to read or write in this situation are stalled until the clock is enabled or the counter times out as specified by the MAXWAIT field."]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == Celow::On
    }
}
#[doc = "Field `CELOW` writer - Clock Enable Low"]
pub type CelowW<'a, REG> = crate::BitWriter<'a, REG, Celow>;
impl<'a, REG> CelowW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The external device is not gating the clock."]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Celow::Off)
    }
    #[doc = "The external device is gating the clock (iRDY is low). Attempts to read or write in this situation are stalled until the clock is enabled or the counter times out as specified by the MAXWAIT field."]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(Celow::On)
    }
}
impl R {
    #[doc = "Bit 0 - Register Active"]
    #[inline(always)]
    pub fn active(&self) -> ActiveR {
        ActiveR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - Non-Blocking Read Busy"]
    #[inline(always)]
    pub fn nbrbusy(&self) -> NbrbusyR {
        NbrbusyR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Write Busy"]
    #[inline(always)]
    pub fn wbusy(&self) -> WbusyR {
        WbusyR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Initialization Sequence"]
    #[inline(always)]
    pub fn initseq(&self) -> InitseqR {
        InitseqR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - External FIFO Empty"]
    #[inline(always)]
    pub fn xfempty(&self) -> XfemptyR {
        XfemptyR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - External FIFO Full"]
    #[inline(always)]
    pub fn xffull(&self) -> XffullR {
        XffullR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Clock Enable Low"]
    #[inline(always)]
    pub fn celow(&self) -> CelowR {
        CelowR::new(((self.bits >> 9) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STAT")
            .field("active", &self.active())
            .field("nbrbusy", &self.nbrbusy())
            .field("wbusy", &self.wbusy())
            .field("initseq", &self.initseq())
            .field("xfempty", &self.xfempty())
            .field("xffull", &self.xffull())
            .field("celow", &self.celow())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Register Active"]
    #[inline(always)]
    pub fn active(&mut self) -> ActiveW<'_, StatSpec> {
        ActiveW::new(self, 0)
    }
    #[doc = "Bit 4 - Non-Blocking Read Busy"]
    #[inline(always)]
    pub fn nbrbusy(&mut self) -> NbrbusyW<'_, StatSpec> {
        NbrbusyW::new(self, 4)
    }
    #[doc = "Bit 5 - Write Busy"]
    #[inline(always)]
    pub fn wbusy(&mut self) -> WbusyW<'_, StatSpec> {
        WbusyW::new(self, 5)
    }
    #[doc = "Bit 6 - Initialization Sequence"]
    #[inline(always)]
    pub fn initseq(&mut self) -> InitseqW<'_, StatSpec> {
        InitseqW::new(self, 6)
    }
    #[doc = "Bit 7 - External FIFO Empty"]
    #[inline(always)]
    pub fn xfempty(&mut self) -> XfemptyW<'_, StatSpec> {
        XfemptyW::new(self, 7)
    }
    #[doc = "Bit 8 - External FIFO Full"]
    #[inline(always)]
    pub fn xffull(&mut self) -> XffullW<'_, StatSpec> {
        XffullW::new(self, 8)
    }
    #[doc = "Bit 9 - Clock Enable Low"]
    #[inline(always)]
    pub fn celow(&mut self) -> CelowW<'_, StatSpec> {
        CelowW::new(self, 9)
    }
}
#[doc = "EPI Status\n\nYou can [`read`](crate::Reg::read) this register and get [`stat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatSpec;
impl crate::RegisterSpec for StatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stat::R`](R) reader structure"]
impl crate::Readable for StatSpec {}
#[doc = "`write(|w| ..)` method takes [`stat::W`](W) writer structure"]
impl crate::Writable for StatSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets STAT to value 0"]
impl crate::Resettable for StatSpec {}
