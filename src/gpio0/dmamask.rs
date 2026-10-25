#[doc = "Register `DMAMASK` reader"]
pub type R = crate::R<DmamaskSpec>;
#[doc = "Register `DMAMASK` writer"]
pub type W = crate::W<DmamaskSpec>;
#[doc = "DMA is allowed to modify DOUT0\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dout0 {
    #[doc = "0: DMA is not allowed to modify this bit lane"]
    Disable = 0,
    #[doc = "1: DMA is allowed to modify this bit lane"]
    Enable = 1,
}
impl From<Dout0> for bool {
    #[inline(always)]
    fn from(variant: Dout0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUT0` reader - DMA is allowed to modify DOUT0"]
pub type Dout0R = crate::BitReader<Dout0>;
impl Dout0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dout0 {
        match self.bits {
            false => Dout0::Disable,
            true => Dout0::Enable,
        }
    }
    #[doc = "DMA is not allowed to modify this bit lane"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Dout0::Disable
    }
    #[doc = "DMA is allowed to modify this bit lane"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Dout0::Enable
    }
}
#[doc = "Field `DOUT0` writer - DMA is allowed to modify DOUT0"]
pub type Dout0W<'a, REG> = crate::BitWriter<'a, REG, Dout0>;
impl<'a, REG> Dout0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DMA is not allowed to modify this bit lane"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Dout0::Disable)
    }
    #[doc = "DMA is allowed to modify this bit lane"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Dout0::Enable)
    }
}
#[doc = "DMA is allowed to modify DOUT1\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dout1 {
    #[doc = "0: DMA is not allowed to modify this bit lane"]
    Disable = 0,
    #[doc = "1: DMA is allowed to modify this bit lane"]
    Enable = 1,
}
impl From<Dout1> for bool {
    #[inline(always)]
    fn from(variant: Dout1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUT1` reader - DMA is allowed to modify DOUT1"]
pub type Dout1R = crate::BitReader<Dout1>;
impl Dout1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dout1 {
        match self.bits {
            false => Dout1::Disable,
            true => Dout1::Enable,
        }
    }
    #[doc = "DMA is not allowed to modify this bit lane"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Dout1::Disable
    }
    #[doc = "DMA is allowed to modify this bit lane"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Dout1::Enable
    }
}
#[doc = "Field `DOUT1` writer - DMA is allowed to modify DOUT1"]
pub type Dout1W<'a, REG> = crate::BitWriter<'a, REG, Dout1>;
impl<'a, REG> Dout1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DMA is not allowed to modify this bit lane"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Dout1::Disable)
    }
    #[doc = "DMA is allowed to modify this bit lane"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Dout1::Enable)
    }
}
#[doc = "DMA is allowed to modify DOUT2\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dout2 {
    #[doc = "0: DMA is not allowed to modify this bit lane"]
    Disable = 0,
    #[doc = "1: DMA is allowed to modify this bit lane"]
    Enable = 1,
}
impl From<Dout2> for bool {
    #[inline(always)]
    fn from(variant: Dout2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUT2` reader - DMA is allowed to modify DOUT2"]
pub type Dout2R = crate::BitReader<Dout2>;
impl Dout2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dout2 {
        match self.bits {
            false => Dout2::Disable,
            true => Dout2::Enable,
        }
    }
    #[doc = "DMA is not allowed to modify this bit lane"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Dout2::Disable
    }
    #[doc = "DMA is allowed to modify this bit lane"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Dout2::Enable
    }
}
#[doc = "Field `DOUT2` writer - DMA is allowed to modify DOUT2"]
pub type Dout2W<'a, REG> = crate::BitWriter<'a, REG, Dout2>;
impl<'a, REG> Dout2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DMA is not allowed to modify this bit lane"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Dout2::Disable)
    }
    #[doc = "DMA is allowed to modify this bit lane"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Dout2::Enable)
    }
}
#[doc = "DMA is allowed to modify DOUT3\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dout3 {
    #[doc = "0: DMA is not allowed to modify this bit lane"]
    Disable = 0,
    #[doc = "1: DMA is allowed to modify this bit lane"]
    Enable = 1,
}
impl From<Dout3> for bool {
    #[inline(always)]
    fn from(variant: Dout3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUT3` reader - DMA is allowed to modify DOUT3"]
pub type Dout3R = crate::BitReader<Dout3>;
impl Dout3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dout3 {
        match self.bits {
            false => Dout3::Disable,
            true => Dout3::Enable,
        }
    }
    #[doc = "DMA is not allowed to modify this bit lane"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Dout3::Disable
    }
    #[doc = "DMA is allowed to modify this bit lane"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Dout3::Enable
    }
}
#[doc = "Field `DOUT3` writer - DMA is allowed to modify DOUT3"]
pub type Dout3W<'a, REG> = crate::BitWriter<'a, REG, Dout3>;
impl<'a, REG> Dout3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DMA is not allowed to modify this bit lane"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Dout3::Disable)
    }
    #[doc = "DMA is allowed to modify this bit lane"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Dout3::Enable)
    }
}
#[doc = "DMA is allowed to modify DOUT4\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dout4 {
    #[doc = "0: DMA is not allowed to modify this bit lane"]
    Disable = 0,
    #[doc = "1: DMA is allowed to modify this bit lane"]
    Enable = 1,
}
impl From<Dout4> for bool {
    #[inline(always)]
    fn from(variant: Dout4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUT4` reader - DMA is allowed to modify DOUT4"]
pub type Dout4R = crate::BitReader<Dout4>;
impl Dout4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dout4 {
        match self.bits {
            false => Dout4::Disable,
            true => Dout4::Enable,
        }
    }
    #[doc = "DMA is not allowed to modify this bit lane"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Dout4::Disable
    }
    #[doc = "DMA is allowed to modify this bit lane"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Dout4::Enable
    }
}
#[doc = "Field `DOUT4` writer - DMA is allowed to modify DOUT4"]
pub type Dout4W<'a, REG> = crate::BitWriter<'a, REG, Dout4>;
impl<'a, REG> Dout4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DMA is not allowed to modify this bit lane"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Dout4::Disable)
    }
    #[doc = "DMA is allowed to modify this bit lane"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Dout4::Enable)
    }
}
#[doc = "DMA is allowed to modify DOUT5\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dout5 {
    #[doc = "0: DMA is not allowed to modify this bit lane"]
    Disable = 0,
    #[doc = "1: DMA is allowed to modify this bit lane"]
    Enable = 1,
}
impl From<Dout5> for bool {
    #[inline(always)]
    fn from(variant: Dout5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUT5` reader - DMA is allowed to modify DOUT5"]
pub type Dout5R = crate::BitReader<Dout5>;
impl Dout5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dout5 {
        match self.bits {
            false => Dout5::Disable,
            true => Dout5::Enable,
        }
    }
    #[doc = "DMA is not allowed to modify this bit lane"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Dout5::Disable
    }
    #[doc = "DMA is allowed to modify this bit lane"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Dout5::Enable
    }
}
#[doc = "Field `DOUT5` writer - DMA is allowed to modify DOUT5"]
pub type Dout5W<'a, REG> = crate::BitWriter<'a, REG, Dout5>;
impl<'a, REG> Dout5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DMA is not allowed to modify this bit lane"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Dout5::Disable)
    }
    #[doc = "DMA is allowed to modify this bit lane"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Dout5::Enable)
    }
}
#[doc = "DMA is allowed to modify DOUT6\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dout6 {
    #[doc = "0: DMA is not allowed to modify this bit lane"]
    Disable = 0,
    #[doc = "1: DMA is allowed to modify this bit lane"]
    Enable = 1,
}
impl From<Dout6> for bool {
    #[inline(always)]
    fn from(variant: Dout6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUT6` reader - DMA is allowed to modify DOUT6"]
pub type Dout6R = crate::BitReader<Dout6>;
impl Dout6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dout6 {
        match self.bits {
            false => Dout6::Disable,
            true => Dout6::Enable,
        }
    }
    #[doc = "DMA is not allowed to modify this bit lane"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Dout6::Disable
    }
    #[doc = "DMA is allowed to modify this bit lane"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Dout6::Enable
    }
}
#[doc = "Field `DOUT6` writer - DMA is allowed to modify DOUT6"]
pub type Dout6W<'a, REG> = crate::BitWriter<'a, REG, Dout6>;
impl<'a, REG> Dout6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DMA is not allowed to modify this bit lane"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Dout6::Disable)
    }
    #[doc = "DMA is allowed to modify this bit lane"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Dout6::Enable)
    }
}
#[doc = "DMA is allowed to modify DOUT7\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dout7 {
    #[doc = "0: DMA is not allowed to modify this bit lane"]
    Disable = 0,
    #[doc = "1: DMA is allowed to modify this bit lane"]
    Enable = 1,
}
impl From<Dout7> for bool {
    #[inline(always)]
    fn from(variant: Dout7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUT7` reader - DMA is allowed to modify DOUT7"]
pub type Dout7R = crate::BitReader<Dout7>;
impl Dout7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dout7 {
        match self.bits {
            false => Dout7::Disable,
            true => Dout7::Enable,
        }
    }
    #[doc = "DMA is not allowed to modify this bit lane"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Dout7::Disable
    }
    #[doc = "DMA is allowed to modify this bit lane"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Dout7::Enable
    }
}
#[doc = "Field `DOUT7` writer - DMA is allowed to modify DOUT7"]
pub type Dout7W<'a, REG> = crate::BitWriter<'a, REG, Dout7>;
impl<'a, REG> Dout7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DMA is not allowed to modify this bit lane"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Dout7::Disable)
    }
    #[doc = "DMA is allowed to modify this bit lane"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Dout7::Enable)
    }
}
#[doc = "DMA is allowed to modify DOUT8\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dout8 {
    #[doc = "0: DMA is not allowed to modify this bit lane"]
    Disable = 0,
    #[doc = "1: DMA is allowed to modify this bit lane"]
    Enable = 1,
}
impl From<Dout8> for bool {
    #[inline(always)]
    fn from(variant: Dout8) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUT8` reader - DMA is allowed to modify DOUT8"]
pub type Dout8R = crate::BitReader<Dout8>;
impl Dout8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dout8 {
        match self.bits {
            false => Dout8::Disable,
            true => Dout8::Enable,
        }
    }
    #[doc = "DMA is not allowed to modify this bit lane"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Dout8::Disable
    }
    #[doc = "DMA is allowed to modify this bit lane"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Dout8::Enable
    }
}
#[doc = "Field `DOUT8` writer - DMA is allowed to modify DOUT8"]
pub type Dout8W<'a, REG> = crate::BitWriter<'a, REG, Dout8>;
impl<'a, REG> Dout8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DMA is not allowed to modify this bit lane"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Dout8::Disable)
    }
    #[doc = "DMA is allowed to modify this bit lane"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Dout8::Enable)
    }
}
#[doc = "DMA is allowed to modify DOUT9\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dout9 {
    #[doc = "0: DMA is not allowed to modify this bit lane"]
    Disable = 0,
    #[doc = "1: DMA is allowed to modify this bit lane"]
    Enable = 1,
}
impl From<Dout9> for bool {
    #[inline(always)]
    fn from(variant: Dout9) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUT9` reader - DMA is allowed to modify DOUT9"]
pub type Dout9R = crate::BitReader<Dout9>;
impl Dout9R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dout9 {
        match self.bits {
            false => Dout9::Disable,
            true => Dout9::Enable,
        }
    }
    #[doc = "DMA is not allowed to modify this bit lane"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Dout9::Disable
    }
    #[doc = "DMA is allowed to modify this bit lane"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Dout9::Enable
    }
}
#[doc = "Field `DOUT9` writer - DMA is allowed to modify DOUT9"]
pub type Dout9W<'a, REG> = crate::BitWriter<'a, REG, Dout9>;
impl<'a, REG> Dout9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DMA is not allowed to modify this bit lane"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Dout9::Disable)
    }
    #[doc = "DMA is allowed to modify this bit lane"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Dout9::Enable)
    }
}
#[doc = "DMA is allowed to modify DOUT10\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dout10 {
    #[doc = "0: DMA is not allowed to modify this bit lane"]
    Disable = 0,
    #[doc = "1: DMA is allowed to modify this bit lane"]
    Enable = 1,
}
impl From<Dout10> for bool {
    #[inline(always)]
    fn from(variant: Dout10) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUT10` reader - DMA is allowed to modify DOUT10"]
pub type Dout10R = crate::BitReader<Dout10>;
impl Dout10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dout10 {
        match self.bits {
            false => Dout10::Disable,
            true => Dout10::Enable,
        }
    }
    #[doc = "DMA is not allowed to modify this bit lane"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Dout10::Disable
    }
    #[doc = "DMA is allowed to modify this bit lane"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Dout10::Enable
    }
}
#[doc = "Field `DOUT10` writer - DMA is allowed to modify DOUT10"]
pub type Dout10W<'a, REG> = crate::BitWriter<'a, REG, Dout10>;
impl<'a, REG> Dout10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DMA is not allowed to modify this bit lane"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Dout10::Disable)
    }
    #[doc = "DMA is allowed to modify this bit lane"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Dout10::Enable)
    }
}
#[doc = "DMA is allowed to modify DOUT11\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dout11 {
    #[doc = "0: DMA is not allowed to modify this bit lane"]
    Disable = 0,
    #[doc = "1: DMA is allowed to modify this bit lane"]
    Enable = 1,
}
impl From<Dout11> for bool {
    #[inline(always)]
    fn from(variant: Dout11) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUT11` reader - DMA is allowed to modify DOUT11"]
pub type Dout11R = crate::BitReader<Dout11>;
impl Dout11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dout11 {
        match self.bits {
            false => Dout11::Disable,
            true => Dout11::Enable,
        }
    }
    #[doc = "DMA is not allowed to modify this bit lane"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Dout11::Disable
    }
    #[doc = "DMA is allowed to modify this bit lane"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Dout11::Enable
    }
}
#[doc = "Field `DOUT11` writer - DMA is allowed to modify DOUT11"]
pub type Dout11W<'a, REG> = crate::BitWriter<'a, REG, Dout11>;
impl<'a, REG> Dout11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DMA is not allowed to modify this bit lane"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Dout11::Disable)
    }
    #[doc = "DMA is allowed to modify this bit lane"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Dout11::Enable)
    }
}
#[doc = "DMA is allowed to modify DOUT12\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dout12 {
    #[doc = "0: DMA is not allowed to modify this bit lane"]
    Disable = 0,
    #[doc = "1: DMA is allowed to modify this bit lane"]
    Enable = 1,
}
impl From<Dout12> for bool {
    #[inline(always)]
    fn from(variant: Dout12) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUT12` reader - DMA is allowed to modify DOUT12"]
pub type Dout12R = crate::BitReader<Dout12>;
impl Dout12R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dout12 {
        match self.bits {
            false => Dout12::Disable,
            true => Dout12::Enable,
        }
    }
    #[doc = "DMA is not allowed to modify this bit lane"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Dout12::Disable
    }
    #[doc = "DMA is allowed to modify this bit lane"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Dout12::Enable
    }
}
#[doc = "Field `DOUT12` writer - DMA is allowed to modify DOUT12"]
pub type Dout12W<'a, REG> = crate::BitWriter<'a, REG, Dout12>;
impl<'a, REG> Dout12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DMA is not allowed to modify this bit lane"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Dout12::Disable)
    }
    #[doc = "DMA is allowed to modify this bit lane"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Dout12::Enable)
    }
}
#[doc = "DMA is allowed to modify DOUT13\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dout13 {
    #[doc = "0: DMA is not allowed to modify this bit lane"]
    Disable = 0,
    #[doc = "1: DMA is allowed to modify this bit lane"]
    Enable = 1,
}
impl From<Dout13> for bool {
    #[inline(always)]
    fn from(variant: Dout13) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUT13` reader - DMA is allowed to modify DOUT13"]
pub type Dout13R = crate::BitReader<Dout13>;
impl Dout13R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dout13 {
        match self.bits {
            false => Dout13::Disable,
            true => Dout13::Enable,
        }
    }
    #[doc = "DMA is not allowed to modify this bit lane"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Dout13::Disable
    }
    #[doc = "DMA is allowed to modify this bit lane"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Dout13::Enable
    }
}
#[doc = "Field `DOUT13` writer - DMA is allowed to modify DOUT13"]
pub type Dout13W<'a, REG> = crate::BitWriter<'a, REG, Dout13>;
impl<'a, REG> Dout13W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DMA is not allowed to modify this bit lane"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Dout13::Disable)
    }
    #[doc = "DMA is allowed to modify this bit lane"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Dout13::Enable)
    }
}
#[doc = "DMA is allowed to modify DOUT14\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dout14 {
    #[doc = "0: DMA is not allowed to modify this bit lane"]
    Disable = 0,
    #[doc = "1: DMA is allowed to modify this bit lane"]
    Enable = 1,
}
impl From<Dout14> for bool {
    #[inline(always)]
    fn from(variant: Dout14) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUT14` reader - DMA is allowed to modify DOUT14"]
pub type Dout14R = crate::BitReader<Dout14>;
impl Dout14R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dout14 {
        match self.bits {
            false => Dout14::Disable,
            true => Dout14::Enable,
        }
    }
    #[doc = "DMA is not allowed to modify this bit lane"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Dout14::Disable
    }
    #[doc = "DMA is allowed to modify this bit lane"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Dout14::Enable
    }
}
#[doc = "Field `DOUT14` writer - DMA is allowed to modify DOUT14"]
pub type Dout14W<'a, REG> = crate::BitWriter<'a, REG, Dout14>;
impl<'a, REG> Dout14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DMA is not allowed to modify this bit lane"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Dout14::Disable)
    }
    #[doc = "DMA is allowed to modify this bit lane"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Dout14::Enable)
    }
}
#[doc = "DMA is allowed to modify DOUT15\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dout15 {
    #[doc = "0: DMA is not allowed to modify this bit lane"]
    Disable = 0,
    #[doc = "1: DMA is allowed to modify this bit lane"]
    Enable = 1,
}
impl From<Dout15> for bool {
    #[inline(always)]
    fn from(variant: Dout15) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUT15` reader - DMA is allowed to modify DOUT15"]
pub type Dout15R = crate::BitReader<Dout15>;
impl Dout15R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dout15 {
        match self.bits {
            false => Dout15::Disable,
            true => Dout15::Enable,
        }
    }
    #[doc = "DMA is not allowed to modify this bit lane"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Dout15::Disable
    }
    #[doc = "DMA is allowed to modify this bit lane"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Dout15::Enable
    }
}
#[doc = "Field `DOUT15` writer - DMA is allowed to modify DOUT15"]
pub type Dout15W<'a, REG> = crate::BitWriter<'a, REG, Dout15>;
impl<'a, REG> Dout15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DMA is not allowed to modify this bit lane"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Dout15::Disable)
    }
    #[doc = "DMA is allowed to modify this bit lane"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Dout15::Enable)
    }
}
#[doc = "DMA is allowed to modify DOUT16\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dout16 {
    #[doc = "0: DMA is not allowed to modify this bit lane"]
    Disable = 0,
    #[doc = "1: DMA is allowed to modify this bit lane"]
    Enable = 1,
}
impl From<Dout16> for bool {
    #[inline(always)]
    fn from(variant: Dout16) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUT16` reader - DMA is allowed to modify DOUT16"]
pub type Dout16R = crate::BitReader<Dout16>;
impl Dout16R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dout16 {
        match self.bits {
            false => Dout16::Disable,
            true => Dout16::Enable,
        }
    }
    #[doc = "DMA is not allowed to modify this bit lane"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Dout16::Disable
    }
    #[doc = "DMA is allowed to modify this bit lane"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Dout16::Enable
    }
}
#[doc = "Field `DOUT16` writer - DMA is allowed to modify DOUT16"]
pub type Dout16W<'a, REG> = crate::BitWriter<'a, REG, Dout16>;
impl<'a, REG> Dout16W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DMA is not allowed to modify this bit lane"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Dout16::Disable)
    }
    #[doc = "DMA is allowed to modify this bit lane"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Dout16::Enable)
    }
}
#[doc = "DMA is allowed to modify DOUT17\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dout17 {
    #[doc = "0: DMA is not allowed to modify this bit lane"]
    Disable = 0,
    #[doc = "1: DMA is allowed to modify this bit lane"]
    Enable = 1,
}
impl From<Dout17> for bool {
    #[inline(always)]
    fn from(variant: Dout17) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUT17` reader - DMA is allowed to modify DOUT17"]
pub type Dout17R = crate::BitReader<Dout17>;
impl Dout17R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dout17 {
        match self.bits {
            false => Dout17::Disable,
            true => Dout17::Enable,
        }
    }
    #[doc = "DMA is not allowed to modify this bit lane"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Dout17::Disable
    }
    #[doc = "DMA is allowed to modify this bit lane"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Dout17::Enable
    }
}
#[doc = "Field `DOUT17` writer - DMA is allowed to modify DOUT17"]
pub type Dout17W<'a, REG> = crate::BitWriter<'a, REG, Dout17>;
impl<'a, REG> Dout17W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DMA is not allowed to modify this bit lane"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Dout17::Disable)
    }
    #[doc = "DMA is allowed to modify this bit lane"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Dout17::Enable)
    }
}
#[doc = "DMA is allowed to modify DOUT18\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dout18 {
    #[doc = "0: DMA is not allowed to modify this bit lane"]
    Disable = 0,
    #[doc = "1: DMA is allowed to modify this bit lane"]
    Enable = 1,
}
impl From<Dout18> for bool {
    #[inline(always)]
    fn from(variant: Dout18) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUT18` reader - DMA is allowed to modify DOUT18"]
pub type Dout18R = crate::BitReader<Dout18>;
impl Dout18R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dout18 {
        match self.bits {
            false => Dout18::Disable,
            true => Dout18::Enable,
        }
    }
    #[doc = "DMA is not allowed to modify this bit lane"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Dout18::Disable
    }
    #[doc = "DMA is allowed to modify this bit lane"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Dout18::Enable
    }
}
#[doc = "Field `DOUT18` writer - DMA is allowed to modify DOUT18"]
pub type Dout18W<'a, REG> = crate::BitWriter<'a, REG, Dout18>;
impl<'a, REG> Dout18W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DMA is not allowed to modify this bit lane"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Dout18::Disable)
    }
    #[doc = "DMA is allowed to modify this bit lane"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Dout18::Enable)
    }
}
#[doc = "DMA is allowed to modify DOUT19\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dout19 {
    #[doc = "0: DMA is not allowed to modify this bit lane"]
    Disable = 0,
    #[doc = "1: DMA is allowed to modify this bit lane"]
    Enable = 1,
}
impl From<Dout19> for bool {
    #[inline(always)]
    fn from(variant: Dout19) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUT19` reader - DMA is allowed to modify DOUT19"]
pub type Dout19R = crate::BitReader<Dout19>;
impl Dout19R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dout19 {
        match self.bits {
            false => Dout19::Disable,
            true => Dout19::Enable,
        }
    }
    #[doc = "DMA is not allowed to modify this bit lane"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Dout19::Disable
    }
    #[doc = "DMA is allowed to modify this bit lane"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Dout19::Enable
    }
}
#[doc = "Field `DOUT19` writer - DMA is allowed to modify DOUT19"]
pub type Dout19W<'a, REG> = crate::BitWriter<'a, REG, Dout19>;
impl<'a, REG> Dout19W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DMA is not allowed to modify this bit lane"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Dout19::Disable)
    }
    #[doc = "DMA is allowed to modify this bit lane"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Dout19::Enable)
    }
}
#[doc = "DMA is allowed to modify DOUT20\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dout20 {
    #[doc = "0: DMA is not allowed to modify this bit lane"]
    Disable = 0,
    #[doc = "1: DMA is allowed to modify this bit lane"]
    Enable = 1,
}
impl From<Dout20> for bool {
    #[inline(always)]
    fn from(variant: Dout20) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUT20` reader - DMA is allowed to modify DOUT20"]
pub type Dout20R = crate::BitReader<Dout20>;
impl Dout20R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dout20 {
        match self.bits {
            false => Dout20::Disable,
            true => Dout20::Enable,
        }
    }
    #[doc = "DMA is not allowed to modify this bit lane"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Dout20::Disable
    }
    #[doc = "DMA is allowed to modify this bit lane"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Dout20::Enable
    }
}
#[doc = "Field `DOUT20` writer - DMA is allowed to modify DOUT20"]
pub type Dout20W<'a, REG> = crate::BitWriter<'a, REG, Dout20>;
impl<'a, REG> Dout20W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DMA is not allowed to modify this bit lane"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Dout20::Disable)
    }
    #[doc = "DMA is allowed to modify this bit lane"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Dout20::Enable)
    }
}
#[doc = "DMA is allowed to modify DOUT21\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dout21 {
    #[doc = "0: DMA is not allowed to modify this bit lane"]
    Disable = 0,
    #[doc = "1: DMA is allowed to modify this bit lane"]
    Enable = 1,
}
impl From<Dout21> for bool {
    #[inline(always)]
    fn from(variant: Dout21) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUT21` reader - DMA is allowed to modify DOUT21"]
pub type Dout21R = crate::BitReader<Dout21>;
impl Dout21R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dout21 {
        match self.bits {
            false => Dout21::Disable,
            true => Dout21::Enable,
        }
    }
    #[doc = "DMA is not allowed to modify this bit lane"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Dout21::Disable
    }
    #[doc = "DMA is allowed to modify this bit lane"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Dout21::Enable
    }
}
#[doc = "Field `DOUT21` writer - DMA is allowed to modify DOUT21"]
pub type Dout21W<'a, REG> = crate::BitWriter<'a, REG, Dout21>;
impl<'a, REG> Dout21W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DMA is not allowed to modify this bit lane"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Dout21::Disable)
    }
    #[doc = "DMA is allowed to modify this bit lane"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Dout21::Enable)
    }
}
#[doc = "DMA is allowed to modify DOUT22\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dout22 {
    #[doc = "0: DMA is not allowed to modify this bit lane"]
    Disable = 0,
    #[doc = "1: DMA is allowed to modify this bit lane"]
    Enable = 1,
}
impl From<Dout22> for bool {
    #[inline(always)]
    fn from(variant: Dout22) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUT22` reader - DMA is allowed to modify DOUT22"]
pub type Dout22R = crate::BitReader<Dout22>;
impl Dout22R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dout22 {
        match self.bits {
            false => Dout22::Disable,
            true => Dout22::Enable,
        }
    }
    #[doc = "DMA is not allowed to modify this bit lane"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Dout22::Disable
    }
    #[doc = "DMA is allowed to modify this bit lane"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Dout22::Enable
    }
}
#[doc = "Field `DOUT22` writer - DMA is allowed to modify DOUT22"]
pub type Dout22W<'a, REG> = crate::BitWriter<'a, REG, Dout22>;
impl<'a, REG> Dout22W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DMA is not allowed to modify this bit lane"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Dout22::Disable)
    }
    #[doc = "DMA is allowed to modify this bit lane"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Dout22::Enable)
    }
}
#[doc = "DMA is allowed to modify DOUT23\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dout23 {
    #[doc = "0: DMA is not allowed to modify this bit lane"]
    Disable = 0,
    #[doc = "1: DMA is allowed to modify this bit lane"]
    Enable = 1,
}
impl From<Dout23> for bool {
    #[inline(always)]
    fn from(variant: Dout23) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUT23` reader - DMA is allowed to modify DOUT23"]
pub type Dout23R = crate::BitReader<Dout23>;
impl Dout23R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dout23 {
        match self.bits {
            false => Dout23::Disable,
            true => Dout23::Enable,
        }
    }
    #[doc = "DMA is not allowed to modify this bit lane"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Dout23::Disable
    }
    #[doc = "DMA is allowed to modify this bit lane"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Dout23::Enable
    }
}
#[doc = "Field `DOUT23` writer - DMA is allowed to modify DOUT23"]
pub type Dout23W<'a, REG> = crate::BitWriter<'a, REG, Dout23>;
impl<'a, REG> Dout23W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DMA is not allowed to modify this bit lane"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Dout23::Disable)
    }
    #[doc = "DMA is allowed to modify this bit lane"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Dout23::Enable)
    }
}
#[doc = "DMA is allowed to modify DOUT24\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dout24 {
    #[doc = "0: DMA is not allowed to modify this bit lane"]
    Disable = 0,
    #[doc = "1: DMA is allowed to modify this bit lane"]
    Enable = 1,
}
impl From<Dout24> for bool {
    #[inline(always)]
    fn from(variant: Dout24) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUT24` reader - DMA is allowed to modify DOUT24"]
pub type Dout24R = crate::BitReader<Dout24>;
impl Dout24R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dout24 {
        match self.bits {
            false => Dout24::Disable,
            true => Dout24::Enable,
        }
    }
    #[doc = "DMA is not allowed to modify this bit lane"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Dout24::Disable
    }
    #[doc = "DMA is allowed to modify this bit lane"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Dout24::Enable
    }
}
#[doc = "Field `DOUT24` writer - DMA is allowed to modify DOUT24"]
pub type Dout24W<'a, REG> = crate::BitWriter<'a, REG, Dout24>;
impl<'a, REG> Dout24W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DMA is not allowed to modify this bit lane"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Dout24::Disable)
    }
    #[doc = "DMA is allowed to modify this bit lane"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Dout24::Enable)
    }
}
#[doc = "DMA is allowed to modify DOUT25\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dout25 {
    #[doc = "0: DMA is not allowed to modify this bit lane"]
    Disable = 0,
    #[doc = "1: DMA is allowed to modify this bit lane"]
    Enable = 1,
}
impl From<Dout25> for bool {
    #[inline(always)]
    fn from(variant: Dout25) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUT25` reader - DMA is allowed to modify DOUT25"]
pub type Dout25R = crate::BitReader<Dout25>;
impl Dout25R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dout25 {
        match self.bits {
            false => Dout25::Disable,
            true => Dout25::Enable,
        }
    }
    #[doc = "DMA is not allowed to modify this bit lane"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Dout25::Disable
    }
    #[doc = "DMA is allowed to modify this bit lane"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Dout25::Enable
    }
}
#[doc = "Field `DOUT25` writer - DMA is allowed to modify DOUT25"]
pub type Dout25W<'a, REG> = crate::BitWriter<'a, REG, Dout25>;
impl<'a, REG> Dout25W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DMA is not allowed to modify this bit lane"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Dout25::Disable)
    }
    #[doc = "DMA is allowed to modify this bit lane"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Dout25::Enable)
    }
}
#[doc = "DMA is allowed to modify DOUT26\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dout26 {
    #[doc = "0: DMA is not allowed to modify this bit lane"]
    Disable = 0,
    #[doc = "1: DMA is allowed to modify this bit lane"]
    Enable = 1,
}
impl From<Dout26> for bool {
    #[inline(always)]
    fn from(variant: Dout26) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUT26` reader - DMA is allowed to modify DOUT26"]
pub type Dout26R = crate::BitReader<Dout26>;
impl Dout26R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dout26 {
        match self.bits {
            false => Dout26::Disable,
            true => Dout26::Enable,
        }
    }
    #[doc = "DMA is not allowed to modify this bit lane"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Dout26::Disable
    }
    #[doc = "DMA is allowed to modify this bit lane"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Dout26::Enable
    }
}
#[doc = "Field `DOUT26` writer - DMA is allowed to modify DOUT26"]
pub type Dout26W<'a, REG> = crate::BitWriter<'a, REG, Dout26>;
impl<'a, REG> Dout26W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DMA is not allowed to modify this bit lane"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Dout26::Disable)
    }
    #[doc = "DMA is allowed to modify this bit lane"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Dout26::Enable)
    }
}
#[doc = "DMA is allowed to modify DOUT27\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dout27 {
    #[doc = "0: DMA is not allowed to modify this bit lane"]
    Disable = 0,
    #[doc = "1: DMA is allowed to modify this bit lane"]
    Enable = 1,
}
impl From<Dout27> for bool {
    #[inline(always)]
    fn from(variant: Dout27) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUT27` reader - DMA is allowed to modify DOUT27"]
pub type Dout27R = crate::BitReader<Dout27>;
impl Dout27R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dout27 {
        match self.bits {
            false => Dout27::Disable,
            true => Dout27::Enable,
        }
    }
    #[doc = "DMA is not allowed to modify this bit lane"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Dout27::Disable
    }
    #[doc = "DMA is allowed to modify this bit lane"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Dout27::Enable
    }
}
#[doc = "Field `DOUT27` writer - DMA is allowed to modify DOUT27"]
pub type Dout27W<'a, REG> = crate::BitWriter<'a, REG, Dout27>;
impl<'a, REG> Dout27W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DMA is not allowed to modify this bit lane"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Dout27::Disable)
    }
    #[doc = "DMA is allowed to modify this bit lane"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Dout27::Enable)
    }
}
#[doc = "DMA is allowed to modify DOUT28\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dout28 {
    #[doc = "0: DMA is not allowed to modify this bit lane"]
    Disable = 0,
    #[doc = "1: DMA is allowed to modify this bit lane"]
    Enable = 1,
}
impl From<Dout28> for bool {
    #[inline(always)]
    fn from(variant: Dout28) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUT28` reader - DMA is allowed to modify DOUT28"]
pub type Dout28R = crate::BitReader<Dout28>;
impl Dout28R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dout28 {
        match self.bits {
            false => Dout28::Disable,
            true => Dout28::Enable,
        }
    }
    #[doc = "DMA is not allowed to modify this bit lane"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Dout28::Disable
    }
    #[doc = "DMA is allowed to modify this bit lane"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Dout28::Enable
    }
}
#[doc = "Field `DOUT28` writer - DMA is allowed to modify DOUT28"]
pub type Dout28W<'a, REG> = crate::BitWriter<'a, REG, Dout28>;
impl<'a, REG> Dout28W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DMA is not allowed to modify this bit lane"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Dout28::Disable)
    }
    #[doc = "DMA is allowed to modify this bit lane"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Dout28::Enable)
    }
}
#[doc = "DMA is allowed to modify DOUT29\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dout29 {
    #[doc = "0: DMA is not allowed to modify this bit lane"]
    Disable = 0,
    #[doc = "1: DMA is allowed to modify this bit lane"]
    Enable = 1,
}
impl From<Dout29> for bool {
    #[inline(always)]
    fn from(variant: Dout29) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUT29` reader - DMA is allowed to modify DOUT29"]
pub type Dout29R = crate::BitReader<Dout29>;
impl Dout29R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dout29 {
        match self.bits {
            false => Dout29::Disable,
            true => Dout29::Enable,
        }
    }
    #[doc = "DMA is not allowed to modify this bit lane"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Dout29::Disable
    }
    #[doc = "DMA is allowed to modify this bit lane"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Dout29::Enable
    }
}
#[doc = "Field `DOUT29` writer - DMA is allowed to modify DOUT29"]
pub type Dout29W<'a, REG> = crate::BitWriter<'a, REG, Dout29>;
impl<'a, REG> Dout29W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DMA is not allowed to modify this bit lane"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Dout29::Disable)
    }
    #[doc = "DMA is allowed to modify this bit lane"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Dout29::Enable)
    }
}
#[doc = "DMA is allowed to modify DOUT30\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dout30 {
    #[doc = "0: DMA is not allowed to modify this bit lane"]
    Disable = 0,
    #[doc = "1: DMA is allowed to modify this bit lane"]
    Enable = 1,
}
impl From<Dout30> for bool {
    #[inline(always)]
    fn from(variant: Dout30) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUT30` reader - DMA is allowed to modify DOUT30"]
pub type Dout30R = crate::BitReader<Dout30>;
impl Dout30R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dout30 {
        match self.bits {
            false => Dout30::Disable,
            true => Dout30::Enable,
        }
    }
    #[doc = "DMA is not allowed to modify this bit lane"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Dout30::Disable
    }
    #[doc = "DMA is allowed to modify this bit lane"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Dout30::Enable
    }
}
#[doc = "Field `DOUT30` writer - DMA is allowed to modify DOUT30"]
pub type Dout30W<'a, REG> = crate::BitWriter<'a, REG, Dout30>;
impl<'a, REG> Dout30W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DMA is not allowed to modify this bit lane"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Dout30::Disable)
    }
    #[doc = "DMA is allowed to modify this bit lane"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Dout30::Enable)
    }
}
#[doc = "DMA is allowed to modify DOUT31\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dout31 {
    #[doc = "0: DMA is not allowed to modify this bit lane"]
    Disable = 0,
    #[doc = "1: DMA is allowed to modify this bit lane"]
    Enable = 1,
}
impl From<Dout31> for bool {
    #[inline(always)]
    fn from(variant: Dout31) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUT31` reader - DMA is allowed to modify DOUT31"]
pub type Dout31R = crate::BitReader<Dout31>;
impl Dout31R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dout31 {
        match self.bits {
            false => Dout31::Disable,
            true => Dout31::Enable,
        }
    }
    #[doc = "DMA is not allowed to modify this bit lane"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Dout31::Disable
    }
    #[doc = "DMA is allowed to modify this bit lane"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Dout31::Enable
    }
}
#[doc = "Field `DOUT31` writer - DMA is allowed to modify DOUT31"]
pub type Dout31W<'a, REG> = crate::BitWriter<'a, REG, Dout31>;
impl<'a, REG> Dout31W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DMA is not allowed to modify this bit lane"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Dout31::Disable)
    }
    #[doc = "DMA is allowed to modify this bit lane"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Dout31::Enable)
    }
}
impl R {
    #[doc = "Bit 0 - DMA is allowed to modify DOUT0"]
    #[inline(always)]
    pub fn dout0(&self) -> Dout0R {
        Dout0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMA is allowed to modify DOUT1"]
    #[inline(always)]
    pub fn dout1(&self) -> Dout1R {
        Dout1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DMA is allowed to modify DOUT2"]
    #[inline(always)]
    pub fn dout2(&self) -> Dout2R {
        Dout2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DMA is allowed to modify DOUT3"]
    #[inline(always)]
    pub fn dout3(&self) -> Dout3R {
        Dout3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - DMA is allowed to modify DOUT4"]
    #[inline(always)]
    pub fn dout4(&self) -> Dout4R {
        Dout4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - DMA is allowed to modify DOUT5"]
    #[inline(always)]
    pub fn dout5(&self) -> Dout5R {
        Dout5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - DMA is allowed to modify DOUT6"]
    #[inline(always)]
    pub fn dout6(&self) -> Dout6R {
        Dout6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - DMA is allowed to modify DOUT7"]
    #[inline(always)]
    pub fn dout7(&self) -> Dout7R {
        Dout7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - DMA is allowed to modify DOUT8"]
    #[inline(always)]
    pub fn dout8(&self) -> Dout8R {
        Dout8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - DMA is allowed to modify DOUT9"]
    #[inline(always)]
    pub fn dout9(&self) -> Dout9R {
        Dout9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - DMA is allowed to modify DOUT10"]
    #[inline(always)]
    pub fn dout10(&self) -> Dout10R {
        Dout10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - DMA is allowed to modify DOUT11"]
    #[inline(always)]
    pub fn dout11(&self) -> Dout11R {
        Dout11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - DMA is allowed to modify DOUT12"]
    #[inline(always)]
    pub fn dout12(&self) -> Dout12R {
        Dout12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - DMA is allowed to modify DOUT13"]
    #[inline(always)]
    pub fn dout13(&self) -> Dout13R {
        Dout13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - DMA is allowed to modify DOUT14"]
    #[inline(always)]
    pub fn dout14(&self) -> Dout14R {
        Dout14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - DMA is allowed to modify DOUT15"]
    #[inline(always)]
    pub fn dout15(&self) -> Dout15R {
        Dout15R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - DMA is allowed to modify DOUT16"]
    #[inline(always)]
    pub fn dout16(&self) -> Dout16R {
        Dout16R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - DMA is allowed to modify DOUT17"]
    #[inline(always)]
    pub fn dout17(&self) -> Dout17R {
        Dout17R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - DMA is allowed to modify DOUT18"]
    #[inline(always)]
    pub fn dout18(&self) -> Dout18R {
        Dout18R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - DMA is allowed to modify DOUT19"]
    #[inline(always)]
    pub fn dout19(&self) -> Dout19R {
        Dout19R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - DMA is allowed to modify DOUT20"]
    #[inline(always)]
    pub fn dout20(&self) -> Dout20R {
        Dout20R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - DMA is allowed to modify DOUT21"]
    #[inline(always)]
    pub fn dout21(&self) -> Dout21R {
        Dout21R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - DMA is allowed to modify DOUT22"]
    #[inline(always)]
    pub fn dout22(&self) -> Dout22R {
        Dout22R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - DMA is allowed to modify DOUT23"]
    #[inline(always)]
    pub fn dout23(&self) -> Dout23R {
        Dout23R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - DMA is allowed to modify DOUT24"]
    #[inline(always)]
    pub fn dout24(&self) -> Dout24R {
        Dout24R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - DMA is allowed to modify DOUT25"]
    #[inline(always)]
    pub fn dout25(&self) -> Dout25R {
        Dout25R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - DMA is allowed to modify DOUT26"]
    #[inline(always)]
    pub fn dout26(&self) -> Dout26R {
        Dout26R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - DMA is allowed to modify DOUT27"]
    #[inline(always)]
    pub fn dout27(&self) -> Dout27R {
        Dout27R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - DMA is allowed to modify DOUT28"]
    #[inline(always)]
    pub fn dout28(&self) -> Dout28R {
        Dout28R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - DMA is allowed to modify DOUT29"]
    #[inline(always)]
    pub fn dout29(&self) -> Dout29R {
        Dout29R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - DMA is allowed to modify DOUT30"]
    #[inline(always)]
    pub fn dout30(&self) -> Dout30R {
        Dout30R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - DMA is allowed to modify DOUT31"]
    #[inline(always)]
    pub fn dout31(&self) -> Dout31R {
        Dout31R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMAMASK")
            .field("dout0", &self.dout0())
            .field("dout1", &self.dout1())
            .field("dout2", &self.dout2())
            .field("dout3", &self.dout3())
            .field("dout4", &self.dout4())
            .field("dout5", &self.dout5())
            .field("dout6", &self.dout6())
            .field("dout7", &self.dout7())
            .field("dout8", &self.dout8())
            .field("dout9", &self.dout9())
            .field("dout10", &self.dout10())
            .field("dout11", &self.dout11())
            .field("dout12", &self.dout12())
            .field("dout13", &self.dout13())
            .field("dout14", &self.dout14())
            .field("dout15", &self.dout15())
            .field("dout16", &self.dout16())
            .field("dout17", &self.dout17())
            .field("dout18", &self.dout18())
            .field("dout19", &self.dout19())
            .field("dout20", &self.dout20())
            .field("dout21", &self.dout21())
            .field("dout22", &self.dout22())
            .field("dout23", &self.dout23())
            .field("dout24", &self.dout24())
            .field("dout25", &self.dout25())
            .field("dout26", &self.dout26())
            .field("dout27", &self.dout27())
            .field("dout28", &self.dout28())
            .field("dout29", &self.dout29())
            .field("dout30", &self.dout30())
            .field("dout31", &self.dout31())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - DMA is allowed to modify DOUT0"]
    #[inline(always)]
    pub fn dout0(&mut self) -> Dout0W<'_, DmamaskSpec> {
        Dout0W::new(self, 0)
    }
    #[doc = "Bit 1 - DMA is allowed to modify DOUT1"]
    #[inline(always)]
    pub fn dout1(&mut self) -> Dout1W<'_, DmamaskSpec> {
        Dout1W::new(self, 1)
    }
    #[doc = "Bit 2 - DMA is allowed to modify DOUT2"]
    #[inline(always)]
    pub fn dout2(&mut self) -> Dout2W<'_, DmamaskSpec> {
        Dout2W::new(self, 2)
    }
    #[doc = "Bit 3 - DMA is allowed to modify DOUT3"]
    #[inline(always)]
    pub fn dout3(&mut self) -> Dout3W<'_, DmamaskSpec> {
        Dout3W::new(self, 3)
    }
    #[doc = "Bit 4 - DMA is allowed to modify DOUT4"]
    #[inline(always)]
    pub fn dout4(&mut self) -> Dout4W<'_, DmamaskSpec> {
        Dout4W::new(self, 4)
    }
    #[doc = "Bit 5 - DMA is allowed to modify DOUT5"]
    #[inline(always)]
    pub fn dout5(&mut self) -> Dout5W<'_, DmamaskSpec> {
        Dout5W::new(self, 5)
    }
    #[doc = "Bit 6 - DMA is allowed to modify DOUT6"]
    #[inline(always)]
    pub fn dout6(&mut self) -> Dout6W<'_, DmamaskSpec> {
        Dout6W::new(self, 6)
    }
    #[doc = "Bit 7 - DMA is allowed to modify DOUT7"]
    #[inline(always)]
    pub fn dout7(&mut self) -> Dout7W<'_, DmamaskSpec> {
        Dout7W::new(self, 7)
    }
    #[doc = "Bit 8 - DMA is allowed to modify DOUT8"]
    #[inline(always)]
    pub fn dout8(&mut self) -> Dout8W<'_, DmamaskSpec> {
        Dout8W::new(self, 8)
    }
    #[doc = "Bit 9 - DMA is allowed to modify DOUT9"]
    #[inline(always)]
    pub fn dout9(&mut self) -> Dout9W<'_, DmamaskSpec> {
        Dout9W::new(self, 9)
    }
    #[doc = "Bit 10 - DMA is allowed to modify DOUT10"]
    #[inline(always)]
    pub fn dout10(&mut self) -> Dout10W<'_, DmamaskSpec> {
        Dout10W::new(self, 10)
    }
    #[doc = "Bit 11 - DMA is allowed to modify DOUT11"]
    #[inline(always)]
    pub fn dout11(&mut self) -> Dout11W<'_, DmamaskSpec> {
        Dout11W::new(self, 11)
    }
    #[doc = "Bit 12 - DMA is allowed to modify DOUT12"]
    #[inline(always)]
    pub fn dout12(&mut self) -> Dout12W<'_, DmamaskSpec> {
        Dout12W::new(self, 12)
    }
    #[doc = "Bit 13 - DMA is allowed to modify DOUT13"]
    #[inline(always)]
    pub fn dout13(&mut self) -> Dout13W<'_, DmamaskSpec> {
        Dout13W::new(self, 13)
    }
    #[doc = "Bit 14 - DMA is allowed to modify DOUT14"]
    #[inline(always)]
    pub fn dout14(&mut self) -> Dout14W<'_, DmamaskSpec> {
        Dout14W::new(self, 14)
    }
    #[doc = "Bit 15 - DMA is allowed to modify DOUT15"]
    #[inline(always)]
    pub fn dout15(&mut self) -> Dout15W<'_, DmamaskSpec> {
        Dout15W::new(self, 15)
    }
    #[doc = "Bit 16 - DMA is allowed to modify DOUT16"]
    #[inline(always)]
    pub fn dout16(&mut self) -> Dout16W<'_, DmamaskSpec> {
        Dout16W::new(self, 16)
    }
    #[doc = "Bit 17 - DMA is allowed to modify DOUT17"]
    #[inline(always)]
    pub fn dout17(&mut self) -> Dout17W<'_, DmamaskSpec> {
        Dout17W::new(self, 17)
    }
    #[doc = "Bit 18 - DMA is allowed to modify DOUT18"]
    #[inline(always)]
    pub fn dout18(&mut self) -> Dout18W<'_, DmamaskSpec> {
        Dout18W::new(self, 18)
    }
    #[doc = "Bit 19 - DMA is allowed to modify DOUT19"]
    #[inline(always)]
    pub fn dout19(&mut self) -> Dout19W<'_, DmamaskSpec> {
        Dout19W::new(self, 19)
    }
    #[doc = "Bit 20 - DMA is allowed to modify DOUT20"]
    #[inline(always)]
    pub fn dout20(&mut self) -> Dout20W<'_, DmamaskSpec> {
        Dout20W::new(self, 20)
    }
    #[doc = "Bit 21 - DMA is allowed to modify DOUT21"]
    #[inline(always)]
    pub fn dout21(&mut self) -> Dout21W<'_, DmamaskSpec> {
        Dout21W::new(self, 21)
    }
    #[doc = "Bit 22 - DMA is allowed to modify DOUT22"]
    #[inline(always)]
    pub fn dout22(&mut self) -> Dout22W<'_, DmamaskSpec> {
        Dout22W::new(self, 22)
    }
    #[doc = "Bit 23 - DMA is allowed to modify DOUT23"]
    #[inline(always)]
    pub fn dout23(&mut self) -> Dout23W<'_, DmamaskSpec> {
        Dout23W::new(self, 23)
    }
    #[doc = "Bit 24 - DMA is allowed to modify DOUT24"]
    #[inline(always)]
    pub fn dout24(&mut self) -> Dout24W<'_, DmamaskSpec> {
        Dout24W::new(self, 24)
    }
    #[doc = "Bit 25 - DMA is allowed to modify DOUT25"]
    #[inline(always)]
    pub fn dout25(&mut self) -> Dout25W<'_, DmamaskSpec> {
        Dout25W::new(self, 25)
    }
    #[doc = "Bit 26 - DMA is allowed to modify DOUT26"]
    #[inline(always)]
    pub fn dout26(&mut self) -> Dout26W<'_, DmamaskSpec> {
        Dout26W::new(self, 26)
    }
    #[doc = "Bit 27 - DMA is allowed to modify DOUT27"]
    #[inline(always)]
    pub fn dout27(&mut self) -> Dout27W<'_, DmamaskSpec> {
        Dout27W::new(self, 27)
    }
    #[doc = "Bit 28 - DMA is allowed to modify DOUT28"]
    #[inline(always)]
    pub fn dout28(&mut self) -> Dout28W<'_, DmamaskSpec> {
        Dout28W::new(self, 28)
    }
    #[doc = "Bit 29 - DMA is allowed to modify DOUT29"]
    #[inline(always)]
    pub fn dout29(&mut self) -> Dout29W<'_, DmamaskSpec> {
        Dout29W::new(self, 29)
    }
    #[doc = "Bit 30 - DMA is allowed to modify DOUT30"]
    #[inline(always)]
    pub fn dout30(&mut self) -> Dout30W<'_, DmamaskSpec> {
        Dout30W::new(self, 30)
    }
    #[doc = "Bit 31 - DMA is allowed to modify DOUT31"]
    #[inline(always)]
    pub fn dout31(&mut self) -> Dout31W<'_, DmamaskSpec> {
        Dout31W::new(self, 31)
    }
}
#[doc = "DMA Write MASK\n\nYou can [`read`](crate::Reg::read) this register and get [`dmamask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmamask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmamaskSpec;
impl crate::RegisterSpec for DmamaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmamask::R`](R) reader structure"]
impl crate::Readable for DmamaskSpec {}
#[doc = "`write(|w| ..)` method takes [`dmamask::W`](W) writer structure"]
impl crate::Writable for DmamaskSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets DMAMASK to value 0"]
impl crate::Resettable for DmamaskSpec {}
