#[doc = "Register `QEPCTL` reader"]
pub type R = crate::R<QepctlSpec>;
#[doc = "Register `QEPCTL` writer"]
pub type W = crate::W<QepctlSpec>;
#[doc = "QEP watchdog enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wde {
    #[doc = "0: Disable the eQEP watchdog timer"]
    Disable = 0,
    #[doc = "1: Enable the eQEP watchdog timer"]
    Enable = 1,
}
impl From<Wde> for bool {
    #[inline(always)]
    fn from(variant: Wde) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WDE` reader - QEP watchdog enable"]
pub type WdeR = crate::BitReader<Wde>;
impl WdeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wde {
        match self.bits {
            false => Wde::Disable,
            true => Wde::Enable,
        }
    }
    #[doc = "Disable the eQEP watchdog timer"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Wde::Disable
    }
    #[doc = "Enable the eQEP watchdog timer"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Wde::Enable
    }
}
#[doc = "Field `WDE` writer - QEP watchdog enable"]
pub type WdeW<'a, REG> = crate::BitWriter<'a, REG, Wde>;
impl<'a, REG> WdeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable the eQEP watchdog timer"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Wde::Disable)
    }
    #[doc = "Enable the eQEP watchdog timer"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Wde::Enable)
    }
}
#[doc = "QEP unit timer enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ute {
    #[doc = "0: Disable eQEP unit timer"]
    Disable = 0,
    #[doc = "1: Enable unit timer"]
    Enable = 1,
}
impl From<Ute> for bool {
    #[inline(always)]
    fn from(variant: Ute) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UTE` reader - QEP unit timer enable"]
pub type UteR = crate::BitReader<Ute>;
impl UteR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ute {
        match self.bits {
            false => Ute::Disable,
            true => Ute::Enable,
        }
    }
    #[doc = "Disable eQEP unit timer"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Ute::Disable
    }
    #[doc = "Enable unit timer"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Ute::Enable
    }
}
#[doc = "Field `UTE` writer - QEP unit timer enable"]
pub type UteW<'a, REG> = crate::BitWriter<'a, REG, Ute>;
impl<'a, REG> UteW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable eQEP unit timer"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Ute::Disable)
    }
    #[doc = "Enable unit timer"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Ute::Enable)
    }
}
#[doc = "QEP capture latch mode\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Qclm {
    #[doc = "0: Latch on position counter read by CPU. Capture timer and capture period values are latched into QCTMRLAT and QCPRDLAT registers when CPU reads the QPOSCNT register."]
    Cpu = 0,
    #[doc = "1: Latch on unit time out. Position counter, capture timer and capture period values are latched into QPOSLAT, QCTMRLAT and QCPRDLAT registers on unit time out."]
    Timeout = 1,
}
impl From<Qclm> for bool {
    #[inline(always)]
    fn from(variant: Qclm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `QCLM` reader - QEP capture latch mode"]
pub type QclmR = crate::BitReader<Qclm>;
impl QclmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Qclm {
        match self.bits {
            false => Qclm::Cpu,
            true => Qclm::Timeout,
        }
    }
    #[doc = "Latch on position counter read by CPU. Capture timer and capture period values are latched into QCTMRLAT and QCPRDLAT registers when CPU reads the QPOSCNT register."]
    #[inline(always)]
    pub fn is_cpu(&self) -> bool {
        *self == Qclm::Cpu
    }
    #[doc = "Latch on unit time out. Position counter, capture timer and capture period values are latched into QPOSLAT, QCTMRLAT and QCPRDLAT registers on unit time out."]
    #[inline(always)]
    pub fn is_timeout(&self) -> bool {
        *self == Qclm::Timeout
    }
}
#[doc = "Field `QCLM` writer - QEP capture latch mode"]
pub type QclmW<'a, REG> = crate::BitWriter<'a, REG, Qclm>;
impl<'a, REG> QclmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Latch on position counter read by CPU. Capture timer and capture period values are latched into QCTMRLAT and QCPRDLAT registers when CPU reads the QPOSCNT register."]
    #[inline(always)]
    pub fn cpu(self) -> &'a mut crate::W<REG> {
        self.variant(Qclm::Cpu)
    }
    #[doc = "Latch on unit time out. Position counter, capture timer and capture period values are latched into QPOSLAT, QCTMRLAT and QCPRDLAT registers on unit time out."]
    #[inline(always)]
    pub fn timeout(self) -> &'a mut crate::W<REG> {
        self.variant(Qclm::Timeout)
    }
}
#[doc = "Quadrature postotion counter enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Qpen {
    #[doc = "0: Reset the eQEP peripheral internal operating flags/read-only registers. Control/configuration registers are not disturbed by a software reset. #br#When QPEN is disabled, some flags in the QFLG register do not get reset or cleared and show the actual state of that flag."]
    Reset = 0,
    #[doc = "1: eQEP position counter is enabled"]
    Enable = 1,
}
impl From<Qpen> for bool {
    #[inline(always)]
    fn from(variant: Qpen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `QPEN` reader - Quadrature postotion counter enable"]
pub type QpenR = crate::BitReader<Qpen>;
impl QpenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Qpen {
        match self.bits {
            false => Qpen::Reset,
            true => Qpen::Enable,
        }
    }
    #[doc = "Reset the eQEP peripheral internal operating flags/read-only registers. Control/configuration registers are not disturbed by a software reset. #br#When QPEN is disabled, some flags in the QFLG register do not get reset or cleared and show the actual state of that flag."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == Qpen::Reset
    }
    #[doc = "eQEP position counter is enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Qpen::Enable
    }
}
#[doc = "Field `QPEN` writer - Quadrature postotion counter enable"]
pub type QpenW<'a, REG> = crate::BitWriter<'a, REG, Qpen>;
impl<'a, REG> QpenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset the eQEP peripheral internal operating flags/read-only registers. Control/configuration registers are not disturbed by a software reset. #br#When QPEN is disabled, some flags in the QFLG register do not get reset or cleared and show the actual state of that flag."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(Qpen::Reset)
    }
    #[doc = "eQEP position counter is enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Qpen::Enable)
    }
}
#[doc = "Index event latch\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Iel {
    #[doc = "0: Reserved"]
    Rsvd = 0,
    #[doc = "1: Latches position counter on rising edge of the index signal"]
    Posrising = 1,
    #[doc = "2: Latches position counter on falling edge of the index signal"]
    Posfalling = 2,
    #[doc = "3: Software index marker. Latches the position counter and quadrature direction flag on index event marker. The position counter is latched to the QPOSILAT register and the direction flag is latched in the QEPSTS\\[QDLF\\] bit. This mode is useful for software index marking."]
    Sim = 3,
}
impl From<Iel> for u8 {
    #[inline(always)]
    fn from(variant: Iel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Iel {
    type Ux = u8;
}
impl crate::IsEnum for Iel {}
#[doc = "Field `IEL` reader - Index event latch"]
pub type IelR = crate::FieldReader<Iel>;
impl IelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Iel {
        match self.bits {
            0 => Iel::Rsvd,
            1 => Iel::Posrising,
            2 => Iel::Posfalling,
            3 => Iel::Sim,
            _ => unreachable!(),
        }
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn is_rsvd(&self) -> bool {
        *self == Iel::Rsvd
    }
    #[doc = "Latches position counter on rising edge of the index signal"]
    #[inline(always)]
    pub fn is_posrising(&self) -> bool {
        *self == Iel::Posrising
    }
    #[doc = "Latches position counter on falling edge of the index signal"]
    #[inline(always)]
    pub fn is_posfalling(&self) -> bool {
        *self == Iel::Posfalling
    }
    #[doc = "Software index marker. Latches the position counter and quadrature direction flag on index event marker. The position counter is latched to the QPOSILAT register and the direction flag is latched in the QEPSTS\\[QDLF\\] bit. This mode is useful for software index marking."]
    #[inline(always)]
    pub fn is_sim(&self) -> bool {
        *self == Iel::Sim
    }
}
#[doc = "Field `IEL` writer - Index event latch"]
pub type IelW<'a, REG> = crate::FieldWriter<'a, REG, 2, Iel, crate::Safe>;
impl<'a, REG> IelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn rsvd(self) -> &'a mut crate::W<REG> {
        self.variant(Iel::Rsvd)
    }
    #[doc = "Latches position counter on rising edge of the index signal"]
    #[inline(always)]
    pub fn posrising(self) -> &'a mut crate::W<REG> {
        self.variant(Iel::Posrising)
    }
    #[doc = "Latches position counter on falling edge of the index signal"]
    #[inline(always)]
    pub fn posfalling(self) -> &'a mut crate::W<REG> {
        self.variant(Iel::Posfalling)
    }
    #[doc = "Software index marker. Latches the position counter and quadrature direction flag on index event marker. The position counter is latched to the QPOSILAT register and the direction flag is latched in the QEPSTS\\[QDLF\\] bit. This mode is useful for software index marking."]
    #[inline(always)]
    pub fn sim(self) -> &'a mut crate::W<REG> {
        self.variant(Iel::Sim)
    }
}
#[doc = "Strobe event latch\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sel {
    #[doc = "0: The position counter is latched on the rising edge of QEPS strobe (QPOSSLAT = POSCCNT). Latching on the falling edge can be done by inverting the strobe input using the QSP bit in the QDECCTL register"]
    Qepsrising = 0,
    #[doc = "1: Clockwise Direction: #br#Position counter is latched on rising edge of QEPS strobe #br#Counter Clockwise Direction: #br#Position counter is latched on falling edge of QEPS strobe"]
    Qepsclock = 1,
}
impl From<Sel> for bool {
    #[inline(always)]
    fn from(variant: Sel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SEL` reader - Strobe event latch"]
pub type SelR = crate::BitReader<Sel>;
impl SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sel {
        match self.bits {
            false => Sel::Qepsrising,
            true => Sel::Qepsclock,
        }
    }
    #[doc = "The position counter is latched on the rising edge of QEPS strobe (QPOSSLAT = POSCCNT). Latching on the falling edge can be done by inverting the strobe input using the QSP bit in the QDECCTL register"]
    #[inline(always)]
    pub fn is_qepsrising(&self) -> bool {
        *self == Sel::Qepsrising
    }
    #[doc = "Clockwise Direction: #br#Position counter is latched on rising edge of QEPS strobe #br#Counter Clockwise Direction: #br#Position counter is latched on falling edge of QEPS strobe"]
    #[inline(always)]
    pub fn is_qepsclock(&self) -> bool {
        *self == Sel::Qepsclock
    }
}
#[doc = "Field `SEL` writer - Strobe event latch"]
pub type SelW<'a, REG> = crate::BitWriter<'a, REG, Sel>;
impl<'a, REG> SelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The position counter is latched on the rising edge of QEPS strobe (QPOSSLAT = POSCCNT). Latching on the falling edge can be done by inverting the strobe input using the QSP bit in the QDECCTL register"]
    #[inline(always)]
    pub fn qepsrising(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::Qepsrising)
    }
    #[doc = "Clockwise Direction: #br#Position counter is latched on rising edge of QEPS strobe #br#Counter Clockwise Direction: #br#Position counter is latched on falling edge of QEPS strobe"]
    #[inline(always)]
    pub fn qepsclock(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::Qepsclock)
    }
}
#[doc = "Software init position counter\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Swi {
    #[doc = "0: Do nothing (action disabled)"]
    Nothing = 0,
    #[doc = "1: Initialize position counter (QPOSCNT=QPOSINIT). This bit is not cleared automatically"]
    Initpos = 1,
}
impl From<Swi> for bool {
    #[inline(always)]
    fn from(variant: Swi) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWI` reader - Software init position counter"]
pub type SwiR = crate::BitReader<Swi>;
impl SwiR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Swi {
        match self.bits {
            false => Swi::Nothing,
            true => Swi::Initpos,
        }
    }
    #[doc = "Do nothing (action disabled)"]
    #[inline(always)]
    pub fn is_nothing(&self) -> bool {
        *self == Swi::Nothing
    }
    #[doc = "Initialize position counter (QPOSCNT=QPOSINIT). This bit is not cleared automatically"]
    #[inline(always)]
    pub fn is_initpos(&self) -> bool {
        *self == Swi::Initpos
    }
}
#[doc = "Field `SWI` writer - Software init position counter"]
pub type SwiW<'a, REG> = crate::BitWriter<'a, REG, Swi>;
impl<'a, REG> SwiW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Do nothing (action disabled)"]
    #[inline(always)]
    pub fn nothing(self) -> &'a mut crate::W<REG> {
        self.variant(Swi::Nothing)
    }
    #[doc = "Initialize position counter (QPOSCNT=QPOSINIT). This bit is not cleared automatically"]
    #[inline(always)]
    pub fn initpos(self) -> &'a mut crate::W<REG> {
        self.variant(Swi::Initpos)
    }
}
#[doc = "Index event init of position count\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Iei {
    #[doc = "0: Do nothing (action disabled)"]
    Nothing0 = 0,
    #[doc = "1: Do nothing (action disabled)"]
    Nothing1 = 1,
    #[doc = "2: Initializes the position counter on the rising edge of the QEPI signal (QPOSCNT = QPOSINIT)"]
    Initrising = 2,
    #[doc = "3: Initializes the position counter on the falling edge of QEPI signal (QPOSCNT = QPOSINIT)"]
    Initfalling = 3,
}
impl From<Iei> for u8 {
    #[inline(always)]
    fn from(variant: Iei) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Iei {
    type Ux = u8;
}
impl crate::IsEnum for Iei {}
#[doc = "Field `IEI` reader - Index event init of position count"]
pub type IeiR = crate::FieldReader<Iei>;
impl IeiR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Iei {
        match self.bits {
            0 => Iei::Nothing0,
            1 => Iei::Nothing1,
            2 => Iei::Initrising,
            3 => Iei::Initfalling,
            _ => unreachable!(),
        }
    }
    #[doc = "Do nothing (action disabled)"]
    #[inline(always)]
    pub fn is_nothing0(&self) -> bool {
        *self == Iei::Nothing0
    }
    #[doc = "Do nothing (action disabled)"]
    #[inline(always)]
    pub fn is_nothing1(&self) -> bool {
        *self == Iei::Nothing1
    }
    #[doc = "Initializes the position counter on the rising edge of the QEPI signal (QPOSCNT = QPOSINIT)"]
    #[inline(always)]
    pub fn is_initrising(&self) -> bool {
        *self == Iei::Initrising
    }
    #[doc = "Initializes the position counter on the falling edge of QEPI signal (QPOSCNT = QPOSINIT)"]
    #[inline(always)]
    pub fn is_initfalling(&self) -> bool {
        *self == Iei::Initfalling
    }
}
#[doc = "Field `IEI` writer - Index event init of position count"]
pub type IeiW<'a, REG> = crate::FieldWriter<'a, REG, 2, Iei, crate::Safe>;
impl<'a, REG> IeiW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Do nothing (action disabled)"]
    #[inline(always)]
    pub fn nothing0(self) -> &'a mut crate::W<REG> {
        self.variant(Iei::Nothing0)
    }
    #[doc = "Do nothing (action disabled)"]
    #[inline(always)]
    pub fn nothing1(self) -> &'a mut crate::W<REG> {
        self.variant(Iei::Nothing1)
    }
    #[doc = "Initializes the position counter on the rising edge of the QEPI signal (QPOSCNT = QPOSINIT)"]
    #[inline(always)]
    pub fn initrising(self) -> &'a mut crate::W<REG> {
        self.variant(Iei::Initrising)
    }
    #[doc = "Initializes the position counter on the falling edge of QEPI signal (QPOSCNT = QPOSINIT)"]
    #[inline(always)]
    pub fn initfalling(self) -> &'a mut crate::W<REG> {
        self.variant(Iei::Initfalling)
    }
}
#[doc = "Strobe event init\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sei {
    #[doc = "0: Does nothing (action disabled)"]
    Nothing0 = 0,
    #[doc = "1: Does nothing (action disabled)"]
    Nothing1 = 1,
    #[doc = "2: Initializes the position counter on rising edge of the QEPS signal"]
    Initqepsrising = 2,
    #[doc = "3: Clockwise Direction: #br#Initializes the position counter on the rising edge of QEPS strobe #br#Counter Clockwise Direction: #br#Initializes the position counter on the falling edge of QEPS strobe"]
    Initqepsclock = 3,
}
impl From<Sei> for u8 {
    #[inline(always)]
    fn from(variant: Sei) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sei {
    type Ux = u8;
}
impl crate::IsEnum for Sei {}
#[doc = "Field `SEI` reader - Strobe event init"]
pub type SeiR = crate::FieldReader<Sei>;
impl SeiR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sei {
        match self.bits {
            0 => Sei::Nothing0,
            1 => Sei::Nothing1,
            2 => Sei::Initqepsrising,
            3 => Sei::Initqepsclock,
            _ => unreachable!(),
        }
    }
    #[doc = "Does nothing (action disabled)"]
    #[inline(always)]
    pub fn is_nothing0(&self) -> bool {
        *self == Sei::Nothing0
    }
    #[doc = "Does nothing (action disabled)"]
    #[inline(always)]
    pub fn is_nothing1(&self) -> bool {
        *self == Sei::Nothing1
    }
    #[doc = "Initializes the position counter on rising edge of the QEPS signal"]
    #[inline(always)]
    pub fn is_initqepsrising(&self) -> bool {
        *self == Sei::Initqepsrising
    }
    #[doc = "Clockwise Direction: #br#Initializes the position counter on the rising edge of QEPS strobe #br#Counter Clockwise Direction: #br#Initializes the position counter on the falling edge of QEPS strobe"]
    #[inline(always)]
    pub fn is_initqepsclock(&self) -> bool {
        *self == Sei::Initqepsclock
    }
}
#[doc = "Field `SEI` writer - Strobe event init"]
pub type SeiW<'a, REG> = crate::FieldWriter<'a, REG, 2, Sei, crate::Safe>;
impl<'a, REG> SeiW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Does nothing (action disabled)"]
    #[inline(always)]
    pub fn nothing0(self) -> &'a mut crate::W<REG> {
        self.variant(Sei::Nothing0)
    }
    #[doc = "Does nothing (action disabled)"]
    #[inline(always)]
    pub fn nothing1(self) -> &'a mut crate::W<REG> {
        self.variant(Sei::Nothing1)
    }
    #[doc = "Initializes the position counter on rising edge of the QEPS signal"]
    #[inline(always)]
    pub fn initqepsrising(self) -> &'a mut crate::W<REG> {
        self.variant(Sei::Initqepsrising)
    }
    #[doc = "Clockwise Direction: #br#Initializes the position counter on the rising edge of QEPS strobe #br#Counter Clockwise Direction: #br#Initializes the position counter on the falling edge of QEPS strobe"]
    #[inline(always)]
    pub fn initqepsclock(self) -> &'a mut crate::W<REG> {
        self.variant(Sei::Initqepsclock)
    }
}
#[doc = "Postion counter reset\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pcrm {
    #[doc = "0: Position counter reset on an index event"]
    Index = 0,
    #[doc = "1: Position counter reset on the maximum position"]
    Maxpos = 1,
    #[doc = "2: Position counter reset on the first index event"]
    Firstindex = 2,
    #[doc = "3: Position counter reset on a unit time event"]
    Timeevent = 3,
}
impl From<Pcrm> for u8 {
    #[inline(always)]
    fn from(variant: Pcrm) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pcrm {
    type Ux = u8;
}
impl crate::IsEnum for Pcrm {}
#[doc = "Field `PCRM` reader - Postion counter reset"]
pub type PcrmR = crate::FieldReader<Pcrm>;
impl PcrmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pcrm {
        match self.bits {
            0 => Pcrm::Index,
            1 => Pcrm::Maxpos,
            2 => Pcrm::Firstindex,
            3 => Pcrm::Timeevent,
            _ => unreachable!(),
        }
    }
    #[doc = "Position counter reset on an index event"]
    #[inline(always)]
    pub fn is_index(&self) -> bool {
        *self == Pcrm::Index
    }
    #[doc = "Position counter reset on the maximum position"]
    #[inline(always)]
    pub fn is_maxpos(&self) -> bool {
        *self == Pcrm::Maxpos
    }
    #[doc = "Position counter reset on the first index event"]
    #[inline(always)]
    pub fn is_firstindex(&self) -> bool {
        *self == Pcrm::Firstindex
    }
    #[doc = "Position counter reset on a unit time event"]
    #[inline(always)]
    pub fn is_timeevent(&self) -> bool {
        *self == Pcrm::Timeevent
    }
}
#[doc = "Field `PCRM` writer - Postion counter reset"]
pub type PcrmW<'a, REG> = crate::FieldWriter<'a, REG, 2, Pcrm, crate::Safe>;
impl<'a, REG> PcrmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Position counter reset on an index event"]
    #[inline(always)]
    pub fn index(self) -> &'a mut crate::W<REG> {
        self.variant(Pcrm::Index)
    }
    #[doc = "Position counter reset on the maximum position"]
    #[inline(always)]
    pub fn maxpos(self) -> &'a mut crate::W<REG> {
        self.variant(Pcrm::Maxpos)
    }
    #[doc = "Position counter reset on the first index event"]
    #[inline(always)]
    pub fn firstindex(self) -> &'a mut crate::W<REG> {
        self.variant(Pcrm::Firstindex)
    }
    #[doc = "Position counter reset on a unit time event"]
    #[inline(always)]
    pub fn timeevent(self) -> &'a mut crate::W<REG> {
        self.variant(Pcrm::Timeevent)
    }
}
#[doc = "Emulation mode\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FreeSoft {
    #[doc = "0: QPOSCNT behavior #br#Position counter stops immediately on emulation suspend #br#0h (R/W) = QWDTMR behavior #br#Watchdog counter stops immediately #br#0h (R/W) = QUTMR behavior #br#Unit timer stops immediately #br#0h (R/W) = QCTMR behavior #br#Capture Timer stops immediately"]
    FreeSoft0 = 0,
    #[doc = "1: QPOSCNT behavior #br#Position counter continues to count until the rollover #br#1h (R/W) = QWDTMR behavior #br#Watchdog counter counts until WD period match roll over #br#1h (R/W) = QUTMR behavior #br#Unit timer counts until period rollover #br#1h (R/W) = QCTMR behavior #br#Capture Timer counts until next unit period event"]
    FreeSoft1 = 1,
    #[doc = "2: QPOSCNT behavior #br#Position counter is unaffected by emulation suspend #br#2h (R/W) = QWDTMR behavior #br#Watchdog counter is unaffected by emulation suspend #br#2h (R/W) = QUTMR behavior #br#Unit timer is unaffected by emulation suspend #br#2h (R/W) = QCTMR behavior #br#Capture Timer is unaffected by emulation suspend"]
    FreeSoft2 = 2,
    #[doc = "3: Same as FREE_SOFT_2"]
    FreeSoft3 = 3,
}
impl From<FreeSoft> for u8 {
    #[inline(always)]
    fn from(variant: FreeSoft) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FreeSoft {
    type Ux = u8;
}
impl crate::IsEnum for FreeSoft {}
#[doc = "Field `FREE_SOFT` reader - Emulation mode"]
pub type FreeSoftR = crate::FieldReader<FreeSoft>;
impl FreeSoftR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FreeSoft {
        match self.bits {
            0 => FreeSoft::FreeSoft0,
            1 => FreeSoft::FreeSoft1,
            2 => FreeSoft::FreeSoft2,
            3 => FreeSoft::FreeSoft3,
            _ => unreachable!(),
        }
    }
    #[doc = "QPOSCNT behavior #br#Position counter stops immediately on emulation suspend #br#0h (R/W) = QWDTMR behavior #br#Watchdog counter stops immediately #br#0h (R/W) = QUTMR behavior #br#Unit timer stops immediately #br#0h (R/W) = QCTMR behavior #br#Capture Timer stops immediately"]
    #[inline(always)]
    pub fn is_free_soft_0(&self) -> bool {
        *self == FreeSoft::FreeSoft0
    }
    #[doc = "QPOSCNT behavior #br#Position counter continues to count until the rollover #br#1h (R/W) = QWDTMR behavior #br#Watchdog counter counts until WD period match roll over #br#1h (R/W) = QUTMR behavior #br#Unit timer counts until period rollover #br#1h (R/W) = QCTMR behavior #br#Capture Timer counts until next unit period event"]
    #[inline(always)]
    pub fn is_free_soft_1(&self) -> bool {
        *self == FreeSoft::FreeSoft1
    }
    #[doc = "QPOSCNT behavior #br#Position counter is unaffected by emulation suspend #br#2h (R/W) = QWDTMR behavior #br#Watchdog counter is unaffected by emulation suspend #br#2h (R/W) = QUTMR behavior #br#Unit timer is unaffected by emulation suspend #br#2h (R/W) = QCTMR behavior #br#Capture Timer is unaffected by emulation suspend"]
    #[inline(always)]
    pub fn is_free_soft_2(&self) -> bool {
        *self == FreeSoft::FreeSoft2
    }
    #[doc = "Same as FREE_SOFT_2"]
    #[inline(always)]
    pub fn is_free_soft_3(&self) -> bool {
        *self == FreeSoft::FreeSoft3
    }
}
#[doc = "Field `FREE_SOFT` writer - Emulation mode"]
pub type FreeSoftW<'a, REG> = crate::FieldWriter<'a, REG, 2, FreeSoft, crate::Safe>;
impl<'a, REG> FreeSoftW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "QPOSCNT behavior #br#Position counter stops immediately on emulation suspend #br#0h (R/W) = QWDTMR behavior #br#Watchdog counter stops immediately #br#0h (R/W) = QUTMR behavior #br#Unit timer stops immediately #br#0h (R/W) = QCTMR behavior #br#Capture Timer stops immediately"]
    #[inline(always)]
    pub fn free_soft_0(self) -> &'a mut crate::W<REG> {
        self.variant(FreeSoft::FreeSoft0)
    }
    #[doc = "QPOSCNT behavior #br#Position counter continues to count until the rollover #br#1h (R/W) = QWDTMR behavior #br#Watchdog counter counts until WD period match roll over #br#1h (R/W) = QUTMR behavior #br#Unit timer counts until period rollover #br#1h (R/W) = QCTMR behavior #br#Capture Timer counts until next unit period event"]
    #[inline(always)]
    pub fn free_soft_1(self) -> &'a mut crate::W<REG> {
        self.variant(FreeSoft::FreeSoft1)
    }
    #[doc = "QPOSCNT behavior #br#Position counter is unaffected by emulation suspend #br#2h (R/W) = QWDTMR behavior #br#Watchdog counter is unaffected by emulation suspend #br#2h (R/W) = QUTMR behavior #br#Unit timer is unaffected by emulation suspend #br#2h (R/W) = QCTMR behavior #br#Capture Timer is unaffected by emulation suspend"]
    #[inline(always)]
    pub fn free_soft_2(self) -> &'a mut crate::W<REG> {
        self.variant(FreeSoft::FreeSoft2)
    }
    #[doc = "Same as FREE_SOFT_2"]
    #[inline(always)]
    pub fn free_soft_3(self) -> &'a mut crate::W<REG> {
        self.variant(FreeSoft::FreeSoft3)
    }
}
impl R {
    #[doc = "Bit 0 - QEP watchdog enable"]
    #[inline(always)]
    pub fn wde(&self) -> WdeR {
        WdeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - QEP unit timer enable"]
    #[inline(always)]
    pub fn ute(&self) -> UteR {
        UteR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - QEP capture latch mode"]
    #[inline(always)]
    pub fn qclm(&self) -> QclmR {
        QclmR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Quadrature postotion counter enable"]
    #[inline(always)]
    pub fn qpen(&self) -> QpenR {
        QpenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Index event latch"]
    #[inline(always)]
    pub fn iel(&self) -> IelR {
        IelR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - Strobe event latch"]
    #[inline(always)]
    pub fn sel(&self) -> SelR {
        SelR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Software init position counter"]
    #[inline(always)]
    pub fn swi(&self) -> SwiR {
        SwiR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Index event init of position count"]
    #[inline(always)]
    pub fn iei(&self) -> IeiR {
        IeiR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Strobe event init"]
    #[inline(always)]
    pub fn sei(&self) -> SeiR {
        SeiR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Postion counter reset"]
    #[inline(always)]
    pub fn pcrm(&self) -> PcrmR {
        PcrmR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Emulation mode"]
    #[inline(always)]
    pub fn free_soft(&self) -> FreeSoftR {
        FreeSoftR::new(((self.bits >> 14) & 3) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("QEPCTL")
            .field("wde", &self.wde())
            .field("ute", &self.ute())
            .field("qclm", &self.qclm())
            .field("qpen", &self.qpen())
            .field("iel", &self.iel())
            .field("sel", &self.sel())
            .field("swi", &self.swi())
            .field("iei", &self.iei())
            .field("sei", &self.sei())
            .field("pcrm", &self.pcrm())
            .field("free_soft", &self.free_soft())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - QEP watchdog enable"]
    #[inline(always)]
    pub fn wde(&mut self) -> WdeW<'_, QepctlSpec> {
        WdeW::new(self, 0)
    }
    #[doc = "Bit 1 - QEP unit timer enable"]
    #[inline(always)]
    pub fn ute(&mut self) -> UteW<'_, QepctlSpec> {
        UteW::new(self, 1)
    }
    #[doc = "Bit 2 - QEP capture latch mode"]
    #[inline(always)]
    pub fn qclm(&mut self) -> QclmW<'_, QepctlSpec> {
        QclmW::new(self, 2)
    }
    #[doc = "Bit 3 - Quadrature postotion counter enable"]
    #[inline(always)]
    pub fn qpen(&mut self) -> QpenW<'_, QepctlSpec> {
        QpenW::new(self, 3)
    }
    #[doc = "Bits 4:5 - Index event latch"]
    #[inline(always)]
    pub fn iel(&mut self) -> IelW<'_, QepctlSpec> {
        IelW::new(self, 4)
    }
    #[doc = "Bit 6 - Strobe event latch"]
    #[inline(always)]
    pub fn sel(&mut self) -> SelW<'_, QepctlSpec> {
        SelW::new(self, 6)
    }
    #[doc = "Bit 7 - Software init position counter"]
    #[inline(always)]
    pub fn swi(&mut self) -> SwiW<'_, QepctlSpec> {
        SwiW::new(self, 7)
    }
    #[doc = "Bits 8:9 - Index event init of position count"]
    #[inline(always)]
    pub fn iei(&mut self) -> IeiW<'_, QepctlSpec> {
        IeiW::new(self, 8)
    }
    #[doc = "Bits 10:11 - Strobe event init"]
    #[inline(always)]
    pub fn sei(&mut self) -> SeiW<'_, QepctlSpec> {
        SeiW::new(self, 10)
    }
    #[doc = "Bits 12:13 - Postion counter reset"]
    #[inline(always)]
    pub fn pcrm(&mut self) -> PcrmW<'_, QepctlSpec> {
        PcrmW::new(self, 12)
    }
    #[doc = "Bits 14:15 - Emulation mode"]
    #[inline(always)]
    pub fn free_soft(&mut self) -> FreeSoftW<'_, QepctlSpec> {
        FreeSoftW::new(self, 14)
    }
}
#[doc = "QEP Control\n\nYou can [`read`](crate::Reg::read) this register and get [`qepctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qepctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QepctlSpec;
impl crate::RegisterSpec for QepctlSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`qepctl::R`](R) reader structure"]
impl crate::Readable for QepctlSpec {}
#[doc = "`write(|w| ..)` method takes [`qepctl::W`](W) writer structure"]
impl crate::Writable for QepctlSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets QEPCTL to value 0"]
impl crate::Resettable for QepctlSpec {}
