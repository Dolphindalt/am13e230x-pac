#[doc = "Register `HB16CFG` reader"]
pub type R = crate::R<Hb16cfgSpec>;
#[doc = "Register `HB16CFG` writer"]
pub type W = crate::W<Hb16cfgSpec>;
#[doc = "Host Bus Sub-Mode\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mode {
    #[doc = "0: ADMUX - AD\\[15:0\\]Data and Address are muxed."]
    Admux = 0,
    #[doc = "1: ADNONMUX - D\\[15:0\\]Data and address are separate. This mode is not practical in HB16 mode for normal peripherals because there are generally not enough address bits available."]
    Adnonmux = 1,
    #[doc = "2: Continuous Read - D\\[15:0\\]This mode is the same as ADNONMUX, but uses address switch for multiple reads instead of OEn strobing. This mode is not practical in HB16 mode for normal SRAMs because there are generally not enough address bits available."]
    ContRead = 2,
    #[doc = "3: XFIFO - D\\[15:0\\]This mode adds XFIFO controls with sense of XFIFO full and XFIFO empty. This mode uses no address or ALE.Note that the XFIFO can only be used in asynchronous mode."]
    Xfifo = 3,
}
impl From<Mode> for u8 {
    #[inline(always)]
    fn from(variant: Mode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mode {
    type Ux = u8;
}
impl crate::IsEnum for Mode {}
#[doc = "Field `MODE` reader - Host Bus Sub-Mode"]
pub type ModeR = crate::FieldReader<Mode>;
impl ModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mode {
        match self.bits {
            0 => Mode::Admux,
            1 => Mode::Adnonmux,
            2 => Mode::ContRead,
            3 => Mode::Xfifo,
            _ => unreachable!(),
        }
    }
    #[doc = "ADMUX - AD\\[15:0\\]Data and Address are muxed."]
    #[inline(always)]
    pub fn is_admux(&self) -> bool {
        *self == Mode::Admux
    }
    #[doc = "ADNONMUX - D\\[15:0\\]Data and address are separate. This mode is not practical in HB16 mode for normal peripherals because there are generally not enough address bits available."]
    #[inline(always)]
    pub fn is_adnonmux(&self) -> bool {
        *self == Mode::Adnonmux
    }
    #[doc = "Continuous Read - D\\[15:0\\]This mode is the same as ADNONMUX, but uses address switch for multiple reads instead of OEn strobing. This mode is not practical in HB16 mode for normal SRAMs because there are generally not enough address bits available."]
    #[inline(always)]
    pub fn is_cont_read(&self) -> bool {
        *self == Mode::ContRead
    }
    #[doc = "XFIFO - D\\[15:0\\]This mode adds XFIFO controls with sense of XFIFO full and XFIFO empty. This mode uses no address or ALE.Note that the XFIFO can only be used in asynchronous mode."]
    #[inline(always)]
    pub fn is_xfifo(&self) -> bool {
        *self == Mode::Xfifo
    }
}
#[doc = "Field `MODE` writer - Host Bus Sub-Mode"]
pub type ModeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Mode, crate::Safe>;
impl<'a, REG> ModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "ADMUX - AD\\[15:0\\]Data and Address are muxed."]
    #[inline(always)]
    pub fn admux(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Admux)
    }
    #[doc = "ADNONMUX - D\\[15:0\\]Data and address are separate. This mode is not practical in HB16 mode for normal peripherals because there are generally not enough address bits available."]
    #[inline(always)]
    pub fn adnonmux(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Adnonmux)
    }
    #[doc = "Continuous Read - D\\[15:0\\]This mode is the same as ADNONMUX, but uses address switch for multiple reads instead of OEn strobing. This mode is not practical in HB16 mode for normal SRAMs because there are generally not enough address bits available."]
    #[inline(always)]
    pub fn cont_read(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::ContRead)
    }
    #[doc = "XFIFO - D\\[15:0\\]This mode adds XFIFO controls with sense of XFIFO full and XFIFO empty. This mode uses no address or ALE.Note that the XFIFO can only be used in asynchronous mode."]
    #[inline(always)]
    pub fn xfifo(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Xfifo)
    }
}
#[doc = "Byte Select Configuration\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bsel {
    #[doc = "0: No Byte SelectsData is read and written as 16 bits."]
    NoByteSelects = 0,
    #[doc = "1: Enable Byte SelectsTwo EPI signals function as byte select signals to allow 8-bit transfers. See for details on which EPI signals are used."]
    EnableByleSelect = 1,
}
impl From<Bsel> for bool {
    #[inline(always)]
    fn from(variant: Bsel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BSEL` reader - Byte Select Configuration"]
pub type BselR = crate::BitReader<Bsel>;
impl BselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bsel {
        match self.bits {
            false => Bsel::NoByteSelects,
            true => Bsel::EnableByleSelect,
        }
    }
    #[doc = "No Byte SelectsData is read and written as 16 bits."]
    #[inline(always)]
    pub fn is_no_byte_selects(&self) -> bool {
        *self == Bsel::NoByteSelects
    }
    #[doc = "Enable Byte SelectsTwo EPI signals function as byte select signals to allow 8-bit transfers. See for details on which EPI signals are used."]
    #[inline(always)]
    pub fn is_enable_byle_select(&self) -> bool {
        *self == Bsel::EnableByleSelect
    }
}
#[doc = "Field `BSEL` writer - Byte Select Configuration"]
pub type BselW<'a, REG> = crate::BitWriter<'a, REG, Bsel>;
impl<'a, REG> BselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Byte SelectsData is read and written as 16 bits."]
    #[inline(always)]
    pub fn no_byte_selects(self) -> &'a mut crate::W<REG> {
        self.variant(Bsel::NoByteSelects)
    }
    #[doc = "Enable Byte SelectsTwo EPI signals function as byte select signals to allow 8-bit transfers. See for details on which EPI signals are used."]
    #[inline(always)]
    pub fn enable_byle_select(self) -> &'a mut crate::W<REG> {
        self.variant(Bsel::EnableByleSelect)
    }
}
#[doc = "Read Wait States\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rdws {
    #[doc = "0: Active RDn is 2 EPI clocks."]
    Rdn2 = 0,
    #[doc = "1: Active RDn is 4 EPI clocks."]
    Rdn4 = 1,
    #[doc = "2: Active RDn is 6 EPI clocks."]
    Rdn6 = 2,
    #[doc = "3: Active RDn is 8 EPI clocks."]
    Rdn8 = 3,
}
impl From<Rdws> for u8 {
    #[inline(always)]
    fn from(variant: Rdws) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rdws {
    type Ux = u8;
}
impl crate::IsEnum for Rdws {}
#[doc = "Field `RDWS` reader - Read Wait States"]
pub type RdwsR = crate::FieldReader<Rdws>;
impl RdwsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rdws {
        match self.bits {
            0 => Rdws::Rdn2,
            1 => Rdws::Rdn4,
            2 => Rdws::Rdn6,
            3 => Rdws::Rdn8,
            _ => unreachable!(),
        }
    }
    #[doc = "Active RDn is 2 EPI clocks."]
    #[inline(always)]
    pub fn is_rdn2(&self) -> bool {
        *self == Rdws::Rdn2
    }
    #[doc = "Active RDn is 4 EPI clocks."]
    #[inline(always)]
    pub fn is_rdn4(&self) -> bool {
        *self == Rdws::Rdn4
    }
    #[doc = "Active RDn is 6 EPI clocks."]
    #[inline(always)]
    pub fn is_rdn6(&self) -> bool {
        *self == Rdws::Rdn6
    }
    #[doc = "Active RDn is 8 EPI clocks."]
    #[inline(always)]
    pub fn is_rdn8(&self) -> bool {
        *self == Rdws::Rdn8
    }
}
#[doc = "Field `RDWS` writer - Read Wait States"]
pub type RdwsW<'a, REG> = crate::FieldWriter<'a, REG, 2, Rdws, crate::Safe>;
impl<'a, REG> RdwsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Active RDn is 2 EPI clocks."]
    #[inline(always)]
    pub fn rdn2(self) -> &'a mut crate::W<REG> {
        self.variant(Rdws::Rdn2)
    }
    #[doc = "Active RDn is 4 EPI clocks."]
    #[inline(always)]
    pub fn rdn4(self) -> &'a mut crate::W<REG> {
        self.variant(Rdws::Rdn4)
    }
    #[doc = "Active RDn is 6 EPI clocks."]
    #[inline(always)]
    pub fn rdn6(self) -> &'a mut crate::W<REG> {
        self.variant(Rdws::Rdn6)
    }
    #[doc = "Active RDn is 8 EPI clocks."]
    #[inline(always)]
    pub fn rdn8(self) -> &'a mut crate::W<REG> {
        self.variant(Rdws::Rdn8)
    }
}
#[doc = "Write Wait States\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Wrws {
    #[doc = "0: Active WRn is 2 EPI clocks."]
    Wrn2 = 0,
    #[doc = "1: Active WRn is 4 EPI clocks."]
    Wrn4 = 1,
    #[doc = "2: Active WRn is 6 EPI clocks."]
    Wrn6 = 2,
    #[doc = "3: Active WRn is 8 EPI clocks."]
    Wrn8 = 3,
}
impl From<Wrws> for u8 {
    #[inline(always)]
    fn from(variant: Wrws) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Wrws {
    type Ux = u8;
}
impl crate::IsEnum for Wrws {}
#[doc = "Field `WRWS` reader - Write Wait States"]
pub type WrwsR = crate::FieldReader<Wrws>;
impl WrwsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wrws {
        match self.bits {
            0 => Wrws::Wrn2,
            1 => Wrws::Wrn4,
            2 => Wrws::Wrn6,
            3 => Wrws::Wrn8,
            _ => unreachable!(),
        }
    }
    #[doc = "Active WRn is 2 EPI clocks."]
    #[inline(always)]
    pub fn is_wrn2(&self) -> bool {
        *self == Wrws::Wrn2
    }
    #[doc = "Active WRn is 4 EPI clocks."]
    #[inline(always)]
    pub fn is_wrn4(&self) -> bool {
        *self == Wrws::Wrn4
    }
    #[doc = "Active WRn is 6 EPI clocks."]
    #[inline(always)]
    pub fn is_wrn6(&self) -> bool {
        *self == Wrws::Wrn6
    }
    #[doc = "Active WRn is 8 EPI clocks."]
    #[inline(always)]
    pub fn is_wrn8(&self) -> bool {
        *self == Wrws::Wrn8
    }
}
#[doc = "Field `WRWS` writer - Write Wait States"]
pub type WrwsW<'a, REG> = crate::FieldWriter<'a, REG, 2, Wrws, crate::Safe>;
impl<'a, REG> WrwsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Active WRn is 2 EPI clocks."]
    #[inline(always)]
    pub fn wrn2(self) -> &'a mut crate::W<REG> {
        self.variant(Wrws::Wrn2)
    }
    #[doc = "Active WRn is 4 EPI clocks."]
    #[inline(always)]
    pub fn wrn4(self) -> &'a mut crate::W<REG> {
        self.variant(Wrws::Wrn4)
    }
    #[doc = "Active WRn is 6 EPI clocks."]
    #[inline(always)]
    pub fn wrn6(self) -> &'a mut crate::W<REG> {
        self.variant(Wrws::Wrn6)
    }
    #[doc = "Active WRn is 8 EPI clocks."]
    #[inline(always)]
    pub fn wrn8(self) -> &'a mut crate::W<REG> {
        self.variant(Wrws::Wrn8)
    }
}
#[doc = "Field `MAXWAIT` reader - Maximum Wait"]
pub type MaxwaitR = crate::FieldReader;
#[doc = "Field `MAXWAIT` writer - Maximum Wait"]
pub type MaxwaitW<'a, REG> = crate::FieldWriter<'a, REG, 8, u8, crate::Safe>;
#[doc = "Burst Mode\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Burst {
    #[doc = "0: Burst mode is disabled."]
    Disable = 0,
    #[doc = "1: Burst mode is enabled for CS0n or single chip access."]
    Enable = 1,
}
impl From<Burst> for bool {
    #[inline(always)]
    fn from(variant: Burst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BURST` reader - Burst Mode"]
pub type BurstR = crate::BitReader<Burst>;
impl BurstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Burst {
        match self.bits {
            false => Burst::Disable,
            true => Burst::Enable,
        }
    }
    #[doc = "Burst mode is disabled."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Burst::Disable
    }
    #[doc = "Burst mode is enabled for CS0n or single chip access."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Burst::Enable
    }
}
#[doc = "Field `BURST` writer - Burst Mode"]
pub type BurstW<'a, REG> = crate::BitWriter<'a, REG, Burst>;
impl<'a, REG> BurstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Burst mode is disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Burst::Disable)
    }
    #[doc = "Burst mode is enabled for CS0n or single chip access."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Burst::Enable)
    }
}
#[doc = "PSRAM Configuration Register Read\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rdcre {
    #[doc = "0: No Action."]
    ForceClear = 0,
    #[doc = "1: Start CRE read transaction for CS0n."]
    Trigger = 1,
}
impl From<Rdcre> for bool {
    #[inline(always)]
    fn from(variant: Rdcre) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RDCRE` reader - PSRAM Configuration Register Read"]
pub type RdcreR = crate::BitReader<Rdcre>;
impl RdcreR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rdcre {
        match self.bits {
            false => Rdcre::ForceClear,
            true => Rdcre::Trigger,
        }
    }
    #[doc = "No Action."]
    #[inline(always)]
    pub fn is_force_clear(&self) -> bool {
        *self == Rdcre::ForceClear
    }
    #[doc = "Start CRE read transaction for CS0n."]
    #[inline(always)]
    pub fn is_trigger(&self) -> bool {
        *self == Rdcre::Trigger
    }
}
#[doc = "Field `RDCRE` writer - PSRAM Configuration Register Read"]
pub type RdcreW<'a, REG> = crate::BitWriter<'a, REG, Rdcre>;
impl<'a, REG> RdcreW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Action."]
    #[inline(always)]
    pub fn force_clear(self) -> &'a mut crate::W<REG> {
        self.variant(Rdcre::ForceClear)
    }
    #[doc = "Start CRE read transaction for CS0n."]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut crate::W<REG> {
        self.variant(Rdcre::Trigger)
    }
}
#[doc = "PSRAM Configuration Register Write\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wrcre {
    #[doc = "0: No Action."]
    ForceClear = 0,
    #[doc = "1: Start CRE write transaction for CS0n."]
    Trigger = 1,
}
impl From<Wrcre> for bool {
    #[inline(always)]
    fn from(variant: Wrcre) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WRCRE` reader - PSRAM Configuration Register Write"]
pub type WrcreR = crate::BitReader<Wrcre>;
impl WrcreR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wrcre {
        match self.bits {
            false => Wrcre::ForceClear,
            true => Wrcre::Trigger,
        }
    }
    #[doc = "No Action."]
    #[inline(always)]
    pub fn is_force_clear(&self) -> bool {
        *self == Wrcre::ForceClear
    }
    #[doc = "Start CRE write transaction for CS0n."]
    #[inline(always)]
    pub fn is_trigger(&self) -> bool {
        *self == Wrcre::Trigger
    }
}
#[doc = "Field `WRCRE` writer - PSRAM Configuration Register Write"]
pub type WrcreW<'a, REG> = crate::BitWriter<'a, REG, Wrcre>;
impl<'a, REG> WrcreW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Action."]
    #[inline(always)]
    pub fn force_clear(self) -> &'a mut crate::W<REG> {
        self.variant(Wrcre::ForceClear)
    }
    #[doc = "Start CRE write transaction for CS0n."]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut crate::W<REG> {
        self.variant(Wrcre::Trigger)
    }
}
#[doc = "ALE Strobe Polarity\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Alehigh {
    #[doc = "0: The address latch strobe for CS0n is ALEn (active Low)."]
    Low = 0,
    #[doc = "1: The address latch strobe for CS0n is ALE (active High)."]
    High = 1,
}
impl From<Alehigh> for bool {
    #[inline(always)]
    fn from(variant: Alehigh) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ALEHIGH` reader - ALE Strobe Polarity"]
pub type AlehighR = crate::BitReader<Alehigh>;
impl AlehighR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Alehigh {
        match self.bits {
            false => Alehigh::Low,
            true => Alehigh::High,
        }
    }
    #[doc = "The address latch strobe for CS0n is ALEn (active Low)."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Alehigh::Low
    }
    #[doc = "The address latch strobe for CS0n is ALE (active High)."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Alehigh::High
    }
}
#[doc = "Field `ALEHIGH` writer - ALE Strobe Polarity"]
pub type AlehighW<'a, REG> = crate::BitWriter<'a, REG, Alehigh>;
impl<'a, REG> AlehighW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The address latch strobe for CS0n is ALEn (active Low)."]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Alehigh::Low)
    }
    #[doc = "The address latch strobe for CS0n is ALE (active High)."]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Alehigh::High)
    }
}
#[doc = "READ Strobe Polarity\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rdhigh {
    #[doc = "0: The READ strobe for CS0n is RDn (active Low)."]
    Low = 0,
    #[doc = "1: The READ strobe for CS0n is RD (active High)."]
    High = 1,
}
impl From<Rdhigh> for bool {
    #[inline(always)]
    fn from(variant: Rdhigh) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RDHIGH` reader - READ Strobe Polarity"]
pub type RdhighR = crate::BitReader<Rdhigh>;
impl RdhighR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rdhigh {
        match self.bits {
            false => Rdhigh::Low,
            true => Rdhigh::High,
        }
    }
    #[doc = "The READ strobe for CS0n is RDn (active Low)."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Rdhigh::Low
    }
    #[doc = "The READ strobe for CS0n is RD (active High)."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Rdhigh::High
    }
}
#[doc = "Field `RDHIGH` writer - READ Strobe Polarity"]
pub type RdhighW<'a, REG> = crate::BitWriter<'a, REG, Rdhigh>;
impl<'a, REG> RdhighW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The READ strobe for CS0n is RDn (active Low)."]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Rdhigh::Low)
    }
    #[doc = "The READ strobe for CS0n is RD (active High)."]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Rdhigh::High)
    }
}
#[doc = "WRITE Strobe Polarity\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wrhigh {
    #[doc = "0: The WRITE strobe for CS0n is WRn (active Low)."]
    Low = 0,
    #[doc = "1: The WRITE strobe for CS0n is WR (active High)."]
    High = 1,
}
impl From<Wrhigh> for bool {
    #[inline(always)]
    fn from(variant: Wrhigh) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WRHIGH` reader - WRITE Strobe Polarity"]
pub type WrhighR = crate::BitReader<Wrhigh>;
impl WrhighR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wrhigh {
        match self.bits {
            false => Wrhigh::Low,
            true => Wrhigh::High,
        }
    }
    #[doc = "The WRITE strobe for CS0n is WRn (active Low)."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Wrhigh::Low
    }
    #[doc = "The WRITE strobe for CS0n is WR (active High)."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Wrhigh::High
    }
}
#[doc = "Field `WRHIGH` writer - WRITE Strobe Polarity"]
pub type WrhighW<'a, REG> = crate::BitWriter<'a, REG, Wrhigh>;
impl<'a, REG> WrhighW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The WRITE strobe for CS0n is WRn (active Low)."]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Wrhigh::Low)
    }
    #[doc = "The WRITE strobe for CS0n is WR (active High)."]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Wrhigh::High)
    }
}
#[doc = "External FIFO EMPTY Enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Xfeen {
    #[doc = "0: No effect."]
    MonitorOff = 0,
    #[doc = "1: An external FIFO empty signal can be used to control read cycles. If this bit is set and the FEMPTY signal is high, XFIFO reads are stalled."]
    Monitor = 1,
}
impl From<Xfeen> for bool {
    #[inline(always)]
    fn from(variant: Xfeen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `XFEEN` reader - External FIFO EMPTY Enable"]
pub type XfeenR = crate::BitReader<Xfeen>;
impl XfeenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Xfeen {
        match self.bits {
            false => Xfeen::MonitorOff,
            true => Xfeen::Monitor,
        }
    }
    #[doc = "No effect."]
    #[inline(always)]
    pub fn is_monitor_off(&self) -> bool {
        *self == Xfeen::MonitorOff
    }
    #[doc = "An external FIFO empty signal can be used to control read cycles. If this bit is set and the FEMPTY signal is high, XFIFO reads are stalled."]
    #[inline(always)]
    pub fn is_monitor(&self) -> bool {
        *self == Xfeen::Monitor
    }
}
#[doc = "Field `XFEEN` writer - External FIFO EMPTY Enable"]
pub type XfeenW<'a, REG> = crate::BitWriter<'a, REG, Xfeen>;
impl<'a, REG> XfeenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect."]
    #[inline(always)]
    pub fn monitor_off(self) -> &'a mut crate::W<REG> {
        self.variant(Xfeen::MonitorOff)
    }
    #[doc = "An external FIFO empty signal can be used to control read cycles. If this bit is set and the FEMPTY signal is high, XFIFO reads are stalled."]
    #[inline(always)]
    pub fn monitor(self) -> &'a mut crate::W<REG> {
        self.variant(Xfeen::Monitor)
    }
}
#[doc = "External FIFO FULL Enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Xffen {
    #[doc = "0: No effect."]
    MonitorOff = 0,
    #[doc = "1: An external FIFO full signal can be used to control write cycles. If this bit is set and the FFULL signal is high, XFIFO writes are stalled."]
    Monitor = 1,
}
impl From<Xffen> for bool {
    #[inline(always)]
    fn from(variant: Xffen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `XFFEN` reader - External FIFO FULL Enable"]
pub type XffenR = crate::BitReader<Xffen>;
impl XffenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Xffen {
        match self.bits {
            false => Xffen::MonitorOff,
            true => Xffen::Monitor,
        }
    }
    #[doc = "No effect."]
    #[inline(always)]
    pub fn is_monitor_off(&self) -> bool {
        *self == Xffen::MonitorOff
    }
    #[doc = "An external FIFO full signal can be used to control write cycles. If this bit is set and the FFULL signal is high, XFIFO writes are stalled."]
    #[inline(always)]
    pub fn is_monitor(&self) -> bool {
        *self == Xffen::Monitor
    }
}
#[doc = "Field `XFFEN` writer - External FIFO FULL Enable"]
pub type XffenW<'a, REG> = crate::BitWriter<'a, REG, Xffen>;
impl<'a, REG> XffenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect."]
    #[inline(always)]
    pub fn monitor_off(self) -> &'a mut crate::W<REG> {
        self.variant(Xffen::MonitorOff)
    }
    #[doc = "An external FIFO full signal can be used to control write cycles. If this bit is set and the FFULL signal is high, XFIFO writes are stalled."]
    #[inline(always)]
    pub fn monitor(self) -> &'a mut crate::W<REG> {
        self.variant(Xffen::Monitor)
    }
}
#[doc = "Input Ready Invert\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Irdyinv {
    #[doc = "0: No effect."]
    Normal = 0,
    #[doc = "1: Invert polarity of incoming external ready. If this bit is set and the iRDY signal (EPIS032) is high the current access is stalled."]
    Invert = 1,
}
impl From<Irdyinv> for bool {
    #[inline(always)]
    fn from(variant: Irdyinv) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IRDYINV` reader - Input Ready Invert"]
pub type IrdyinvR = crate::BitReader<Irdyinv>;
impl IrdyinvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Irdyinv {
        match self.bits {
            false => Irdyinv::Normal,
            true => Irdyinv::Invert,
        }
    }
    #[doc = "No effect."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == Irdyinv::Normal
    }
    #[doc = "Invert polarity of incoming external ready. If this bit is set and the iRDY signal (EPIS032) is high the current access is stalled."]
    #[inline(always)]
    pub fn is_invert(&self) -> bool {
        *self == Irdyinv::Invert
    }
}
#[doc = "Field `IRDYINV` writer - Input Ready Invert"]
pub type IrdyinvW<'a, REG> = crate::BitWriter<'a, REG, Irdyinv>;
impl<'a, REG> IrdyinvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(Irdyinv::Normal)
    }
    #[doc = "Invert polarity of incoming external ready. If this bit is set and the iRDY signal (EPIS032) is high the current access is stalled."]
    #[inline(always)]
    pub fn invert(self) -> &'a mut crate::W<REG> {
        self.variant(Irdyinv::Invert)
    }
}
#[doc = "Input Ready Enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rdyen {
    #[doc = "0: No effect."]
    Disable = 0,
    #[doc = "1: An external ready (iRDY) can be used to control the continuation of the current access. If this bit is set and the iRDY signal (EPIS032) is low, the current access is stalled."]
    Enable = 1,
}
impl From<Rdyen> for bool {
    #[inline(always)]
    fn from(variant: Rdyen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RDYEN` reader - Input Ready Enable"]
pub type RdyenR = crate::BitReader<Rdyen>;
impl RdyenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rdyen {
        match self.bits {
            false => Rdyen::Disable,
            true => Rdyen::Enable,
        }
    }
    #[doc = "No effect."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Rdyen::Disable
    }
    #[doc = "An external ready (iRDY) can be used to control the continuation of the current access. If this bit is set and the iRDY signal (EPIS032) is low, the current access is stalled."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Rdyen::Enable
    }
}
#[doc = "Field `RDYEN` writer - Input Ready Enable"]
pub type RdyenW<'a, REG> = crate::BitWriter<'a, REG, Rdyen>;
impl<'a, REG> RdyenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Rdyen::Disable)
    }
    #[doc = "An external ready (iRDY) can be used to control the continuation of the current access. If this bit is set and the iRDY signal (EPIS032) is low, the current access is stalled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Rdyen::Enable)
    }
}
#[doc = "Invert Output Clock Enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Clkinv {
    #[doc = "0: No effect."]
    Normal = 0,
    #[doc = "1: Invert EPI clock to ensure the rising edge is centered for outbound signals setup and hold. Inbound signal is captured on rising edge EPI clock."]
    Invert = 1,
}
impl From<Clkinv> for bool {
    #[inline(always)]
    fn from(variant: Clkinv) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLKINV` reader - Invert Output Clock Enable"]
pub type ClkinvR = crate::BitReader<Clkinv>;
impl ClkinvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Clkinv {
        match self.bits {
            false => Clkinv::Normal,
            true => Clkinv::Invert,
        }
    }
    #[doc = "No effect."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == Clkinv::Normal
    }
    #[doc = "Invert EPI clock to ensure the rising edge is centered for outbound signals setup and hold. Inbound signal is captured on rising edge EPI clock."]
    #[inline(always)]
    pub fn is_invert(&self) -> bool {
        *self == Clkinv::Invert
    }
}
#[doc = "Field `CLKINV` writer - Invert Output Clock Enable"]
pub type ClkinvW<'a, REG> = crate::BitWriter<'a, REG, Clkinv>;
impl<'a, REG> ClkinvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(Clkinv::Normal)
    }
    #[doc = "Invert EPI clock to ensure the rising edge is centered for outbound signals setup and hold. Inbound signal is captured on rising edge EPI clock."]
    #[inline(always)]
    pub fn invert(self) -> &'a mut crate::W<REG> {
        self.variant(Clkinv::Invert)
    }
}
#[doc = "Clock Gated Idle\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Clkgatei {
    #[doc = "0: The EPI clock is free running."]
    FreeRun = 0,
    #[doc = "1: The EPI clock is output only when there is data to write or read (current transaction); otherwise the EPI clock is held low."]
    ForceLow = 1,
}
impl From<Clkgatei> for bool {
    #[inline(always)]
    fn from(variant: Clkgatei) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLKGATEI` reader - Clock Gated Idle"]
pub type ClkgateiR = crate::BitReader<Clkgatei>;
impl ClkgateiR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Clkgatei {
        match self.bits {
            false => Clkgatei::FreeRun,
            true => Clkgatei::ForceLow,
        }
    }
    #[doc = "The EPI clock is free running."]
    #[inline(always)]
    pub fn is_free_run(&self) -> bool {
        *self == Clkgatei::FreeRun
    }
    #[doc = "The EPI clock is output only when there is data to write or read (current transaction); otherwise the EPI clock is held low."]
    #[inline(always)]
    pub fn is_force_low(&self) -> bool {
        *self == Clkgatei::ForceLow
    }
}
#[doc = "Field `CLKGATEI` writer - Clock Gated Idle"]
pub type ClkgateiW<'a, REG> = crate::BitWriter<'a, REG, Clkgatei>;
impl<'a, REG> ClkgateiW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The EPI clock is free running."]
    #[inline(always)]
    pub fn free_run(self) -> &'a mut crate::W<REG> {
        self.variant(Clkgatei::FreeRun)
    }
    #[doc = "The EPI clock is output only when there is data to write or read (current transaction); otherwise the EPI clock is held low."]
    #[inline(always)]
    pub fn force_low(self) -> &'a mut crate::W<REG> {
        self.variant(Clkgatei::ForceLow)
    }
}
#[doc = "Clock Gated\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Clkgate {
    #[doc = "0: The EPI clock is free running."]
    FreeRun = 0,
    #[doc = "1: The EPI clock is held low."]
    ForceLow = 1,
}
impl From<Clkgate> for bool {
    #[inline(always)]
    fn from(variant: Clkgate) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLKGATE` reader - Clock Gated"]
pub type ClkgateR = crate::BitReader<Clkgate>;
impl ClkgateR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Clkgate {
        match self.bits {
            false => Clkgate::FreeRun,
            true => Clkgate::ForceLow,
        }
    }
    #[doc = "The EPI clock is free running."]
    #[inline(always)]
    pub fn is_free_run(&self) -> bool {
        *self == Clkgate::FreeRun
    }
    #[doc = "The EPI clock is held low."]
    #[inline(always)]
    pub fn is_force_low(&self) -> bool {
        *self == Clkgate::ForceLow
    }
}
#[doc = "Field `CLKGATE` writer - Clock Gated"]
pub type ClkgateW<'a, REG> = crate::BitWriter<'a, REG, Clkgate>;
impl<'a, REG> ClkgateW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The EPI clock is free running."]
    #[inline(always)]
    pub fn free_run(self) -> &'a mut crate::W<REG> {
        self.variant(Clkgate::FreeRun)
    }
    #[doc = "The EPI clock is held low."]
    #[inline(always)]
    pub fn force_low(self) -> &'a mut crate::W<REG> {
        self.variant(Clkgate::ForceLow)
    }
}
impl R {
    #[doc = "Bits 0:1 - Host Bus Sub-Mode"]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Byte Select Configuration"]
    #[inline(always)]
    pub fn bsel(&self) -> BselR {
        BselR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Read Wait States"]
    #[inline(always)]
    pub fn rdws(&self) -> RdwsR {
        RdwsR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Write Wait States"]
    #[inline(always)]
    pub fn wrws(&self) -> WrwsR {
        WrwsR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:15 - Maximum Wait"]
    #[inline(always)]
    pub fn maxwait(&self) -> MaxwaitR {
        MaxwaitR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16 - Burst Mode"]
    #[inline(always)]
    pub fn burst(&self) -> BurstR {
        BurstR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - PSRAM Configuration Register Read"]
    #[inline(always)]
    pub fn rdcre(&self) -> RdcreR {
        RdcreR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - PSRAM Configuration Register Write"]
    #[inline(always)]
    pub fn wrcre(&self) -> WrcreR {
        WrcreR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - ALE Strobe Polarity"]
    #[inline(always)]
    pub fn alehigh(&self) -> AlehighR {
        AlehighR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - READ Strobe Polarity"]
    #[inline(always)]
    pub fn rdhigh(&self) -> RdhighR {
        RdhighR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - WRITE Strobe Polarity"]
    #[inline(always)]
    pub fn wrhigh(&self) -> WrhighR {
        WrhighR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - External FIFO EMPTY Enable"]
    #[inline(always)]
    pub fn xfeen(&self) -> XfeenR {
        XfeenR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - External FIFO FULL Enable"]
    #[inline(always)]
    pub fn xffen(&self) -> XffenR {
        XffenR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 27 - Input Ready Invert"]
    #[inline(always)]
    pub fn irdyinv(&self) -> IrdyinvR {
        IrdyinvR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Input Ready Enable"]
    #[inline(always)]
    pub fn rdyen(&self) -> RdyenR {
        RdyenR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Invert Output Clock Enable"]
    #[inline(always)]
    pub fn clkinv(&self) -> ClkinvR {
        ClkinvR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Clock Gated Idle"]
    #[inline(always)]
    pub fn clkgatei(&self) -> ClkgateiR {
        ClkgateiR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Clock Gated"]
    #[inline(always)]
    pub fn clkgate(&self) -> ClkgateR {
        ClkgateR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HB16CFG")
            .field("mode", &self.mode())
            .field("bsel", &self.bsel())
            .field("rdws", &self.rdws())
            .field("wrws", &self.wrws())
            .field("maxwait", &self.maxwait())
            .field("burst", &self.burst())
            .field("rdcre", &self.rdcre())
            .field("wrcre", &self.wrcre())
            .field("alehigh", &self.alehigh())
            .field("rdhigh", &self.rdhigh())
            .field("wrhigh", &self.wrhigh())
            .field("xfeen", &self.xfeen())
            .field("xffen", &self.xffen())
            .field("irdyinv", &self.irdyinv())
            .field("rdyen", &self.rdyen())
            .field("clkinv", &self.clkinv())
            .field("clkgatei", &self.clkgatei())
            .field("clkgate", &self.clkgate())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - Host Bus Sub-Mode"]
    #[inline(always)]
    pub fn mode(&mut self) -> ModeW<'_, Hb16cfgSpec> {
        ModeW::new(self, 0)
    }
    #[doc = "Bit 2 - Byte Select Configuration"]
    #[inline(always)]
    pub fn bsel(&mut self) -> BselW<'_, Hb16cfgSpec> {
        BselW::new(self, 2)
    }
    #[doc = "Bits 4:5 - Read Wait States"]
    #[inline(always)]
    pub fn rdws(&mut self) -> RdwsW<'_, Hb16cfgSpec> {
        RdwsW::new(self, 4)
    }
    #[doc = "Bits 6:7 - Write Wait States"]
    #[inline(always)]
    pub fn wrws(&mut self) -> WrwsW<'_, Hb16cfgSpec> {
        WrwsW::new(self, 6)
    }
    #[doc = "Bits 8:15 - Maximum Wait"]
    #[inline(always)]
    pub fn maxwait(&mut self) -> MaxwaitW<'_, Hb16cfgSpec> {
        MaxwaitW::new(self, 8)
    }
    #[doc = "Bit 16 - Burst Mode"]
    #[inline(always)]
    pub fn burst(&mut self) -> BurstW<'_, Hb16cfgSpec> {
        BurstW::new(self, 16)
    }
    #[doc = "Bit 17 - PSRAM Configuration Register Read"]
    #[inline(always)]
    pub fn rdcre(&mut self) -> RdcreW<'_, Hb16cfgSpec> {
        RdcreW::new(self, 17)
    }
    #[doc = "Bit 18 - PSRAM Configuration Register Write"]
    #[inline(always)]
    pub fn wrcre(&mut self) -> WrcreW<'_, Hb16cfgSpec> {
        WrcreW::new(self, 18)
    }
    #[doc = "Bit 19 - ALE Strobe Polarity"]
    #[inline(always)]
    pub fn alehigh(&mut self) -> AlehighW<'_, Hb16cfgSpec> {
        AlehighW::new(self, 19)
    }
    #[doc = "Bit 20 - READ Strobe Polarity"]
    #[inline(always)]
    pub fn rdhigh(&mut self) -> RdhighW<'_, Hb16cfgSpec> {
        RdhighW::new(self, 20)
    }
    #[doc = "Bit 21 - WRITE Strobe Polarity"]
    #[inline(always)]
    pub fn wrhigh(&mut self) -> WrhighW<'_, Hb16cfgSpec> {
        WrhighW::new(self, 21)
    }
    #[doc = "Bit 22 - External FIFO EMPTY Enable"]
    #[inline(always)]
    pub fn xfeen(&mut self) -> XfeenW<'_, Hb16cfgSpec> {
        XfeenW::new(self, 22)
    }
    #[doc = "Bit 23 - External FIFO FULL Enable"]
    #[inline(always)]
    pub fn xffen(&mut self) -> XffenW<'_, Hb16cfgSpec> {
        XffenW::new(self, 23)
    }
    #[doc = "Bit 27 - Input Ready Invert"]
    #[inline(always)]
    pub fn irdyinv(&mut self) -> IrdyinvW<'_, Hb16cfgSpec> {
        IrdyinvW::new(self, 27)
    }
    #[doc = "Bit 28 - Input Ready Enable"]
    #[inline(always)]
    pub fn rdyen(&mut self) -> RdyenW<'_, Hb16cfgSpec> {
        RdyenW::new(self, 28)
    }
    #[doc = "Bit 29 - Invert Output Clock Enable"]
    #[inline(always)]
    pub fn clkinv(&mut self) -> ClkinvW<'_, Hb16cfgSpec> {
        ClkinvW::new(self, 29)
    }
    #[doc = "Bit 30 - Clock Gated Idle"]
    #[inline(always)]
    pub fn clkgatei(&mut self) -> ClkgateiW<'_, Hb16cfgSpec> {
        ClkgateiW::new(self, 30)
    }
    #[doc = "Bit 31 - Clock Gated"]
    #[inline(always)]
    pub fn clkgate(&mut self) -> ClkgateW<'_, Hb16cfgSpec> {
        ClkgateW::new(self, 31)
    }
}
#[doc = "EPI Host-Bus 16 Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`hb16cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hb16cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hb16cfgSpec;
impl crate::RegisterSpec for Hb16cfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hb16cfg::R`](R) reader structure"]
impl crate::Readable for Hb16cfgSpec {}
#[doc = "`write(|w| ..)` method takes [`hb16cfg::W`](W) writer structure"]
impl crate::Writable for Hb16cfgSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets HB16CFG to value 0"]
impl crate::Resettable for Hb16cfgSpec {}
