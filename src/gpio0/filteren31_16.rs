#[doc = "Register `FILTEREN31_16` reader"]
pub type R = crate::R<Filteren31_16Spec>;
#[doc = "Register `FILTEREN31_16` writer"]
pub type W = crate::W<Filteren31_16Spec>;
#[doc = "Programmable counter length of digital glitch filter for DIN16\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Din16 {
    #[doc = "0: No additional filter beyond the CDC synchronization sample"]
    Disable = 0,
    #[doc = "1: 1 ULPCLK minimum sample"]
    OneCycle = 1,
    #[doc = "2: 3 ULPCLK minimum sample"]
    ThreeCycle = 2,
    #[doc = "3: 8 ULPCLK minimum sample"]
    EightCycle = 3,
}
impl From<Din16> for u8 {
    #[inline(always)]
    fn from(variant: Din16) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Din16 {
    type Ux = u8;
}
impl crate::IsEnum for Din16 {}
#[doc = "Field `DIN16` reader - Programmable counter length of digital glitch filter for DIN16"]
pub type Din16R = crate::FieldReader<Din16>;
impl Din16R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Din16 {
        match self.bits {
            0 => Din16::Disable,
            1 => Din16::OneCycle,
            2 => Din16::ThreeCycle,
            3 => Din16::EightCycle,
            _ => unreachable!(),
        }
    }
    #[doc = "No additional filter beyond the CDC synchronization sample"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Din16::Disable
    }
    #[doc = "1 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn is_one_cycle(&self) -> bool {
        *self == Din16::OneCycle
    }
    #[doc = "3 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn is_three_cycle(&self) -> bool {
        *self == Din16::ThreeCycle
    }
    #[doc = "8 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn is_eight_cycle(&self) -> bool {
        *self == Din16::EightCycle
    }
}
#[doc = "Field `DIN16` writer - Programmable counter length of digital glitch filter for DIN16"]
pub type Din16W<'a, REG> = crate::FieldWriter<'a, REG, 2, Din16, crate::Safe>;
impl<'a, REG> Din16W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No additional filter beyond the CDC synchronization sample"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Din16::Disable)
    }
    #[doc = "1 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn one_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Din16::OneCycle)
    }
    #[doc = "3 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn three_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Din16::ThreeCycle)
    }
    #[doc = "8 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn eight_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Din16::EightCycle)
    }
}
#[doc = "Programmable counter length of digital glitch filter for DIN17\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Din17 {
    #[doc = "0: No additional filter beyond the CDC synchronization sample"]
    Disable = 0,
    #[doc = "1: 1 ULPCLK minimum sample"]
    OneCycle = 1,
    #[doc = "2: 3 ULPCLK minimum sample"]
    ThreeCycle = 2,
    #[doc = "3: 8 ULPCLK minimum sample"]
    EightCycle = 3,
}
impl From<Din17> for u8 {
    #[inline(always)]
    fn from(variant: Din17) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Din17 {
    type Ux = u8;
}
impl crate::IsEnum for Din17 {}
#[doc = "Field `DIN17` reader - Programmable counter length of digital glitch filter for DIN17"]
pub type Din17R = crate::FieldReader<Din17>;
impl Din17R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Din17 {
        match self.bits {
            0 => Din17::Disable,
            1 => Din17::OneCycle,
            2 => Din17::ThreeCycle,
            3 => Din17::EightCycle,
            _ => unreachable!(),
        }
    }
    #[doc = "No additional filter beyond the CDC synchronization sample"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Din17::Disable
    }
    #[doc = "1 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn is_one_cycle(&self) -> bool {
        *self == Din17::OneCycle
    }
    #[doc = "3 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn is_three_cycle(&self) -> bool {
        *self == Din17::ThreeCycle
    }
    #[doc = "8 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn is_eight_cycle(&self) -> bool {
        *self == Din17::EightCycle
    }
}
#[doc = "Field `DIN17` writer - Programmable counter length of digital glitch filter for DIN17"]
pub type Din17W<'a, REG> = crate::FieldWriter<'a, REG, 2, Din17, crate::Safe>;
impl<'a, REG> Din17W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No additional filter beyond the CDC synchronization sample"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Din17::Disable)
    }
    #[doc = "1 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn one_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Din17::OneCycle)
    }
    #[doc = "3 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn three_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Din17::ThreeCycle)
    }
    #[doc = "8 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn eight_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Din17::EightCycle)
    }
}
#[doc = "Programmable counter length of digital glitch filter for DIN18\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Din18 {
    #[doc = "0: No additional filter beyond the CDC synchronization sample"]
    Disable = 0,
    #[doc = "1: 1 ULPCLK minimum sample"]
    OneCycle = 1,
    #[doc = "2: 3 ULPCLK minimum sample"]
    ThreeCycle = 2,
    #[doc = "3: 8 ULPCLK minimum sample"]
    EightCycle = 3,
}
impl From<Din18> for u8 {
    #[inline(always)]
    fn from(variant: Din18) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Din18 {
    type Ux = u8;
}
impl crate::IsEnum for Din18 {}
#[doc = "Field `DIN18` reader - Programmable counter length of digital glitch filter for DIN18"]
pub type Din18R = crate::FieldReader<Din18>;
impl Din18R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Din18 {
        match self.bits {
            0 => Din18::Disable,
            1 => Din18::OneCycle,
            2 => Din18::ThreeCycle,
            3 => Din18::EightCycle,
            _ => unreachable!(),
        }
    }
    #[doc = "No additional filter beyond the CDC synchronization sample"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Din18::Disable
    }
    #[doc = "1 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn is_one_cycle(&self) -> bool {
        *self == Din18::OneCycle
    }
    #[doc = "3 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn is_three_cycle(&self) -> bool {
        *self == Din18::ThreeCycle
    }
    #[doc = "8 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn is_eight_cycle(&self) -> bool {
        *self == Din18::EightCycle
    }
}
#[doc = "Field `DIN18` writer - Programmable counter length of digital glitch filter for DIN18"]
pub type Din18W<'a, REG> = crate::FieldWriter<'a, REG, 2, Din18, crate::Safe>;
impl<'a, REG> Din18W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No additional filter beyond the CDC synchronization sample"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Din18::Disable)
    }
    #[doc = "1 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn one_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Din18::OneCycle)
    }
    #[doc = "3 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn three_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Din18::ThreeCycle)
    }
    #[doc = "8 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn eight_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Din18::EightCycle)
    }
}
#[doc = "Programmable counter length of digital glitch filter for DIN19\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Din19 {
    #[doc = "0: No additional filter beyond the CDC synchronization sample"]
    Disable = 0,
    #[doc = "1: 1 ULPCLK minimum sample"]
    OneCycle = 1,
    #[doc = "2: 3 ULPCLK minimum sample"]
    ThreeCycle = 2,
    #[doc = "3: 8 ULPCLK minimum sample"]
    EightCycle = 3,
}
impl From<Din19> for u8 {
    #[inline(always)]
    fn from(variant: Din19) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Din19 {
    type Ux = u8;
}
impl crate::IsEnum for Din19 {}
#[doc = "Field `DIN19` reader - Programmable counter length of digital glitch filter for DIN19"]
pub type Din19R = crate::FieldReader<Din19>;
impl Din19R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Din19 {
        match self.bits {
            0 => Din19::Disable,
            1 => Din19::OneCycle,
            2 => Din19::ThreeCycle,
            3 => Din19::EightCycle,
            _ => unreachable!(),
        }
    }
    #[doc = "No additional filter beyond the CDC synchronization sample"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Din19::Disable
    }
    #[doc = "1 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn is_one_cycle(&self) -> bool {
        *self == Din19::OneCycle
    }
    #[doc = "3 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn is_three_cycle(&self) -> bool {
        *self == Din19::ThreeCycle
    }
    #[doc = "8 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn is_eight_cycle(&self) -> bool {
        *self == Din19::EightCycle
    }
}
#[doc = "Field `DIN19` writer - Programmable counter length of digital glitch filter for DIN19"]
pub type Din19W<'a, REG> = crate::FieldWriter<'a, REG, 2, Din19, crate::Safe>;
impl<'a, REG> Din19W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No additional filter beyond the CDC synchronization sample"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Din19::Disable)
    }
    #[doc = "1 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn one_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Din19::OneCycle)
    }
    #[doc = "3 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn three_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Din19::ThreeCycle)
    }
    #[doc = "8 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn eight_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Din19::EightCycle)
    }
}
#[doc = "Programmable counter length of digital glitch filter for DIN20\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Din20 {
    #[doc = "0: No additional filter beyond the CDC synchronization sample"]
    Disable = 0,
    #[doc = "1: 1 ULPCLK minimum sample"]
    OneCycle = 1,
    #[doc = "2: 3 ULPCLK minimum sample"]
    ThreeCycle = 2,
    #[doc = "3: 8 ULPCLK minimum sample"]
    EightCycle = 3,
}
impl From<Din20> for u8 {
    #[inline(always)]
    fn from(variant: Din20) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Din20 {
    type Ux = u8;
}
impl crate::IsEnum for Din20 {}
#[doc = "Field `DIN20` reader - Programmable counter length of digital glitch filter for DIN20"]
pub type Din20R = crate::FieldReader<Din20>;
impl Din20R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Din20 {
        match self.bits {
            0 => Din20::Disable,
            1 => Din20::OneCycle,
            2 => Din20::ThreeCycle,
            3 => Din20::EightCycle,
            _ => unreachable!(),
        }
    }
    #[doc = "No additional filter beyond the CDC synchronization sample"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Din20::Disable
    }
    #[doc = "1 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn is_one_cycle(&self) -> bool {
        *self == Din20::OneCycle
    }
    #[doc = "3 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn is_three_cycle(&self) -> bool {
        *self == Din20::ThreeCycle
    }
    #[doc = "8 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn is_eight_cycle(&self) -> bool {
        *self == Din20::EightCycle
    }
}
#[doc = "Field `DIN20` writer - Programmable counter length of digital glitch filter for DIN20"]
pub type Din20W<'a, REG> = crate::FieldWriter<'a, REG, 2, Din20, crate::Safe>;
impl<'a, REG> Din20W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No additional filter beyond the CDC synchronization sample"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Din20::Disable)
    }
    #[doc = "1 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn one_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Din20::OneCycle)
    }
    #[doc = "3 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn three_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Din20::ThreeCycle)
    }
    #[doc = "8 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn eight_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Din20::EightCycle)
    }
}
#[doc = "Programmable counter length of digital glitch filter for DIN21\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Din21 {
    #[doc = "0: No additional filter beyond the CDC synchronization sample"]
    Disable = 0,
    #[doc = "1: 1 ULPCLK minimum sample"]
    OneCycle = 1,
    #[doc = "2: 3 ULPCLK minimum sample"]
    ThreeCycle = 2,
    #[doc = "3: 8 ULPCLK minimum sample"]
    EightCycle = 3,
}
impl From<Din21> for u8 {
    #[inline(always)]
    fn from(variant: Din21) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Din21 {
    type Ux = u8;
}
impl crate::IsEnum for Din21 {}
#[doc = "Field `DIN21` reader - Programmable counter length of digital glitch filter for DIN21"]
pub type Din21R = crate::FieldReader<Din21>;
impl Din21R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Din21 {
        match self.bits {
            0 => Din21::Disable,
            1 => Din21::OneCycle,
            2 => Din21::ThreeCycle,
            3 => Din21::EightCycle,
            _ => unreachable!(),
        }
    }
    #[doc = "No additional filter beyond the CDC synchronization sample"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Din21::Disable
    }
    #[doc = "1 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn is_one_cycle(&self) -> bool {
        *self == Din21::OneCycle
    }
    #[doc = "3 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn is_three_cycle(&self) -> bool {
        *self == Din21::ThreeCycle
    }
    #[doc = "8 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn is_eight_cycle(&self) -> bool {
        *self == Din21::EightCycle
    }
}
#[doc = "Field `DIN21` writer - Programmable counter length of digital glitch filter for DIN21"]
pub type Din21W<'a, REG> = crate::FieldWriter<'a, REG, 2, Din21, crate::Safe>;
impl<'a, REG> Din21W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No additional filter beyond the CDC synchronization sample"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Din21::Disable)
    }
    #[doc = "1 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn one_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Din21::OneCycle)
    }
    #[doc = "3 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn three_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Din21::ThreeCycle)
    }
    #[doc = "8 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn eight_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Din21::EightCycle)
    }
}
#[doc = "Programmable counter length of digital glitch filter for DIN22\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Din22 {
    #[doc = "0: No additional filter beyond the CDC synchronization sample"]
    Disable = 0,
    #[doc = "1: 1 ULPCLK minimum sample"]
    OneCycle = 1,
    #[doc = "2: 3 ULPCLK minimum sample"]
    ThreeCycle = 2,
    #[doc = "3: 8 ULPCLK minimum sample"]
    EightCycle = 3,
}
impl From<Din22> for u8 {
    #[inline(always)]
    fn from(variant: Din22) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Din22 {
    type Ux = u8;
}
impl crate::IsEnum for Din22 {}
#[doc = "Field `DIN22` reader - Programmable counter length of digital glitch filter for DIN22"]
pub type Din22R = crate::FieldReader<Din22>;
impl Din22R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Din22 {
        match self.bits {
            0 => Din22::Disable,
            1 => Din22::OneCycle,
            2 => Din22::ThreeCycle,
            3 => Din22::EightCycle,
            _ => unreachable!(),
        }
    }
    #[doc = "No additional filter beyond the CDC synchronization sample"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Din22::Disable
    }
    #[doc = "1 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn is_one_cycle(&self) -> bool {
        *self == Din22::OneCycle
    }
    #[doc = "3 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn is_three_cycle(&self) -> bool {
        *self == Din22::ThreeCycle
    }
    #[doc = "8 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn is_eight_cycle(&self) -> bool {
        *self == Din22::EightCycle
    }
}
#[doc = "Field `DIN22` writer - Programmable counter length of digital glitch filter for DIN22"]
pub type Din22W<'a, REG> = crate::FieldWriter<'a, REG, 2, Din22, crate::Safe>;
impl<'a, REG> Din22W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No additional filter beyond the CDC synchronization sample"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Din22::Disable)
    }
    #[doc = "1 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn one_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Din22::OneCycle)
    }
    #[doc = "3 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn three_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Din22::ThreeCycle)
    }
    #[doc = "8 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn eight_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Din22::EightCycle)
    }
}
#[doc = "Programmable counter length of digital glitch filter for DIN23\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Din23 {
    #[doc = "0: No additional filter beyond the CDC synchronization sample"]
    Disable = 0,
    #[doc = "1: 1 ULPCLK minimum sample"]
    OneCycle = 1,
    #[doc = "2: 3 ULPCLK minimum sample"]
    ThreeCycle = 2,
    #[doc = "3: 8 ULPCLK minimum sample"]
    EightCycle = 3,
}
impl From<Din23> for u8 {
    #[inline(always)]
    fn from(variant: Din23) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Din23 {
    type Ux = u8;
}
impl crate::IsEnum for Din23 {}
#[doc = "Field `DIN23` reader - Programmable counter length of digital glitch filter for DIN23"]
pub type Din23R = crate::FieldReader<Din23>;
impl Din23R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Din23 {
        match self.bits {
            0 => Din23::Disable,
            1 => Din23::OneCycle,
            2 => Din23::ThreeCycle,
            3 => Din23::EightCycle,
            _ => unreachable!(),
        }
    }
    #[doc = "No additional filter beyond the CDC synchronization sample"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Din23::Disable
    }
    #[doc = "1 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn is_one_cycle(&self) -> bool {
        *self == Din23::OneCycle
    }
    #[doc = "3 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn is_three_cycle(&self) -> bool {
        *self == Din23::ThreeCycle
    }
    #[doc = "8 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn is_eight_cycle(&self) -> bool {
        *self == Din23::EightCycle
    }
}
#[doc = "Field `DIN23` writer - Programmable counter length of digital glitch filter for DIN23"]
pub type Din23W<'a, REG> = crate::FieldWriter<'a, REG, 2, Din23, crate::Safe>;
impl<'a, REG> Din23W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No additional filter beyond the CDC synchronization sample"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Din23::Disable)
    }
    #[doc = "1 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn one_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Din23::OneCycle)
    }
    #[doc = "3 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn three_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Din23::ThreeCycle)
    }
    #[doc = "8 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn eight_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Din23::EightCycle)
    }
}
#[doc = "Programmable counter length of digital glitch filter for DIN24\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Din24 {
    #[doc = "0: No additional filter beyond the CDC synchronization sample"]
    Disable = 0,
    #[doc = "1: 1 ULPCLK minimum sample"]
    OneCycle = 1,
    #[doc = "2: 3 ULPCLK minimum sample"]
    ThreeCycle = 2,
    #[doc = "3: 8 ULPCLK minimum sample"]
    EightCycle = 3,
}
impl From<Din24> for u8 {
    #[inline(always)]
    fn from(variant: Din24) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Din24 {
    type Ux = u8;
}
impl crate::IsEnum for Din24 {}
#[doc = "Field `DIN24` reader - Programmable counter length of digital glitch filter for DIN24"]
pub type Din24R = crate::FieldReader<Din24>;
impl Din24R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Din24 {
        match self.bits {
            0 => Din24::Disable,
            1 => Din24::OneCycle,
            2 => Din24::ThreeCycle,
            3 => Din24::EightCycle,
            _ => unreachable!(),
        }
    }
    #[doc = "No additional filter beyond the CDC synchronization sample"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Din24::Disable
    }
    #[doc = "1 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn is_one_cycle(&self) -> bool {
        *self == Din24::OneCycle
    }
    #[doc = "3 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn is_three_cycle(&self) -> bool {
        *self == Din24::ThreeCycle
    }
    #[doc = "8 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn is_eight_cycle(&self) -> bool {
        *self == Din24::EightCycle
    }
}
#[doc = "Field `DIN24` writer - Programmable counter length of digital glitch filter for DIN24"]
pub type Din24W<'a, REG> = crate::FieldWriter<'a, REG, 2, Din24, crate::Safe>;
impl<'a, REG> Din24W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No additional filter beyond the CDC synchronization sample"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Din24::Disable)
    }
    #[doc = "1 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn one_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Din24::OneCycle)
    }
    #[doc = "3 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn three_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Din24::ThreeCycle)
    }
    #[doc = "8 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn eight_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Din24::EightCycle)
    }
}
#[doc = "Programmable counter length of digital glitch filter for DIN25\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Din25 {
    #[doc = "0: No additional filter beyond the CDC synchronization sample"]
    Disable = 0,
    #[doc = "1: 1 ULPCLK minimum sample"]
    OneCycle = 1,
    #[doc = "2: 3 ULPCLK minimum sample"]
    ThreeCycle = 2,
    #[doc = "3: 8 ULPCLK minimum sample"]
    EightCycle = 3,
}
impl From<Din25> for u8 {
    #[inline(always)]
    fn from(variant: Din25) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Din25 {
    type Ux = u8;
}
impl crate::IsEnum for Din25 {}
#[doc = "Field `DIN25` reader - Programmable counter length of digital glitch filter for DIN25"]
pub type Din25R = crate::FieldReader<Din25>;
impl Din25R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Din25 {
        match self.bits {
            0 => Din25::Disable,
            1 => Din25::OneCycle,
            2 => Din25::ThreeCycle,
            3 => Din25::EightCycle,
            _ => unreachable!(),
        }
    }
    #[doc = "No additional filter beyond the CDC synchronization sample"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Din25::Disable
    }
    #[doc = "1 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn is_one_cycle(&self) -> bool {
        *self == Din25::OneCycle
    }
    #[doc = "3 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn is_three_cycle(&self) -> bool {
        *self == Din25::ThreeCycle
    }
    #[doc = "8 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn is_eight_cycle(&self) -> bool {
        *self == Din25::EightCycle
    }
}
#[doc = "Field `DIN25` writer - Programmable counter length of digital glitch filter for DIN25"]
pub type Din25W<'a, REG> = crate::FieldWriter<'a, REG, 2, Din25, crate::Safe>;
impl<'a, REG> Din25W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No additional filter beyond the CDC synchronization sample"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Din25::Disable)
    }
    #[doc = "1 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn one_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Din25::OneCycle)
    }
    #[doc = "3 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn three_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Din25::ThreeCycle)
    }
    #[doc = "8 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn eight_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Din25::EightCycle)
    }
}
#[doc = "Programmable counter length of digital glitch filter for DIN26\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Din26 {
    #[doc = "0: No additional filter beyond the CDC synchronization sample"]
    Disable = 0,
    #[doc = "1: 1 ULPCLK minimum sample"]
    OneCycle = 1,
    #[doc = "2: 3 ULPCLK minimum sample"]
    ThreeCycle = 2,
    #[doc = "3: 8 ULPCLK minimum sample"]
    EightCycle = 3,
}
impl From<Din26> for u8 {
    #[inline(always)]
    fn from(variant: Din26) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Din26 {
    type Ux = u8;
}
impl crate::IsEnum for Din26 {}
#[doc = "Field `DIN26` reader - Programmable counter length of digital glitch filter for DIN26"]
pub type Din26R = crate::FieldReader<Din26>;
impl Din26R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Din26 {
        match self.bits {
            0 => Din26::Disable,
            1 => Din26::OneCycle,
            2 => Din26::ThreeCycle,
            3 => Din26::EightCycle,
            _ => unreachable!(),
        }
    }
    #[doc = "No additional filter beyond the CDC synchronization sample"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Din26::Disable
    }
    #[doc = "1 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn is_one_cycle(&self) -> bool {
        *self == Din26::OneCycle
    }
    #[doc = "3 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn is_three_cycle(&self) -> bool {
        *self == Din26::ThreeCycle
    }
    #[doc = "8 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn is_eight_cycle(&self) -> bool {
        *self == Din26::EightCycle
    }
}
#[doc = "Field `DIN26` writer - Programmable counter length of digital glitch filter for DIN26"]
pub type Din26W<'a, REG> = crate::FieldWriter<'a, REG, 2, Din26, crate::Safe>;
impl<'a, REG> Din26W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No additional filter beyond the CDC synchronization sample"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Din26::Disable)
    }
    #[doc = "1 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn one_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Din26::OneCycle)
    }
    #[doc = "3 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn three_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Din26::ThreeCycle)
    }
    #[doc = "8 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn eight_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Din26::EightCycle)
    }
}
#[doc = "Programmable counter length of digital glitch filter for DIN27\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Din27 {
    #[doc = "0: No additional filter beyond the CDC synchronization sample"]
    Disable = 0,
    #[doc = "1: 1 ULPCLK minimum sample"]
    OneCycle = 1,
    #[doc = "2: 3 ULPCLK minimum sample"]
    ThreeCycle = 2,
    #[doc = "3: 8 ULPCLK minimum sample"]
    EightCycle = 3,
}
impl From<Din27> for u8 {
    #[inline(always)]
    fn from(variant: Din27) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Din27 {
    type Ux = u8;
}
impl crate::IsEnum for Din27 {}
#[doc = "Field `DIN27` reader - Programmable counter length of digital glitch filter for DIN27"]
pub type Din27R = crate::FieldReader<Din27>;
impl Din27R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Din27 {
        match self.bits {
            0 => Din27::Disable,
            1 => Din27::OneCycle,
            2 => Din27::ThreeCycle,
            3 => Din27::EightCycle,
            _ => unreachable!(),
        }
    }
    #[doc = "No additional filter beyond the CDC synchronization sample"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Din27::Disable
    }
    #[doc = "1 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn is_one_cycle(&self) -> bool {
        *self == Din27::OneCycle
    }
    #[doc = "3 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn is_three_cycle(&self) -> bool {
        *self == Din27::ThreeCycle
    }
    #[doc = "8 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn is_eight_cycle(&self) -> bool {
        *self == Din27::EightCycle
    }
}
#[doc = "Field `DIN27` writer - Programmable counter length of digital glitch filter for DIN27"]
pub type Din27W<'a, REG> = crate::FieldWriter<'a, REG, 2, Din27, crate::Safe>;
impl<'a, REG> Din27W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No additional filter beyond the CDC synchronization sample"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Din27::Disable)
    }
    #[doc = "1 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn one_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Din27::OneCycle)
    }
    #[doc = "3 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn three_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Din27::ThreeCycle)
    }
    #[doc = "8 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn eight_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Din27::EightCycle)
    }
}
#[doc = "Programmable counter length of digital glitch filter for DIN28\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Din28 {
    #[doc = "0: No additional filter beyond the CDC synchronization sample"]
    Disable = 0,
    #[doc = "1: 1 ULPCLK minimum sample"]
    OneCycle = 1,
    #[doc = "2: 3 ULPCLK minimum sample"]
    ThreeCycle = 2,
    #[doc = "3: 8 ULPCLK minimum sample"]
    EightCycle = 3,
}
impl From<Din28> for u8 {
    #[inline(always)]
    fn from(variant: Din28) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Din28 {
    type Ux = u8;
}
impl crate::IsEnum for Din28 {}
#[doc = "Field `DIN28` reader - Programmable counter length of digital glitch filter for DIN28"]
pub type Din28R = crate::FieldReader<Din28>;
impl Din28R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Din28 {
        match self.bits {
            0 => Din28::Disable,
            1 => Din28::OneCycle,
            2 => Din28::ThreeCycle,
            3 => Din28::EightCycle,
            _ => unreachable!(),
        }
    }
    #[doc = "No additional filter beyond the CDC synchronization sample"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Din28::Disable
    }
    #[doc = "1 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn is_one_cycle(&self) -> bool {
        *self == Din28::OneCycle
    }
    #[doc = "3 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn is_three_cycle(&self) -> bool {
        *self == Din28::ThreeCycle
    }
    #[doc = "8 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn is_eight_cycle(&self) -> bool {
        *self == Din28::EightCycle
    }
}
#[doc = "Field `DIN28` writer - Programmable counter length of digital glitch filter for DIN28"]
pub type Din28W<'a, REG> = crate::FieldWriter<'a, REG, 2, Din28, crate::Safe>;
impl<'a, REG> Din28W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No additional filter beyond the CDC synchronization sample"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Din28::Disable)
    }
    #[doc = "1 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn one_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Din28::OneCycle)
    }
    #[doc = "3 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn three_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Din28::ThreeCycle)
    }
    #[doc = "8 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn eight_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Din28::EightCycle)
    }
}
#[doc = "Programmable counter length of digital glitch filter for DIN29\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Din29 {
    #[doc = "0: No additional filter beyond the CDC synchronization sample"]
    Disable = 0,
    #[doc = "1: 1 ULPCLK minimum sample"]
    OneCycle = 1,
    #[doc = "2: 3 ULPCLK minimum sample"]
    ThreeCycle = 2,
    #[doc = "3: 8 ULPCLK minimum sample"]
    EightCycle = 3,
}
impl From<Din29> for u8 {
    #[inline(always)]
    fn from(variant: Din29) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Din29 {
    type Ux = u8;
}
impl crate::IsEnum for Din29 {}
#[doc = "Field `DIN29` reader - Programmable counter length of digital glitch filter for DIN29"]
pub type Din29R = crate::FieldReader<Din29>;
impl Din29R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Din29 {
        match self.bits {
            0 => Din29::Disable,
            1 => Din29::OneCycle,
            2 => Din29::ThreeCycle,
            3 => Din29::EightCycle,
            _ => unreachable!(),
        }
    }
    #[doc = "No additional filter beyond the CDC synchronization sample"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Din29::Disable
    }
    #[doc = "1 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn is_one_cycle(&self) -> bool {
        *self == Din29::OneCycle
    }
    #[doc = "3 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn is_three_cycle(&self) -> bool {
        *self == Din29::ThreeCycle
    }
    #[doc = "8 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn is_eight_cycle(&self) -> bool {
        *self == Din29::EightCycle
    }
}
#[doc = "Field `DIN29` writer - Programmable counter length of digital glitch filter for DIN29"]
pub type Din29W<'a, REG> = crate::FieldWriter<'a, REG, 2, Din29, crate::Safe>;
impl<'a, REG> Din29W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No additional filter beyond the CDC synchronization sample"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Din29::Disable)
    }
    #[doc = "1 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn one_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Din29::OneCycle)
    }
    #[doc = "3 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn three_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Din29::ThreeCycle)
    }
    #[doc = "8 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn eight_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Din29::EightCycle)
    }
}
#[doc = "Programmable counter length of digital glitch filter for DIN30\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Din30 {
    #[doc = "0: No additional filter beyond the CDC synchronization sample"]
    Disable = 0,
    #[doc = "1: 1 ULPCLK minimum sample"]
    OneCycle = 1,
    #[doc = "2: 3 ULPCLK minimum sample"]
    ThreeCycle = 2,
    #[doc = "3: 8 ULPCLK minimum sample"]
    EightCycle = 3,
}
impl From<Din30> for u8 {
    #[inline(always)]
    fn from(variant: Din30) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Din30 {
    type Ux = u8;
}
impl crate::IsEnum for Din30 {}
#[doc = "Field `DIN30` reader - Programmable counter length of digital glitch filter for DIN30"]
pub type Din30R = crate::FieldReader<Din30>;
impl Din30R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Din30 {
        match self.bits {
            0 => Din30::Disable,
            1 => Din30::OneCycle,
            2 => Din30::ThreeCycle,
            3 => Din30::EightCycle,
            _ => unreachable!(),
        }
    }
    #[doc = "No additional filter beyond the CDC synchronization sample"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Din30::Disable
    }
    #[doc = "1 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn is_one_cycle(&self) -> bool {
        *self == Din30::OneCycle
    }
    #[doc = "3 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn is_three_cycle(&self) -> bool {
        *self == Din30::ThreeCycle
    }
    #[doc = "8 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn is_eight_cycle(&self) -> bool {
        *self == Din30::EightCycle
    }
}
#[doc = "Field `DIN30` writer - Programmable counter length of digital glitch filter for DIN30"]
pub type Din30W<'a, REG> = crate::FieldWriter<'a, REG, 2, Din30, crate::Safe>;
impl<'a, REG> Din30W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No additional filter beyond the CDC synchronization sample"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Din30::Disable)
    }
    #[doc = "1 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn one_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Din30::OneCycle)
    }
    #[doc = "3 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn three_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Din30::ThreeCycle)
    }
    #[doc = "8 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn eight_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Din30::EightCycle)
    }
}
#[doc = "Programmable counter length of digital glitch filter for DIN31\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Din31 {
    #[doc = "0: No additional filter beyond the CDC synchronization sample"]
    Disable = 0,
    #[doc = "1: 1 ULPCLK minimum sample"]
    OneCycle = 1,
    #[doc = "2: 3 ULPCLK minimum sample"]
    ThreeCycle = 2,
    #[doc = "3: 8 ULPCLK minimum sample"]
    EightCycle = 3,
}
impl From<Din31> for u8 {
    #[inline(always)]
    fn from(variant: Din31) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Din31 {
    type Ux = u8;
}
impl crate::IsEnum for Din31 {}
#[doc = "Field `DIN31` reader - Programmable counter length of digital glitch filter for DIN31"]
pub type Din31R = crate::FieldReader<Din31>;
impl Din31R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Din31 {
        match self.bits {
            0 => Din31::Disable,
            1 => Din31::OneCycle,
            2 => Din31::ThreeCycle,
            3 => Din31::EightCycle,
            _ => unreachable!(),
        }
    }
    #[doc = "No additional filter beyond the CDC synchronization sample"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Din31::Disable
    }
    #[doc = "1 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn is_one_cycle(&self) -> bool {
        *self == Din31::OneCycle
    }
    #[doc = "3 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn is_three_cycle(&self) -> bool {
        *self == Din31::ThreeCycle
    }
    #[doc = "8 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn is_eight_cycle(&self) -> bool {
        *self == Din31::EightCycle
    }
}
#[doc = "Field `DIN31` writer - Programmable counter length of digital glitch filter for DIN31"]
pub type Din31W<'a, REG> = crate::FieldWriter<'a, REG, 2, Din31, crate::Safe>;
impl<'a, REG> Din31W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No additional filter beyond the CDC synchronization sample"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Din31::Disable)
    }
    #[doc = "1 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn one_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Din31::OneCycle)
    }
    #[doc = "3 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn three_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Din31::ThreeCycle)
    }
    #[doc = "8 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn eight_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Din31::EightCycle)
    }
}
impl R {
    #[doc = "Bits 0:1 - Programmable counter length of digital glitch filter for DIN16"]
    #[inline(always)]
    pub fn din16(&self) -> Din16R {
        Din16R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Programmable counter length of digital glitch filter for DIN17"]
    #[inline(always)]
    pub fn din17(&self) -> Din17R {
        Din17R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Programmable counter length of digital glitch filter for DIN18"]
    #[inline(always)]
    pub fn din18(&self) -> Din18R {
        Din18R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Programmable counter length of digital glitch filter for DIN19"]
    #[inline(always)]
    pub fn din19(&self) -> Din19R {
        Din19R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Programmable counter length of digital glitch filter for DIN20"]
    #[inline(always)]
    pub fn din20(&self) -> Din20R {
        Din20R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Programmable counter length of digital glitch filter for DIN21"]
    #[inline(always)]
    pub fn din21(&self) -> Din21R {
        Din21R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Programmable counter length of digital glitch filter for DIN22"]
    #[inline(always)]
    pub fn din22(&self) -> Din22R {
        Din22R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Programmable counter length of digital glitch filter for DIN23"]
    #[inline(always)]
    pub fn din23(&self) -> Din23R {
        Din23R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Programmable counter length of digital glitch filter for DIN24"]
    #[inline(always)]
    pub fn din24(&self) -> Din24R {
        Din24R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Programmable counter length of digital glitch filter for DIN25"]
    #[inline(always)]
    pub fn din25(&self) -> Din25R {
        Din25R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Programmable counter length of digital glitch filter for DIN26"]
    #[inline(always)]
    pub fn din26(&self) -> Din26R {
        Din26R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Programmable counter length of digital glitch filter for DIN27"]
    #[inline(always)]
    pub fn din27(&self) -> Din27R {
        Din27R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Programmable counter length of digital glitch filter for DIN28"]
    #[inline(always)]
    pub fn din28(&self) -> Din28R {
        Din28R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - Programmable counter length of digital glitch filter for DIN29"]
    #[inline(always)]
    pub fn din29(&self) -> Din29R {
        Din29R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Programmable counter length of digital glitch filter for DIN30"]
    #[inline(always)]
    pub fn din30(&self) -> Din30R {
        Din30R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Programmable counter length of digital glitch filter for DIN31"]
    #[inline(always)]
    pub fn din31(&self) -> Din31R {
        Din31R::new(((self.bits >> 30) & 3) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FILTEREN31_16")
            .field("din16", &self.din16())
            .field("din17", &self.din17())
            .field("din18", &self.din18())
            .field("din19", &self.din19())
            .field("din20", &self.din20())
            .field("din21", &self.din21())
            .field("din22", &self.din22())
            .field("din23", &self.din23())
            .field("din24", &self.din24())
            .field("din25", &self.din25())
            .field("din26", &self.din26())
            .field("din27", &self.din27())
            .field("din28", &self.din28())
            .field("din29", &self.din29())
            .field("din30", &self.din30())
            .field("din31", &self.din31())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - Programmable counter length of digital glitch filter for DIN16"]
    #[inline(always)]
    pub fn din16(&mut self) -> Din16W<'_, Filteren31_16Spec> {
        Din16W::new(self, 0)
    }
    #[doc = "Bits 2:3 - Programmable counter length of digital glitch filter for DIN17"]
    #[inline(always)]
    pub fn din17(&mut self) -> Din17W<'_, Filteren31_16Spec> {
        Din17W::new(self, 2)
    }
    #[doc = "Bits 4:5 - Programmable counter length of digital glitch filter for DIN18"]
    #[inline(always)]
    pub fn din18(&mut self) -> Din18W<'_, Filteren31_16Spec> {
        Din18W::new(self, 4)
    }
    #[doc = "Bits 6:7 - Programmable counter length of digital glitch filter for DIN19"]
    #[inline(always)]
    pub fn din19(&mut self) -> Din19W<'_, Filteren31_16Spec> {
        Din19W::new(self, 6)
    }
    #[doc = "Bits 8:9 - Programmable counter length of digital glitch filter for DIN20"]
    #[inline(always)]
    pub fn din20(&mut self) -> Din20W<'_, Filteren31_16Spec> {
        Din20W::new(self, 8)
    }
    #[doc = "Bits 10:11 - Programmable counter length of digital glitch filter for DIN21"]
    #[inline(always)]
    pub fn din21(&mut self) -> Din21W<'_, Filteren31_16Spec> {
        Din21W::new(self, 10)
    }
    #[doc = "Bits 12:13 - Programmable counter length of digital glitch filter for DIN22"]
    #[inline(always)]
    pub fn din22(&mut self) -> Din22W<'_, Filteren31_16Spec> {
        Din22W::new(self, 12)
    }
    #[doc = "Bits 14:15 - Programmable counter length of digital glitch filter for DIN23"]
    #[inline(always)]
    pub fn din23(&mut self) -> Din23W<'_, Filteren31_16Spec> {
        Din23W::new(self, 14)
    }
    #[doc = "Bits 16:17 - Programmable counter length of digital glitch filter for DIN24"]
    #[inline(always)]
    pub fn din24(&mut self) -> Din24W<'_, Filteren31_16Spec> {
        Din24W::new(self, 16)
    }
    #[doc = "Bits 18:19 - Programmable counter length of digital glitch filter for DIN25"]
    #[inline(always)]
    pub fn din25(&mut self) -> Din25W<'_, Filteren31_16Spec> {
        Din25W::new(self, 18)
    }
    #[doc = "Bits 20:21 - Programmable counter length of digital glitch filter for DIN26"]
    #[inline(always)]
    pub fn din26(&mut self) -> Din26W<'_, Filteren31_16Spec> {
        Din26W::new(self, 20)
    }
    #[doc = "Bits 22:23 - Programmable counter length of digital glitch filter for DIN27"]
    #[inline(always)]
    pub fn din27(&mut self) -> Din27W<'_, Filteren31_16Spec> {
        Din27W::new(self, 22)
    }
    #[doc = "Bits 24:25 - Programmable counter length of digital glitch filter for DIN28"]
    #[inline(always)]
    pub fn din28(&mut self) -> Din28W<'_, Filteren31_16Spec> {
        Din28W::new(self, 24)
    }
    #[doc = "Bits 26:27 - Programmable counter length of digital glitch filter for DIN29"]
    #[inline(always)]
    pub fn din29(&mut self) -> Din29W<'_, Filteren31_16Spec> {
        Din29W::new(self, 26)
    }
    #[doc = "Bits 28:29 - Programmable counter length of digital glitch filter for DIN30"]
    #[inline(always)]
    pub fn din30(&mut self) -> Din30W<'_, Filteren31_16Spec> {
        Din30W::new(self, 28)
    }
    #[doc = "Bits 30:31 - Programmable counter length of digital glitch filter for DIN31"]
    #[inline(always)]
    pub fn din31(&mut self) -> Din31W<'_, Filteren31_16Spec> {
        Din31W::new(self, 30)
    }
}
#[doc = "Filter Enable 31 to 16\n\nYou can [`read`](crate::Reg::read) this register and get [`filteren31_16::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`filteren31_16::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Filteren31_16Spec;
impl crate::RegisterSpec for Filteren31_16Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`filteren31_16::R`](R) reader structure"]
impl crate::Readable for Filteren31_16Spec {}
#[doc = "`write(|w| ..)` method takes [`filteren31_16::W`](W) writer structure"]
impl crate::Writable for Filteren31_16Spec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets FILTEREN31_16 to value 0"]
impl crate::Resettable for Filteren31_16Spec {}
