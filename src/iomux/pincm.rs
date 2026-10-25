#[doc = "Register `PINCM[%s]` reader"]
pub type R = crate::R<PincmSpec>;
#[doc = "Register `PINCM[%s]` writer"]
pub type W = crate::W<PincmSpec>;
#[doc = "Field `PF` reader - P channel Function selection bits"]
pub type PfR = crate::FieldReader;
#[doc = "Field `PF` writer - P channel Function selection bits"]
pub type PfW<'a, REG> = crate::FieldWriter<'a, REG, 6, u8, crate::Safe>;
#[doc = "Peripheral is Connected\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pc {
    #[doc = "0: The output of the peripheral (and its output enable) will not propagate to the IOCELL"]
    Unconnected = 0,
    #[doc = "1: The output latch of the dataflow will be transparent"]
    Connected = 1,
}
impl From<Pc> for bool {
    #[inline(always)]
    fn from(variant: Pc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PC` reader - Peripheral is Connected"]
pub type PcR = crate::BitReader<Pc>;
impl PcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pc {
        match self.bits {
            false => Pc::Unconnected,
            true => Pc::Connected,
        }
    }
    #[doc = "The output of the peripheral (and its output enable) will not propagate to the IOCELL"]
    #[inline(always)]
    pub fn is_unconnected(&self) -> bool {
        *self == Pc::Unconnected
    }
    #[doc = "The output latch of the dataflow will be transparent"]
    #[inline(always)]
    pub fn is_connected(&self) -> bool {
        *self == Pc::Connected
    }
}
#[doc = "Field `PC` writer - Peripheral is Connected"]
pub type PcW<'a, REG> = crate::BitWriter<'a, REG, Pc>;
impl<'a, REG> PcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The output of the peripheral (and its output enable) will not propagate to the IOCELL"]
    #[inline(always)]
    pub fn unconnected(self) -> &'a mut crate::W<REG> {
        self.variant(Pc::Unconnected)
    }
    #[doc = "The output latch of the dataflow will be transparent"]
    #[inline(always)]
    pub fn connected(self) -> &'a mut crate::W<REG> {
        self.variant(Pc::Connected)
    }
}
#[doc = "This has the IOPAD WAKEUP signal as status bit.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wakestat {
    #[doc = "0: wakeup source is NOT from this IOCELL"]
    Disable = 0,
    #[doc = "1: wakeup source is from this IOCELL"]
    Enable = 1,
}
impl From<Wakestat> for bool {
    #[inline(always)]
    fn from(variant: Wakestat) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WAKESTAT` reader - This has the IOPAD WAKEUP signal as status bit."]
pub type WakestatR = crate::BitReader<Wakestat>;
impl WakestatR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wakestat {
        match self.bits {
            false => Wakestat::Disable,
            true => Wakestat::Enable,
        }
    }
    #[doc = "wakeup source is NOT from this IOCELL"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Wakestat::Disable
    }
    #[doc = "wakeup source is from this IOCELL"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Wakestat::Enable
    }
}
#[doc = "Pull Down control selection\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pipd {
    #[doc = "0: Pull down is disabled."]
    Disable = 0,
    #[doc = "1: Pull down is enabled"]
    Enable = 1,
}
impl From<Pipd> for bool {
    #[inline(always)]
    fn from(variant: Pipd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIPD` reader - Pull Down control selection"]
pub type PipdR = crate::BitReader<Pipd>;
impl PipdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pipd {
        match self.bits {
            false => Pipd::Disable,
            true => Pipd::Enable,
        }
    }
    #[doc = "Pull down is disabled."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Pipd::Disable
    }
    #[doc = "Pull down is enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Pipd::Enable
    }
}
#[doc = "Field `PIPD` writer - Pull Down control selection"]
pub type PipdW<'a, REG> = crate::BitWriter<'a, REG, Pipd>;
impl<'a, REG> PipdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pull down is disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Pipd::Disable)
    }
    #[doc = "Pull down is enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Pipd::Enable)
    }
}
#[doc = "Pull Up control selection\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pipu {
    #[doc = "0: Pull up is disabled."]
    Disable = 0,
    #[doc = "1: Pull up is enabled"]
    Enable = 1,
}
impl From<Pipu> for bool {
    #[inline(always)]
    fn from(variant: Pipu) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIPU` reader - Pull Up control selection"]
pub type PipuR = crate::BitReader<Pipu>;
impl PipuR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pipu {
        match self.bits {
            false => Pipu::Disable,
            true => Pipu::Enable,
        }
    }
    #[doc = "Pull up is disabled."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Pipu::Disable
    }
    #[doc = "Pull up is enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Pipu::Enable
    }
}
#[doc = "Field `PIPU` writer - Pull Up control selection"]
pub type PipuW<'a, REG> = crate::BitWriter<'a, REG, Pipu>;
impl<'a, REG> PipuW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pull up is disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Pipu::Disable)
    }
    #[doc = "Pull up is enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Pipu::Enable)
    }
}
#[doc = "Input Enable Control Selection\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Inena {
    #[doc = "0: Input enable is disabled."]
    Disable = 0,
    #[doc = "1: Input enable is enabled."]
    Enable = 1,
}
impl From<Inena> for bool {
    #[inline(always)]
    fn from(variant: Inena) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INENA` reader - Input Enable Control Selection"]
pub type InenaR = crate::BitReader<Inena>;
impl InenaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Inena {
        match self.bits {
            false => Inena::Disable,
            true => Inena::Enable,
        }
    }
    #[doc = "Input enable is disabled."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Inena::Disable
    }
    #[doc = "Input enable is enabled."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Inena::Enable
    }
}
#[doc = "Field `INENA` writer - Input Enable Control Selection"]
pub type InenaW<'a, REG> = crate::BitWriter<'a, REG, Inena>;
impl<'a, REG> InenaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Input enable is disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Inena::Disable)
    }
    #[doc = "Input enable is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Inena::Enable)
    }
}
#[doc = "Hystersis Enable Control Selection\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hysten {
    #[doc = "0: hysteresis is disabled."]
    Disable = 0,
    #[doc = "1: hysteresis is enabled"]
    Enable = 1,
}
impl From<Hysten> for bool {
    #[inline(always)]
    fn from(variant: Hysten) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HYSTEN` reader - Hystersis Enable Control Selection"]
pub type HystenR = crate::BitReader<Hysten>;
impl HystenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hysten {
        match self.bits {
            false => Hysten::Disable,
            true => Hysten::Enable,
        }
    }
    #[doc = "hysteresis is disabled."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Hysten::Disable
    }
    #[doc = "hysteresis is enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Hysten::Enable
    }
}
#[doc = "Field `HYSTEN` writer - Hystersis Enable Control Selection"]
pub type HystenW<'a, REG> = crate::BitWriter<'a, REG, Hysten>;
impl<'a, REG> HystenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "hysteresis is disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Hysten::Disable)
    }
    #[doc = "hysteresis is enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Hysten::Enable)
    }
}
#[doc = "Drive strength control selection, for HS IOCELL only\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Drv {
    #[doc = "0: Drive setting of 0 selected"]
    Drvval0 = 0,
    #[doc = "1: Drive setting of 1 selected"]
    Drvval1 = 1,
}
impl From<Drv> for bool {
    #[inline(always)]
    fn from(variant: Drv) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DRV` reader - Drive strength control selection, for HS IOCELL only"]
pub type DrvR = crate::BitReader<Drv>;
impl DrvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Drv {
        match self.bits {
            false => Drv::Drvval0,
            true => Drv::Drvval1,
        }
    }
    #[doc = "Drive setting of 0 selected"]
    #[inline(always)]
    pub fn is_drvval0(&self) -> bool {
        *self == Drv::Drvval0
    }
    #[doc = "Drive setting of 1 selected"]
    #[inline(always)]
    pub fn is_drvval1(&self) -> bool {
        *self == Drv::Drvval1
    }
}
#[doc = "Field `DRV` writer - Drive strength control selection, for HS IOCELL only"]
pub type DrvW<'a, REG> = crate::BitWriter<'a, REG, Drv>;
impl<'a, REG> DrvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Drive setting of 0 selected"]
    #[inline(always)]
    pub fn drvval0(self) -> &'a mut crate::W<REG> {
        self.variant(Drv::Drvval0)
    }
    #[doc = "Drive setting of 1 selected"]
    #[inline(always)]
    pub fn drvval1(self) -> &'a mut crate::W<REG> {
        self.variant(Drv::Drvval1)
    }
}
#[doc = "High output value will tri-state the output when this bit is enabled\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hiz1 {
    #[doc = "0: open-drain is disabled."]
    Disable = 0,
    #[doc = "1: open-drain is enabled."]
    Enable = 1,
}
impl From<Hiz1> for bool {
    #[inline(always)]
    fn from(variant: Hiz1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HIZ1` reader - High output value will tri-state the output when this bit is enabled"]
pub type Hiz1R = crate::BitReader<Hiz1>;
impl Hiz1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hiz1 {
        match self.bits {
            false => Hiz1::Disable,
            true => Hiz1::Enable,
        }
    }
    #[doc = "open-drain is disabled."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Hiz1::Disable
    }
    #[doc = "open-drain is enabled."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Hiz1::Enable
    }
}
#[doc = "Field `HIZ1` writer - High output value will tri-state the output when this bit is enabled"]
pub type Hiz1W<'a, REG> = crate::BitWriter<'a, REG, Hiz1>;
impl<'a, REG> Hiz1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "open-drain is disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Hiz1::Disable)
    }
    #[doc = "open-drain is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Hiz1::Enable)
    }
}
#[doc = "Data inversion selection\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Inv {
    #[doc = "0: Data inversion is disabled."]
    Disable = 0,
    #[doc = "1: Data inversion is enabled"]
    Enable = 1,
}
impl From<Inv> for bool {
    #[inline(always)]
    fn from(variant: Inv) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INV` reader - Data inversion selection"]
pub type InvR = crate::BitReader<Inv>;
impl InvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Inv {
        match self.bits {
            false => Inv::Disable,
            true => Inv::Enable,
        }
    }
    #[doc = "Data inversion is disabled."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Inv::Disable
    }
    #[doc = "Data inversion is enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Inv::Enable
    }
}
#[doc = "Field `INV` writer - Data inversion selection"]
pub type InvW<'a, REG> = crate::BitWriter<'a, REG, Inv>;
impl<'a, REG> InvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Data inversion is disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Inv::Disable)
    }
    #[doc = "Data inversion is enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Inv::Enable)
    }
}
#[doc = "Wakeup Enable bit\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wuen {
    #[doc = "0: wakeup is disabled."]
    Disable = 0,
    #[doc = "1: wakeup is enabled"]
    Enable = 1,
}
impl From<Wuen> for bool {
    #[inline(always)]
    fn from(variant: Wuen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUEN` reader - Wakeup Enable bit"]
pub type WuenR = crate::BitReader<Wuen>;
impl WuenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wuen {
        match self.bits {
            false => Wuen::Disable,
            true => Wuen::Enable,
        }
    }
    #[doc = "wakeup is disabled."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Wuen::Disable
    }
    #[doc = "wakeup is enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Wuen::Enable
    }
}
#[doc = "Field `WUEN` writer - Wakeup Enable bit"]
pub type WuenW<'a, REG> = crate::BitWriter<'a, REG, Wuen>;
impl<'a, REG> WuenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "wakeup is disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Wuen::Disable)
    }
    #[doc = "wakeup is enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Wuen::Enable)
    }
}
#[doc = "Wakeup Compare Value bit\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wcomp {
    #[doc = "0: Wakeup on a match of 0"]
    Match0 = 0,
    #[doc = "1: Wakeup on a match of 1"]
    Match1 = 1,
}
impl From<Wcomp> for bool {
    #[inline(always)]
    fn from(variant: Wcomp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WCOMP` reader - Wakeup Compare Value bit"]
pub type WcompR = crate::BitReader<Wcomp>;
impl WcompR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wcomp {
        match self.bits {
            false => Wcomp::Match0,
            true => Wcomp::Match1,
        }
    }
    #[doc = "Wakeup on a match of 0"]
    #[inline(always)]
    pub fn is_match0(&self) -> bool {
        *self == Wcomp::Match0
    }
    #[doc = "Wakeup on a match of 1"]
    #[inline(always)]
    pub fn is_match1(&self) -> bool {
        *self == Wcomp::Match1
    }
}
#[doc = "Field `WCOMP` writer - Wakeup Compare Value bit"]
pub type WcompW<'a, REG> = crate::BitWriter<'a, REG, Wcomp>;
impl<'a, REG> WcompW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wakeup on a match of 0"]
    #[inline(always)]
    pub fn match0(self) -> &'a mut crate::W<REG> {
        self.variant(Wcomp::Match0)
    }
    #[doc = "Wakeup on a match of 1"]
    #[inline(always)]
    pub fn match1(self) -> &'a mut crate::W<REG> {
        self.variant(Wcomp::Match1)
    }
}
impl R {
    #[doc = "Bits 0:5 - P channel Function selection bits"]
    #[inline(always)]
    pub fn pf(&self) -> PfR {
        PfR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 7 - Peripheral is Connected"]
    #[inline(always)]
    pub fn pc(&self) -> PcR {
        PcR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 13 - This has the IOPAD WAKEUP signal as status bit."]
    #[inline(always)]
    pub fn wakestat(&self) -> WakestatR {
        WakestatR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - Pull Down control selection"]
    #[inline(always)]
    pub fn pipd(&self) -> PipdR {
        PipdR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Pull Up control selection"]
    #[inline(always)]
    pub fn pipu(&self) -> PipuR {
        PipuR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Input Enable Control Selection"]
    #[inline(always)]
    pub fn inena(&self) -> InenaR {
        InenaR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Hystersis Enable Control Selection"]
    #[inline(always)]
    pub fn hysten(&self) -> HystenR {
        HystenR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Drive strength control selection, for HS IOCELL only"]
    #[inline(always)]
    pub fn drv(&self) -> DrvR {
        DrvR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 25 - High output value will tri-state the output when this bit is enabled"]
    #[inline(always)]
    pub fn hiz1(&self) -> Hiz1R {
        Hiz1R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Data inversion selection"]
    #[inline(always)]
    pub fn inv(&self) -> InvR {
        InvR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Wakeup Enable bit"]
    #[inline(always)]
    pub fn wuen(&self) -> WuenR {
        WuenR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Wakeup Compare Value bit"]
    #[inline(always)]
    pub fn wcomp(&self) -> WcompR {
        WcompR::new(((self.bits >> 28) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PINCM")
            .field("pf", &self.pf())
            .field("pc", &self.pc())
            .field("wakestat", &self.wakestat())
            .field("pipd", &self.pipd())
            .field("pipu", &self.pipu())
            .field("inena", &self.inena())
            .field("hysten", &self.hysten())
            .field("drv", &self.drv())
            .field("hiz1", &self.hiz1())
            .field("wuen", &self.wuen())
            .field("wcomp", &self.wcomp())
            .field("inv", &self.inv())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:5 - P channel Function selection bits"]
    #[inline(always)]
    pub fn pf(&mut self) -> PfW<'_, PincmSpec> {
        PfW::new(self, 0)
    }
    #[doc = "Bit 7 - Peripheral is Connected"]
    #[inline(always)]
    pub fn pc(&mut self) -> PcW<'_, PincmSpec> {
        PcW::new(self, 7)
    }
    #[doc = "Bit 16 - Pull Down control selection"]
    #[inline(always)]
    pub fn pipd(&mut self) -> PipdW<'_, PincmSpec> {
        PipdW::new(self, 16)
    }
    #[doc = "Bit 17 - Pull Up control selection"]
    #[inline(always)]
    pub fn pipu(&mut self) -> PipuW<'_, PincmSpec> {
        PipuW::new(self, 17)
    }
    #[doc = "Bit 18 - Input Enable Control Selection"]
    #[inline(always)]
    pub fn inena(&mut self) -> InenaW<'_, PincmSpec> {
        InenaW::new(self, 18)
    }
    #[doc = "Bit 19 - Hystersis Enable Control Selection"]
    #[inline(always)]
    pub fn hysten(&mut self) -> HystenW<'_, PincmSpec> {
        HystenW::new(self, 19)
    }
    #[doc = "Bit 20 - Drive strength control selection, for HS IOCELL only"]
    #[inline(always)]
    pub fn drv(&mut self) -> DrvW<'_, PincmSpec> {
        DrvW::new(self, 20)
    }
    #[doc = "Bit 25 - High output value will tri-state the output when this bit is enabled"]
    #[inline(always)]
    pub fn hiz1(&mut self) -> Hiz1W<'_, PincmSpec> {
        Hiz1W::new(self, 25)
    }
    #[doc = "Bit 26 - Data inversion selection"]
    #[inline(always)]
    pub fn inv(&mut self) -> InvW<'_, PincmSpec> {
        InvW::new(self, 26)
    }
    #[doc = "Bit 27 - Wakeup Enable bit"]
    #[inline(always)]
    pub fn wuen(&mut self) -> WuenW<'_, PincmSpec> {
        WuenW::new(self, 27)
    }
    #[doc = "Bit 28 - Wakeup Compare Value bit"]
    #[inline(always)]
    pub fn wcomp(&mut self) -> WcompW<'_, PincmSpec> {
        WcompW::new(self, 28)
    }
}
#[doc = "Pin Control Management Register in SECCFG region\n\nYou can [`read`](crate::Reg::read) this register and get [`pincm::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pincm::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PincmSpec;
impl crate::RegisterSpec for PincmSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pincm::R`](R) reader structure"]
impl crate::Readable for PincmSpec {}
#[doc = "`write(|w| ..)` method takes [`pincm::W`](W) writer structure"]
impl crate::Writable for PincmSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets PINCM[%s] to value 0"]
impl crate::Resettable for PincmSpec {}
