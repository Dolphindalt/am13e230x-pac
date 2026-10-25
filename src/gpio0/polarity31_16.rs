#[doc = "Register `POLARITY31_16` reader"]
pub type R = crate::R<Polarity31_16Spec>;
#[doc = "Register `POLARITY31_16` writer"]
pub type W = crate::W<Polarity31_16Spec>;
#[doc = "Enables and configures edge detection polarity for DIO16.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dio16 {
    #[doc = "0: Edge detection disabled"]
    Disable = 0,
    #[doc = "1: Detects rising edge of input event"]
    Rise = 1,
    #[doc = "2: Detects falling edge of input event"]
    Fall = 2,
    #[doc = "3: Detects both rising and falling edge of input event"]
    RiseFall = 3,
}
impl From<Dio16> for u8 {
    #[inline(always)]
    fn from(variant: Dio16) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dio16 {
    type Ux = u8;
}
impl crate::IsEnum for Dio16 {}
#[doc = "Field `DIO16` reader - Enables and configures edge detection polarity for DIO16."]
pub type Dio16R = crate::FieldReader<Dio16>;
impl Dio16R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio16 {
        match self.bits {
            0 => Dio16::Disable,
            1 => Dio16::Rise,
            2 => Dio16::Fall,
            3 => Dio16::RiseFall,
            _ => unreachable!(),
        }
    }
    #[doc = "Edge detection disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Dio16::Disable
    }
    #[doc = "Detects rising edge of input event"]
    #[inline(always)]
    pub fn is_rise(&self) -> bool {
        *self == Dio16::Rise
    }
    #[doc = "Detects falling edge of input event"]
    #[inline(always)]
    pub fn is_fall(&self) -> bool {
        *self == Dio16::Fall
    }
    #[doc = "Detects both rising and falling edge of input event"]
    #[inline(always)]
    pub fn is_rise_fall(&self) -> bool {
        *self == Dio16::RiseFall
    }
}
#[doc = "Field `DIO16` writer - Enables and configures edge detection polarity for DIO16."]
pub type Dio16W<'a, REG> = crate::FieldWriter<'a, REG, 2, Dio16, crate::Safe>;
impl<'a, REG> Dio16W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Edge detection disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Dio16::Disable)
    }
    #[doc = "Detects rising edge of input event"]
    #[inline(always)]
    pub fn rise(self) -> &'a mut crate::W<REG> {
        self.variant(Dio16::Rise)
    }
    #[doc = "Detects falling edge of input event"]
    #[inline(always)]
    pub fn fall(self) -> &'a mut crate::W<REG> {
        self.variant(Dio16::Fall)
    }
    #[doc = "Detects both rising and falling edge of input event"]
    #[inline(always)]
    pub fn rise_fall(self) -> &'a mut crate::W<REG> {
        self.variant(Dio16::RiseFall)
    }
}
#[doc = "Enables and configures edge detection polarity for DIO17.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dio17 {
    #[doc = "0: Edge detection disabled"]
    Disable = 0,
    #[doc = "1: Detects rising edge of input event"]
    Rise = 1,
    #[doc = "2: Detects falling edge of input event"]
    Fall = 2,
    #[doc = "3: Detects both rising and falling edge of input event"]
    RiseFall = 3,
}
impl From<Dio17> for u8 {
    #[inline(always)]
    fn from(variant: Dio17) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dio17 {
    type Ux = u8;
}
impl crate::IsEnum for Dio17 {}
#[doc = "Field `DIO17` reader - Enables and configures edge detection polarity for DIO17."]
pub type Dio17R = crate::FieldReader<Dio17>;
impl Dio17R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio17 {
        match self.bits {
            0 => Dio17::Disable,
            1 => Dio17::Rise,
            2 => Dio17::Fall,
            3 => Dio17::RiseFall,
            _ => unreachable!(),
        }
    }
    #[doc = "Edge detection disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Dio17::Disable
    }
    #[doc = "Detects rising edge of input event"]
    #[inline(always)]
    pub fn is_rise(&self) -> bool {
        *self == Dio17::Rise
    }
    #[doc = "Detects falling edge of input event"]
    #[inline(always)]
    pub fn is_fall(&self) -> bool {
        *self == Dio17::Fall
    }
    #[doc = "Detects both rising and falling edge of input event"]
    #[inline(always)]
    pub fn is_rise_fall(&self) -> bool {
        *self == Dio17::RiseFall
    }
}
#[doc = "Field `DIO17` writer - Enables and configures edge detection polarity for DIO17."]
pub type Dio17W<'a, REG> = crate::FieldWriter<'a, REG, 2, Dio17, crate::Safe>;
impl<'a, REG> Dio17W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Edge detection disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Dio17::Disable)
    }
    #[doc = "Detects rising edge of input event"]
    #[inline(always)]
    pub fn rise(self) -> &'a mut crate::W<REG> {
        self.variant(Dio17::Rise)
    }
    #[doc = "Detects falling edge of input event"]
    #[inline(always)]
    pub fn fall(self) -> &'a mut crate::W<REG> {
        self.variant(Dio17::Fall)
    }
    #[doc = "Detects both rising and falling edge of input event"]
    #[inline(always)]
    pub fn rise_fall(self) -> &'a mut crate::W<REG> {
        self.variant(Dio17::RiseFall)
    }
}
#[doc = "Enables and configures edge detection polarity for DIO18.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dio18 {
    #[doc = "0: Edge detection disabled"]
    Disable = 0,
    #[doc = "1: Detects rising edge of input event"]
    Rise = 1,
    #[doc = "2: Detects falling edge of input event"]
    Fall = 2,
    #[doc = "3: Detects both rising and falling edge of input event"]
    RiseFall = 3,
}
impl From<Dio18> for u8 {
    #[inline(always)]
    fn from(variant: Dio18) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dio18 {
    type Ux = u8;
}
impl crate::IsEnum for Dio18 {}
#[doc = "Field `DIO18` reader - Enables and configures edge detection polarity for DIO18."]
pub type Dio18R = crate::FieldReader<Dio18>;
impl Dio18R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio18 {
        match self.bits {
            0 => Dio18::Disable,
            1 => Dio18::Rise,
            2 => Dio18::Fall,
            3 => Dio18::RiseFall,
            _ => unreachable!(),
        }
    }
    #[doc = "Edge detection disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Dio18::Disable
    }
    #[doc = "Detects rising edge of input event"]
    #[inline(always)]
    pub fn is_rise(&self) -> bool {
        *self == Dio18::Rise
    }
    #[doc = "Detects falling edge of input event"]
    #[inline(always)]
    pub fn is_fall(&self) -> bool {
        *self == Dio18::Fall
    }
    #[doc = "Detects both rising and falling edge of input event"]
    #[inline(always)]
    pub fn is_rise_fall(&self) -> bool {
        *self == Dio18::RiseFall
    }
}
#[doc = "Field `DIO18` writer - Enables and configures edge detection polarity for DIO18."]
pub type Dio18W<'a, REG> = crate::FieldWriter<'a, REG, 2, Dio18, crate::Safe>;
impl<'a, REG> Dio18W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Edge detection disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Dio18::Disable)
    }
    #[doc = "Detects rising edge of input event"]
    #[inline(always)]
    pub fn rise(self) -> &'a mut crate::W<REG> {
        self.variant(Dio18::Rise)
    }
    #[doc = "Detects falling edge of input event"]
    #[inline(always)]
    pub fn fall(self) -> &'a mut crate::W<REG> {
        self.variant(Dio18::Fall)
    }
    #[doc = "Detects both rising and falling edge of input event"]
    #[inline(always)]
    pub fn rise_fall(self) -> &'a mut crate::W<REG> {
        self.variant(Dio18::RiseFall)
    }
}
#[doc = "Enables and configures edge detection polarity for DIO19.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dio19 {
    #[doc = "0: Edge detection disabled"]
    Disable = 0,
    #[doc = "1: Detects rising edge of input event"]
    Rise = 1,
    #[doc = "2: Detects falling edge of input event"]
    Fall = 2,
    #[doc = "3: Detects both rising and falling edge of input event"]
    RiseFall = 3,
}
impl From<Dio19> for u8 {
    #[inline(always)]
    fn from(variant: Dio19) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dio19 {
    type Ux = u8;
}
impl crate::IsEnum for Dio19 {}
#[doc = "Field `DIO19` reader - Enables and configures edge detection polarity for DIO19."]
pub type Dio19R = crate::FieldReader<Dio19>;
impl Dio19R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio19 {
        match self.bits {
            0 => Dio19::Disable,
            1 => Dio19::Rise,
            2 => Dio19::Fall,
            3 => Dio19::RiseFall,
            _ => unreachable!(),
        }
    }
    #[doc = "Edge detection disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Dio19::Disable
    }
    #[doc = "Detects rising edge of input event"]
    #[inline(always)]
    pub fn is_rise(&self) -> bool {
        *self == Dio19::Rise
    }
    #[doc = "Detects falling edge of input event"]
    #[inline(always)]
    pub fn is_fall(&self) -> bool {
        *self == Dio19::Fall
    }
    #[doc = "Detects both rising and falling edge of input event"]
    #[inline(always)]
    pub fn is_rise_fall(&self) -> bool {
        *self == Dio19::RiseFall
    }
}
#[doc = "Field `DIO19` writer - Enables and configures edge detection polarity for DIO19."]
pub type Dio19W<'a, REG> = crate::FieldWriter<'a, REG, 2, Dio19, crate::Safe>;
impl<'a, REG> Dio19W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Edge detection disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Dio19::Disable)
    }
    #[doc = "Detects rising edge of input event"]
    #[inline(always)]
    pub fn rise(self) -> &'a mut crate::W<REG> {
        self.variant(Dio19::Rise)
    }
    #[doc = "Detects falling edge of input event"]
    #[inline(always)]
    pub fn fall(self) -> &'a mut crate::W<REG> {
        self.variant(Dio19::Fall)
    }
    #[doc = "Detects both rising and falling edge of input event"]
    #[inline(always)]
    pub fn rise_fall(self) -> &'a mut crate::W<REG> {
        self.variant(Dio19::RiseFall)
    }
}
#[doc = "Enables and configures edge detection polarity for DIO20.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dio20 {
    #[doc = "0: Edge detection disabled"]
    Disable = 0,
    #[doc = "1: Detects rising edge of input event"]
    Rise = 1,
    #[doc = "2: Detects falling edge of input event"]
    Fall = 2,
    #[doc = "3: Detects both rising and falling edge of input event"]
    RiseFall = 3,
}
impl From<Dio20> for u8 {
    #[inline(always)]
    fn from(variant: Dio20) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dio20 {
    type Ux = u8;
}
impl crate::IsEnum for Dio20 {}
#[doc = "Field `DIO20` reader - Enables and configures edge detection polarity for DIO20."]
pub type Dio20R = crate::FieldReader<Dio20>;
impl Dio20R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio20 {
        match self.bits {
            0 => Dio20::Disable,
            1 => Dio20::Rise,
            2 => Dio20::Fall,
            3 => Dio20::RiseFall,
            _ => unreachable!(),
        }
    }
    #[doc = "Edge detection disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Dio20::Disable
    }
    #[doc = "Detects rising edge of input event"]
    #[inline(always)]
    pub fn is_rise(&self) -> bool {
        *self == Dio20::Rise
    }
    #[doc = "Detects falling edge of input event"]
    #[inline(always)]
    pub fn is_fall(&self) -> bool {
        *self == Dio20::Fall
    }
    #[doc = "Detects both rising and falling edge of input event"]
    #[inline(always)]
    pub fn is_rise_fall(&self) -> bool {
        *self == Dio20::RiseFall
    }
}
#[doc = "Field `DIO20` writer - Enables and configures edge detection polarity for DIO20."]
pub type Dio20W<'a, REG> = crate::FieldWriter<'a, REG, 2, Dio20, crate::Safe>;
impl<'a, REG> Dio20W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Edge detection disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Dio20::Disable)
    }
    #[doc = "Detects rising edge of input event"]
    #[inline(always)]
    pub fn rise(self) -> &'a mut crate::W<REG> {
        self.variant(Dio20::Rise)
    }
    #[doc = "Detects falling edge of input event"]
    #[inline(always)]
    pub fn fall(self) -> &'a mut crate::W<REG> {
        self.variant(Dio20::Fall)
    }
    #[doc = "Detects both rising and falling edge of input event"]
    #[inline(always)]
    pub fn rise_fall(self) -> &'a mut crate::W<REG> {
        self.variant(Dio20::RiseFall)
    }
}
#[doc = "Enables and configures edge detection polarity for DIO21.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dio21 {
    #[doc = "0: Edge detection disabled"]
    Disable = 0,
    #[doc = "1: Detects rising edge of input event"]
    Rise = 1,
    #[doc = "2: Detects falling edge of input event"]
    Fall = 2,
    #[doc = "3: Detects both rising and falling edge of input event"]
    RiseFall = 3,
}
impl From<Dio21> for u8 {
    #[inline(always)]
    fn from(variant: Dio21) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dio21 {
    type Ux = u8;
}
impl crate::IsEnum for Dio21 {}
#[doc = "Field `DIO21` reader - Enables and configures edge detection polarity for DIO21."]
pub type Dio21R = crate::FieldReader<Dio21>;
impl Dio21R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio21 {
        match self.bits {
            0 => Dio21::Disable,
            1 => Dio21::Rise,
            2 => Dio21::Fall,
            3 => Dio21::RiseFall,
            _ => unreachable!(),
        }
    }
    #[doc = "Edge detection disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Dio21::Disable
    }
    #[doc = "Detects rising edge of input event"]
    #[inline(always)]
    pub fn is_rise(&self) -> bool {
        *self == Dio21::Rise
    }
    #[doc = "Detects falling edge of input event"]
    #[inline(always)]
    pub fn is_fall(&self) -> bool {
        *self == Dio21::Fall
    }
    #[doc = "Detects both rising and falling edge of input event"]
    #[inline(always)]
    pub fn is_rise_fall(&self) -> bool {
        *self == Dio21::RiseFall
    }
}
#[doc = "Field `DIO21` writer - Enables and configures edge detection polarity for DIO21."]
pub type Dio21W<'a, REG> = crate::FieldWriter<'a, REG, 2, Dio21, crate::Safe>;
impl<'a, REG> Dio21W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Edge detection disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Dio21::Disable)
    }
    #[doc = "Detects rising edge of input event"]
    #[inline(always)]
    pub fn rise(self) -> &'a mut crate::W<REG> {
        self.variant(Dio21::Rise)
    }
    #[doc = "Detects falling edge of input event"]
    #[inline(always)]
    pub fn fall(self) -> &'a mut crate::W<REG> {
        self.variant(Dio21::Fall)
    }
    #[doc = "Detects both rising and falling edge of input event"]
    #[inline(always)]
    pub fn rise_fall(self) -> &'a mut crate::W<REG> {
        self.variant(Dio21::RiseFall)
    }
}
#[doc = "Enables and configures edge detection polarity for DIO22.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dio22 {
    #[doc = "0: Edge detection disabled"]
    Disable = 0,
    #[doc = "1: Detects rising edge of input event"]
    Rise = 1,
    #[doc = "2: Detects falling edge of input event"]
    Fall = 2,
    #[doc = "3: Detects both rising and falling edge of input event"]
    RiseFall = 3,
}
impl From<Dio22> for u8 {
    #[inline(always)]
    fn from(variant: Dio22) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dio22 {
    type Ux = u8;
}
impl crate::IsEnum for Dio22 {}
#[doc = "Field `DIO22` reader - Enables and configures edge detection polarity for DIO22."]
pub type Dio22R = crate::FieldReader<Dio22>;
impl Dio22R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio22 {
        match self.bits {
            0 => Dio22::Disable,
            1 => Dio22::Rise,
            2 => Dio22::Fall,
            3 => Dio22::RiseFall,
            _ => unreachable!(),
        }
    }
    #[doc = "Edge detection disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Dio22::Disable
    }
    #[doc = "Detects rising edge of input event"]
    #[inline(always)]
    pub fn is_rise(&self) -> bool {
        *self == Dio22::Rise
    }
    #[doc = "Detects falling edge of input event"]
    #[inline(always)]
    pub fn is_fall(&self) -> bool {
        *self == Dio22::Fall
    }
    #[doc = "Detects both rising and falling edge of input event"]
    #[inline(always)]
    pub fn is_rise_fall(&self) -> bool {
        *self == Dio22::RiseFall
    }
}
#[doc = "Field `DIO22` writer - Enables and configures edge detection polarity for DIO22."]
pub type Dio22W<'a, REG> = crate::FieldWriter<'a, REG, 2, Dio22, crate::Safe>;
impl<'a, REG> Dio22W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Edge detection disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Dio22::Disable)
    }
    #[doc = "Detects rising edge of input event"]
    #[inline(always)]
    pub fn rise(self) -> &'a mut crate::W<REG> {
        self.variant(Dio22::Rise)
    }
    #[doc = "Detects falling edge of input event"]
    #[inline(always)]
    pub fn fall(self) -> &'a mut crate::W<REG> {
        self.variant(Dio22::Fall)
    }
    #[doc = "Detects both rising and falling edge of input event"]
    #[inline(always)]
    pub fn rise_fall(self) -> &'a mut crate::W<REG> {
        self.variant(Dio22::RiseFall)
    }
}
#[doc = "Enables and configures edge detection polarity for DIO23.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dio23 {
    #[doc = "0: Edge detection disabled"]
    Disable = 0,
    #[doc = "1: Detects rising edge of input event"]
    Rise = 1,
    #[doc = "2: Detects falling edge of input event"]
    Fall = 2,
    #[doc = "3: Detects both rising and falling edge of input event"]
    RiseFall = 3,
}
impl From<Dio23> for u8 {
    #[inline(always)]
    fn from(variant: Dio23) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dio23 {
    type Ux = u8;
}
impl crate::IsEnum for Dio23 {}
#[doc = "Field `DIO23` reader - Enables and configures edge detection polarity for DIO23."]
pub type Dio23R = crate::FieldReader<Dio23>;
impl Dio23R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio23 {
        match self.bits {
            0 => Dio23::Disable,
            1 => Dio23::Rise,
            2 => Dio23::Fall,
            3 => Dio23::RiseFall,
            _ => unreachable!(),
        }
    }
    #[doc = "Edge detection disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Dio23::Disable
    }
    #[doc = "Detects rising edge of input event"]
    #[inline(always)]
    pub fn is_rise(&self) -> bool {
        *self == Dio23::Rise
    }
    #[doc = "Detects falling edge of input event"]
    #[inline(always)]
    pub fn is_fall(&self) -> bool {
        *self == Dio23::Fall
    }
    #[doc = "Detects both rising and falling edge of input event"]
    #[inline(always)]
    pub fn is_rise_fall(&self) -> bool {
        *self == Dio23::RiseFall
    }
}
#[doc = "Field `DIO23` writer - Enables and configures edge detection polarity for DIO23."]
pub type Dio23W<'a, REG> = crate::FieldWriter<'a, REG, 2, Dio23, crate::Safe>;
impl<'a, REG> Dio23W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Edge detection disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Dio23::Disable)
    }
    #[doc = "Detects rising edge of input event"]
    #[inline(always)]
    pub fn rise(self) -> &'a mut crate::W<REG> {
        self.variant(Dio23::Rise)
    }
    #[doc = "Detects falling edge of input event"]
    #[inline(always)]
    pub fn fall(self) -> &'a mut crate::W<REG> {
        self.variant(Dio23::Fall)
    }
    #[doc = "Detects both rising and falling edge of input event"]
    #[inline(always)]
    pub fn rise_fall(self) -> &'a mut crate::W<REG> {
        self.variant(Dio23::RiseFall)
    }
}
#[doc = "Enables and configures edge detection polarity for DIO24.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dio24 {
    #[doc = "0: Edge detection disabled"]
    Disable = 0,
    #[doc = "1: Detects rising edge of input event"]
    Rise = 1,
    #[doc = "2: Detects falling edge of input event"]
    Fall = 2,
    #[doc = "3: Detects both rising and falling edge of input event"]
    RiseFall = 3,
}
impl From<Dio24> for u8 {
    #[inline(always)]
    fn from(variant: Dio24) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dio24 {
    type Ux = u8;
}
impl crate::IsEnum for Dio24 {}
#[doc = "Field `DIO24` reader - Enables and configures edge detection polarity for DIO24."]
pub type Dio24R = crate::FieldReader<Dio24>;
impl Dio24R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio24 {
        match self.bits {
            0 => Dio24::Disable,
            1 => Dio24::Rise,
            2 => Dio24::Fall,
            3 => Dio24::RiseFall,
            _ => unreachable!(),
        }
    }
    #[doc = "Edge detection disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Dio24::Disable
    }
    #[doc = "Detects rising edge of input event"]
    #[inline(always)]
    pub fn is_rise(&self) -> bool {
        *self == Dio24::Rise
    }
    #[doc = "Detects falling edge of input event"]
    #[inline(always)]
    pub fn is_fall(&self) -> bool {
        *self == Dio24::Fall
    }
    #[doc = "Detects both rising and falling edge of input event"]
    #[inline(always)]
    pub fn is_rise_fall(&self) -> bool {
        *self == Dio24::RiseFall
    }
}
#[doc = "Field `DIO24` writer - Enables and configures edge detection polarity for DIO24."]
pub type Dio24W<'a, REG> = crate::FieldWriter<'a, REG, 2, Dio24, crate::Safe>;
impl<'a, REG> Dio24W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Edge detection disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Dio24::Disable)
    }
    #[doc = "Detects rising edge of input event"]
    #[inline(always)]
    pub fn rise(self) -> &'a mut crate::W<REG> {
        self.variant(Dio24::Rise)
    }
    #[doc = "Detects falling edge of input event"]
    #[inline(always)]
    pub fn fall(self) -> &'a mut crate::W<REG> {
        self.variant(Dio24::Fall)
    }
    #[doc = "Detects both rising and falling edge of input event"]
    #[inline(always)]
    pub fn rise_fall(self) -> &'a mut crate::W<REG> {
        self.variant(Dio24::RiseFall)
    }
}
#[doc = "Enables and configures edge detection polarity for DIO25.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dio25 {
    #[doc = "0: Edge detection disabled"]
    Disable = 0,
    #[doc = "1: Detects rising edge of input event"]
    Rise = 1,
    #[doc = "2: Detects falling edge of input event"]
    Fall = 2,
    #[doc = "3: Detects both rising and falling edge of input event"]
    RiseFall = 3,
}
impl From<Dio25> for u8 {
    #[inline(always)]
    fn from(variant: Dio25) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dio25 {
    type Ux = u8;
}
impl crate::IsEnum for Dio25 {}
#[doc = "Field `DIO25` reader - Enables and configures edge detection polarity for DIO25."]
pub type Dio25R = crate::FieldReader<Dio25>;
impl Dio25R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio25 {
        match self.bits {
            0 => Dio25::Disable,
            1 => Dio25::Rise,
            2 => Dio25::Fall,
            3 => Dio25::RiseFall,
            _ => unreachable!(),
        }
    }
    #[doc = "Edge detection disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Dio25::Disable
    }
    #[doc = "Detects rising edge of input event"]
    #[inline(always)]
    pub fn is_rise(&self) -> bool {
        *self == Dio25::Rise
    }
    #[doc = "Detects falling edge of input event"]
    #[inline(always)]
    pub fn is_fall(&self) -> bool {
        *self == Dio25::Fall
    }
    #[doc = "Detects both rising and falling edge of input event"]
    #[inline(always)]
    pub fn is_rise_fall(&self) -> bool {
        *self == Dio25::RiseFall
    }
}
#[doc = "Field `DIO25` writer - Enables and configures edge detection polarity for DIO25."]
pub type Dio25W<'a, REG> = crate::FieldWriter<'a, REG, 2, Dio25, crate::Safe>;
impl<'a, REG> Dio25W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Edge detection disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Dio25::Disable)
    }
    #[doc = "Detects rising edge of input event"]
    #[inline(always)]
    pub fn rise(self) -> &'a mut crate::W<REG> {
        self.variant(Dio25::Rise)
    }
    #[doc = "Detects falling edge of input event"]
    #[inline(always)]
    pub fn fall(self) -> &'a mut crate::W<REG> {
        self.variant(Dio25::Fall)
    }
    #[doc = "Detects both rising and falling edge of input event"]
    #[inline(always)]
    pub fn rise_fall(self) -> &'a mut crate::W<REG> {
        self.variant(Dio25::RiseFall)
    }
}
#[doc = "Enables and configures edge detection polarity for DIO26.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dio26 {
    #[doc = "0: Edge detection disabled"]
    Disable = 0,
    #[doc = "1: Detects rising edge of input event"]
    Rise = 1,
    #[doc = "2: Detects falling edge of input event"]
    Fall = 2,
    #[doc = "3: Detects both rising and falling edge of input event"]
    RiseFall = 3,
}
impl From<Dio26> for u8 {
    #[inline(always)]
    fn from(variant: Dio26) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dio26 {
    type Ux = u8;
}
impl crate::IsEnum for Dio26 {}
#[doc = "Field `DIO26` reader - Enables and configures edge detection polarity for DIO26."]
pub type Dio26R = crate::FieldReader<Dio26>;
impl Dio26R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio26 {
        match self.bits {
            0 => Dio26::Disable,
            1 => Dio26::Rise,
            2 => Dio26::Fall,
            3 => Dio26::RiseFall,
            _ => unreachable!(),
        }
    }
    #[doc = "Edge detection disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Dio26::Disable
    }
    #[doc = "Detects rising edge of input event"]
    #[inline(always)]
    pub fn is_rise(&self) -> bool {
        *self == Dio26::Rise
    }
    #[doc = "Detects falling edge of input event"]
    #[inline(always)]
    pub fn is_fall(&self) -> bool {
        *self == Dio26::Fall
    }
    #[doc = "Detects both rising and falling edge of input event"]
    #[inline(always)]
    pub fn is_rise_fall(&self) -> bool {
        *self == Dio26::RiseFall
    }
}
#[doc = "Field `DIO26` writer - Enables and configures edge detection polarity for DIO26."]
pub type Dio26W<'a, REG> = crate::FieldWriter<'a, REG, 2, Dio26, crate::Safe>;
impl<'a, REG> Dio26W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Edge detection disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Dio26::Disable)
    }
    #[doc = "Detects rising edge of input event"]
    #[inline(always)]
    pub fn rise(self) -> &'a mut crate::W<REG> {
        self.variant(Dio26::Rise)
    }
    #[doc = "Detects falling edge of input event"]
    #[inline(always)]
    pub fn fall(self) -> &'a mut crate::W<REG> {
        self.variant(Dio26::Fall)
    }
    #[doc = "Detects both rising and falling edge of input event"]
    #[inline(always)]
    pub fn rise_fall(self) -> &'a mut crate::W<REG> {
        self.variant(Dio26::RiseFall)
    }
}
#[doc = "Enables and configures edge detection polarity for DIO27.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dio27 {
    #[doc = "0: Edge detection disabled"]
    Disable = 0,
    #[doc = "1: Detects rising edge of input event"]
    Rise = 1,
    #[doc = "2: Detects falling edge of input event"]
    Fall = 2,
    #[doc = "3: Detects both rising and falling edge of input event"]
    RiseFall = 3,
}
impl From<Dio27> for u8 {
    #[inline(always)]
    fn from(variant: Dio27) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dio27 {
    type Ux = u8;
}
impl crate::IsEnum for Dio27 {}
#[doc = "Field `DIO27` reader - Enables and configures edge detection polarity for DIO27."]
pub type Dio27R = crate::FieldReader<Dio27>;
impl Dio27R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio27 {
        match self.bits {
            0 => Dio27::Disable,
            1 => Dio27::Rise,
            2 => Dio27::Fall,
            3 => Dio27::RiseFall,
            _ => unreachable!(),
        }
    }
    #[doc = "Edge detection disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Dio27::Disable
    }
    #[doc = "Detects rising edge of input event"]
    #[inline(always)]
    pub fn is_rise(&self) -> bool {
        *self == Dio27::Rise
    }
    #[doc = "Detects falling edge of input event"]
    #[inline(always)]
    pub fn is_fall(&self) -> bool {
        *self == Dio27::Fall
    }
    #[doc = "Detects both rising and falling edge of input event"]
    #[inline(always)]
    pub fn is_rise_fall(&self) -> bool {
        *self == Dio27::RiseFall
    }
}
#[doc = "Field `DIO27` writer - Enables and configures edge detection polarity for DIO27."]
pub type Dio27W<'a, REG> = crate::FieldWriter<'a, REG, 2, Dio27, crate::Safe>;
impl<'a, REG> Dio27W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Edge detection disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Dio27::Disable)
    }
    #[doc = "Detects rising edge of input event"]
    #[inline(always)]
    pub fn rise(self) -> &'a mut crate::W<REG> {
        self.variant(Dio27::Rise)
    }
    #[doc = "Detects falling edge of input event"]
    #[inline(always)]
    pub fn fall(self) -> &'a mut crate::W<REG> {
        self.variant(Dio27::Fall)
    }
    #[doc = "Detects both rising and falling edge of input event"]
    #[inline(always)]
    pub fn rise_fall(self) -> &'a mut crate::W<REG> {
        self.variant(Dio27::RiseFall)
    }
}
#[doc = "Enables and configures edge detection polarity for DIO28.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dio28 {
    #[doc = "0: Edge detection disabled"]
    Disable = 0,
    #[doc = "1: Detects rising edge of input event"]
    Rise = 1,
    #[doc = "2: Detects falling edge of input event"]
    Fall = 2,
    #[doc = "3: Detects both rising and falling edge of input event"]
    RiseFall = 3,
}
impl From<Dio28> for u8 {
    #[inline(always)]
    fn from(variant: Dio28) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dio28 {
    type Ux = u8;
}
impl crate::IsEnum for Dio28 {}
#[doc = "Field `DIO28` reader - Enables and configures edge detection polarity for DIO28."]
pub type Dio28R = crate::FieldReader<Dio28>;
impl Dio28R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio28 {
        match self.bits {
            0 => Dio28::Disable,
            1 => Dio28::Rise,
            2 => Dio28::Fall,
            3 => Dio28::RiseFall,
            _ => unreachable!(),
        }
    }
    #[doc = "Edge detection disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Dio28::Disable
    }
    #[doc = "Detects rising edge of input event"]
    #[inline(always)]
    pub fn is_rise(&self) -> bool {
        *self == Dio28::Rise
    }
    #[doc = "Detects falling edge of input event"]
    #[inline(always)]
    pub fn is_fall(&self) -> bool {
        *self == Dio28::Fall
    }
    #[doc = "Detects both rising and falling edge of input event"]
    #[inline(always)]
    pub fn is_rise_fall(&self) -> bool {
        *self == Dio28::RiseFall
    }
}
#[doc = "Field `DIO28` writer - Enables and configures edge detection polarity for DIO28."]
pub type Dio28W<'a, REG> = crate::FieldWriter<'a, REG, 2, Dio28, crate::Safe>;
impl<'a, REG> Dio28W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Edge detection disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Dio28::Disable)
    }
    #[doc = "Detects rising edge of input event"]
    #[inline(always)]
    pub fn rise(self) -> &'a mut crate::W<REG> {
        self.variant(Dio28::Rise)
    }
    #[doc = "Detects falling edge of input event"]
    #[inline(always)]
    pub fn fall(self) -> &'a mut crate::W<REG> {
        self.variant(Dio28::Fall)
    }
    #[doc = "Detects both rising and falling edge of input event"]
    #[inline(always)]
    pub fn rise_fall(self) -> &'a mut crate::W<REG> {
        self.variant(Dio28::RiseFall)
    }
}
#[doc = "Enables and configures edge detection polarity for DIO29.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dio29 {
    #[doc = "0: Edge detection disabled"]
    Disable = 0,
    #[doc = "1: Detects rising edge of input event"]
    Rise = 1,
    #[doc = "2: Detects falling edge of input event"]
    Fall = 2,
    #[doc = "3: Detects both rising and falling edge of input event"]
    RiseFall = 3,
}
impl From<Dio29> for u8 {
    #[inline(always)]
    fn from(variant: Dio29) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dio29 {
    type Ux = u8;
}
impl crate::IsEnum for Dio29 {}
#[doc = "Field `DIO29` reader - Enables and configures edge detection polarity for DIO29."]
pub type Dio29R = crate::FieldReader<Dio29>;
impl Dio29R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio29 {
        match self.bits {
            0 => Dio29::Disable,
            1 => Dio29::Rise,
            2 => Dio29::Fall,
            3 => Dio29::RiseFall,
            _ => unreachable!(),
        }
    }
    #[doc = "Edge detection disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Dio29::Disable
    }
    #[doc = "Detects rising edge of input event"]
    #[inline(always)]
    pub fn is_rise(&self) -> bool {
        *self == Dio29::Rise
    }
    #[doc = "Detects falling edge of input event"]
    #[inline(always)]
    pub fn is_fall(&self) -> bool {
        *self == Dio29::Fall
    }
    #[doc = "Detects both rising and falling edge of input event"]
    #[inline(always)]
    pub fn is_rise_fall(&self) -> bool {
        *self == Dio29::RiseFall
    }
}
#[doc = "Field `DIO29` writer - Enables and configures edge detection polarity for DIO29."]
pub type Dio29W<'a, REG> = crate::FieldWriter<'a, REG, 2, Dio29, crate::Safe>;
impl<'a, REG> Dio29W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Edge detection disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Dio29::Disable)
    }
    #[doc = "Detects rising edge of input event"]
    #[inline(always)]
    pub fn rise(self) -> &'a mut crate::W<REG> {
        self.variant(Dio29::Rise)
    }
    #[doc = "Detects falling edge of input event"]
    #[inline(always)]
    pub fn fall(self) -> &'a mut crate::W<REG> {
        self.variant(Dio29::Fall)
    }
    #[doc = "Detects both rising and falling edge of input event"]
    #[inline(always)]
    pub fn rise_fall(self) -> &'a mut crate::W<REG> {
        self.variant(Dio29::RiseFall)
    }
}
#[doc = "Enables and configures edge detection polarity for DIO30.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dio30 {
    #[doc = "0: Edge detection disabled"]
    Disable = 0,
    #[doc = "1: Detects rising edge of input event"]
    Rise = 1,
    #[doc = "2: Detects falling edge of input event"]
    Fall = 2,
    #[doc = "3: Detects both rising and falling edge of input event"]
    RiseFall = 3,
}
impl From<Dio30> for u8 {
    #[inline(always)]
    fn from(variant: Dio30) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dio30 {
    type Ux = u8;
}
impl crate::IsEnum for Dio30 {}
#[doc = "Field `DIO30` reader - Enables and configures edge detection polarity for DIO30."]
pub type Dio30R = crate::FieldReader<Dio30>;
impl Dio30R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio30 {
        match self.bits {
            0 => Dio30::Disable,
            1 => Dio30::Rise,
            2 => Dio30::Fall,
            3 => Dio30::RiseFall,
            _ => unreachable!(),
        }
    }
    #[doc = "Edge detection disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Dio30::Disable
    }
    #[doc = "Detects rising edge of input event"]
    #[inline(always)]
    pub fn is_rise(&self) -> bool {
        *self == Dio30::Rise
    }
    #[doc = "Detects falling edge of input event"]
    #[inline(always)]
    pub fn is_fall(&self) -> bool {
        *self == Dio30::Fall
    }
    #[doc = "Detects both rising and falling edge of input event"]
    #[inline(always)]
    pub fn is_rise_fall(&self) -> bool {
        *self == Dio30::RiseFall
    }
}
#[doc = "Field `DIO30` writer - Enables and configures edge detection polarity for DIO30."]
pub type Dio30W<'a, REG> = crate::FieldWriter<'a, REG, 2, Dio30, crate::Safe>;
impl<'a, REG> Dio30W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Edge detection disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Dio30::Disable)
    }
    #[doc = "Detects rising edge of input event"]
    #[inline(always)]
    pub fn rise(self) -> &'a mut crate::W<REG> {
        self.variant(Dio30::Rise)
    }
    #[doc = "Detects falling edge of input event"]
    #[inline(always)]
    pub fn fall(self) -> &'a mut crate::W<REG> {
        self.variant(Dio30::Fall)
    }
    #[doc = "Detects both rising and falling edge of input event"]
    #[inline(always)]
    pub fn rise_fall(self) -> &'a mut crate::W<REG> {
        self.variant(Dio30::RiseFall)
    }
}
#[doc = "Enables and configures edge detection polarity for DIO31.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dio31 {
    #[doc = "0: Edge detection disabled"]
    Disable = 0,
    #[doc = "1: Detects rising edge of input event"]
    Rise = 1,
    #[doc = "2: Detects falling edge of input event"]
    Fall = 2,
    #[doc = "3: Detects both rising and falling edge of input event"]
    RiseFall = 3,
}
impl From<Dio31> for u8 {
    #[inline(always)]
    fn from(variant: Dio31) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dio31 {
    type Ux = u8;
}
impl crate::IsEnum for Dio31 {}
#[doc = "Field `DIO31` reader - Enables and configures edge detection polarity for DIO31."]
pub type Dio31R = crate::FieldReader<Dio31>;
impl Dio31R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio31 {
        match self.bits {
            0 => Dio31::Disable,
            1 => Dio31::Rise,
            2 => Dio31::Fall,
            3 => Dio31::RiseFall,
            _ => unreachable!(),
        }
    }
    #[doc = "Edge detection disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Dio31::Disable
    }
    #[doc = "Detects rising edge of input event"]
    #[inline(always)]
    pub fn is_rise(&self) -> bool {
        *self == Dio31::Rise
    }
    #[doc = "Detects falling edge of input event"]
    #[inline(always)]
    pub fn is_fall(&self) -> bool {
        *self == Dio31::Fall
    }
    #[doc = "Detects both rising and falling edge of input event"]
    #[inline(always)]
    pub fn is_rise_fall(&self) -> bool {
        *self == Dio31::RiseFall
    }
}
#[doc = "Field `DIO31` writer - Enables and configures edge detection polarity for DIO31."]
pub type Dio31W<'a, REG> = crate::FieldWriter<'a, REG, 2, Dio31, crate::Safe>;
impl<'a, REG> Dio31W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Edge detection disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Dio31::Disable)
    }
    #[doc = "Detects rising edge of input event"]
    #[inline(always)]
    pub fn rise(self) -> &'a mut crate::W<REG> {
        self.variant(Dio31::Rise)
    }
    #[doc = "Detects falling edge of input event"]
    #[inline(always)]
    pub fn fall(self) -> &'a mut crate::W<REG> {
        self.variant(Dio31::Fall)
    }
    #[doc = "Detects both rising and falling edge of input event"]
    #[inline(always)]
    pub fn rise_fall(self) -> &'a mut crate::W<REG> {
        self.variant(Dio31::RiseFall)
    }
}
impl R {
    #[doc = "Bits 0:1 - Enables and configures edge detection polarity for DIO16."]
    #[inline(always)]
    pub fn dio16(&self) -> Dio16R {
        Dio16R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Enables and configures edge detection polarity for DIO17."]
    #[inline(always)]
    pub fn dio17(&self) -> Dio17R {
        Dio17R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Enables and configures edge detection polarity for DIO18."]
    #[inline(always)]
    pub fn dio18(&self) -> Dio18R {
        Dio18R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Enables and configures edge detection polarity for DIO19."]
    #[inline(always)]
    pub fn dio19(&self) -> Dio19R {
        Dio19R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Enables and configures edge detection polarity for DIO20."]
    #[inline(always)]
    pub fn dio20(&self) -> Dio20R {
        Dio20R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Enables and configures edge detection polarity for DIO21."]
    #[inline(always)]
    pub fn dio21(&self) -> Dio21R {
        Dio21R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Enables and configures edge detection polarity for DIO22."]
    #[inline(always)]
    pub fn dio22(&self) -> Dio22R {
        Dio22R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Enables and configures edge detection polarity for DIO23."]
    #[inline(always)]
    pub fn dio23(&self) -> Dio23R {
        Dio23R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Enables and configures edge detection polarity for DIO24."]
    #[inline(always)]
    pub fn dio24(&self) -> Dio24R {
        Dio24R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Enables and configures edge detection polarity for DIO25."]
    #[inline(always)]
    pub fn dio25(&self) -> Dio25R {
        Dio25R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Enables and configures edge detection polarity for DIO26."]
    #[inline(always)]
    pub fn dio26(&self) -> Dio26R {
        Dio26R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Enables and configures edge detection polarity for DIO27."]
    #[inline(always)]
    pub fn dio27(&self) -> Dio27R {
        Dio27R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Enables and configures edge detection polarity for DIO28."]
    #[inline(always)]
    pub fn dio28(&self) -> Dio28R {
        Dio28R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - Enables and configures edge detection polarity for DIO29."]
    #[inline(always)]
    pub fn dio29(&self) -> Dio29R {
        Dio29R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Enables and configures edge detection polarity for DIO30."]
    #[inline(always)]
    pub fn dio30(&self) -> Dio30R {
        Dio30R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Enables and configures edge detection polarity for DIO31."]
    #[inline(always)]
    pub fn dio31(&self) -> Dio31R {
        Dio31R::new(((self.bits >> 30) & 3) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("POLARITY31_16")
            .field("dio16", &self.dio16())
            .field("dio17", &self.dio17())
            .field("dio18", &self.dio18())
            .field("dio19", &self.dio19())
            .field("dio20", &self.dio20())
            .field("dio21", &self.dio21())
            .field("dio22", &self.dio22())
            .field("dio23", &self.dio23())
            .field("dio24", &self.dio24())
            .field("dio25", &self.dio25())
            .field("dio26", &self.dio26())
            .field("dio27", &self.dio27())
            .field("dio28", &self.dio28())
            .field("dio29", &self.dio29())
            .field("dio30", &self.dio30())
            .field("dio31", &self.dio31())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - Enables and configures edge detection polarity for DIO16."]
    #[inline(always)]
    pub fn dio16(&mut self) -> Dio16W<'_, Polarity31_16Spec> {
        Dio16W::new(self, 0)
    }
    #[doc = "Bits 2:3 - Enables and configures edge detection polarity for DIO17."]
    #[inline(always)]
    pub fn dio17(&mut self) -> Dio17W<'_, Polarity31_16Spec> {
        Dio17W::new(self, 2)
    }
    #[doc = "Bits 4:5 - Enables and configures edge detection polarity for DIO18."]
    #[inline(always)]
    pub fn dio18(&mut self) -> Dio18W<'_, Polarity31_16Spec> {
        Dio18W::new(self, 4)
    }
    #[doc = "Bits 6:7 - Enables and configures edge detection polarity for DIO19."]
    #[inline(always)]
    pub fn dio19(&mut self) -> Dio19W<'_, Polarity31_16Spec> {
        Dio19W::new(self, 6)
    }
    #[doc = "Bits 8:9 - Enables and configures edge detection polarity for DIO20."]
    #[inline(always)]
    pub fn dio20(&mut self) -> Dio20W<'_, Polarity31_16Spec> {
        Dio20W::new(self, 8)
    }
    #[doc = "Bits 10:11 - Enables and configures edge detection polarity for DIO21."]
    #[inline(always)]
    pub fn dio21(&mut self) -> Dio21W<'_, Polarity31_16Spec> {
        Dio21W::new(self, 10)
    }
    #[doc = "Bits 12:13 - Enables and configures edge detection polarity for DIO22."]
    #[inline(always)]
    pub fn dio22(&mut self) -> Dio22W<'_, Polarity31_16Spec> {
        Dio22W::new(self, 12)
    }
    #[doc = "Bits 14:15 - Enables and configures edge detection polarity for DIO23."]
    #[inline(always)]
    pub fn dio23(&mut self) -> Dio23W<'_, Polarity31_16Spec> {
        Dio23W::new(self, 14)
    }
    #[doc = "Bits 16:17 - Enables and configures edge detection polarity for DIO24."]
    #[inline(always)]
    pub fn dio24(&mut self) -> Dio24W<'_, Polarity31_16Spec> {
        Dio24W::new(self, 16)
    }
    #[doc = "Bits 18:19 - Enables and configures edge detection polarity for DIO25."]
    #[inline(always)]
    pub fn dio25(&mut self) -> Dio25W<'_, Polarity31_16Spec> {
        Dio25W::new(self, 18)
    }
    #[doc = "Bits 20:21 - Enables and configures edge detection polarity for DIO26."]
    #[inline(always)]
    pub fn dio26(&mut self) -> Dio26W<'_, Polarity31_16Spec> {
        Dio26W::new(self, 20)
    }
    #[doc = "Bits 22:23 - Enables and configures edge detection polarity for DIO27."]
    #[inline(always)]
    pub fn dio27(&mut self) -> Dio27W<'_, Polarity31_16Spec> {
        Dio27W::new(self, 22)
    }
    #[doc = "Bits 24:25 - Enables and configures edge detection polarity for DIO28."]
    #[inline(always)]
    pub fn dio28(&mut self) -> Dio28W<'_, Polarity31_16Spec> {
        Dio28W::new(self, 24)
    }
    #[doc = "Bits 26:27 - Enables and configures edge detection polarity for DIO29."]
    #[inline(always)]
    pub fn dio29(&mut self) -> Dio29W<'_, Polarity31_16Spec> {
        Dio29W::new(self, 26)
    }
    #[doc = "Bits 28:29 - Enables and configures edge detection polarity for DIO30."]
    #[inline(always)]
    pub fn dio30(&mut self) -> Dio30W<'_, Polarity31_16Spec> {
        Dio30W::new(self, 28)
    }
    #[doc = "Bits 30:31 - Enables and configures edge detection polarity for DIO31."]
    #[inline(always)]
    pub fn dio31(&mut self) -> Dio31W<'_, Polarity31_16Spec> {
        Dio31W::new(self, 30)
    }
}
#[doc = "Polarity 31 to 16\n\nYou can [`read`](crate::Reg::read) this register and get [`polarity31_16::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`polarity31_16::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Polarity31_16Spec;
impl crate::RegisterSpec for Polarity31_16Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`polarity31_16::R`](R) reader structure"]
impl crate::Readable for Polarity31_16Spec {}
#[doc = "`write(|w| ..)` method takes [`polarity31_16::W`](W) writer structure"]
impl crate::Writable for Polarity31_16Spec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets POLARITY31_16 to value 0"]
impl crate::Resettable for Polarity31_16Spec {}
