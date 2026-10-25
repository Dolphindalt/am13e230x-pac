#[doc = "Register `ECCTL1` reader"]
pub type R = crate::R<Ecctl1Spec>;
#[doc = "Register `ECCTL1` writer"]
pub type W = crate::W<Ecctl1Spec>;
#[doc = "Capture Event 1 Polarity select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cap1pol {
    #[doc = "0: Capture Event 1 triggered on a rising edge (RE)"]
    Rise = 0,
    #[doc = "1: Capture Event 1 triggered on a falling edge (FE)"]
    Fall = 1,
}
impl From<Cap1pol> for bool {
    #[inline(always)]
    fn from(variant: Cap1pol) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAP1POL` reader - Capture Event 1 Polarity select"]
pub type Cap1polR = crate::BitReader<Cap1pol>;
impl Cap1polR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cap1pol {
        match self.bits {
            false => Cap1pol::Rise,
            true => Cap1pol::Fall,
        }
    }
    #[doc = "Capture Event 1 triggered on a rising edge (RE)"]
    #[inline(always)]
    pub fn is_rise(&self) -> bool {
        *self == Cap1pol::Rise
    }
    #[doc = "Capture Event 1 triggered on a falling edge (FE)"]
    #[inline(always)]
    pub fn is_fall(&self) -> bool {
        *self == Cap1pol::Fall
    }
}
#[doc = "Field `CAP1POL` writer - Capture Event 1 Polarity select"]
pub type Cap1polW<'a, REG> = crate::BitWriter<'a, REG, Cap1pol>;
impl<'a, REG> Cap1polW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Capture Event 1 triggered on a rising edge (RE)"]
    #[inline(always)]
    pub fn rise(self) -> &'a mut crate::W<REG> {
        self.variant(Cap1pol::Rise)
    }
    #[doc = "Capture Event 1 triggered on a falling edge (FE)"]
    #[inline(always)]
    pub fn fall(self) -> &'a mut crate::W<REG> {
        self.variant(Cap1pol::Fall)
    }
}
#[doc = "Counter Reset on Capture Event 1\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctrrst1 {
    #[doc = "0: Do not reset counter on Capture Event 1 (absolute time stamp)"]
    DoNotReset = 0,
    #[doc = "1: Reset counter after Event 1 time-stamp has been captured (used in difference mode operation)"]
    Reset = 1,
}
impl From<Ctrrst1> for bool {
    #[inline(always)]
    fn from(variant: Ctrrst1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTRRST1` reader - Counter Reset on Capture Event 1"]
pub type Ctrrst1R = crate::BitReader<Ctrrst1>;
impl Ctrrst1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctrrst1 {
        match self.bits {
            false => Ctrrst1::DoNotReset,
            true => Ctrrst1::Reset,
        }
    }
    #[doc = "Do not reset counter on Capture Event 1 (absolute time stamp)"]
    #[inline(always)]
    pub fn is_do_not_reset(&self) -> bool {
        *self == Ctrrst1::DoNotReset
    }
    #[doc = "Reset counter after Event 1 time-stamp has been captured (used in difference mode operation)"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == Ctrrst1::Reset
    }
}
#[doc = "Field `CTRRST1` writer - Counter Reset on Capture Event 1"]
pub type Ctrrst1W<'a, REG> = crate::BitWriter<'a, REG, Ctrrst1>;
impl<'a, REG> Ctrrst1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Do not reset counter on Capture Event 1 (absolute time stamp)"]
    #[inline(always)]
    pub fn do_not_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Ctrrst1::DoNotReset)
    }
    #[doc = "Reset counter after Event 1 time-stamp has been captured (used in difference mode operation)"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(Ctrrst1::Reset)
    }
}
#[doc = "Capture Event 2 Polarity select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cap2pol {
    #[doc = "0: Capture Event 2 triggered on a rising edge (RE)"]
    Rise = 0,
    #[doc = "1: Capture Event 2 triggered on a falling edge (FE)"]
    Fall = 1,
}
impl From<Cap2pol> for bool {
    #[inline(always)]
    fn from(variant: Cap2pol) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAP2POL` reader - Capture Event 2 Polarity select"]
pub type Cap2polR = crate::BitReader<Cap2pol>;
impl Cap2polR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cap2pol {
        match self.bits {
            false => Cap2pol::Rise,
            true => Cap2pol::Fall,
        }
    }
    #[doc = "Capture Event 2 triggered on a rising edge (RE)"]
    #[inline(always)]
    pub fn is_rise(&self) -> bool {
        *self == Cap2pol::Rise
    }
    #[doc = "Capture Event 2 triggered on a falling edge (FE)"]
    #[inline(always)]
    pub fn is_fall(&self) -> bool {
        *self == Cap2pol::Fall
    }
}
#[doc = "Field `CAP2POL` writer - Capture Event 2 Polarity select"]
pub type Cap2polW<'a, REG> = crate::BitWriter<'a, REG, Cap2pol>;
impl<'a, REG> Cap2polW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Capture Event 2 triggered on a rising edge (RE)"]
    #[inline(always)]
    pub fn rise(self) -> &'a mut crate::W<REG> {
        self.variant(Cap2pol::Rise)
    }
    #[doc = "Capture Event 2 triggered on a falling edge (FE)"]
    #[inline(always)]
    pub fn fall(self) -> &'a mut crate::W<REG> {
        self.variant(Cap2pol::Fall)
    }
}
#[doc = "Counter Reset on Capture Event 2\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctrrst2 {
    #[doc = "0: Do not reset counter on Capture Event 2 (absolute time stamp)"]
    DoNotReset = 0,
    #[doc = "1: Reset counter after Event 2 time-stamp has been captured (used in difference mode operation)"]
    Reset = 1,
}
impl From<Ctrrst2> for bool {
    #[inline(always)]
    fn from(variant: Ctrrst2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTRRST2` reader - Counter Reset on Capture Event 2"]
pub type Ctrrst2R = crate::BitReader<Ctrrst2>;
impl Ctrrst2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctrrst2 {
        match self.bits {
            false => Ctrrst2::DoNotReset,
            true => Ctrrst2::Reset,
        }
    }
    #[doc = "Do not reset counter on Capture Event 2 (absolute time stamp)"]
    #[inline(always)]
    pub fn is_do_not_reset(&self) -> bool {
        *self == Ctrrst2::DoNotReset
    }
    #[doc = "Reset counter after Event 2 time-stamp has been captured (used in difference mode operation)"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == Ctrrst2::Reset
    }
}
#[doc = "Field `CTRRST2` writer - Counter Reset on Capture Event 2"]
pub type Ctrrst2W<'a, REG> = crate::BitWriter<'a, REG, Ctrrst2>;
impl<'a, REG> Ctrrst2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Do not reset counter on Capture Event 2 (absolute time stamp)"]
    #[inline(always)]
    pub fn do_not_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Ctrrst2::DoNotReset)
    }
    #[doc = "Reset counter after Event 2 time-stamp has been captured (used in difference mode operation)"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(Ctrrst2::Reset)
    }
}
#[doc = "Capture Event 3 Polarity select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cap3pol {
    #[doc = "0: Capture Event 3 triggered on a rising edge (RE)"]
    Rise = 0,
    #[doc = "1: Capture Event 3 triggered on a falling edge (FE)"]
    Fall = 1,
}
impl From<Cap3pol> for bool {
    #[inline(always)]
    fn from(variant: Cap3pol) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAP3POL` reader - Capture Event 3 Polarity select"]
pub type Cap3polR = crate::BitReader<Cap3pol>;
impl Cap3polR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cap3pol {
        match self.bits {
            false => Cap3pol::Rise,
            true => Cap3pol::Fall,
        }
    }
    #[doc = "Capture Event 3 triggered on a rising edge (RE)"]
    #[inline(always)]
    pub fn is_rise(&self) -> bool {
        *self == Cap3pol::Rise
    }
    #[doc = "Capture Event 3 triggered on a falling edge (FE)"]
    #[inline(always)]
    pub fn is_fall(&self) -> bool {
        *self == Cap3pol::Fall
    }
}
#[doc = "Field `CAP3POL` writer - Capture Event 3 Polarity select"]
pub type Cap3polW<'a, REG> = crate::BitWriter<'a, REG, Cap3pol>;
impl<'a, REG> Cap3polW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Capture Event 3 triggered on a rising edge (RE)"]
    #[inline(always)]
    pub fn rise(self) -> &'a mut crate::W<REG> {
        self.variant(Cap3pol::Rise)
    }
    #[doc = "Capture Event 3 triggered on a falling edge (FE)"]
    #[inline(always)]
    pub fn fall(self) -> &'a mut crate::W<REG> {
        self.variant(Cap3pol::Fall)
    }
}
#[doc = "Counter Reset on Capture Event 3\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctrrst3 {
    #[doc = "0: Do not reset counter on Capture Event 3 (absolute time stamp)"]
    DoNotReset = 0,
    #[doc = "1: Reset counter after Event 3 time-stamp has been captured (used in difference mode operation)"]
    Reset = 1,
}
impl From<Ctrrst3> for bool {
    #[inline(always)]
    fn from(variant: Ctrrst3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTRRST3` reader - Counter Reset on Capture Event 3"]
pub type Ctrrst3R = crate::BitReader<Ctrrst3>;
impl Ctrrst3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctrrst3 {
        match self.bits {
            false => Ctrrst3::DoNotReset,
            true => Ctrrst3::Reset,
        }
    }
    #[doc = "Do not reset counter on Capture Event 3 (absolute time stamp)"]
    #[inline(always)]
    pub fn is_do_not_reset(&self) -> bool {
        *self == Ctrrst3::DoNotReset
    }
    #[doc = "Reset counter after Event 3 time-stamp has been captured (used in difference mode operation)"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == Ctrrst3::Reset
    }
}
#[doc = "Field `CTRRST3` writer - Counter Reset on Capture Event 3"]
pub type Ctrrst3W<'a, REG> = crate::BitWriter<'a, REG, Ctrrst3>;
impl<'a, REG> Ctrrst3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Do not reset counter on Capture Event 3 (absolute time stamp)"]
    #[inline(always)]
    pub fn do_not_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Ctrrst3::DoNotReset)
    }
    #[doc = "Reset counter after Event 3 time-stamp has been captured (used in difference mode operation)"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(Ctrrst3::Reset)
    }
}
#[doc = "Capture Event 4 Polarity select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cap4pol {
    #[doc = "0: Capture Event 4 triggered on a rising edge (RE)"]
    Rise = 0,
    #[doc = "1: Capture Event 4 triggered on a falling edge (FE)"]
    Fall = 1,
}
impl From<Cap4pol> for bool {
    #[inline(always)]
    fn from(variant: Cap4pol) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAP4POL` reader - Capture Event 4 Polarity select"]
pub type Cap4polR = crate::BitReader<Cap4pol>;
impl Cap4polR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cap4pol {
        match self.bits {
            false => Cap4pol::Rise,
            true => Cap4pol::Fall,
        }
    }
    #[doc = "Capture Event 4 triggered on a rising edge (RE)"]
    #[inline(always)]
    pub fn is_rise(&self) -> bool {
        *self == Cap4pol::Rise
    }
    #[doc = "Capture Event 4 triggered on a falling edge (FE)"]
    #[inline(always)]
    pub fn is_fall(&self) -> bool {
        *self == Cap4pol::Fall
    }
}
#[doc = "Field `CAP4POL` writer - Capture Event 4 Polarity select"]
pub type Cap4polW<'a, REG> = crate::BitWriter<'a, REG, Cap4pol>;
impl<'a, REG> Cap4polW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Capture Event 4 triggered on a rising edge (RE)"]
    #[inline(always)]
    pub fn rise(self) -> &'a mut crate::W<REG> {
        self.variant(Cap4pol::Rise)
    }
    #[doc = "Capture Event 4 triggered on a falling edge (FE)"]
    #[inline(always)]
    pub fn fall(self) -> &'a mut crate::W<REG> {
        self.variant(Cap4pol::Fall)
    }
}
#[doc = "Counter Reset on Capture Event 4\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctrrst4 {
    #[doc = "0: Do not reset counter on Capture Event 4 (absolute time stamp operation)"]
    DoNotReset = 0,
    #[doc = "1: Reset counter after Capture Event 4 time-stamp has been captured (used in difference mode operation)"]
    Reset = 1,
}
impl From<Ctrrst4> for bool {
    #[inline(always)]
    fn from(variant: Ctrrst4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTRRST4` reader - Counter Reset on Capture Event 4"]
pub type Ctrrst4R = crate::BitReader<Ctrrst4>;
impl Ctrrst4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctrrst4 {
        match self.bits {
            false => Ctrrst4::DoNotReset,
            true => Ctrrst4::Reset,
        }
    }
    #[doc = "Do not reset counter on Capture Event 4 (absolute time stamp operation)"]
    #[inline(always)]
    pub fn is_do_not_reset(&self) -> bool {
        *self == Ctrrst4::DoNotReset
    }
    #[doc = "Reset counter after Capture Event 4 time-stamp has been captured (used in difference mode operation)"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == Ctrrst4::Reset
    }
}
#[doc = "Field `CTRRST4` writer - Counter Reset on Capture Event 4"]
pub type Ctrrst4W<'a, REG> = crate::BitWriter<'a, REG, Ctrrst4>;
impl<'a, REG> Ctrrst4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Do not reset counter on Capture Event 4 (absolute time stamp operation)"]
    #[inline(always)]
    pub fn do_not_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Ctrrst4::DoNotReset)
    }
    #[doc = "Reset counter after Capture Event 4 time-stamp has been captured (used in difference mode operation)"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(Ctrrst4::Reset)
    }
}
#[doc = "Enable Loading CAP1-4 regs on a Cap Event\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Caplden {
    #[doc = "0: Disable CAP1-4 register loads at capture event time."]
    Disable = 0,
    #[doc = "1: Enable CAP1-4 register loads at capture event time."]
    Enable = 1,
}
impl From<Caplden> for bool {
    #[inline(always)]
    fn from(variant: Caplden) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAPLDEN` reader - Enable Loading CAP1-4 regs on a Cap Event"]
pub type CapldenR = crate::BitReader<Caplden>;
impl CapldenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Caplden {
        match self.bits {
            false => Caplden::Disable,
            true => Caplden::Enable,
        }
    }
    #[doc = "Disable CAP1-4 register loads at capture event time."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Caplden::Disable
    }
    #[doc = "Enable CAP1-4 register loads at capture event time."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Caplden::Enable
    }
}
#[doc = "Field `CAPLDEN` writer - Enable Loading CAP1-4 regs on a Cap Event"]
pub type CapldenW<'a, REG> = crate::BitWriter<'a, REG, Caplden>;
impl<'a, REG> CapldenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable CAP1-4 register loads at capture event time."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Caplden::Disable)
    }
    #[doc = "Enable CAP1-4 register loads at capture event time."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Caplden::Enable)
    }
}
#[doc = "Event Filter prescale select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Prescale {
    #[doc = "0: Divide by 1 (i.e,. no prescale, by-pass the prescaler)"]
    Div1 = 0,
    #[doc = "1: Divide by 2"]
    Div2 = 1,
    #[doc = "2: Divide by 4"]
    Div4 = 2,
    #[doc = "3: Divide by 6"]
    Div6 = 3,
    #[doc = "4: Divide by 8"]
    Div8 = 4,
    #[doc = "5: Divide by 10"]
    Div10 = 5,
    #[doc = "30: Divide by 60"]
    Div60 = 30,
    #[doc = "31: Divide by 62"]
    Div62 = 31,
}
impl From<Prescale> for u8 {
    #[inline(always)]
    fn from(variant: Prescale) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Prescale {
    type Ux = u8;
}
impl crate::IsEnum for Prescale {}
#[doc = "Field `PRESCALE` reader - Event Filter prescale select"]
pub type PrescaleR = crate::FieldReader<Prescale>;
impl PrescaleR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Prescale> {
        match self.bits {
            0 => Some(Prescale::Div1),
            1 => Some(Prescale::Div2),
            2 => Some(Prescale::Div4),
            3 => Some(Prescale::Div6),
            4 => Some(Prescale::Div8),
            5 => Some(Prescale::Div10),
            30 => Some(Prescale::Div60),
            31 => Some(Prescale::Div62),
            _ => None,
        }
    }
    #[doc = "Divide by 1 (i.e,. no prescale, by-pass the prescaler)"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == Prescale::Div1
    }
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == Prescale::Div2
    }
    #[doc = "Divide by 4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == Prescale::Div4
    }
    #[doc = "Divide by 6"]
    #[inline(always)]
    pub fn is_div6(&self) -> bool {
        *self == Prescale::Div6
    }
    #[doc = "Divide by 8"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == Prescale::Div8
    }
    #[doc = "Divide by 10"]
    #[inline(always)]
    pub fn is_div10(&self) -> bool {
        *self == Prescale::Div10
    }
    #[doc = "Divide by 60"]
    #[inline(always)]
    pub fn is_div60(&self) -> bool {
        *self == Prescale::Div60
    }
    #[doc = "Divide by 62"]
    #[inline(always)]
    pub fn is_div62(&self) -> bool {
        *self == Prescale::Div62
    }
}
#[doc = "Field `PRESCALE` writer - Event Filter prescale select"]
pub type PrescaleW<'a, REG> = crate::FieldWriter<'a, REG, 5, Prescale, crate::Safe>;
impl<'a, REG> PrescaleW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Divide by 1 (i.e,. no prescale, by-pass the prescaler)"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(Prescale::Div1)
    }
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(Prescale::Div2)
    }
    #[doc = "Divide by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(Prescale::Div4)
    }
    #[doc = "Divide by 6"]
    #[inline(always)]
    pub fn div6(self) -> &'a mut crate::W<REG> {
        self.variant(Prescale::Div6)
    }
    #[doc = "Divide by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(Prescale::Div8)
    }
    #[doc = "Divide by 10"]
    #[inline(always)]
    pub fn div10(self) -> &'a mut crate::W<REG> {
        self.variant(Prescale::Div10)
    }
    #[doc = "Divide by 60"]
    #[inline(always)]
    pub fn div60(self) -> &'a mut crate::W<REG> {
        self.variant(Prescale::Div60)
    }
    #[doc = "Divide by 62"]
    #[inline(always)]
    pub fn div62(self) -> &'a mut crate::W<REG> {
        self.variant(Prescale::Div62)
    }
}
#[doc = "Emulation mode\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FreeSoft {
    #[doc = "0: TSCTR counter stops immediately on emulation suspend"]
    StopEmu = 0,
    #[doc = "1: TSCTR counter runs until = 0"]
    RunsUntil = 1,
    #[doc = "2: TSCTR counter is unaffected by emulation suspend (Run Free)"]
    UnafEmuSus = 2,
    #[doc = "3: TSCTR counter is unaffected by emulation suspend (Run Free)"]
    UnafEmuSus2 = 3,
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
            0 => FreeSoft::StopEmu,
            1 => FreeSoft::RunsUntil,
            2 => FreeSoft::UnafEmuSus,
            3 => FreeSoft::UnafEmuSus2,
            _ => unreachable!(),
        }
    }
    #[doc = "TSCTR counter stops immediately on emulation suspend"]
    #[inline(always)]
    pub fn is_stop_emu(&self) -> bool {
        *self == FreeSoft::StopEmu
    }
    #[doc = "TSCTR counter runs until = 0"]
    #[inline(always)]
    pub fn is_runs_until(&self) -> bool {
        *self == FreeSoft::RunsUntil
    }
    #[doc = "TSCTR counter is unaffected by emulation suspend (Run Free)"]
    #[inline(always)]
    pub fn is_unaf_emu_sus(&self) -> bool {
        *self == FreeSoft::UnafEmuSus
    }
    #[doc = "TSCTR counter is unaffected by emulation suspend (Run Free)"]
    #[inline(always)]
    pub fn is_unaf_emu_sus2(&self) -> bool {
        *self == FreeSoft::UnafEmuSus2
    }
}
#[doc = "Field `FREE_SOFT` writer - Emulation mode"]
pub type FreeSoftW<'a, REG> = crate::FieldWriter<'a, REG, 2, FreeSoft, crate::Safe>;
impl<'a, REG> FreeSoftW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "TSCTR counter stops immediately on emulation suspend"]
    #[inline(always)]
    pub fn stop_emu(self) -> &'a mut crate::W<REG> {
        self.variant(FreeSoft::StopEmu)
    }
    #[doc = "TSCTR counter runs until = 0"]
    #[inline(always)]
    pub fn runs_until(self) -> &'a mut crate::W<REG> {
        self.variant(FreeSoft::RunsUntil)
    }
    #[doc = "TSCTR counter is unaffected by emulation suspend (Run Free)"]
    #[inline(always)]
    pub fn unaf_emu_sus(self) -> &'a mut crate::W<REG> {
        self.variant(FreeSoft::UnafEmuSus)
    }
    #[doc = "TSCTR counter is unaffected by emulation suspend (Run Free)"]
    #[inline(always)]
    pub fn unaf_emu_sus2(self) -> &'a mut crate::W<REG> {
        self.variant(FreeSoft::UnafEmuSus2)
    }
}
impl R {
    #[doc = "Bit 0 - Capture Event 1 Polarity select"]
    #[inline(always)]
    pub fn cap1pol(&self) -> Cap1polR {
        Cap1polR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Counter Reset on Capture Event 1"]
    #[inline(always)]
    pub fn ctrrst1(&self) -> Ctrrst1R {
        Ctrrst1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Capture Event 2 Polarity select"]
    #[inline(always)]
    pub fn cap2pol(&self) -> Cap2polR {
        Cap2polR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Counter Reset on Capture Event 2"]
    #[inline(always)]
    pub fn ctrrst2(&self) -> Ctrrst2R {
        Ctrrst2R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Capture Event 3 Polarity select"]
    #[inline(always)]
    pub fn cap3pol(&self) -> Cap3polR {
        Cap3polR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Counter Reset on Capture Event 3"]
    #[inline(always)]
    pub fn ctrrst3(&self) -> Ctrrst3R {
        Ctrrst3R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Capture Event 4 Polarity select"]
    #[inline(always)]
    pub fn cap4pol(&self) -> Cap4polR {
        Cap4polR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Counter Reset on Capture Event 4"]
    #[inline(always)]
    pub fn ctrrst4(&self) -> Ctrrst4R {
        Ctrrst4R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable Loading CAP1-4 regs on a Cap Event"]
    #[inline(always)]
    pub fn caplden(&self) -> CapldenR {
        CapldenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:13 - Event Filter prescale select"]
    #[inline(always)]
    pub fn prescale(&self) -> PrescaleR {
        PrescaleR::new(((self.bits >> 9) & 0x1f) as u8)
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
        f.debug_struct("ECCTL1")
            .field("cap1pol", &self.cap1pol())
            .field("ctrrst1", &self.ctrrst1())
            .field("cap2pol", &self.cap2pol())
            .field("ctrrst2", &self.ctrrst2())
            .field("cap3pol", &self.cap3pol())
            .field("ctrrst3", &self.ctrrst3())
            .field("cap4pol", &self.cap4pol())
            .field("ctrrst4", &self.ctrrst4())
            .field("caplden", &self.caplden())
            .field("prescale", &self.prescale())
            .field("free_soft", &self.free_soft())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Capture Event 1 Polarity select"]
    #[inline(always)]
    pub fn cap1pol(&mut self) -> Cap1polW<'_, Ecctl1Spec> {
        Cap1polW::new(self, 0)
    }
    #[doc = "Bit 1 - Counter Reset on Capture Event 1"]
    #[inline(always)]
    pub fn ctrrst1(&mut self) -> Ctrrst1W<'_, Ecctl1Spec> {
        Ctrrst1W::new(self, 1)
    }
    #[doc = "Bit 2 - Capture Event 2 Polarity select"]
    #[inline(always)]
    pub fn cap2pol(&mut self) -> Cap2polW<'_, Ecctl1Spec> {
        Cap2polW::new(self, 2)
    }
    #[doc = "Bit 3 - Counter Reset on Capture Event 2"]
    #[inline(always)]
    pub fn ctrrst2(&mut self) -> Ctrrst2W<'_, Ecctl1Spec> {
        Ctrrst2W::new(self, 3)
    }
    #[doc = "Bit 4 - Capture Event 3 Polarity select"]
    #[inline(always)]
    pub fn cap3pol(&mut self) -> Cap3polW<'_, Ecctl1Spec> {
        Cap3polW::new(self, 4)
    }
    #[doc = "Bit 5 - Counter Reset on Capture Event 3"]
    #[inline(always)]
    pub fn ctrrst3(&mut self) -> Ctrrst3W<'_, Ecctl1Spec> {
        Ctrrst3W::new(self, 5)
    }
    #[doc = "Bit 6 - Capture Event 4 Polarity select"]
    #[inline(always)]
    pub fn cap4pol(&mut self) -> Cap4polW<'_, Ecctl1Spec> {
        Cap4polW::new(self, 6)
    }
    #[doc = "Bit 7 - Counter Reset on Capture Event 4"]
    #[inline(always)]
    pub fn ctrrst4(&mut self) -> Ctrrst4W<'_, Ecctl1Spec> {
        Ctrrst4W::new(self, 7)
    }
    #[doc = "Bit 8 - Enable Loading CAP1-4 regs on a Cap Event"]
    #[inline(always)]
    pub fn caplden(&mut self) -> CapldenW<'_, Ecctl1Spec> {
        CapldenW::new(self, 8)
    }
    #[doc = "Bits 9:13 - Event Filter prescale select"]
    #[inline(always)]
    pub fn prescale(&mut self) -> PrescaleW<'_, Ecctl1Spec> {
        PrescaleW::new(self, 9)
    }
    #[doc = "Bits 14:15 - Emulation mode"]
    #[inline(always)]
    pub fn free_soft(&mut self) -> FreeSoftW<'_, Ecctl1Spec> {
        FreeSoftW::new(self, 14)
    }
}
#[doc = "Capture Control Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ecctl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ecctl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ecctl1Spec;
impl crate::RegisterSpec for Ecctl1Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ecctl1::R`](R) reader structure"]
impl crate::Readable for Ecctl1Spec {}
#[doc = "`write(|w| ..)` method takes [`ecctl1::W`](W) writer structure"]
impl crate::Writable for Ecctl1Spec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets ECCTL1 to value 0"]
impl crate::Resettable for Ecctl1Spec {}
