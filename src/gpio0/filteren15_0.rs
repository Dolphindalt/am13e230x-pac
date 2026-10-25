#[doc = "Register `FILTEREN15_0` reader"]
pub type R = crate::R<Filteren15_0Spec>;
#[doc = "Register `FILTEREN15_0` writer"]
pub type W = crate::W<Filteren15_0Spec>;
#[doc = "Programmable counter length of digital glitch filter for DIN0\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Din0 {
    #[doc = "0: No additional filter beyond the CDC synchronization sample"]
    Disable = 0,
    #[doc = "1: 1 ULPCLK minimum sample"]
    OneCycle = 1,
    #[doc = "2: 3 ULPCLK minimum sample"]
    ThreeCycle = 2,
    #[doc = "3: 8 ULPCLK minimum sample"]
    EightCycle = 3,
}
impl From<Din0> for u8 {
    #[inline(always)]
    fn from(variant: Din0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Din0 {
    type Ux = u8;
}
impl crate::IsEnum for Din0 {}
#[doc = "Field `DIN0` reader - Programmable counter length of digital glitch filter for DIN0"]
pub type Din0R = crate::FieldReader<Din0>;
impl Din0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Din0 {
        match self.bits {
            0 => Din0::Disable,
            1 => Din0::OneCycle,
            2 => Din0::ThreeCycle,
            3 => Din0::EightCycle,
            _ => unreachable!(),
        }
    }
    #[doc = "No additional filter beyond the CDC synchronization sample"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Din0::Disable
    }
    #[doc = "1 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn is_one_cycle(&self) -> bool {
        *self == Din0::OneCycle
    }
    #[doc = "3 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn is_three_cycle(&self) -> bool {
        *self == Din0::ThreeCycle
    }
    #[doc = "8 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn is_eight_cycle(&self) -> bool {
        *self == Din0::EightCycle
    }
}
#[doc = "Field `DIN0` writer - Programmable counter length of digital glitch filter for DIN0"]
pub type Din0W<'a, REG> = crate::FieldWriter<'a, REG, 2, Din0, crate::Safe>;
impl<'a, REG> Din0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No additional filter beyond the CDC synchronization sample"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Din0::Disable)
    }
    #[doc = "1 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn one_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Din0::OneCycle)
    }
    #[doc = "3 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn three_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Din0::ThreeCycle)
    }
    #[doc = "8 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn eight_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Din0::EightCycle)
    }
}
#[doc = "Programmable counter length of digital glitch filter for DIN1\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Din1 {
    #[doc = "0: No additional filter beyond the CDC synchronization sample"]
    Disable = 0,
    #[doc = "1: 1 ULPCLK minimum sample"]
    OneCycle = 1,
    #[doc = "2: 3 ULPCLK minimum sample"]
    ThreeCycle = 2,
    #[doc = "3: 8 ULPCLK minimum sample"]
    EightCycle = 3,
}
impl From<Din1> for u8 {
    #[inline(always)]
    fn from(variant: Din1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Din1 {
    type Ux = u8;
}
impl crate::IsEnum for Din1 {}
#[doc = "Field `DIN1` reader - Programmable counter length of digital glitch filter for DIN1"]
pub type Din1R = crate::FieldReader<Din1>;
impl Din1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Din1 {
        match self.bits {
            0 => Din1::Disable,
            1 => Din1::OneCycle,
            2 => Din1::ThreeCycle,
            3 => Din1::EightCycle,
            _ => unreachable!(),
        }
    }
    #[doc = "No additional filter beyond the CDC synchronization sample"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Din1::Disable
    }
    #[doc = "1 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn is_one_cycle(&self) -> bool {
        *self == Din1::OneCycle
    }
    #[doc = "3 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn is_three_cycle(&self) -> bool {
        *self == Din1::ThreeCycle
    }
    #[doc = "8 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn is_eight_cycle(&self) -> bool {
        *self == Din1::EightCycle
    }
}
#[doc = "Field `DIN1` writer - Programmable counter length of digital glitch filter for DIN1"]
pub type Din1W<'a, REG> = crate::FieldWriter<'a, REG, 2, Din1, crate::Safe>;
impl<'a, REG> Din1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No additional filter beyond the CDC synchronization sample"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Din1::Disable)
    }
    #[doc = "1 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn one_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Din1::OneCycle)
    }
    #[doc = "3 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn three_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Din1::ThreeCycle)
    }
    #[doc = "8 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn eight_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Din1::EightCycle)
    }
}
#[doc = "Programmable counter length of digital glitch filter for DIN2\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Din2 {
    #[doc = "0: No additional filter beyond the CDC synchronization sample"]
    Disable = 0,
    #[doc = "1: 1 ULPCLK minimum sample"]
    OneCycle = 1,
    #[doc = "2: 3 ULPCLK minimum sample"]
    ThreeCycle = 2,
    #[doc = "3: 8 ULPCLK minimum sample"]
    EightCycle = 3,
}
impl From<Din2> for u8 {
    #[inline(always)]
    fn from(variant: Din2) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Din2 {
    type Ux = u8;
}
impl crate::IsEnum for Din2 {}
#[doc = "Field `DIN2` reader - Programmable counter length of digital glitch filter for DIN2"]
pub type Din2R = crate::FieldReader<Din2>;
impl Din2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Din2 {
        match self.bits {
            0 => Din2::Disable,
            1 => Din2::OneCycle,
            2 => Din2::ThreeCycle,
            3 => Din2::EightCycle,
            _ => unreachable!(),
        }
    }
    #[doc = "No additional filter beyond the CDC synchronization sample"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Din2::Disable
    }
    #[doc = "1 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn is_one_cycle(&self) -> bool {
        *self == Din2::OneCycle
    }
    #[doc = "3 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn is_three_cycle(&self) -> bool {
        *self == Din2::ThreeCycle
    }
    #[doc = "8 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn is_eight_cycle(&self) -> bool {
        *self == Din2::EightCycle
    }
}
#[doc = "Field `DIN2` writer - Programmable counter length of digital glitch filter for DIN2"]
pub type Din2W<'a, REG> = crate::FieldWriter<'a, REG, 2, Din2, crate::Safe>;
impl<'a, REG> Din2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No additional filter beyond the CDC synchronization sample"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Din2::Disable)
    }
    #[doc = "1 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn one_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Din2::OneCycle)
    }
    #[doc = "3 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn three_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Din2::ThreeCycle)
    }
    #[doc = "8 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn eight_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Din2::EightCycle)
    }
}
#[doc = "Programmable counter length of digital glitch filter for DIN3\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Din3 {
    #[doc = "0: No additional filter beyond the CDC synchronization sample"]
    Disable = 0,
    #[doc = "1: 1 ULPCLK minimum sample"]
    OneCycle = 1,
    #[doc = "2: 3 ULPCLK minimum sample"]
    ThreeCycle = 2,
    #[doc = "3: 8 ULPCLK minimum sample"]
    EightCycle = 3,
}
impl From<Din3> for u8 {
    #[inline(always)]
    fn from(variant: Din3) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Din3 {
    type Ux = u8;
}
impl crate::IsEnum for Din3 {}
#[doc = "Field `DIN3` reader - Programmable counter length of digital glitch filter for DIN3"]
pub type Din3R = crate::FieldReader<Din3>;
impl Din3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Din3 {
        match self.bits {
            0 => Din3::Disable,
            1 => Din3::OneCycle,
            2 => Din3::ThreeCycle,
            3 => Din3::EightCycle,
            _ => unreachable!(),
        }
    }
    #[doc = "No additional filter beyond the CDC synchronization sample"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Din3::Disable
    }
    #[doc = "1 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn is_one_cycle(&self) -> bool {
        *self == Din3::OneCycle
    }
    #[doc = "3 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn is_three_cycle(&self) -> bool {
        *self == Din3::ThreeCycle
    }
    #[doc = "8 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn is_eight_cycle(&self) -> bool {
        *self == Din3::EightCycle
    }
}
#[doc = "Field `DIN3` writer - Programmable counter length of digital glitch filter for DIN3"]
pub type Din3W<'a, REG> = crate::FieldWriter<'a, REG, 2, Din3, crate::Safe>;
impl<'a, REG> Din3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No additional filter beyond the CDC synchronization sample"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Din3::Disable)
    }
    #[doc = "1 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn one_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Din3::OneCycle)
    }
    #[doc = "3 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn three_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Din3::ThreeCycle)
    }
    #[doc = "8 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn eight_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Din3::EightCycle)
    }
}
#[doc = "Programmable counter length of digital glitch filter for DIN4\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Din4 {
    #[doc = "0: No additional filter beyond the CDC synchronization sample"]
    Disable = 0,
    #[doc = "1: 1 ULPCLK minimum sample"]
    OneCycle = 1,
    #[doc = "2: 3 ULPCLK minimum sample"]
    ThreeCycle = 2,
    #[doc = "3: 8 ULPCLK minimum sample"]
    EightCycle = 3,
}
impl From<Din4> for u8 {
    #[inline(always)]
    fn from(variant: Din4) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Din4 {
    type Ux = u8;
}
impl crate::IsEnum for Din4 {}
#[doc = "Field `DIN4` reader - Programmable counter length of digital glitch filter for DIN4"]
pub type Din4R = crate::FieldReader<Din4>;
impl Din4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Din4 {
        match self.bits {
            0 => Din4::Disable,
            1 => Din4::OneCycle,
            2 => Din4::ThreeCycle,
            3 => Din4::EightCycle,
            _ => unreachable!(),
        }
    }
    #[doc = "No additional filter beyond the CDC synchronization sample"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Din4::Disable
    }
    #[doc = "1 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn is_one_cycle(&self) -> bool {
        *self == Din4::OneCycle
    }
    #[doc = "3 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn is_three_cycle(&self) -> bool {
        *self == Din4::ThreeCycle
    }
    #[doc = "8 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn is_eight_cycle(&self) -> bool {
        *self == Din4::EightCycle
    }
}
#[doc = "Field `DIN4` writer - Programmable counter length of digital glitch filter for DIN4"]
pub type Din4W<'a, REG> = crate::FieldWriter<'a, REG, 2, Din4, crate::Safe>;
impl<'a, REG> Din4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No additional filter beyond the CDC synchronization sample"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Din4::Disable)
    }
    #[doc = "1 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn one_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Din4::OneCycle)
    }
    #[doc = "3 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn three_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Din4::ThreeCycle)
    }
    #[doc = "8 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn eight_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Din4::EightCycle)
    }
}
#[doc = "Programmable counter length of digital glitch filter for DIN5\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Din5 {
    #[doc = "0: No additional filter beyond the CDC synchronization sample"]
    Disable = 0,
    #[doc = "1: 1 ULPCLK minimum sample"]
    OneCycle = 1,
    #[doc = "2: 3 ULPCLK minimum sample"]
    ThreeCycle = 2,
    #[doc = "3: 8 ULPCLK minimum sample"]
    EightCycle = 3,
}
impl From<Din5> for u8 {
    #[inline(always)]
    fn from(variant: Din5) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Din5 {
    type Ux = u8;
}
impl crate::IsEnum for Din5 {}
#[doc = "Field `DIN5` reader - Programmable counter length of digital glitch filter for DIN5"]
pub type Din5R = crate::FieldReader<Din5>;
impl Din5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Din5 {
        match self.bits {
            0 => Din5::Disable,
            1 => Din5::OneCycle,
            2 => Din5::ThreeCycle,
            3 => Din5::EightCycle,
            _ => unreachable!(),
        }
    }
    #[doc = "No additional filter beyond the CDC synchronization sample"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Din5::Disable
    }
    #[doc = "1 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn is_one_cycle(&self) -> bool {
        *self == Din5::OneCycle
    }
    #[doc = "3 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn is_three_cycle(&self) -> bool {
        *self == Din5::ThreeCycle
    }
    #[doc = "8 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn is_eight_cycle(&self) -> bool {
        *self == Din5::EightCycle
    }
}
#[doc = "Field `DIN5` writer - Programmable counter length of digital glitch filter for DIN5"]
pub type Din5W<'a, REG> = crate::FieldWriter<'a, REG, 2, Din5, crate::Safe>;
impl<'a, REG> Din5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No additional filter beyond the CDC synchronization sample"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Din5::Disable)
    }
    #[doc = "1 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn one_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Din5::OneCycle)
    }
    #[doc = "3 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn three_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Din5::ThreeCycle)
    }
    #[doc = "8 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn eight_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Din5::EightCycle)
    }
}
#[doc = "Programmable counter length of digital glitch filter for DIN6\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Din6 {
    #[doc = "0: No additional filter beyond the CDC synchronization sample"]
    Disable = 0,
    #[doc = "1: 1 ULPCLK minimum sample"]
    OneCycle = 1,
    #[doc = "2: 3 ULPCLK minimum sample"]
    ThreeCycle = 2,
    #[doc = "3: 8 ULPCLK minimum sample"]
    EightCycle = 3,
}
impl From<Din6> for u8 {
    #[inline(always)]
    fn from(variant: Din6) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Din6 {
    type Ux = u8;
}
impl crate::IsEnum for Din6 {}
#[doc = "Field `DIN6` reader - Programmable counter length of digital glitch filter for DIN6"]
pub type Din6R = crate::FieldReader<Din6>;
impl Din6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Din6 {
        match self.bits {
            0 => Din6::Disable,
            1 => Din6::OneCycle,
            2 => Din6::ThreeCycle,
            3 => Din6::EightCycle,
            _ => unreachable!(),
        }
    }
    #[doc = "No additional filter beyond the CDC synchronization sample"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Din6::Disable
    }
    #[doc = "1 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn is_one_cycle(&self) -> bool {
        *self == Din6::OneCycle
    }
    #[doc = "3 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn is_three_cycle(&self) -> bool {
        *self == Din6::ThreeCycle
    }
    #[doc = "8 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn is_eight_cycle(&self) -> bool {
        *self == Din6::EightCycle
    }
}
#[doc = "Field `DIN6` writer - Programmable counter length of digital glitch filter for DIN6"]
pub type Din6W<'a, REG> = crate::FieldWriter<'a, REG, 2, Din6, crate::Safe>;
impl<'a, REG> Din6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No additional filter beyond the CDC synchronization sample"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Din6::Disable)
    }
    #[doc = "1 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn one_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Din6::OneCycle)
    }
    #[doc = "3 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn three_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Din6::ThreeCycle)
    }
    #[doc = "8 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn eight_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Din6::EightCycle)
    }
}
#[doc = "Programmable counter length of digital glitch filter for DIN7\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Din7 {
    #[doc = "0: No additional filter beyond the CDC synchronization sample"]
    Disable = 0,
    #[doc = "1: 1 ULPCLK minimum sample"]
    OneCycle = 1,
    #[doc = "2: 3 ULPCLK minimum sample"]
    ThreeCycle = 2,
    #[doc = "3: 8 ULPCLK minimum sample"]
    EightCycle = 3,
}
impl From<Din7> for u8 {
    #[inline(always)]
    fn from(variant: Din7) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Din7 {
    type Ux = u8;
}
impl crate::IsEnum for Din7 {}
#[doc = "Field `DIN7` reader - Programmable counter length of digital glitch filter for DIN7"]
pub type Din7R = crate::FieldReader<Din7>;
impl Din7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Din7 {
        match self.bits {
            0 => Din7::Disable,
            1 => Din7::OneCycle,
            2 => Din7::ThreeCycle,
            3 => Din7::EightCycle,
            _ => unreachable!(),
        }
    }
    #[doc = "No additional filter beyond the CDC synchronization sample"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Din7::Disable
    }
    #[doc = "1 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn is_one_cycle(&self) -> bool {
        *self == Din7::OneCycle
    }
    #[doc = "3 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn is_three_cycle(&self) -> bool {
        *self == Din7::ThreeCycle
    }
    #[doc = "8 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn is_eight_cycle(&self) -> bool {
        *self == Din7::EightCycle
    }
}
#[doc = "Field `DIN7` writer - Programmable counter length of digital glitch filter for DIN7"]
pub type Din7W<'a, REG> = crate::FieldWriter<'a, REG, 2, Din7, crate::Safe>;
impl<'a, REG> Din7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No additional filter beyond the CDC synchronization sample"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Din7::Disable)
    }
    #[doc = "1 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn one_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Din7::OneCycle)
    }
    #[doc = "3 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn three_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Din7::ThreeCycle)
    }
    #[doc = "8 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn eight_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Din7::EightCycle)
    }
}
#[doc = "Programmable counter length of digital glitch filter for DIN8\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Din8 {
    #[doc = "0: No additional filter beyond the CDC synchronization sample"]
    Disable = 0,
    #[doc = "1: 1 ULPCLK minimum sample"]
    OneCycle = 1,
    #[doc = "2: 3 ULPCLK minimum sample"]
    ThreeCycle = 2,
    #[doc = "3: 8 ULPCLK minimum sample"]
    EightCycle = 3,
}
impl From<Din8> for u8 {
    #[inline(always)]
    fn from(variant: Din8) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Din8 {
    type Ux = u8;
}
impl crate::IsEnum for Din8 {}
#[doc = "Field `DIN8` reader - Programmable counter length of digital glitch filter for DIN8"]
pub type Din8R = crate::FieldReader<Din8>;
impl Din8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Din8 {
        match self.bits {
            0 => Din8::Disable,
            1 => Din8::OneCycle,
            2 => Din8::ThreeCycle,
            3 => Din8::EightCycle,
            _ => unreachable!(),
        }
    }
    #[doc = "No additional filter beyond the CDC synchronization sample"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Din8::Disable
    }
    #[doc = "1 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn is_one_cycle(&self) -> bool {
        *self == Din8::OneCycle
    }
    #[doc = "3 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn is_three_cycle(&self) -> bool {
        *self == Din8::ThreeCycle
    }
    #[doc = "8 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn is_eight_cycle(&self) -> bool {
        *self == Din8::EightCycle
    }
}
#[doc = "Field `DIN8` writer - Programmable counter length of digital glitch filter for DIN8"]
pub type Din8W<'a, REG> = crate::FieldWriter<'a, REG, 2, Din8, crate::Safe>;
impl<'a, REG> Din8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No additional filter beyond the CDC synchronization sample"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Din8::Disable)
    }
    #[doc = "1 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn one_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Din8::OneCycle)
    }
    #[doc = "3 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn three_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Din8::ThreeCycle)
    }
    #[doc = "8 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn eight_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Din8::EightCycle)
    }
}
#[doc = "Programmable counter length of digital glitch filter for DIN9\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Din9 {
    #[doc = "0: No additional filter beyond the CDC synchronization sample"]
    Disable = 0,
    #[doc = "1: 1 ULPCLK minimum sample"]
    OneCycle = 1,
    #[doc = "2: 3 ULPCLK minimum sample"]
    ThreeCycle = 2,
    #[doc = "3: 8 ULPCLK minimum sample"]
    EightCycle = 3,
}
impl From<Din9> for u8 {
    #[inline(always)]
    fn from(variant: Din9) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Din9 {
    type Ux = u8;
}
impl crate::IsEnum for Din9 {}
#[doc = "Field `DIN9` reader - Programmable counter length of digital glitch filter for DIN9"]
pub type Din9R = crate::FieldReader<Din9>;
impl Din9R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Din9 {
        match self.bits {
            0 => Din9::Disable,
            1 => Din9::OneCycle,
            2 => Din9::ThreeCycle,
            3 => Din9::EightCycle,
            _ => unreachable!(),
        }
    }
    #[doc = "No additional filter beyond the CDC synchronization sample"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Din9::Disable
    }
    #[doc = "1 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn is_one_cycle(&self) -> bool {
        *self == Din9::OneCycle
    }
    #[doc = "3 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn is_three_cycle(&self) -> bool {
        *self == Din9::ThreeCycle
    }
    #[doc = "8 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn is_eight_cycle(&self) -> bool {
        *self == Din9::EightCycle
    }
}
#[doc = "Field `DIN9` writer - Programmable counter length of digital glitch filter for DIN9"]
pub type Din9W<'a, REG> = crate::FieldWriter<'a, REG, 2, Din9, crate::Safe>;
impl<'a, REG> Din9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No additional filter beyond the CDC synchronization sample"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Din9::Disable)
    }
    #[doc = "1 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn one_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Din9::OneCycle)
    }
    #[doc = "3 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn three_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Din9::ThreeCycle)
    }
    #[doc = "8 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn eight_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Din9::EightCycle)
    }
}
#[doc = "Programmable counter length of digital glitch filter for DIN10\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Din10 {
    #[doc = "0: No additional filter beyond the CDC synchronization sample"]
    Disable = 0,
    #[doc = "1: 1 ULPCLK minimum sample"]
    OneCycle = 1,
    #[doc = "2: 3 ULPCLK minimum sample"]
    ThreeCycle = 2,
    #[doc = "3: 8 ULPCLK minimum sample"]
    EightCycle = 3,
}
impl From<Din10> for u8 {
    #[inline(always)]
    fn from(variant: Din10) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Din10 {
    type Ux = u8;
}
impl crate::IsEnum for Din10 {}
#[doc = "Field `DIN10` reader - Programmable counter length of digital glitch filter for DIN10"]
pub type Din10R = crate::FieldReader<Din10>;
impl Din10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Din10 {
        match self.bits {
            0 => Din10::Disable,
            1 => Din10::OneCycle,
            2 => Din10::ThreeCycle,
            3 => Din10::EightCycle,
            _ => unreachable!(),
        }
    }
    #[doc = "No additional filter beyond the CDC synchronization sample"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Din10::Disable
    }
    #[doc = "1 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn is_one_cycle(&self) -> bool {
        *self == Din10::OneCycle
    }
    #[doc = "3 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn is_three_cycle(&self) -> bool {
        *self == Din10::ThreeCycle
    }
    #[doc = "8 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn is_eight_cycle(&self) -> bool {
        *self == Din10::EightCycle
    }
}
#[doc = "Field `DIN10` writer - Programmable counter length of digital glitch filter for DIN10"]
pub type Din10W<'a, REG> = crate::FieldWriter<'a, REG, 2, Din10, crate::Safe>;
impl<'a, REG> Din10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No additional filter beyond the CDC synchronization sample"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Din10::Disable)
    }
    #[doc = "1 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn one_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Din10::OneCycle)
    }
    #[doc = "3 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn three_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Din10::ThreeCycle)
    }
    #[doc = "8 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn eight_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Din10::EightCycle)
    }
}
#[doc = "Programmable counter length of digital glitch filter for DIN11\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Din11 {
    #[doc = "0: No additional filter beyond the CDC synchronization sample"]
    Disable = 0,
    #[doc = "1: 1 ULPCLK minimum sample"]
    OneCycle = 1,
    #[doc = "2: 3 ULPCLK minimum sample"]
    ThreeCycle = 2,
    #[doc = "3: 8 ULPCLK minimum sample"]
    EightCycle = 3,
}
impl From<Din11> for u8 {
    #[inline(always)]
    fn from(variant: Din11) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Din11 {
    type Ux = u8;
}
impl crate::IsEnum for Din11 {}
#[doc = "Field `DIN11` reader - Programmable counter length of digital glitch filter for DIN11"]
pub type Din11R = crate::FieldReader<Din11>;
impl Din11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Din11 {
        match self.bits {
            0 => Din11::Disable,
            1 => Din11::OneCycle,
            2 => Din11::ThreeCycle,
            3 => Din11::EightCycle,
            _ => unreachable!(),
        }
    }
    #[doc = "No additional filter beyond the CDC synchronization sample"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Din11::Disable
    }
    #[doc = "1 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn is_one_cycle(&self) -> bool {
        *self == Din11::OneCycle
    }
    #[doc = "3 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn is_three_cycle(&self) -> bool {
        *self == Din11::ThreeCycle
    }
    #[doc = "8 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn is_eight_cycle(&self) -> bool {
        *self == Din11::EightCycle
    }
}
#[doc = "Field `DIN11` writer - Programmable counter length of digital glitch filter for DIN11"]
pub type Din11W<'a, REG> = crate::FieldWriter<'a, REG, 2, Din11, crate::Safe>;
impl<'a, REG> Din11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No additional filter beyond the CDC synchronization sample"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Din11::Disable)
    }
    #[doc = "1 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn one_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Din11::OneCycle)
    }
    #[doc = "3 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn three_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Din11::ThreeCycle)
    }
    #[doc = "8 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn eight_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Din11::EightCycle)
    }
}
#[doc = "Programmable counter length of digital glitch filter for DIN12\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Din12 {
    #[doc = "0: No additional filter beyond the CDC synchronization sample"]
    Disable = 0,
    #[doc = "1: 1 ULPCLK minimum sample"]
    OneCycle = 1,
    #[doc = "2: 3 ULPCLK minimum sample"]
    ThreeCycle = 2,
    #[doc = "3: 8 ULPCLK minimum sample"]
    EightCycle = 3,
}
impl From<Din12> for u8 {
    #[inline(always)]
    fn from(variant: Din12) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Din12 {
    type Ux = u8;
}
impl crate::IsEnum for Din12 {}
#[doc = "Field `DIN12` reader - Programmable counter length of digital glitch filter for DIN12"]
pub type Din12R = crate::FieldReader<Din12>;
impl Din12R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Din12 {
        match self.bits {
            0 => Din12::Disable,
            1 => Din12::OneCycle,
            2 => Din12::ThreeCycle,
            3 => Din12::EightCycle,
            _ => unreachable!(),
        }
    }
    #[doc = "No additional filter beyond the CDC synchronization sample"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Din12::Disable
    }
    #[doc = "1 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn is_one_cycle(&self) -> bool {
        *self == Din12::OneCycle
    }
    #[doc = "3 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn is_three_cycle(&self) -> bool {
        *self == Din12::ThreeCycle
    }
    #[doc = "8 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn is_eight_cycle(&self) -> bool {
        *self == Din12::EightCycle
    }
}
#[doc = "Field `DIN12` writer - Programmable counter length of digital glitch filter for DIN12"]
pub type Din12W<'a, REG> = crate::FieldWriter<'a, REG, 2, Din12, crate::Safe>;
impl<'a, REG> Din12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No additional filter beyond the CDC synchronization sample"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Din12::Disable)
    }
    #[doc = "1 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn one_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Din12::OneCycle)
    }
    #[doc = "3 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn three_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Din12::ThreeCycle)
    }
    #[doc = "8 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn eight_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Din12::EightCycle)
    }
}
#[doc = "Programmable counter length of digital glitch filter for DIN13\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Din13 {
    #[doc = "0: No additional filter beyond the CDC synchronization sample"]
    Disable = 0,
    #[doc = "1: 1 ULPCLK minimum sample"]
    OneCycle = 1,
    #[doc = "2: 3 ULPCLK minimum sample"]
    ThreeCycle = 2,
    #[doc = "3: 8 ULPCLK minimum sample"]
    EightCycle = 3,
}
impl From<Din13> for u8 {
    #[inline(always)]
    fn from(variant: Din13) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Din13 {
    type Ux = u8;
}
impl crate::IsEnum for Din13 {}
#[doc = "Field `DIN13` reader - Programmable counter length of digital glitch filter for DIN13"]
pub type Din13R = crate::FieldReader<Din13>;
impl Din13R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Din13 {
        match self.bits {
            0 => Din13::Disable,
            1 => Din13::OneCycle,
            2 => Din13::ThreeCycle,
            3 => Din13::EightCycle,
            _ => unreachable!(),
        }
    }
    #[doc = "No additional filter beyond the CDC synchronization sample"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Din13::Disable
    }
    #[doc = "1 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn is_one_cycle(&self) -> bool {
        *self == Din13::OneCycle
    }
    #[doc = "3 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn is_three_cycle(&self) -> bool {
        *self == Din13::ThreeCycle
    }
    #[doc = "8 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn is_eight_cycle(&self) -> bool {
        *self == Din13::EightCycle
    }
}
#[doc = "Field `DIN13` writer - Programmable counter length of digital glitch filter for DIN13"]
pub type Din13W<'a, REG> = crate::FieldWriter<'a, REG, 2, Din13, crate::Safe>;
impl<'a, REG> Din13W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No additional filter beyond the CDC synchronization sample"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Din13::Disable)
    }
    #[doc = "1 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn one_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Din13::OneCycle)
    }
    #[doc = "3 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn three_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Din13::ThreeCycle)
    }
    #[doc = "8 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn eight_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Din13::EightCycle)
    }
}
#[doc = "Programmable counter length of digital glitch filter for DIN14\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Din14 {
    #[doc = "0: No additional filter beyond the CDC synchronization sample"]
    Disable = 0,
    #[doc = "1: 1 ULPCLK minimum sample"]
    OneCycle = 1,
    #[doc = "2: 3 ULPCLK minimum sample"]
    ThreeCycle = 2,
    #[doc = "3: 8 ULPCLK minimum sample"]
    EightCycle = 3,
}
impl From<Din14> for u8 {
    #[inline(always)]
    fn from(variant: Din14) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Din14 {
    type Ux = u8;
}
impl crate::IsEnum for Din14 {}
#[doc = "Field `DIN14` reader - Programmable counter length of digital glitch filter for DIN14"]
pub type Din14R = crate::FieldReader<Din14>;
impl Din14R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Din14 {
        match self.bits {
            0 => Din14::Disable,
            1 => Din14::OneCycle,
            2 => Din14::ThreeCycle,
            3 => Din14::EightCycle,
            _ => unreachable!(),
        }
    }
    #[doc = "No additional filter beyond the CDC synchronization sample"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Din14::Disable
    }
    #[doc = "1 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn is_one_cycle(&self) -> bool {
        *self == Din14::OneCycle
    }
    #[doc = "3 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn is_three_cycle(&self) -> bool {
        *self == Din14::ThreeCycle
    }
    #[doc = "8 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn is_eight_cycle(&self) -> bool {
        *self == Din14::EightCycle
    }
}
#[doc = "Field `DIN14` writer - Programmable counter length of digital glitch filter for DIN14"]
pub type Din14W<'a, REG> = crate::FieldWriter<'a, REG, 2, Din14, crate::Safe>;
impl<'a, REG> Din14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No additional filter beyond the CDC synchronization sample"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Din14::Disable)
    }
    #[doc = "1 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn one_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Din14::OneCycle)
    }
    #[doc = "3 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn three_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Din14::ThreeCycle)
    }
    #[doc = "8 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn eight_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Din14::EightCycle)
    }
}
#[doc = "Programmable counter length of digital glitch filter for DIN15\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Din15 {
    #[doc = "0: No additional filter beyond the CDC synchronization sample"]
    Disable = 0,
    #[doc = "1: 1 ULPCLK minimum sample"]
    OneCycle = 1,
    #[doc = "2: 3 ULPCLK minimum sample"]
    ThreeCycle = 2,
    #[doc = "3: 8 ULPCLK minimum sample"]
    EightCycle = 3,
}
impl From<Din15> for u8 {
    #[inline(always)]
    fn from(variant: Din15) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Din15 {
    type Ux = u8;
}
impl crate::IsEnum for Din15 {}
#[doc = "Field `DIN15` reader - Programmable counter length of digital glitch filter for DIN15"]
pub type Din15R = crate::FieldReader<Din15>;
impl Din15R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Din15 {
        match self.bits {
            0 => Din15::Disable,
            1 => Din15::OneCycle,
            2 => Din15::ThreeCycle,
            3 => Din15::EightCycle,
            _ => unreachable!(),
        }
    }
    #[doc = "No additional filter beyond the CDC synchronization sample"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Din15::Disable
    }
    #[doc = "1 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn is_one_cycle(&self) -> bool {
        *self == Din15::OneCycle
    }
    #[doc = "3 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn is_three_cycle(&self) -> bool {
        *self == Din15::ThreeCycle
    }
    #[doc = "8 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn is_eight_cycle(&self) -> bool {
        *self == Din15::EightCycle
    }
}
#[doc = "Field `DIN15` writer - Programmable counter length of digital glitch filter for DIN15"]
pub type Din15W<'a, REG> = crate::FieldWriter<'a, REG, 2, Din15, crate::Safe>;
impl<'a, REG> Din15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No additional filter beyond the CDC synchronization sample"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Din15::Disable)
    }
    #[doc = "1 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn one_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Din15::OneCycle)
    }
    #[doc = "3 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn three_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Din15::ThreeCycle)
    }
    #[doc = "8 ULPCLK minimum sample"]
    #[inline(always)]
    pub fn eight_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Din15::EightCycle)
    }
}
impl R {
    #[doc = "Bits 0:1 - Programmable counter length of digital glitch filter for DIN0"]
    #[inline(always)]
    pub fn din0(&self) -> Din0R {
        Din0R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Programmable counter length of digital glitch filter for DIN1"]
    #[inline(always)]
    pub fn din1(&self) -> Din1R {
        Din1R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Programmable counter length of digital glitch filter for DIN2"]
    #[inline(always)]
    pub fn din2(&self) -> Din2R {
        Din2R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Programmable counter length of digital glitch filter for DIN3"]
    #[inline(always)]
    pub fn din3(&self) -> Din3R {
        Din3R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Programmable counter length of digital glitch filter for DIN4"]
    #[inline(always)]
    pub fn din4(&self) -> Din4R {
        Din4R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Programmable counter length of digital glitch filter for DIN5"]
    #[inline(always)]
    pub fn din5(&self) -> Din5R {
        Din5R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Programmable counter length of digital glitch filter for DIN6"]
    #[inline(always)]
    pub fn din6(&self) -> Din6R {
        Din6R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Programmable counter length of digital glitch filter for DIN7"]
    #[inline(always)]
    pub fn din7(&self) -> Din7R {
        Din7R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Programmable counter length of digital glitch filter for DIN8"]
    #[inline(always)]
    pub fn din8(&self) -> Din8R {
        Din8R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Programmable counter length of digital glitch filter for DIN9"]
    #[inline(always)]
    pub fn din9(&self) -> Din9R {
        Din9R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Programmable counter length of digital glitch filter for DIN10"]
    #[inline(always)]
    pub fn din10(&self) -> Din10R {
        Din10R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Programmable counter length of digital glitch filter for DIN11"]
    #[inline(always)]
    pub fn din11(&self) -> Din11R {
        Din11R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Programmable counter length of digital glitch filter for DIN12"]
    #[inline(always)]
    pub fn din12(&self) -> Din12R {
        Din12R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - Programmable counter length of digital glitch filter for DIN13"]
    #[inline(always)]
    pub fn din13(&self) -> Din13R {
        Din13R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Programmable counter length of digital glitch filter for DIN14"]
    #[inline(always)]
    pub fn din14(&self) -> Din14R {
        Din14R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Programmable counter length of digital glitch filter for DIN15"]
    #[inline(always)]
    pub fn din15(&self) -> Din15R {
        Din15R::new(((self.bits >> 30) & 3) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FILTEREN15_0")
            .field("din0", &self.din0())
            .field("din1", &self.din1())
            .field("din2", &self.din2())
            .field("din3", &self.din3())
            .field("din4", &self.din4())
            .field("din5", &self.din5())
            .field("din6", &self.din6())
            .field("din7", &self.din7())
            .field("din8", &self.din8())
            .field("din9", &self.din9())
            .field("din10", &self.din10())
            .field("din11", &self.din11())
            .field("din12", &self.din12())
            .field("din13", &self.din13())
            .field("din14", &self.din14())
            .field("din15", &self.din15())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - Programmable counter length of digital glitch filter for DIN0"]
    #[inline(always)]
    pub fn din0(&mut self) -> Din0W<'_, Filteren15_0Spec> {
        Din0W::new(self, 0)
    }
    #[doc = "Bits 2:3 - Programmable counter length of digital glitch filter for DIN1"]
    #[inline(always)]
    pub fn din1(&mut self) -> Din1W<'_, Filteren15_0Spec> {
        Din1W::new(self, 2)
    }
    #[doc = "Bits 4:5 - Programmable counter length of digital glitch filter for DIN2"]
    #[inline(always)]
    pub fn din2(&mut self) -> Din2W<'_, Filteren15_0Spec> {
        Din2W::new(self, 4)
    }
    #[doc = "Bits 6:7 - Programmable counter length of digital glitch filter for DIN3"]
    #[inline(always)]
    pub fn din3(&mut self) -> Din3W<'_, Filteren15_0Spec> {
        Din3W::new(self, 6)
    }
    #[doc = "Bits 8:9 - Programmable counter length of digital glitch filter for DIN4"]
    #[inline(always)]
    pub fn din4(&mut self) -> Din4W<'_, Filteren15_0Spec> {
        Din4W::new(self, 8)
    }
    #[doc = "Bits 10:11 - Programmable counter length of digital glitch filter for DIN5"]
    #[inline(always)]
    pub fn din5(&mut self) -> Din5W<'_, Filteren15_0Spec> {
        Din5W::new(self, 10)
    }
    #[doc = "Bits 12:13 - Programmable counter length of digital glitch filter for DIN6"]
    #[inline(always)]
    pub fn din6(&mut self) -> Din6W<'_, Filteren15_0Spec> {
        Din6W::new(self, 12)
    }
    #[doc = "Bits 14:15 - Programmable counter length of digital glitch filter for DIN7"]
    #[inline(always)]
    pub fn din7(&mut self) -> Din7W<'_, Filteren15_0Spec> {
        Din7W::new(self, 14)
    }
    #[doc = "Bits 16:17 - Programmable counter length of digital glitch filter for DIN8"]
    #[inline(always)]
    pub fn din8(&mut self) -> Din8W<'_, Filteren15_0Spec> {
        Din8W::new(self, 16)
    }
    #[doc = "Bits 18:19 - Programmable counter length of digital glitch filter for DIN9"]
    #[inline(always)]
    pub fn din9(&mut self) -> Din9W<'_, Filteren15_0Spec> {
        Din9W::new(self, 18)
    }
    #[doc = "Bits 20:21 - Programmable counter length of digital glitch filter for DIN10"]
    #[inline(always)]
    pub fn din10(&mut self) -> Din10W<'_, Filteren15_0Spec> {
        Din10W::new(self, 20)
    }
    #[doc = "Bits 22:23 - Programmable counter length of digital glitch filter for DIN11"]
    #[inline(always)]
    pub fn din11(&mut self) -> Din11W<'_, Filteren15_0Spec> {
        Din11W::new(self, 22)
    }
    #[doc = "Bits 24:25 - Programmable counter length of digital glitch filter for DIN12"]
    #[inline(always)]
    pub fn din12(&mut self) -> Din12W<'_, Filteren15_0Spec> {
        Din12W::new(self, 24)
    }
    #[doc = "Bits 26:27 - Programmable counter length of digital glitch filter for DIN13"]
    #[inline(always)]
    pub fn din13(&mut self) -> Din13W<'_, Filteren15_0Spec> {
        Din13W::new(self, 26)
    }
    #[doc = "Bits 28:29 - Programmable counter length of digital glitch filter for DIN14"]
    #[inline(always)]
    pub fn din14(&mut self) -> Din14W<'_, Filteren15_0Spec> {
        Din14W::new(self, 28)
    }
    #[doc = "Bits 30:31 - Programmable counter length of digital glitch filter for DIN15"]
    #[inline(always)]
    pub fn din15(&mut self) -> Din15W<'_, Filteren15_0Spec> {
        Din15W::new(self, 30)
    }
}
#[doc = "Filter Enable 15 to 0\n\nYou can [`read`](crate::Reg::read) this register and get [`filteren15_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`filteren15_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Filteren15_0Spec;
impl crate::RegisterSpec for Filteren15_0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`filteren15_0::R`](R) reader structure"]
impl crate::Readable for Filteren15_0Spec {}
#[doc = "`write(|w| ..)` method takes [`filteren15_0::W`](W) writer structure"]
impl crate::Writable for Filteren15_0Spec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets FILTEREN15_0 to value 0"]
impl crate::Resettable for Filteren15_0Spec {}
