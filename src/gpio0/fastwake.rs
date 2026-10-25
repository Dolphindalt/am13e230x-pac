#[doc = "Register `FASTWAKE` reader"]
pub type R = crate::R<FastwakeSpec>;
#[doc = "Register `FASTWAKE` writer"]
pub type W = crate::W<FastwakeSpec>;
#[doc = "Enable fastwake feature for DIN0\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Din0 {
    #[doc = "0: fastwake feature is disabled"]
    Disable = 0,
    #[doc = "1: fastwake feature is enabled"]
    Enable = 1,
}
impl From<Din0> for bool {
    #[inline(always)]
    fn from(variant: Din0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIN0` reader - Enable fastwake feature for DIN0"]
pub type Din0R = crate::BitReader<Din0>;
impl Din0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Din0 {
        match self.bits {
            false => Din0::Disable,
            true => Din0::Enable,
        }
    }
    #[doc = "fastwake feature is disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Din0::Disable
    }
    #[doc = "fastwake feature is enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Din0::Enable
    }
}
#[doc = "Field `DIN0` writer - Enable fastwake feature for DIN0"]
pub type Din0W<'a, REG> = crate::BitWriter<'a, REG, Din0>;
impl<'a, REG> Din0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "fastwake feature is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Din0::Disable)
    }
    #[doc = "fastwake feature is enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Din0::Enable)
    }
}
#[doc = "Enable fastwake feature for DIN1\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Din1 {
    #[doc = "0: fastwake feature is disabled"]
    Disable = 0,
    #[doc = "1: fastwake feature is enabled"]
    Enable = 1,
}
impl From<Din1> for bool {
    #[inline(always)]
    fn from(variant: Din1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIN1` reader - Enable fastwake feature for DIN1"]
pub type Din1R = crate::BitReader<Din1>;
impl Din1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Din1 {
        match self.bits {
            false => Din1::Disable,
            true => Din1::Enable,
        }
    }
    #[doc = "fastwake feature is disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Din1::Disable
    }
    #[doc = "fastwake feature is enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Din1::Enable
    }
}
#[doc = "Field `DIN1` writer - Enable fastwake feature for DIN1"]
pub type Din1W<'a, REG> = crate::BitWriter<'a, REG, Din1>;
impl<'a, REG> Din1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "fastwake feature is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Din1::Disable)
    }
    #[doc = "fastwake feature is enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Din1::Enable)
    }
}
#[doc = "Enable fastwake feature for DIN2\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Din2 {
    #[doc = "0: fastwake feature is disabled"]
    Disable = 0,
    #[doc = "1: fastwake feature is enabled"]
    Enable = 1,
}
impl From<Din2> for bool {
    #[inline(always)]
    fn from(variant: Din2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIN2` reader - Enable fastwake feature for DIN2"]
pub type Din2R = crate::BitReader<Din2>;
impl Din2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Din2 {
        match self.bits {
            false => Din2::Disable,
            true => Din2::Enable,
        }
    }
    #[doc = "fastwake feature is disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Din2::Disable
    }
    #[doc = "fastwake feature is enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Din2::Enable
    }
}
#[doc = "Field `DIN2` writer - Enable fastwake feature for DIN2"]
pub type Din2W<'a, REG> = crate::BitWriter<'a, REG, Din2>;
impl<'a, REG> Din2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "fastwake feature is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Din2::Disable)
    }
    #[doc = "fastwake feature is enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Din2::Enable)
    }
}
#[doc = "Enable fastwake feature for DIN3\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Din3 {
    #[doc = "0: fastwake feature is disabled"]
    Disable = 0,
    #[doc = "1: fastwake feature is enabled"]
    Enable = 1,
}
impl From<Din3> for bool {
    #[inline(always)]
    fn from(variant: Din3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIN3` reader - Enable fastwake feature for DIN3"]
pub type Din3R = crate::BitReader<Din3>;
impl Din3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Din3 {
        match self.bits {
            false => Din3::Disable,
            true => Din3::Enable,
        }
    }
    #[doc = "fastwake feature is disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Din3::Disable
    }
    #[doc = "fastwake feature is enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Din3::Enable
    }
}
#[doc = "Field `DIN3` writer - Enable fastwake feature for DIN3"]
pub type Din3W<'a, REG> = crate::BitWriter<'a, REG, Din3>;
impl<'a, REG> Din3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "fastwake feature is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Din3::Disable)
    }
    #[doc = "fastwake feature is enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Din3::Enable)
    }
}
#[doc = "Enable fastwake feature for DIN4\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Din4 {
    #[doc = "0: fastwake feature is disabled"]
    Disable = 0,
    #[doc = "1: fastwake feature is enabled"]
    Enable = 1,
}
impl From<Din4> for bool {
    #[inline(always)]
    fn from(variant: Din4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIN4` reader - Enable fastwake feature for DIN4"]
pub type Din4R = crate::BitReader<Din4>;
impl Din4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Din4 {
        match self.bits {
            false => Din4::Disable,
            true => Din4::Enable,
        }
    }
    #[doc = "fastwake feature is disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Din4::Disable
    }
    #[doc = "fastwake feature is enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Din4::Enable
    }
}
#[doc = "Field `DIN4` writer - Enable fastwake feature for DIN4"]
pub type Din4W<'a, REG> = crate::BitWriter<'a, REG, Din4>;
impl<'a, REG> Din4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "fastwake feature is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Din4::Disable)
    }
    #[doc = "fastwake feature is enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Din4::Enable)
    }
}
#[doc = "Enable fastwake feature for DIN5\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Din5 {
    #[doc = "0: fastwake feature is disabled"]
    Disable = 0,
    #[doc = "1: fastwake feature is enabled"]
    Enable = 1,
}
impl From<Din5> for bool {
    #[inline(always)]
    fn from(variant: Din5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIN5` reader - Enable fastwake feature for DIN5"]
pub type Din5R = crate::BitReader<Din5>;
impl Din5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Din5 {
        match self.bits {
            false => Din5::Disable,
            true => Din5::Enable,
        }
    }
    #[doc = "fastwake feature is disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Din5::Disable
    }
    #[doc = "fastwake feature is enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Din5::Enable
    }
}
#[doc = "Field `DIN5` writer - Enable fastwake feature for DIN5"]
pub type Din5W<'a, REG> = crate::BitWriter<'a, REG, Din5>;
impl<'a, REG> Din5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "fastwake feature is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Din5::Disable)
    }
    #[doc = "fastwake feature is enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Din5::Enable)
    }
}
#[doc = "Enable fastwake feature for DIN6\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Din6 {
    #[doc = "0: fastwake feature is disabled"]
    Disable = 0,
    #[doc = "1: fastwake feature is enabled"]
    Enable = 1,
}
impl From<Din6> for bool {
    #[inline(always)]
    fn from(variant: Din6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIN6` reader - Enable fastwake feature for DIN6"]
pub type Din6R = crate::BitReader<Din6>;
impl Din6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Din6 {
        match self.bits {
            false => Din6::Disable,
            true => Din6::Enable,
        }
    }
    #[doc = "fastwake feature is disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Din6::Disable
    }
    #[doc = "fastwake feature is enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Din6::Enable
    }
}
#[doc = "Field `DIN6` writer - Enable fastwake feature for DIN6"]
pub type Din6W<'a, REG> = crate::BitWriter<'a, REG, Din6>;
impl<'a, REG> Din6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "fastwake feature is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Din6::Disable)
    }
    #[doc = "fastwake feature is enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Din6::Enable)
    }
}
#[doc = "Enable fastwake feature for DIN7\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Din7 {
    #[doc = "0: fastwake feature is disabled"]
    Disable = 0,
    #[doc = "1: fastwake feature is enabled"]
    Enable = 1,
}
impl From<Din7> for bool {
    #[inline(always)]
    fn from(variant: Din7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIN7` reader - Enable fastwake feature for DIN7"]
pub type Din7R = crate::BitReader<Din7>;
impl Din7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Din7 {
        match self.bits {
            false => Din7::Disable,
            true => Din7::Enable,
        }
    }
    #[doc = "fastwake feature is disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Din7::Disable
    }
    #[doc = "fastwake feature is enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Din7::Enable
    }
}
#[doc = "Field `DIN7` writer - Enable fastwake feature for DIN7"]
pub type Din7W<'a, REG> = crate::BitWriter<'a, REG, Din7>;
impl<'a, REG> Din7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "fastwake feature is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Din7::Disable)
    }
    #[doc = "fastwake feature is enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Din7::Enable)
    }
}
#[doc = "Enable fastwake feature for DIN8\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Din8 {
    #[doc = "0: fastwake feature is disabled"]
    Disable = 0,
    #[doc = "1: fastwake feature is enabled"]
    Enable = 1,
}
impl From<Din8> for bool {
    #[inline(always)]
    fn from(variant: Din8) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIN8` reader - Enable fastwake feature for DIN8"]
pub type Din8R = crate::BitReader<Din8>;
impl Din8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Din8 {
        match self.bits {
            false => Din8::Disable,
            true => Din8::Enable,
        }
    }
    #[doc = "fastwake feature is disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Din8::Disable
    }
    #[doc = "fastwake feature is enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Din8::Enable
    }
}
#[doc = "Field `DIN8` writer - Enable fastwake feature for DIN8"]
pub type Din8W<'a, REG> = crate::BitWriter<'a, REG, Din8>;
impl<'a, REG> Din8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "fastwake feature is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Din8::Disable)
    }
    #[doc = "fastwake feature is enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Din8::Enable)
    }
}
#[doc = "Enable fastwake feature for DIN9\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Din9 {
    #[doc = "0: fastwake feature is disabled"]
    Disable = 0,
    #[doc = "1: fastwake feature is enabled"]
    Enable = 1,
}
impl From<Din9> for bool {
    #[inline(always)]
    fn from(variant: Din9) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIN9` reader - Enable fastwake feature for DIN9"]
pub type Din9R = crate::BitReader<Din9>;
impl Din9R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Din9 {
        match self.bits {
            false => Din9::Disable,
            true => Din9::Enable,
        }
    }
    #[doc = "fastwake feature is disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Din9::Disable
    }
    #[doc = "fastwake feature is enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Din9::Enable
    }
}
#[doc = "Field `DIN9` writer - Enable fastwake feature for DIN9"]
pub type Din9W<'a, REG> = crate::BitWriter<'a, REG, Din9>;
impl<'a, REG> Din9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "fastwake feature is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Din9::Disable)
    }
    #[doc = "fastwake feature is enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Din9::Enable)
    }
}
#[doc = "Enable fastwake feature for DIN10\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Din10 {
    #[doc = "0: fastwake feature is disabled"]
    Disable = 0,
    #[doc = "1: fastwake feature is enabled"]
    Enable = 1,
}
impl From<Din10> for bool {
    #[inline(always)]
    fn from(variant: Din10) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIN10` reader - Enable fastwake feature for DIN10"]
pub type Din10R = crate::BitReader<Din10>;
impl Din10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Din10 {
        match self.bits {
            false => Din10::Disable,
            true => Din10::Enable,
        }
    }
    #[doc = "fastwake feature is disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Din10::Disable
    }
    #[doc = "fastwake feature is enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Din10::Enable
    }
}
#[doc = "Field `DIN10` writer - Enable fastwake feature for DIN10"]
pub type Din10W<'a, REG> = crate::BitWriter<'a, REG, Din10>;
impl<'a, REG> Din10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "fastwake feature is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Din10::Disable)
    }
    #[doc = "fastwake feature is enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Din10::Enable)
    }
}
#[doc = "Enable fastwake feature for DIN11\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Din11 {
    #[doc = "0: fastwake feature is disabled"]
    Disable = 0,
    #[doc = "1: fastwake feature is enabled"]
    Enable = 1,
}
impl From<Din11> for bool {
    #[inline(always)]
    fn from(variant: Din11) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIN11` reader - Enable fastwake feature for DIN11"]
pub type Din11R = crate::BitReader<Din11>;
impl Din11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Din11 {
        match self.bits {
            false => Din11::Disable,
            true => Din11::Enable,
        }
    }
    #[doc = "fastwake feature is disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Din11::Disable
    }
    #[doc = "fastwake feature is enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Din11::Enable
    }
}
#[doc = "Field `DIN11` writer - Enable fastwake feature for DIN11"]
pub type Din11W<'a, REG> = crate::BitWriter<'a, REG, Din11>;
impl<'a, REG> Din11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "fastwake feature is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Din11::Disable)
    }
    #[doc = "fastwake feature is enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Din11::Enable)
    }
}
#[doc = "Enable fastwake feature for DIN12\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Din12 {
    #[doc = "0: fastwake feature is disabled"]
    Disable = 0,
    #[doc = "1: fastwake feature is enabled"]
    Enable = 1,
}
impl From<Din12> for bool {
    #[inline(always)]
    fn from(variant: Din12) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIN12` reader - Enable fastwake feature for DIN12"]
pub type Din12R = crate::BitReader<Din12>;
impl Din12R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Din12 {
        match self.bits {
            false => Din12::Disable,
            true => Din12::Enable,
        }
    }
    #[doc = "fastwake feature is disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Din12::Disable
    }
    #[doc = "fastwake feature is enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Din12::Enable
    }
}
#[doc = "Field `DIN12` writer - Enable fastwake feature for DIN12"]
pub type Din12W<'a, REG> = crate::BitWriter<'a, REG, Din12>;
impl<'a, REG> Din12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "fastwake feature is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Din12::Disable)
    }
    #[doc = "fastwake feature is enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Din12::Enable)
    }
}
#[doc = "Enable fastwake feature for DIN13\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Din13 {
    #[doc = "0: fastwake feature is disabled"]
    Disable = 0,
    #[doc = "1: fastwake feature is enabled"]
    Enable = 1,
}
impl From<Din13> for bool {
    #[inline(always)]
    fn from(variant: Din13) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIN13` reader - Enable fastwake feature for DIN13"]
pub type Din13R = crate::BitReader<Din13>;
impl Din13R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Din13 {
        match self.bits {
            false => Din13::Disable,
            true => Din13::Enable,
        }
    }
    #[doc = "fastwake feature is disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Din13::Disable
    }
    #[doc = "fastwake feature is enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Din13::Enable
    }
}
#[doc = "Field `DIN13` writer - Enable fastwake feature for DIN13"]
pub type Din13W<'a, REG> = crate::BitWriter<'a, REG, Din13>;
impl<'a, REG> Din13W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "fastwake feature is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Din13::Disable)
    }
    #[doc = "fastwake feature is enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Din13::Enable)
    }
}
#[doc = "Enable fastwake feature for DIN14\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Din14 {
    #[doc = "0: fastwake feature is disabled"]
    Disable = 0,
    #[doc = "1: fastwake feature is enabled"]
    Enable = 1,
}
impl From<Din14> for bool {
    #[inline(always)]
    fn from(variant: Din14) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIN14` reader - Enable fastwake feature for DIN14"]
pub type Din14R = crate::BitReader<Din14>;
impl Din14R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Din14 {
        match self.bits {
            false => Din14::Disable,
            true => Din14::Enable,
        }
    }
    #[doc = "fastwake feature is disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Din14::Disable
    }
    #[doc = "fastwake feature is enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Din14::Enable
    }
}
#[doc = "Field `DIN14` writer - Enable fastwake feature for DIN14"]
pub type Din14W<'a, REG> = crate::BitWriter<'a, REG, Din14>;
impl<'a, REG> Din14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "fastwake feature is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Din14::Disable)
    }
    #[doc = "fastwake feature is enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Din14::Enable)
    }
}
#[doc = "Enable fastwake feature for DIN15\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Din15 {
    #[doc = "0: fastwake feature is disabled"]
    Disable = 0,
    #[doc = "1: fastwake feature is enabled"]
    Enable = 1,
}
impl From<Din15> for bool {
    #[inline(always)]
    fn from(variant: Din15) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIN15` reader - Enable fastwake feature for DIN15"]
pub type Din15R = crate::BitReader<Din15>;
impl Din15R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Din15 {
        match self.bits {
            false => Din15::Disable,
            true => Din15::Enable,
        }
    }
    #[doc = "fastwake feature is disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Din15::Disable
    }
    #[doc = "fastwake feature is enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Din15::Enable
    }
}
#[doc = "Field `DIN15` writer - Enable fastwake feature for DIN15"]
pub type Din15W<'a, REG> = crate::BitWriter<'a, REG, Din15>;
impl<'a, REG> Din15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "fastwake feature is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Din15::Disable)
    }
    #[doc = "fastwake feature is enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Din15::Enable)
    }
}
#[doc = "Enable fastwake feature for DIN16\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Din16 {
    #[doc = "0: fastwake feature is disabled"]
    Disable = 0,
    #[doc = "1: fastwake feature is enabled"]
    Enable = 1,
}
impl From<Din16> for bool {
    #[inline(always)]
    fn from(variant: Din16) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIN16` reader - Enable fastwake feature for DIN16"]
pub type Din16R = crate::BitReader<Din16>;
impl Din16R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Din16 {
        match self.bits {
            false => Din16::Disable,
            true => Din16::Enable,
        }
    }
    #[doc = "fastwake feature is disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Din16::Disable
    }
    #[doc = "fastwake feature is enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Din16::Enable
    }
}
#[doc = "Field `DIN16` writer - Enable fastwake feature for DIN16"]
pub type Din16W<'a, REG> = crate::BitWriter<'a, REG, Din16>;
impl<'a, REG> Din16W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "fastwake feature is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Din16::Disable)
    }
    #[doc = "fastwake feature is enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Din16::Enable)
    }
}
#[doc = "Enable fastwake feature for DIN17\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Din17 {
    #[doc = "0: fastwake feature is disabled"]
    Disable = 0,
    #[doc = "1: fastwake feature is enabled"]
    Enable = 1,
}
impl From<Din17> for bool {
    #[inline(always)]
    fn from(variant: Din17) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIN17` reader - Enable fastwake feature for DIN17"]
pub type Din17R = crate::BitReader<Din17>;
impl Din17R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Din17 {
        match self.bits {
            false => Din17::Disable,
            true => Din17::Enable,
        }
    }
    #[doc = "fastwake feature is disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Din17::Disable
    }
    #[doc = "fastwake feature is enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Din17::Enable
    }
}
#[doc = "Field `DIN17` writer - Enable fastwake feature for DIN17"]
pub type Din17W<'a, REG> = crate::BitWriter<'a, REG, Din17>;
impl<'a, REG> Din17W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "fastwake feature is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Din17::Disable)
    }
    #[doc = "fastwake feature is enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Din17::Enable)
    }
}
#[doc = "Enable fastwake feature for DIN18\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Din18 {
    #[doc = "0: fastwake feature is disabled"]
    Disable = 0,
    #[doc = "1: fastwake feature is enabled"]
    Enable = 1,
}
impl From<Din18> for bool {
    #[inline(always)]
    fn from(variant: Din18) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIN18` reader - Enable fastwake feature for DIN18"]
pub type Din18R = crate::BitReader<Din18>;
impl Din18R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Din18 {
        match self.bits {
            false => Din18::Disable,
            true => Din18::Enable,
        }
    }
    #[doc = "fastwake feature is disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Din18::Disable
    }
    #[doc = "fastwake feature is enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Din18::Enable
    }
}
#[doc = "Field `DIN18` writer - Enable fastwake feature for DIN18"]
pub type Din18W<'a, REG> = crate::BitWriter<'a, REG, Din18>;
impl<'a, REG> Din18W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "fastwake feature is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Din18::Disable)
    }
    #[doc = "fastwake feature is enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Din18::Enable)
    }
}
#[doc = "Enable fastwake feature for DIN19\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Din19 {
    #[doc = "0: fastwake feature is disabled"]
    Disable = 0,
    #[doc = "1: fastwake feature is enabled"]
    Enable = 1,
}
impl From<Din19> for bool {
    #[inline(always)]
    fn from(variant: Din19) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIN19` reader - Enable fastwake feature for DIN19"]
pub type Din19R = crate::BitReader<Din19>;
impl Din19R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Din19 {
        match self.bits {
            false => Din19::Disable,
            true => Din19::Enable,
        }
    }
    #[doc = "fastwake feature is disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Din19::Disable
    }
    #[doc = "fastwake feature is enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Din19::Enable
    }
}
#[doc = "Field `DIN19` writer - Enable fastwake feature for DIN19"]
pub type Din19W<'a, REG> = crate::BitWriter<'a, REG, Din19>;
impl<'a, REG> Din19W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "fastwake feature is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Din19::Disable)
    }
    #[doc = "fastwake feature is enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Din19::Enable)
    }
}
#[doc = "Enable fastwake feature for DIN20\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Din20 {
    #[doc = "0: fastwake feature is disabled"]
    Disable = 0,
    #[doc = "1: fastwake feature is enabled"]
    Enable = 1,
}
impl From<Din20> for bool {
    #[inline(always)]
    fn from(variant: Din20) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIN20` reader - Enable fastwake feature for DIN20"]
pub type Din20R = crate::BitReader<Din20>;
impl Din20R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Din20 {
        match self.bits {
            false => Din20::Disable,
            true => Din20::Enable,
        }
    }
    #[doc = "fastwake feature is disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Din20::Disable
    }
    #[doc = "fastwake feature is enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Din20::Enable
    }
}
#[doc = "Field `DIN20` writer - Enable fastwake feature for DIN20"]
pub type Din20W<'a, REG> = crate::BitWriter<'a, REG, Din20>;
impl<'a, REG> Din20W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "fastwake feature is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Din20::Disable)
    }
    #[doc = "fastwake feature is enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Din20::Enable)
    }
}
#[doc = "Enable fastwake feature for DIN21\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Din21 {
    #[doc = "0: fastwake feature is disabled"]
    Disable = 0,
    #[doc = "1: fastwake feature is enabled"]
    Enable = 1,
}
impl From<Din21> for bool {
    #[inline(always)]
    fn from(variant: Din21) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIN21` reader - Enable fastwake feature for DIN21"]
pub type Din21R = crate::BitReader<Din21>;
impl Din21R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Din21 {
        match self.bits {
            false => Din21::Disable,
            true => Din21::Enable,
        }
    }
    #[doc = "fastwake feature is disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Din21::Disable
    }
    #[doc = "fastwake feature is enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Din21::Enable
    }
}
#[doc = "Field `DIN21` writer - Enable fastwake feature for DIN21"]
pub type Din21W<'a, REG> = crate::BitWriter<'a, REG, Din21>;
impl<'a, REG> Din21W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "fastwake feature is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Din21::Disable)
    }
    #[doc = "fastwake feature is enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Din21::Enable)
    }
}
#[doc = "Enable fastwake feature for DIN22\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Din22 {
    #[doc = "0: fastwake feature is disabled"]
    Disable = 0,
    #[doc = "1: fastwake feature is enabled"]
    Enable = 1,
}
impl From<Din22> for bool {
    #[inline(always)]
    fn from(variant: Din22) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIN22` reader - Enable fastwake feature for DIN22"]
pub type Din22R = crate::BitReader<Din22>;
impl Din22R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Din22 {
        match self.bits {
            false => Din22::Disable,
            true => Din22::Enable,
        }
    }
    #[doc = "fastwake feature is disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Din22::Disable
    }
    #[doc = "fastwake feature is enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Din22::Enable
    }
}
#[doc = "Field `DIN22` writer - Enable fastwake feature for DIN22"]
pub type Din22W<'a, REG> = crate::BitWriter<'a, REG, Din22>;
impl<'a, REG> Din22W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "fastwake feature is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Din22::Disable)
    }
    #[doc = "fastwake feature is enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Din22::Enable)
    }
}
#[doc = "Enable fastwake feature for DIN23\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Din23 {
    #[doc = "0: fastwake feature is disabled"]
    Disable = 0,
    #[doc = "1: fastwake feature is enabled"]
    Enable = 1,
}
impl From<Din23> for bool {
    #[inline(always)]
    fn from(variant: Din23) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIN23` reader - Enable fastwake feature for DIN23"]
pub type Din23R = crate::BitReader<Din23>;
impl Din23R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Din23 {
        match self.bits {
            false => Din23::Disable,
            true => Din23::Enable,
        }
    }
    #[doc = "fastwake feature is disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Din23::Disable
    }
    #[doc = "fastwake feature is enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Din23::Enable
    }
}
#[doc = "Field `DIN23` writer - Enable fastwake feature for DIN23"]
pub type Din23W<'a, REG> = crate::BitWriter<'a, REG, Din23>;
impl<'a, REG> Din23W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "fastwake feature is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Din23::Disable)
    }
    #[doc = "fastwake feature is enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Din23::Enable)
    }
}
#[doc = "Enable fastwake feature for DIN24\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Din24 {
    #[doc = "0: fastwake feature is disabled"]
    Disable = 0,
    #[doc = "1: fastwake feature is enabled"]
    Enable = 1,
}
impl From<Din24> for bool {
    #[inline(always)]
    fn from(variant: Din24) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIN24` reader - Enable fastwake feature for DIN24"]
pub type Din24R = crate::BitReader<Din24>;
impl Din24R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Din24 {
        match self.bits {
            false => Din24::Disable,
            true => Din24::Enable,
        }
    }
    #[doc = "fastwake feature is disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Din24::Disable
    }
    #[doc = "fastwake feature is enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Din24::Enable
    }
}
#[doc = "Field `DIN24` writer - Enable fastwake feature for DIN24"]
pub type Din24W<'a, REG> = crate::BitWriter<'a, REG, Din24>;
impl<'a, REG> Din24W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "fastwake feature is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Din24::Disable)
    }
    #[doc = "fastwake feature is enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Din24::Enable)
    }
}
#[doc = "Enable fastwake feature for DIN25\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Din25 {
    #[doc = "0: fastwake feature is disabled"]
    Disable = 0,
    #[doc = "1: fastwake feature is enabled"]
    Enable = 1,
}
impl From<Din25> for bool {
    #[inline(always)]
    fn from(variant: Din25) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIN25` reader - Enable fastwake feature for DIN25"]
pub type Din25R = crate::BitReader<Din25>;
impl Din25R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Din25 {
        match self.bits {
            false => Din25::Disable,
            true => Din25::Enable,
        }
    }
    #[doc = "fastwake feature is disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Din25::Disable
    }
    #[doc = "fastwake feature is enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Din25::Enable
    }
}
#[doc = "Field `DIN25` writer - Enable fastwake feature for DIN25"]
pub type Din25W<'a, REG> = crate::BitWriter<'a, REG, Din25>;
impl<'a, REG> Din25W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "fastwake feature is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Din25::Disable)
    }
    #[doc = "fastwake feature is enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Din25::Enable)
    }
}
#[doc = "Enable fastwake feature for DIN26\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Din26 {
    #[doc = "0: fastwake feature is disabled"]
    Disable = 0,
    #[doc = "1: fastwake feature is enabled"]
    Enable = 1,
}
impl From<Din26> for bool {
    #[inline(always)]
    fn from(variant: Din26) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIN26` reader - Enable fastwake feature for DIN26"]
pub type Din26R = crate::BitReader<Din26>;
impl Din26R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Din26 {
        match self.bits {
            false => Din26::Disable,
            true => Din26::Enable,
        }
    }
    #[doc = "fastwake feature is disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Din26::Disable
    }
    #[doc = "fastwake feature is enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Din26::Enable
    }
}
#[doc = "Field `DIN26` writer - Enable fastwake feature for DIN26"]
pub type Din26W<'a, REG> = crate::BitWriter<'a, REG, Din26>;
impl<'a, REG> Din26W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "fastwake feature is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Din26::Disable)
    }
    #[doc = "fastwake feature is enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Din26::Enable)
    }
}
#[doc = "Enable fastwake feature for DIN27\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Din27 {
    #[doc = "0: fastwake feature is disabled"]
    Disable = 0,
    #[doc = "1: fastwake feature is enabled"]
    Enable = 1,
}
impl From<Din27> for bool {
    #[inline(always)]
    fn from(variant: Din27) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIN27` reader - Enable fastwake feature for DIN27"]
pub type Din27R = crate::BitReader<Din27>;
impl Din27R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Din27 {
        match self.bits {
            false => Din27::Disable,
            true => Din27::Enable,
        }
    }
    #[doc = "fastwake feature is disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Din27::Disable
    }
    #[doc = "fastwake feature is enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Din27::Enable
    }
}
#[doc = "Field `DIN27` writer - Enable fastwake feature for DIN27"]
pub type Din27W<'a, REG> = crate::BitWriter<'a, REG, Din27>;
impl<'a, REG> Din27W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "fastwake feature is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Din27::Disable)
    }
    #[doc = "fastwake feature is enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Din27::Enable)
    }
}
#[doc = "Enable fastwake feature for DIN29\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Din28 {
    #[doc = "0: fastwake feature is disabled"]
    Disable = 0,
    #[doc = "1: fastwake feature is enabled"]
    Enable = 1,
}
impl From<Din28> for bool {
    #[inline(always)]
    fn from(variant: Din28) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIN28` reader - Enable fastwake feature for DIN29"]
pub type Din28R = crate::BitReader<Din28>;
impl Din28R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Din28 {
        match self.bits {
            false => Din28::Disable,
            true => Din28::Enable,
        }
    }
    #[doc = "fastwake feature is disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Din28::Disable
    }
    #[doc = "fastwake feature is enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Din28::Enable
    }
}
#[doc = "Field `DIN28` writer - Enable fastwake feature for DIN29"]
pub type Din28W<'a, REG> = crate::BitWriter<'a, REG, Din28>;
impl<'a, REG> Din28W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "fastwake feature is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Din28::Disable)
    }
    #[doc = "fastwake feature is enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Din28::Enable)
    }
}
#[doc = "Enable fastwake feature for DIN29\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Din29 {
    #[doc = "0: fastwake feature is disabled"]
    Disable = 0,
    #[doc = "1: fastwake feature is enabled"]
    Enable = 1,
}
impl From<Din29> for bool {
    #[inline(always)]
    fn from(variant: Din29) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIN29` reader - Enable fastwake feature for DIN29"]
pub type Din29R = crate::BitReader<Din29>;
impl Din29R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Din29 {
        match self.bits {
            false => Din29::Disable,
            true => Din29::Enable,
        }
    }
    #[doc = "fastwake feature is disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Din29::Disable
    }
    #[doc = "fastwake feature is enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Din29::Enable
    }
}
#[doc = "Field `DIN29` writer - Enable fastwake feature for DIN29"]
pub type Din29W<'a, REG> = crate::BitWriter<'a, REG, Din29>;
impl<'a, REG> Din29W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "fastwake feature is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Din29::Disable)
    }
    #[doc = "fastwake feature is enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Din29::Enable)
    }
}
#[doc = "Enable fastwake feature for DIN30\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Din30 {
    #[doc = "0: fastwake feature is disabled"]
    Disable = 0,
    #[doc = "1: fastwake feature is enabled"]
    Enable = 1,
}
impl From<Din30> for bool {
    #[inline(always)]
    fn from(variant: Din30) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIN30` reader - Enable fastwake feature for DIN30"]
pub type Din30R = crate::BitReader<Din30>;
impl Din30R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Din30 {
        match self.bits {
            false => Din30::Disable,
            true => Din30::Enable,
        }
    }
    #[doc = "fastwake feature is disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Din30::Disable
    }
    #[doc = "fastwake feature is enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Din30::Enable
    }
}
#[doc = "Field `DIN30` writer - Enable fastwake feature for DIN30"]
pub type Din30W<'a, REG> = crate::BitWriter<'a, REG, Din30>;
impl<'a, REG> Din30W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "fastwake feature is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Din30::Disable)
    }
    #[doc = "fastwake feature is enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Din30::Enable)
    }
}
#[doc = "Enable fastwake feature for DIN31\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Din31 {
    #[doc = "0: fastwake feature is disabled"]
    Disable = 0,
    #[doc = "1: fastwake feature is enabled"]
    Enable = 1,
}
impl From<Din31> for bool {
    #[inline(always)]
    fn from(variant: Din31) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIN31` reader - Enable fastwake feature for DIN31"]
pub type Din31R = crate::BitReader<Din31>;
impl Din31R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Din31 {
        match self.bits {
            false => Din31::Disable,
            true => Din31::Enable,
        }
    }
    #[doc = "fastwake feature is disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Din31::Disable
    }
    #[doc = "fastwake feature is enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Din31::Enable
    }
}
#[doc = "Field `DIN31` writer - Enable fastwake feature for DIN31"]
pub type Din31W<'a, REG> = crate::BitWriter<'a, REG, Din31>;
impl<'a, REG> Din31W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "fastwake feature is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Din31::Disable)
    }
    #[doc = "fastwake feature is enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Din31::Enable)
    }
}
impl R {
    #[doc = "Bit 0 - Enable fastwake feature for DIN0"]
    #[inline(always)]
    pub fn din0(&self) -> Din0R {
        Din0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable fastwake feature for DIN1"]
    #[inline(always)]
    pub fn din1(&self) -> Din1R {
        Din1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable fastwake feature for DIN2"]
    #[inline(always)]
    pub fn din2(&self) -> Din2R {
        Din2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable fastwake feature for DIN3"]
    #[inline(always)]
    pub fn din3(&self) -> Din3R {
        Din3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable fastwake feature for DIN4"]
    #[inline(always)]
    pub fn din4(&self) -> Din4R {
        Din4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable fastwake feature for DIN5"]
    #[inline(always)]
    pub fn din5(&self) -> Din5R {
        Din5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable fastwake feature for DIN6"]
    #[inline(always)]
    pub fn din6(&self) -> Din6R {
        Din6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable fastwake feature for DIN7"]
    #[inline(always)]
    pub fn din7(&self) -> Din7R {
        Din7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable fastwake feature for DIN8"]
    #[inline(always)]
    pub fn din8(&self) -> Din8R {
        Din8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable fastwake feature for DIN9"]
    #[inline(always)]
    pub fn din9(&self) -> Din9R {
        Din9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable fastwake feature for DIN10"]
    #[inline(always)]
    pub fn din10(&self) -> Din10R {
        Din10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable fastwake feature for DIN11"]
    #[inline(always)]
    pub fn din11(&self) -> Din11R {
        Din11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable fastwake feature for DIN12"]
    #[inline(always)]
    pub fn din12(&self) -> Din12R {
        Din12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable fastwake feature for DIN13"]
    #[inline(always)]
    pub fn din13(&self) -> Din13R {
        Din13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Enable fastwake feature for DIN14"]
    #[inline(always)]
    pub fn din14(&self) -> Din14R {
        Din14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enable fastwake feature for DIN15"]
    #[inline(always)]
    pub fn din15(&self) -> Din15R {
        Din15R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable fastwake feature for DIN16"]
    #[inline(always)]
    pub fn din16(&self) -> Din16R {
        Din16R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable fastwake feature for DIN17"]
    #[inline(always)]
    pub fn din17(&self) -> Din17R {
        Din17R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable fastwake feature for DIN18"]
    #[inline(always)]
    pub fn din18(&self) -> Din18R {
        Din18R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable fastwake feature for DIN19"]
    #[inline(always)]
    pub fn din19(&self) -> Din19R {
        Din19R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Enable fastwake feature for DIN20"]
    #[inline(always)]
    pub fn din20(&self) -> Din20R {
        Din20R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Enable fastwake feature for DIN21"]
    #[inline(always)]
    pub fn din21(&self) -> Din21R {
        Din21R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Enable fastwake feature for DIN22"]
    #[inline(always)]
    pub fn din22(&self) -> Din22R {
        Din22R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Enable fastwake feature for DIN23"]
    #[inline(always)]
    pub fn din23(&self) -> Din23R {
        Din23R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable fastwake feature for DIN24"]
    #[inline(always)]
    pub fn din24(&self) -> Din24R {
        Din24R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable fastwake feature for DIN25"]
    #[inline(always)]
    pub fn din25(&self) -> Din25R {
        Din25R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Enable fastwake feature for DIN26"]
    #[inline(always)]
    pub fn din26(&self) -> Din26R {
        Din26R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Enable fastwake feature for DIN27"]
    #[inline(always)]
    pub fn din27(&self) -> Din27R {
        Din27R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Enable fastwake feature for DIN29"]
    #[inline(always)]
    pub fn din28(&self) -> Din28R {
        Din28R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Enable fastwake feature for DIN29"]
    #[inline(always)]
    pub fn din29(&self) -> Din29R {
        Din29R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Enable fastwake feature for DIN30"]
    #[inline(always)]
    pub fn din30(&self) -> Din30R {
        Din30R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Enable fastwake feature for DIN31"]
    #[inline(always)]
    pub fn din31(&self) -> Din31R {
        Din31R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FASTWAKE")
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
    #[doc = "Bit 0 - Enable fastwake feature for DIN0"]
    #[inline(always)]
    pub fn din0(&mut self) -> Din0W<'_, FastwakeSpec> {
        Din0W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable fastwake feature for DIN1"]
    #[inline(always)]
    pub fn din1(&mut self) -> Din1W<'_, FastwakeSpec> {
        Din1W::new(self, 1)
    }
    #[doc = "Bit 2 - Enable fastwake feature for DIN2"]
    #[inline(always)]
    pub fn din2(&mut self) -> Din2W<'_, FastwakeSpec> {
        Din2W::new(self, 2)
    }
    #[doc = "Bit 3 - Enable fastwake feature for DIN3"]
    #[inline(always)]
    pub fn din3(&mut self) -> Din3W<'_, FastwakeSpec> {
        Din3W::new(self, 3)
    }
    #[doc = "Bit 4 - Enable fastwake feature for DIN4"]
    #[inline(always)]
    pub fn din4(&mut self) -> Din4W<'_, FastwakeSpec> {
        Din4W::new(self, 4)
    }
    #[doc = "Bit 5 - Enable fastwake feature for DIN5"]
    #[inline(always)]
    pub fn din5(&mut self) -> Din5W<'_, FastwakeSpec> {
        Din5W::new(self, 5)
    }
    #[doc = "Bit 6 - Enable fastwake feature for DIN6"]
    #[inline(always)]
    pub fn din6(&mut self) -> Din6W<'_, FastwakeSpec> {
        Din6W::new(self, 6)
    }
    #[doc = "Bit 7 - Enable fastwake feature for DIN7"]
    #[inline(always)]
    pub fn din7(&mut self) -> Din7W<'_, FastwakeSpec> {
        Din7W::new(self, 7)
    }
    #[doc = "Bit 8 - Enable fastwake feature for DIN8"]
    #[inline(always)]
    pub fn din8(&mut self) -> Din8W<'_, FastwakeSpec> {
        Din8W::new(self, 8)
    }
    #[doc = "Bit 9 - Enable fastwake feature for DIN9"]
    #[inline(always)]
    pub fn din9(&mut self) -> Din9W<'_, FastwakeSpec> {
        Din9W::new(self, 9)
    }
    #[doc = "Bit 10 - Enable fastwake feature for DIN10"]
    #[inline(always)]
    pub fn din10(&mut self) -> Din10W<'_, FastwakeSpec> {
        Din10W::new(self, 10)
    }
    #[doc = "Bit 11 - Enable fastwake feature for DIN11"]
    #[inline(always)]
    pub fn din11(&mut self) -> Din11W<'_, FastwakeSpec> {
        Din11W::new(self, 11)
    }
    #[doc = "Bit 12 - Enable fastwake feature for DIN12"]
    #[inline(always)]
    pub fn din12(&mut self) -> Din12W<'_, FastwakeSpec> {
        Din12W::new(self, 12)
    }
    #[doc = "Bit 13 - Enable fastwake feature for DIN13"]
    #[inline(always)]
    pub fn din13(&mut self) -> Din13W<'_, FastwakeSpec> {
        Din13W::new(self, 13)
    }
    #[doc = "Bit 14 - Enable fastwake feature for DIN14"]
    #[inline(always)]
    pub fn din14(&mut self) -> Din14W<'_, FastwakeSpec> {
        Din14W::new(self, 14)
    }
    #[doc = "Bit 15 - Enable fastwake feature for DIN15"]
    #[inline(always)]
    pub fn din15(&mut self) -> Din15W<'_, FastwakeSpec> {
        Din15W::new(self, 15)
    }
    #[doc = "Bit 16 - Enable fastwake feature for DIN16"]
    #[inline(always)]
    pub fn din16(&mut self) -> Din16W<'_, FastwakeSpec> {
        Din16W::new(self, 16)
    }
    #[doc = "Bit 17 - Enable fastwake feature for DIN17"]
    #[inline(always)]
    pub fn din17(&mut self) -> Din17W<'_, FastwakeSpec> {
        Din17W::new(self, 17)
    }
    #[doc = "Bit 18 - Enable fastwake feature for DIN18"]
    #[inline(always)]
    pub fn din18(&mut self) -> Din18W<'_, FastwakeSpec> {
        Din18W::new(self, 18)
    }
    #[doc = "Bit 19 - Enable fastwake feature for DIN19"]
    #[inline(always)]
    pub fn din19(&mut self) -> Din19W<'_, FastwakeSpec> {
        Din19W::new(self, 19)
    }
    #[doc = "Bit 20 - Enable fastwake feature for DIN20"]
    #[inline(always)]
    pub fn din20(&mut self) -> Din20W<'_, FastwakeSpec> {
        Din20W::new(self, 20)
    }
    #[doc = "Bit 21 - Enable fastwake feature for DIN21"]
    #[inline(always)]
    pub fn din21(&mut self) -> Din21W<'_, FastwakeSpec> {
        Din21W::new(self, 21)
    }
    #[doc = "Bit 22 - Enable fastwake feature for DIN22"]
    #[inline(always)]
    pub fn din22(&mut self) -> Din22W<'_, FastwakeSpec> {
        Din22W::new(self, 22)
    }
    #[doc = "Bit 23 - Enable fastwake feature for DIN23"]
    #[inline(always)]
    pub fn din23(&mut self) -> Din23W<'_, FastwakeSpec> {
        Din23W::new(self, 23)
    }
    #[doc = "Bit 24 - Enable fastwake feature for DIN24"]
    #[inline(always)]
    pub fn din24(&mut self) -> Din24W<'_, FastwakeSpec> {
        Din24W::new(self, 24)
    }
    #[doc = "Bit 25 - Enable fastwake feature for DIN25"]
    #[inline(always)]
    pub fn din25(&mut self) -> Din25W<'_, FastwakeSpec> {
        Din25W::new(self, 25)
    }
    #[doc = "Bit 26 - Enable fastwake feature for DIN26"]
    #[inline(always)]
    pub fn din26(&mut self) -> Din26W<'_, FastwakeSpec> {
        Din26W::new(self, 26)
    }
    #[doc = "Bit 27 - Enable fastwake feature for DIN27"]
    #[inline(always)]
    pub fn din27(&mut self) -> Din27W<'_, FastwakeSpec> {
        Din27W::new(self, 27)
    }
    #[doc = "Bit 28 - Enable fastwake feature for DIN29"]
    #[inline(always)]
    pub fn din28(&mut self) -> Din28W<'_, FastwakeSpec> {
        Din28W::new(self, 28)
    }
    #[doc = "Bit 29 - Enable fastwake feature for DIN29"]
    #[inline(always)]
    pub fn din29(&mut self) -> Din29W<'_, FastwakeSpec> {
        Din29W::new(self, 29)
    }
    #[doc = "Bit 30 - Enable fastwake feature for DIN30"]
    #[inline(always)]
    pub fn din30(&mut self) -> Din30W<'_, FastwakeSpec> {
        Din30W::new(self, 30)
    }
    #[doc = "Bit 31 - Enable fastwake feature for DIN31"]
    #[inline(always)]
    pub fn din31(&mut self) -> Din31W<'_, FastwakeSpec> {
        Din31W::new(self, 31)
    }
}
#[doc = "FAST WAKE ENABLE\n\nYou can [`read`](crate::Reg::read) this register and get [`fastwake::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fastwake::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FastwakeSpec;
impl crate::RegisterSpec for FastwakeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fastwake::R`](R) reader structure"]
impl crate::Readable for FastwakeSpec {}
#[doc = "`write(|w| ..)` method takes [`fastwake::W`](W) writer structure"]
impl crate::Writable for FastwakeSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets FASTWAKE to value 0"]
impl crate::Resettable for FastwakeSpec {}
