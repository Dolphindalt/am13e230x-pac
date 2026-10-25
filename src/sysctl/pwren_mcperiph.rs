#[doc = "Register `PWREN_MCPERIPH` reader"]
pub type R = crate::R<PwrenMcperiphSpec>;
#[doc = "Register `PWREN_MCPERIPH` writer"]
pub type W = crate::W<PwrenMcperiphSpec>;
#[doc = "Enable the power to IP, KEY must be set to 26h to write to this bit\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eqep0 {
    #[doc = "0: Disable Power"]
    Disable = 0,
    #[doc = "1: Enable Power"]
    Enable = 1,
}
impl From<Eqep0> for bool {
    #[inline(always)]
    fn from(variant: Eqep0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `eqep0` reader - Enable the power to IP, KEY must be set to 26h to write to this bit"]
pub type Eqep0R = crate::BitReader<Eqep0>;
impl Eqep0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Eqep0 {
        match self.bits {
            false => Eqep0::Disable,
            true => Eqep0::Enable,
        }
    }
    #[doc = "Disable Power"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Eqep0::Disable
    }
    #[doc = "Enable Power"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Eqep0::Enable
    }
}
#[doc = "Field `eqep0` writer - Enable the power to IP, KEY must be set to 26h to write to this bit"]
pub type Eqep0W<'a, REG> = crate::BitWriter<'a, REG, Eqep0>;
impl<'a, REG> Eqep0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Power"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Eqep0::Disable)
    }
    #[doc = "Enable Power"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Eqep0::Enable)
    }
}
#[doc = "Enable the power to IP, KEY must be set to 26h to write to this bit\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eqep1 {
    #[doc = "0: Disable Power"]
    Disable = 0,
    #[doc = "1: Enable Power"]
    Enable = 1,
}
impl From<Eqep1> for bool {
    #[inline(always)]
    fn from(variant: Eqep1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `eqep1` reader - Enable the power to IP, KEY must be set to 26h to write to this bit"]
pub type Eqep1R = crate::BitReader<Eqep1>;
impl Eqep1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Eqep1 {
        match self.bits {
            false => Eqep1::Disable,
            true => Eqep1::Enable,
        }
    }
    #[doc = "Disable Power"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Eqep1::Disable
    }
    #[doc = "Enable Power"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Eqep1::Enable
    }
}
#[doc = "Field `eqep1` writer - Enable the power to IP, KEY must be set to 26h to write to this bit"]
pub type Eqep1W<'a, REG> = crate::BitWriter<'a, REG, Eqep1>;
impl<'a, REG> Eqep1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Power"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Eqep1::Disable)
    }
    #[doc = "Enable Power"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Eqep1::Enable)
    }
}
#[doc = "Enable the power to IP, KEY must be set to 26h to write to this bit\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eqep2 {
    #[doc = "0: Disable Power"]
    Disable = 0,
    #[doc = "1: Enable Power"]
    Enable = 1,
}
impl From<Eqep2> for bool {
    #[inline(always)]
    fn from(variant: Eqep2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `eqep2` reader - Enable the power to IP, KEY must be set to 26h to write to this bit"]
pub type Eqep2R = crate::BitReader<Eqep2>;
impl Eqep2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Eqep2 {
        match self.bits {
            false => Eqep2::Disable,
            true => Eqep2::Enable,
        }
    }
    #[doc = "Disable Power"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Eqep2::Disable
    }
    #[doc = "Enable Power"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Eqep2::Enable
    }
}
#[doc = "Field `eqep2` writer - Enable the power to IP, KEY must be set to 26h to write to this bit"]
pub type Eqep2W<'a, REG> = crate::BitWriter<'a, REG, Eqep2>;
impl<'a, REG> Eqep2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Power"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Eqep2::Disable)
    }
    #[doc = "Enable Power"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Eqep2::Enable)
    }
}
#[doc = "Enable the power to IP, KEY must be set to 26h to write to this bit\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ecap0 {
    #[doc = "0: Disable Power"]
    Disable = 0,
    #[doc = "1: Enable Power"]
    Enable = 1,
}
impl From<Ecap0> for bool {
    #[inline(always)]
    fn from(variant: Ecap0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ecap0` reader - Enable the power to IP, KEY must be set to 26h to write to this bit"]
pub type Ecap0R = crate::BitReader<Ecap0>;
impl Ecap0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ecap0 {
        match self.bits {
            false => Ecap0::Disable,
            true => Ecap0::Enable,
        }
    }
    #[doc = "Disable Power"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Ecap0::Disable
    }
    #[doc = "Enable Power"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Ecap0::Enable
    }
}
#[doc = "Field `ecap0` writer - Enable the power to IP, KEY must be set to 26h to write to this bit"]
pub type Ecap0W<'a, REG> = crate::BitWriter<'a, REG, Ecap0>;
impl<'a, REG> Ecap0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Power"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Ecap0::Disable)
    }
    #[doc = "Enable Power"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Ecap0::Enable)
    }
}
#[doc = "Enable the power to IP, KEY must be set to 26h to write to this bit\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ecap1 {
    #[doc = "0: Disable Power"]
    Disable = 0,
    #[doc = "1: Enable Power"]
    Enable = 1,
}
impl From<Ecap1> for bool {
    #[inline(always)]
    fn from(variant: Ecap1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ecap1` reader - Enable the power to IP, KEY must be set to 26h to write to this bit"]
pub type Ecap1R = crate::BitReader<Ecap1>;
impl Ecap1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ecap1 {
        match self.bits {
            false => Ecap1::Disable,
            true => Ecap1::Enable,
        }
    }
    #[doc = "Disable Power"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Ecap1::Disable
    }
    #[doc = "Enable Power"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Ecap1::Enable
    }
}
#[doc = "Field `ecap1` writer - Enable the power to IP, KEY must be set to 26h to write to this bit"]
pub type Ecap1W<'a, REG> = crate::BitWriter<'a, REG, Ecap1>;
impl<'a, REG> Ecap1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Power"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Ecap1::Disable)
    }
    #[doc = "Enable Power"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Ecap1::Enable)
    }
}
#[doc = "Enable the power to IP, KEY must be set to 26h to write to this bit\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwm0 {
    #[doc = "0: Disable Power"]
    Disable = 0,
    #[doc = "1: Enable Power"]
    Enable = 1,
}
impl From<Pwm0> for bool {
    #[inline(always)]
    fn from(variant: Pwm0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `pwm0` reader - Enable the power to IP, KEY must be set to 26h to write to this bit"]
pub type Pwm0R = crate::BitReader<Pwm0>;
impl Pwm0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwm0 {
        match self.bits {
            false => Pwm0::Disable,
            true => Pwm0::Enable,
        }
    }
    #[doc = "Disable Power"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Pwm0::Disable
    }
    #[doc = "Enable Power"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Pwm0::Enable
    }
}
#[doc = "Field `pwm0` writer - Enable the power to IP, KEY must be set to 26h to write to this bit"]
pub type Pwm0W<'a, REG> = crate::BitWriter<'a, REG, Pwm0>;
impl<'a, REG> Pwm0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Power"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Pwm0::Disable)
    }
    #[doc = "Enable Power"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Pwm0::Enable)
    }
}
#[doc = "Enable the power to IP, KEY must be set to 26h to write to this bit\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwm1 {
    #[doc = "0: Disable Power"]
    Disable = 0,
    #[doc = "1: Enable Power"]
    Enable = 1,
}
impl From<Pwm1> for bool {
    #[inline(always)]
    fn from(variant: Pwm1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `pwm1` reader - Enable the power to IP, KEY must be set to 26h to write to this bit"]
pub type Pwm1R = crate::BitReader<Pwm1>;
impl Pwm1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwm1 {
        match self.bits {
            false => Pwm1::Disable,
            true => Pwm1::Enable,
        }
    }
    #[doc = "Disable Power"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Pwm1::Disable
    }
    #[doc = "Enable Power"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Pwm1::Enable
    }
}
#[doc = "Field `pwm1` writer - Enable the power to IP, KEY must be set to 26h to write to this bit"]
pub type Pwm1W<'a, REG> = crate::BitWriter<'a, REG, Pwm1>;
impl<'a, REG> Pwm1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Power"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Pwm1::Disable)
    }
    #[doc = "Enable Power"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Pwm1::Enable)
    }
}
#[doc = "Enable the power to IP, KEY must be set to 26h to write to this bit\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwm2 {
    #[doc = "0: Disable Power"]
    Disable = 0,
    #[doc = "1: Enable Power"]
    Enable = 1,
}
impl From<Pwm2> for bool {
    #[inline(always)]
    fn from(variant: Pwm2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `pwm2` reader - Enable the power to IP, KEY must be set to 26h to write to this bit"]
pub type Pwm2R = crate::BitReader<Pwm2>;
impl Pwm2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwm2 {
        match self.bits {
            false => Pwm2::Disable,
            true => Pwm2::Enable,
        }
    }
    #[doc = "Disable Power"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Pwm2::Disable
    }
    #[doc = "Enable Power"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Pwm2::Enable
    }
}
#[doc = "Field `pwm2` writer - Enable the power to IP, KEY must be set to 26h to write to this bit"]
pub type Pwm2W<'a, REG> = crate::BitWriter<'a, REG, Pwm2>;
impl<'a, REG> Pwm2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Power"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Pwm2::Disable)
    }
    #[doc = "Enable Power"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Pwm2::Enable)
    }
}
#[doc = "Enable the power to IP, KEY must be set to 26h to write to this bit\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwm3 {
    #[doc = "0: Disable Power"]
    Disable = 0,
    #[doc = "1: Enable Power"]
    Enable = 1,
}
impl From<Pwm3> for bool {
    #[inline(always)]
    fn from(variant: Pwm3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `pwm3` reader - Enable the power to IP, KEY must be set to 26h to write to this bit"]
pub type Pwm3R = crate::BitReader<Pwm3>;
impl Pwm3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwm3 {
        match self.bits {
            false => Pwm3::Disable,
            true => Pwm3::Enable,
        }
    }
    #[doc = "Disable Power"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Pwm3::Disable
    }
    #[doc = "Enable Power"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Pwm3::Enable
    }
}
#[doc = "Field `pwm3` writer - Enable the power to IP, KEY must be set to 26h to write to this bit"]
pub type Pwm3W<'a, REG> = crate::BitWriter<'a, REG, Pwm3>;
impl<'a, REG> Pwm3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Power"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Pwm3::Disable)
    }
    #[doc = "Enable Power"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Pwm3::Enable)
    }
}
#[doc = "Enable the power to IP, KEY must be set to 26h to write to this bit\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwm4 {
    #[doc = "0: Disable Power"]
    Disable = 0,
    #[doc = "1: Enable Power"]
    Enable = 1,
}
impl From<Pwm4> for bool {
    #[inline(always)]
    fn from(variant: Pwm4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `pwm4` reader - Enable the power to IP, KEY must be set to 26h to write to this bit"]
pub type Pwm4R = crate::BitReader<Pwm4>;
impl Pwm4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwm4 {
        match self.bits {
            false => Pwm4::Disable,
            true => Pwm4::Enable,
        }
    }
    #[doc = "Disable Power"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Pwm4::Disable
    }
    #[doc = "Enable Power"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Pwm4::Enable
    }
}
#[doc = "Field `pwm4` writer - Enable the power to IP, KEY must be set to 26h to write to this bit"]
pub type Pwm4W<'a, REG> = crate::BitWriter<'a, REG, Pwm4>;
impl<'a, REG> Pwm4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Power"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Pwm4::Disable)
    }
    #[doc = "Enable Power"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Pwm4::Enable)
    }
}
#[doc = "Enable the power to IP, KEY must be set to 26h to write to this bit\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmpss0 {
    #[doc = "0: Disable Power"]
    Disable = 0,
    #[doc = "1: Enable Power"]
    Enable = 1,
}
impl From<Cmpss0> for bool {
    #[inline(always)]
    fn from(variant: Cmpss0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `cmpss0` reader - Enable the power to IP, KEY must be set to 26h to write to this bit"]
pub type Cmpss0R = crate::BitReader<Cmpss0>;
impl Cmpss0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmpss0 {
        match self.bits {
            false => Cmpss0::Disable,
            true => Cmpss0::Enable,
        }
    }
    #[doc = "Disable Power"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Cmpss0::Disable
    }
    #[doc = "Enable Power"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Cmpss0::Enable
    }
}
#[doc = "Field `cmpss0` writer - Enable the power to IP, KEY must be set to 26h to write to this bit"]
pub type Cmpss0W<'a, REG> = crate::BitWriter<'a, REG, Cmpss0>;
impl<'a, REG> Cmpss0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Power"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpss0::Disable)
    }
    #[doc = "Enable Power"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpss0::Enable)
    }
}
#[doc = "Enable the power to IP, KEY must be set to 26h to write to this bit\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmpss1 {
    #[doc = "0: Disable Power"]
    Disable = 0,
    #[doc = "1: Enable Power"]
    Enable = 1,
}
impl From<Cmpss1> for bool {
    #[inline(always)]
    fn from(variant: Cmpss1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `cmpss1` reader - Enable the power to IP, KEY must be set to 26h to write to this bit"]
pub type Cmpss1R = crate::BitReader<Cmpss1>;
impl Cmpss1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmpss1 {
        match self.bits {
            false => Cmpss1::Disable,
            true => Cmpss1::Enable,
        }
    }
    #[doc = "Disable Power"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Cmpss1::Disable
    }
    #[doc = "Enable Power"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Cmpss1::Enable
    }
}
#[doc = "Field `cmpss1` writer - Enable the power to IP, KEY must be set to 26h to write to this bit"]
pub type Cmpss1W<'a, REG> = crate::BitWriter<'a, REG, Cmpss1>;
impl<'a, REG> Cmpss1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Power"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpss1::Disable)
    }
    #[doc = "Enable Power"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpss1::Enable)
    }
}
#[doc = "Enable the power to IP, KEY must be set to 26h to write to this bit\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmpss2 {
    #[doc = "0: Disable Power"]
    Disable = 0,
    #[doc = "1: Enable Power"]
    Enable = 1,
}
impl From<Cmpss2> for bool {
    #[inline(always)]
    fn from(variant: Cmpss2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `cmpss2` reader - Enable the power to IP, KEY must be set to 26h to write to this bit"]
pub type Cmpss2R = crate::BitReader<Cmpss2>;
impl Cmpss2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmpss2 {
        match self.bits {
            false => Cmpss2::Disable,
            true => Cmpss2::Enable,
        }
    }
    #[doc = "Disable Power"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Cmpss2::Disable
    }
    #[doc = "Enable Power"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Cmpss2::Enable
    }
}
#[doc = "Field `cmpss2` writer - Enable the power to IP, KEY must be set to 26h to write to this bit"]
pub type Cmpss2W<'a, REG> = crate::BitWriter<'a, REG, Cmpss2>;
impl<'a, REG> Cmpss2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Power"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpss2::Disable)
    }
    #[doc = "Enable Power"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpss2::Enable)
    }
}
#[doc = "Enable the power to IP, KEY must be set to 26h to write to this bit\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmpss3 {
    #[doc = "0: Disable Power"]
    Disable = 0,
    #[doc = "1: Enable Power"]
    Enable = 1,
}
impl From<Cmpss3> for bool {
    #[inline(always)]
    fn from(variant: Cmpss3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `cmpss3` reader - Enable the power to IP, KEY must be set to 26h to write to this bit"]
pub type Cmpss3R = crate::BitReader<Cmpss3>;
impl Cmpss3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmpss3 {
        match self.bits {
            false => Cmpss3::Disable,
            true => Cmpss3::Enable,
        }
    }
    #[doc = "Disable Power"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Cmpss3::Disable
    }
    #[doc = "Enable Power"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Cmpss3::Enable
    }
}
#[doc = "Field `cmpss3` writer - Enable the power to IP, KEY must be set to 26h to write to this bit"]
pub type Cmpss3W<'a, REG> = crate::BitWriter<'a, REG, Cmpss3>;
impl<'a, REG> Cmpss3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Power"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpss3::Disable)
    }
    #[doc = "Enable Power"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpss3::Enable)
    }
}
#[doc = "Enable the power to IP, KEY must be set to 26h to write to this bit\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Xbar {
    #[doc = "0: Disable Power"]
    Disable = 0,
    #[doc = "1: Enable Power"]
    Enable = 1,
}
impl From<Xbar> for bool {
    #[inline(always)]
    fn from(variant: Xbar) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `xbar` reader - Enable the power to IP, KEY must be set to 26h to write to this bit"]
pub type XbarR = crate::BitReader<Xbar>;
impl XbarR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Xbar {
        match self.bits {
            false => Xbar::Disable,
            true => Xbar::Enable,
        }
    }
    #[doc = "Disable Power"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Xbar::Disable
    }
    #[doc = "Enable Power"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Xbar::Enable
    }
}
#[doc = "Field `xbar` writer - Enable the power to IP, KEY must be set to 26h to write to this bit"]
pub type XbarW<'a, REG> = crate::BitWriter<'a, REG, Xbar>;
impl<'a, REG> XbarW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Power"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Xbar::Disable)
    }
    #[doc = "Enable Power"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Xbar::Enable)
    }
}
#[doc = "Write key: the write only takes effect if KEY is 0x26\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Key {
    #[doc = "38: KEY to allow write access to this register"]
    Unlock = 38,
}
impl From<Key> for u8 {
    #[inline(always)]
    fn from(variant: Key) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Key {
    type Ux = u8;
}
impl crate::IsEnum for Key {}
#[doc = "Field `KEY` writer - Write key: the write only takes effect if KEY is 0x26"]
pub type KeyW<'a, REG> = crate::FieldWriter<'a, REG, 8, Key, crate::Safe>;
impl<'a, REG> KeyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "KEY to allow write access to this register"]
    #[inline(always)]
    pub fn unlock(self) -> &'a mut crate::W<REG> {
        self.variant(Key::Unlock)
    }
}
impl R {
    #[doc = "Bit 0 - Enable the power to IP, KEY must be set to 26h to write to this bit"]
    #[inline(always)]
    pub fn eqep0(&self) -> Eqep0R {
        Eqep0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable the power to IP, KEY must be set to 26h to write to this bit"]
    #[inline(always)]
    pub fn eqep1(&self) -> Eqep1R {
        Eqep1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable the power to IP, KEY must be set to 26h to write to this bit"]
    #[inline(always)]
    pub fn eqep2(&self) -> Eqep2R {
        Eqep2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable the power to IP, KEY must be set to 26h to write to this bit"]
    #[inline(always)]
    pub fn ecap0(&self) -> Ecap0R {
        Ecap0R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable the power to IP, KEY must be set to 26h to write to this bit"]
    #[inline(always)]
    pub fn ecap1(&self) -> Ecap1R {
        Ecap1R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable the power to IP, KEY must be set to 26h to write to this bit"]
    #[inline(always)]
    pub fn pwm0(&self) -> Pwm0R {
        Pwm0R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable the power to IP, KEY must be set to 26h to write to this bit"]
    #[inline(always)]
    pub fn pwm1(&self) -> Pwm1R {
        Pwm1R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable the power to IP, KEY must be set to 26h to write to this bit"]
    #[inline(always)]
    pub fn pwm2(&self) -> Pwm2R {
        Pwm2R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable the power to IP, KEY must be set to 26h to write to this bit"]
    #[inline(always)]
    pub fn pwm3(&self) -> Pwm3R {
        Pwm3R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable the power to IP, KEY must be set to 26h to write to this bit"]
    #[inline(always)]
    pub fn pwm4(&self) -> Pwm4R {
        Pwm4R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable the power to IP, KEY must be set to 26h to write to this bit"]
    #[inline(always)]
    pub fn cmpss0(&self) -> Cmpss0R {
        Cmpss0R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable the power to IP, KEY must be set to 26h to write to this bit"]
    #[inline(always)]
    pub fn cmpss1(&self) -> Cmpss1R {
        Cmpss1R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable the power to IP, KEY must be set to 26h to write to this bit"]
    #[inline(always)]
    pub fn cmpss2(&self) -> Cmpss2R {
        Cmpss2R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable the power to IP, KEY must be set to 26h to write to this bit"]
    #[inline(always)]
    pub fn cmpss3(&self) -> Cmpss3R {
        Cmpss3R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Enable the power to IP, KEY must be set to 26h to write to this bit"]
    #[inline(always)]
    pub fn xbar(&self) -> XbarR {
        XbarR::new(((self.bits >> 14) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PWREN_MCPERIPH")
            .field("eqep0", &self.eqep0())
            .field("eqep1", &self.eqep1())
            .field("eqep2", &self.eqep2())
            .field("ecap0", &self.ecap0())
            .field("ecap1", &self.ecap1())
            .field("pwm0", &self.pwm0())
            .field("pwm1", &self.pwm1())
            .field("pwm2", &self.pwm2())
            .field("pwm3", &self.pwm3())
            .field("pwm4", &self.pwm4())
            .field("cmpss0", &self.cmpss0())
            .field("cmpss1", &self.cmpss1())
            .field("cmpss2", &self.cmpss2())
            .field("cmpss3", &self.cmpss3())
            .field("xbar", &self.xbar())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Enable the power to IP, KEY must be set to 26h to write to this bit"]
    #[inline(always)]
    pub fn eqep0(&mut self) -> Eqep0W<'_, PwrenMcperiphSpec> {
        Eqep0W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable the power to IP, KEY must be set to 26h to write to this bit"]
    #[inline(always)]
    pub fn eqep1(&mut self) -> Eqep1W<'_, PwrenMcperiphSpec> {
        Eqep1W::new(self, 1)
    }
    #[doc = "Bit 2 - Enable the power to IP, KEY must be set to 26h to write to this bit"]
    #[inline(always)]
    pub fn eqep2(&mut self) -> Eqep2W<'_, PwrenMcperiphSpec> {
        Eqep2W::new(self, 2)
    }
    #[doc = "Bit 3 - Enable the power to IP, KEY must be set to 26h to write to this bit"]
    #[inline(always)]
    pub fn ecap0(&mut self) -> Ecap0W<'_, PwrenMcperiphSpec> {
        Ecap0W::new(self, 3)
    }
    #[doc = "Bit 4 - Enable the power to IP, KEY must be set to 26h to write to this bit"]
    #[inline(always)]
    pub fn ecap1(&mut self) -> Ecap1W<'_, PwrenMcperiphSpec> {
        Ecap1W::new(self, 4)
    }
    #[doc = "Bit 5 - Enable the power to IP, KEY must be set to 26h to write to this bit"]
    #[inline(always)]
    pub fn pwm0(&mut self) -> Pwm0W<'_, PwrenMcperiphSpec> {
        Pwm0W::new(self, 5)
    }
    #[doc = "Bit 6 - Enable the power to IP, KEY must be set to 26h to write to this bit"]
    #[inline(always)]
    pub fn pwm1(&mut self) -> Pwm1W<'_, PwrenMcperiphSpec> {
        Pwm1W::new(self, 6)
    }
    #[doc = "Bit 7 - Enable the power to IP, KEY must be set to 26h to write to this bit"]
    #[inline(always)]
    pub fn pwm2(&mut self) -> Pwm2W<'_, PwrenMcperiphSpec> {
        Pwm2W::new(self, 7)
    }
    #[doc = "Bit 8 - Enable the power to IP, KEY must be set to 26h to write to this bit"]
    #[inline(always)]
    pub fn pwm3(&mut self) -> Pwm3W<'_, PwrenMcperiphSpec> {
        Pwm3W::new(self, 8)
    }
    #[doc = "Bit 9 - Enable the power to IP, KEY must be set to 26h to write to this bit"]
    #[inline(always)]
    pub fn pwm4(&mut self) -> Pwm4W<'_, PwrenMcperiphSpec> {
        Pwm4W::new(self, 9)
    }
    #[doc = "Bit 10 - Enable the power to IP, KEY must be set to 26h to write to this bit"]
    #[inline(always)]
    pub fn cmpss0(&mut self) -> Cmpss0W<'_, PwrenMcperiphSpec> {
        Cmpss0W::new(self, 10)
    }
    #[doc = "Bit 11 - Enable the power to IP, KEY must be set to 26h to write to this bit"]
    #[inline(always)]
    pub fn cmpss1(&mut self) -> Cmpss1W<'_, PwrenMcperiphSpec> {
        Cmpss1W::new(self, 11)
    }
    #[doc = "Bit 12 - Enable the power to IP, KEY must be set to 26h to write to this bit"]
    #[inline(always)]
    pub fn cmpss2(&mut self) -> Cmpss2W<'_, PwrenMcperiphSpec> {
        Cmpss2W::new(self, 12)
    }
    #[doc = "Bit 13 - Enable the power to IP, KEY must be set to 26h to write to this bit"]
    #[inline(always)]
    pub fn cmpss3(&mut self) -> Cmpss3W<'_, PwrenMcperiphSpec> {
        Cmpss3W::new(self, 13)
    }
    #[doc = "Bit 14 - Enable the power to IP, KEY must be set to 26h to write to this bit"]
    #[inline(always)]
    pub fn xbar(&mut self) -> XbarW<'_, PwrenMcperiphSpec> {
        XbarW::new(self, 14)
    }
    #[doc = "Bits 24:31 - Write key: the write only takes effect if KEY is 0x26"]
    #[inline(always)]
    pub fn key(&mut self) -> KeyW<'_, PwrenMcperiphSpec> {
        KeyW::new(self, 24)
    }
}
#[doc = "Register to control the power state\n\nYou can [`read`](crate::Reg::read) this register and get [`pwren_mcperiph::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwren_mcperiph::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PwrenMcperiphSpec;
impl crate::RegisterSpec for PwrenMcperiphSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwren_mcperiph::R`](R) reader structure"]
impl crate::Readable for PwrenMcperiphSpec {}
#[doc = "`write(|w| ..)` method takes [`pwren_mcperiph::W`](W) writer structure"]
impl crate::Writable for PwrenMcperiphSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets PWREN_MCPERIPH to value 0"]
impl crate::Resettable for PwrenMcperiphSpec {}
