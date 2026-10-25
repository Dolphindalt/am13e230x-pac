#[doc = "Register `POLARITY15_0` reader"]
pub type R = crate::R<Polarity15_0Spec>;
#[doc = "Register `POLARITY15_0` writer"]
pub type W = crate::W<Polarity15_0Spec>;
#[doc = "Enables and configures edge detection polarity for DIO0.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dio0 {
    #[doc = "0: Edge detection disabled"]
    Disable = 0,
    #[doc = "1: Detects rising edge of input event"]
    Rise = 1,
    #[doc = "2: Detects falling edge of input event"]
    Fall = 2,
    #[doc = "3: Detects both rising and falling edge of input event"]
    RiseFall = 3,
}
impl From<Dio0> for u8 {
    #[inline(always)]
    fn from(variant: Dio0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dio0 {
    type Ux = u8;
}
impl crate::IsEnum for Dio0 {}
#[doc = "Field `DIO0` reader - Enables and configures edge detection polarity for DIO0."]
pub type Dio0R = crate::FieldReader<Dio0>;
impl Dio0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio0 {
        match self.bits {
            0 => Dio0::Disable,
            1 => Dio0::Rise,
            2 => Dio0::Fall,
            3 => Dio0::RiseFall,
            _ => unreachable!(),
        }
    }
    #[doc = "Edge detection disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Dio0::Disable
    }
    #[doc = "Detects rising edge of input event"]
    #[inline(always)]
    pub fn is_rise(&self) -> bool {
        *self == Dio0::Rise
    }
    #[doc = "Detects falling edge of input event"]
    #[inline(always)]
    pub fn is_fall(&self) -> bool {
        *self == Dio0::Fall
    }
    #[doc = "Detects both rising and falling edge of input event"]
    #[inline(always)]
    pub fn is_rise_fall(&self) -> bool {
        *self == Dio0::RiseFall
    }
}
#[doc = "Field `DIO0` writer - Enables and configures edge detection polarity for DIO0."]
pub type Dio0W<'a, REG> = crate::FieldWriter<'a, REG, 2, Dio0, crate::Safe>;
impl<'a, REG> Dio0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Edge detection disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Dio0::Disable)
    }
    #[doc = "Detects rising edge of input event"]
    #[inline(always)]
    pub fn rise(self) -> &'a mut crate::W<REG> {
        self.variant(Dio0::Rise)
    }
    #[doc = "Detects falling edge of input event"]
    #[inline(always)]
    pub fn fall(self) -> &'a mut crate::W<REG> {
        self.variant(Dio0::Fall)
    }
    #[doc = "Detects both rising and falling edge of input event"]
    #[inline(always)]
    pub fn rise_fall(self) -> &'a mut crate::W<REG> {
        self.variant(Dio0::RiseFall)
    }
}
#[doc = "Enables and configures edge detection polarity for DIO1.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dio1 {
    #[doc = "0: Edge detection disabled"]
    Disable = 0,
    #[doc = "1: Detects rising edge of input event"]
    Rise = 1,
    #[doc = "2: Detects falling edge of input event"]
    Fall = 2,
    #[doc = "3: Detects both rising and falling edge of input event"]
    RiseFall = 3,
}
impl From<Dio1> for u8 {
    #[inline(always)]
    fn from(variant: Dio1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dio1 {
    type Ux = u8;
}
impl crate::IsEnum for Dio1 {}
#[doc = "Field `DIO1` reader - Enables and configures edge detection polarity for DIO1."]
pub type Dio1R = crate::FieldReader<Dio1>;
impl Dio1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio1 {
        match self.bits {
            0 => Dio1::Disable,
            1 => Dio1::Rise,
            2 => Dio1::Fall,
            3 => Dio1::RiseFall,
            _ => unreachable!(),
        }
    }
    #[doc = "Edge detection disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Dio1::Disable
    }
    #[doc = "Detects rising edge of input event"]
    #[inline(always)]
    pub fn is_rise(&self) -> bool {
        *self == Dio1::Rise
    }
    #[doc = "Detects falling edge of input event"]
    #[inline(always)]
    pub fn is_fall(&self) -> bool {
        *self == Dio1::Fall
    }
    #[doc = "Detects both rising and falling edge of input event"]
    #[inline(always)]
    pub fn is_rise_fall(&self) -> bool {
        *self == Dio1::RiseFall
    }
}
#[doc = "Field `DIO1` writer - Enables and configures edge detection polarity for DIO1."]
pub type Dio1W<'a, REG> = crate::FieldWriter<'a, REG, 2, Dio1, crate::Safe>;
impl<'a, REG> Dio1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Edge detection disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Dio1::Disable)
    }
    #[doc = "Detects rising edge of input event"]
    #[inline(always)]
    pub fn rise(self) -> &'a mut crate::W<REG> {
        self.variant(Dio1::Rise)
    }
    #[doc = "Detects falling edge of input event"]
    #[inline(always)]
    pub fn fall(self) -> &'a mut crate::W<REG> {
        self.variant(Dio1::Fall)
    }
    #[doc = "Detects both rising and falling edge of input event"]
    #[inline(always)]
    pub fn rise_fall(self) -> &'a mut crate::W<REG> {
        self.variant(Dio1::RiseFall)
    }
}
#[doc = "Enables and configures edge detection polarity for DIO2.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dio2 {
    #[doc = "0: Edge detection disabled"]
    Disable = 0,
    #[doc = "1: Detects rising edge of input event"]
    Rise = 1,
    #[doc = "2: Detects falling edge of input event"]
    Fall = 2,
    #[doc = "3: Detects both rising and falling edge of input event"]
    RiseFall = 3,
}
impl From<Dio2> for u8 {
    #[inline(always)]
    fn from(variant: Dio2) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dio2 {
    type Ux = u8;
}
impl crate::IsEnum for Dio2 {}
#[doc = "Field `DIO2` reader - Enables and configures edge detection polarity for DIO2."]
pub type Dio2R = crate::FieldReader<Dio2>;
impl Dio2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio2 {
        match self.bits {
            0 => Dio2::Disable,
            1 => Dio2::Rise,
            2 => Dio2::Fall,
            3 => Dio2::RiseFall,
            _ => unreachable!(),
        }
    }
    #[doc = "Edge detection disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Dio2::Disable
    }
    #[doc = "Detects rising edge of input event"]
    #[inline(always)]
    pub fn is_rise(&self) -> bool {
        *self == Dio2::Rise
    }
    #[doc = "Detects falling edge of input event"]
    #[inline(always)]
    pub fn is_fall(&self) -> bool {
        *self == Dio2::Fall
    }
    #[doc = "Detects both rising and falling edge of input event"]
    #[inline(always)]
    pub fn is_rise_fall(&self) -> bool {
        *self == Dio2::RiseFall
    }
}
#[doc = "Field `DIO2` writer - Enables and configures edge detection polarity for DIO2."]
pub type Dio2W<'a, REG> = crate::FieldWriter<'a, REG, 2, Dio2, crate::Safe>;
impl<'a, REG> Dio2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Edge detection disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Dio2::Disable)
    }
    #[doc = "Detects rising edge of input event"]
    #[inline(always)]
    pub fn rise(self) -> &'a mut crate::W<REG> {
        self.variant(Dio2::Rise)
    }
    #[doc = "Detects falling edge of input event"]
    #[inline(always)]
    pub fn fall(self) -> &'a mut crate::W<REG> {
        self.variant(Dio2::Fall)
    }
    #[doc = "Detects both rising and falling edge of input event"]
    #[inline(always)]
    pub fn rise_fall(self) -> &'a mut crate::W<REG> {
        self.variant(Dio2::RiseFall)
    }
}
#[doc = "Enables and configures edge detection polarity for DIO3.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dio3 {
    #[doc = "0: Edge detection disabled"]
    Disable = 0,
    #[doc = "1: Detects rising edge of input event"]
    Rise = 1,
    #[doc = "2: Detects falling edge of input event"]
    Fall = 2,
    #[doc = "3: Detects both rising and falling edge of input event"]
    RiseFall = 3,
}
impl From<Dio3> for u8 {
    #[inline(always)]
    fn from(variant: Dio3) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dio3 {
    type Ux = u8;
}
impl crate::IsEnum for Dio3 {}
#[doc = "Field `DIO3` reader - Enables and configures edge detection polarity for DIO3."]
pub type Dio3R = crate::FieldReader<Dio3>;
impl Dio3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio3 {
        match self.bits {
            0 => Dio3::Disable,
            1 => Dio3::Rise,
            2 => Dio3::Fall,
            3 => Dio3::RiseFall,
            _ => unreachable!(),
        }
    }
    #[doc = "Edge detection disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Dio3::Disable
    }
    #[doc = "Detects rising edge of input event"]
    #[inline(always)]
    pub fn is_rise(&self) -> bool {
        *self == Dio3::Rise
    }
    #[doc = "Detects falling edge of input event"]
    #[inline(always)]
    pub fn is_fall(&self) -> bool {
        *self == Dio3::Fall
    }
    #[doc = "Detects both rising and falling edge of input event"]
    #[inline(always)]
    pub fn is_rise_fall(&self) -> bool {
        *self == Dio3::RiseFall
    }
}
#[doc = "Field `DIO3` writer - Enables and configures edge detection polarity for DIO3."]
pub type Dio3W<'a, REG> = crate::FieldWriter<'a, REG, 2, Dio3, crate::Safe>;
impl<'a, REG> Dio3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Edge detection disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Dio3::Disable)
    }
    #[doc = "Detects rising edge of input event"]
    #[inline(always)]
    pub fn rise(self) -> &'a mut crate::W<REG> {
        self.variant(Dio3::Rise)
    }
    #[doc = "Detects falling edge of input event"]
    #[inline(always)]
    pub fn fall(self) -> &'a mut crate::W<REG> {
        self.variant(Dio3::Fall)
    }
    #[doc = "Detects both rising and falling edge of input event"]
    #[inline(always)]
    pub fn rise_fall(self) -> &'a mut crate::W<REG> {
        self.variant(Dio3::RiseFall)
    }
}
#[doc = "Enables and configures edge detection polarity for DIO4.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dio4 {
    #[doc = "0: Edge detection disabled"]
    Disable = 0,
    #[doc = "1: Detects rising edge of input event"]
    Rise = 1,
    #[doc = "2: Detects falling edge of input event"]
    Fall = 2,
    #[doc = "3: Detects both rising and falling edge of input event"]
    RiseFall = 3,
}
impl From<Dio4> for u8 {
    #[inline(always)]
    fn from(variant: Dio4) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dio4 {
    type Ux = u8;
}
impl crate::IsEnum for Dio4 {}
#[doc = "Field `DIO4` reader - Enables and configures edge detection polarity for DIO4."]
pub type Dio4R = crate::FieldReader<Dio4>;
impl Dio4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio4 {
        match self.bits {
            0 => Dio4::Disable,
            1 => Dio4::Rise,
            2 => Dio4::Fall,
            3 => Dio4::RiseFall,
            _ => unreachable!(),
        }
    }
    #[doc = "Edge detection disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Dio4::Disable
    }
    #[doc = "Detects rising edge of input event"]
    #[inline(always)]
    pub fn is_rise(&self) -> bool {
        *self == Dio4::Rise
    }
    #[doc = "Detects falling edge of input event"]
    #[inline(always)]
    pub fn is_fall(&self) -> bool {
        *self == Dio4::Fall
    }
    #[doc = "Detects both rising and falling edge of input event"]
    #[inline(always)]
    pub fn is_rise_fall(&self) -> bool {
        *self == Dio4::RiseFall
    }
}
#[doc = "Field `DIO4` writer - Enables and configures edge detection polarity for DIO4."]
pub type Dio4W<'a, REG> = crate::FieldWriter<'a, REG, 2, Dio4, crate::Safe>;
impl<'a, REG> Dio4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Edge detection disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Dio4::Disable)
    }
    #[doc = "Detects rising edge of input event"]
    #[inline(always)]
    pub fn rise(self) -> &'a mut crate::W<REG> {
        self.variant(Dio4::Rise)
    }
    #[doc = "Detects falling edge of input event"]
    #[inline(always)]
    pub fn fall(self) -> &'a mut crate::W<REG> {
        self.variant(Dio4::Fall)
    }
    #[doc = "Detects both rising and falling edge of input event"]
    #[inline(always)]
    pub fn rise_fall(self) -> &'a mut crate::W<REG> {
        self.variant(Dio4::RiseFall)
    }
}
#[doc = "Enables and configures edge detection polarity for DIO5.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dio5 {
    #[doc = "0: Edge detection disabled"]
    Disable = 0,
    #[doc = "1: Detects rising edge of input event"]
    Rise = 1,
    #[doc = "2: Detects falling edge of input event"]
    Fall = 2,
    #[doc = "3: Detects both rising and falling edge of input event"]
    RiseFall = 3,
}
impl From<Dio5> for u8 {
    #[inline(always)]
    fn from(variant: Dio5) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dio5 {
    type Ux = u8;
}
impl crate::IsEnum for Dio5 {}
#[doc = "Field `DIO5` reader - Enables and configures edge detection polarity for DIO5."]
pub type Dio5R = crate::FieldReader<Dio5>;
impl Dio5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio5 {
        match self.bits {
            0 => Dio5::Disable,
            1 => Dio5::Rise,
            2 => Dio5::Fall,
            3 => Dio5::RiseFall,
            _ => unreachable!(),
        }
    }
    #[doc = "Edge detection disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Dio5::Disable
    }
    #[doc = "Detects rising edge of input event"]
    #[inline(always)]
    pub fn is_rise(&self) -> bool {
        *self == Dio5::Rise
    }
    #[doc = "Detects falling edge of input event"]
    #[inline(always)]
    pub fn is_fall(&self) -> bool {
        *self == Dio5::Fall
    }
    #[doc = "Detects both rising and falling edge of input event"]
    #[inline(always)]
    pub fn is_rise_fall(&self) -> bool {
        *self == Dio5::RiseFall
    }
}
#[doc = "Field `DIO5` writer - Enables and configures edge detection polarity for DIO5."]
pub type Dio5W<'a, REG> = crate::FieldWriter<'a, REG, 2, Dio5, crate::Safe>;
impl<'a, REG> Dio5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Edge detection disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Dio5::Disable)
    }
    #[doc = "Detects rising edge of input event"]
    #[inline(always)]
    pub fn rise(self) -> &'a mut crate::W<REG> {
        self.variant(Dio5::Rise)
    }
    #[doc = "Detects falling edge of input event"]
    #[inline(always)]
    pub fn fall(self) -> &'a mut crate::W<REG> {
        self.variant(Dio5::Fall)
    }
    #[doc = "Detects both rising and falling edge of input event"]
    #[inline(always)]
    pub fn rise_fall(self) -> &'a mut crate::W<REG> {
        self.variant(Dio5::RiseFall)
    }
}
#[doc = "Enables and configures edge detection polarity for DIO6.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dio6 {
    #[doc = "0: Edge detection disabled"]
    Disable = 0,
    #[doc = "1: Detects rising edge of input event"]
    Rise = 1,
    #[doc = "2: Detects falling edge of input event"]
    Fall = 2,
    #[doc = "3: Detects both rising and falling edge of input event"]
    RiseFall = 3,
}
impl From<Dio6> for u8 {
    #[inline(always)]
    fn from(variant: Dio6) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dio6 {
    type Ux = u8;
}
impl crate::IsEnum for Dio6 {}
#[doc = "Field `DIO6` reader - Enables and configures edge detection polarity for DIO6."]
pub type Dio6R = crate::FieldReader<Dio6>;
impl Dio6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio6 {
        match self.bits {
            0 => Dio6::Disable,
            1 => Dio6::Rise,
            2 => Dio6::Fall,
            3 => Dio6::RiseFall,
            _ => unreachable!(),
        }
    }
    #[doc = "Edge detection disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Dio6::Disable
    }
    #[doc = "Detects rising edge of input event"]
    #[inline(always)]
    pub fn is_rise(&self) -> bool {
        *self == Dio6::Rise
    }
    #[doc = "Detects falling edge of input event"]
    #[inline(always)]
    pub fn is_fall(&self) -> bool {
        *self == Dio6::Fall
    }
    #[doc = "Detects both rising and falling edge of input event"]
    #[inline(always)]
    pub fn is_rise_fall(&self) -> bool {
        *self == Dio6::RiseFall
    }
}
#[doc = "Field `DIO6` writer - Enables and configures edge detection polarity for DIO6."]
pub type Dio6W<'a, REG> = crate::FieldWriter<'a, REG, 2, Dio6, crate::Safe>;
impl<'a, REG> Dio6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Edge detection disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Dio6::Disable)
    }
    #[doc = "Detects rising edge of input event"]
    #[inline(always)]
    pub fn rise(self) -> &'a mut crate::W<REG> {
        self.variant(Dio6::Rise)
    }
    #[doc = "Detects falling edge of input event"]
    #[inline(always)]
    pub fn fall(self) -> &'a mut crate::W<REG> {
        self.variant(Dio6::Fall)
    }
    #[doc = "Detects both rising and falling edge of input event"]
    #[inline(always)]
    pub fn rise_fall(self) -> &'a mut crate::W<REG> {
        self.variant(Dio6::RiseFall)
    }
}
#[doc = "Enables and configures edge detection polarity for DIO7.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dio7 {
    #[doc = "0: Edge detection disabled"]
    Disable = 0,
    #[doc = "1: Detects rising edge of input event"]
    Rise = 1,
    #[doc = "2: Detects falling edge of input event"]
    Fall = 2,
    #[doc = "3: Detects both rising and falling edge of input event"]
    RiseFall = 3,
}
impl From<Dio7> for u8 {
    #[inline(always)]
    fn from(variant: Dio7) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dio7 {
    type Ux = u8;
}
impl crate::IsEnum for Dio7 {}
#[doc = "Field `DIO7` reader - Enables and configures edge detection polarity for DIO7."]
pub type Dio7R = crate::FieldReader<Dio7>;
impl Dio7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio7 {
        match self.bits {
            0 => Dio7::Disable,
            1 => Dio7::Rise,
            2 => Dio7::Fall,
            3 => Dio7::RiseFall,
            _ => unreachable!(),
        }
    }
    #[doc = "Edge detection disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Dio7::Disable
    }
    #[doc = "Detects rising edge of input event"]
    #[inline(always)]
    pub fn is_rise(&self) -> bool {
        *self == Dio7::Rise
    }
    #[doc = "Detects falling edge of input event"]
    #[inline(always)]
    pub fn is_fall(&self) -> bool {
        *self == Dio7::Fall
    }
    #[doc = "Detects both rising and falling edge of input event"]
    #[inline(always)]
    pub fn is_rise_fall(&self) -> bool {
        *self == Dio7::RiseFall
    }
}
#[doc = "Field `DIO7` writer - Enables and configures edge detection polarity for DIO7."]
pub type Dio7W<'a, REG> = crate::FieldWriter<'a, REG, 2, Dio7, crate::Safe>;
impl<'a, REG> Dio7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Edge detection disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Dio7::Disable)
    }
    #[doc = "Detects rising edge of input event"]
    #[inline(always)]
    pub fn rise(self) -> &'a mut crate::W<REG> {
        self.variant(Dio7::Rise)
    }
    #[doc = "Detects falling edge of input event"]
    #[inline(always)]
    pub fn fall(self) -> &'a mut crate::W<REG> {
        self.variant(Dio7::Fall)
    }
    #[doc = "Detects both rising and falling edge of input event"]
    #[inline(always)]
    pub fn rise_fall(self) -> &'a mut crate::W<REG> {
        self.variant(Dio7::RiseFall)
    }
}
#[doc = "Enables and configures edge detection polarity for DIO8.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dio8 {
    #[doc = "0: Edge detection disabled"]
    Disable = 0,
    #[doc = "1: Detects rising edge of input event"]
    Rise = 1,
    #[doc = "2: Detects falling edge of input event"]
    Fall = 2,
    #[doc = "3: Detects both rising and falling edge of input event"]
    RiseFall = 3,
}
impl From<Dio8> for u8 {
    #[inline(always)]
    fn from(variant: Dio8) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dio8 {
    type Ux = u8;
}
impl crate::IsEnum for Dio8 {}
#[doc = "Field `DIO8` reader - Enables and configures edge detection polarity for DIO8."]
pub type Dio8R = crate::FieldReader<Dio8>;
impl Dio8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio8 {
        match self.bits {
            0 => Dio8::Disable,
            1 => Dio8::Rise,
            2 => Dio8::Fall,
            3 => Dio8::RiseFall,
            _ => unreachable!(),
        }
    }
    #[doc = "Edge detection disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Dio8::Disable
    }
    #[doc = "Detects rising edge of input event"]
    #[inline(always)]
    pub fn is_rise(&self) -> bool {
        *self == Dio8::Rise
    }
    #[doc = "Detects falling edge of input event"]
    #[inline(always)]
    pub fn is_fall(&self) -> bool {
        *self == Dio8::Fall
    }
    #[doc = "Detects both rising and falling edge of input event"]
    #[inline(always)]
    pub fn is_rise_fall(&self) -> bool {
        *self == Dio8::RiseFall
    }
}
#[doc = "Field `DIO8` writer - Enables and configures edge detection polarity for DIO8."]
pub type Dio8W<'a, REG> = crate::FieldWriter<'a, REG, 2, Dio8, crate::Safe>;
impl<'a, REG> Dio8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Edge detection disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Dio8::Disable)
    }
    #[doc = "Detects rising edge of input event"]
    #[inline(always)]
    pub fn rise(self) -> &'a mut crate::W<REG> {
        self.variant(Dio8::Rise)
    }
    #[doc = "Detects falling edge of input event"]
    #[inline(always)]
    pub fn fall(self) -> &'a mut crate::W<REG> {
        self.variant(Dio8::Fall)
    }
    #[doc = "Detects both rising and falling edge of input event"]
    #[inline(always)]
    pub fn rise_fall(self) -> &'a mut crate::W<REG> {
        self.variant(Dio8::RiseFall)
    }
}
#[doc = "Enables and configures edge detection polarity for DIO9.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dio9 {
    #[doc = "0: Edge detection disabled"]
    Disable = 0,
    #[doc = "1: Detects rising edge of input event"]
    Rise = 1,
    #[doc = "2: Detects falling edge of input event"]
    Fall = 2,
    #[doc = "3: Detects both rising and falling edge of input event"]
    RiseFall = 3,
}
impl From<Dio9> for u8 {
    #[inline(always)]
    fn from(variant: Dio9) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dio9 {
    type Ux = u8;
}
impl crate::IsEnum for Dio9 {}
#[doc = "Field `DIO9` reader - Enables and configures edge detection polarity for DIO9."]
pub type Dio9R = crate::FieldReader<Dio9>;
impl Dio9R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio9 {
        match self.bits {
            0 => Dio9::Disable,
            1 => Dio9::Rise,
            2 => Dio9::Fall,
            3 => Dio9::RiseFall,
            _ => unreachable!(),
        }
    }
    #[doc = "Edge detection disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Dio9::Disable
    }
    #[doc = "Detects rising edge of input event"]
    #[inline(always)]
    pub fn is_rise(&self) -> bool {
        *self == Dio9::Rise
    }
    #[doc = "Detects falling edge of input event"]
    #[inline(always)]
    pub fn is_fall(&self) -> bool {
        *self == Dio9::Fall
    }
    #[doc = "Detects both rising and falling edge of input event"]
    #[inline(always)]
    pub fn is_rise_fall(&self) -> bool {
        *self == Dio9::RiseFall
    }
}
#[doc = "Field `DIO9` writer - Enables and configures edge detection polarity for DIO9."]
pub type Dio9W<'a, REG> = crate::FieldWriter<'a, REG, 2, Dio9, crate::Safe>;
impl<'a, REG> Dio9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Edge detection disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Dio9::Disable)
    }
    #[doc = "Detects rising edge of input event"]
    #[inline(always)]
    pub fn rise(self) -> &'a mut crate::W<REG> {
        self.variant(Dio9::Rise)
    }
    #[doc = "Detects falling edge of input event"]
    #[inline(always)]
    pub fn fall(self) -> &'a mut crate::W<REG> {
        self.variant(Dio9::Fall)
    }
    #[doc = "Detects both rising and falling edge of input event"]
    #[inline(always)]
    pub fn rise_fall(self) -> &'a mut crate::W<REG> {
        self.variant(Dio9::RiseFall)
    }
}
#[doc = "Enables and configures edge detection polarity for DIO10.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dio10 {
    #[doc = "0: Edge detection disabled"]
    Disable = 0,
    #[doc = "1: Detects rising edge of input event"]
    Rise = 1,
    #[doc = "2: Detects falling edge of input event"]
    Fall = 2,
    #[doc = "3: Detects both rising and falling edge of input event"]
    RiseFall = 3,
}
impl From<Dio10> for u8 {
    #[inline(always)]
    fn from(variant: Dio10) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dio10 {
    type Ux = u8;
}
impl crate::IsEnum for Dio10 {}
#[doc = "Field `DIO10` reader - Enables and configures edge detection polarity for DIO10."]
pub type Dio10R = crate::FieldReader<Dio10>;
impl Dio10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio10 {
        match self.bits {
            0 => Dio10::Disable,
            1 => Dio10::Rise,
            2 => Dio10::Fall,
            3 => Dio10::RiseFall,
            _ => unreachable!(),
        }
    }
    #[doc = "Edge detection disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Dio10::Disable
    }
    #[doc = "Detects rising edge of input event"]
    #[inline(always)]
    pub fn is_rise(&self) -> bool {
        *self == Dio10::Rise
    }
    #[doc = "Detects falling edge of input event"]
    #[inline(always)]
    pub fn is_fall(&self) -> bool {
        *self == Dio10::Fall
    }
    #[doc = "Detects both rising and falling edge of input event"]
    #[inline(always)]
    pub fn is_rise_fall(&self) -> bool {
        *self == Dio10::RiseFall
    }
}
#[doc = "Field `DIO10` writer - Enables and configures edge detection polarity for DIO10."]
pub type Dio10W<'a, REG> = crate::FieldWriter<'a, REG, 2, Dio10, crate::Safe>;
impl<'a, REG> Dio10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Edge detection disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Dio10::Disable)
    }
    #[doc = "Detects rising edge of input event"]
    #[inline(always)]
    pub fn rise(self) -> &'a mut crate::W<REG> {
        self.variant(Dio10::Rise)
    }
    #[doc = "Detects falling edge of input event"]
    #[inline(always)]
    pub fn fall(self) -> &'a mut crate::W<REG> {
        self.variant(Dio10::Fall)
    }
    #[doc = "Detects both rising and falling edge of input event"]
    #[inline(always)]
    pub fn rise_fall(self) -> &'a mut crate::W<REG> {
        self.variant(Dio10::RiseFall)
    }
}
#[doc = "Enables and configures edge detection polarity for DIO11.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dio11 {
    #[doc = "0: Edge detection disabled"]
    Disable = 0,
    #[doc = "1: Detects rising edge of input event"]
    Rise = 1,
    #[doc = "2: Detects falling edge of input event"]
    Fall = 2,
    #[doc = "3: Detects both rising and falling edge of input event"]
    RiseFall = 3,
}
impl From<Dio11> for u8 {
    #[inline(always)]
    fn from(variant: Dio11) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dio11 {
    type Ux = u8;
}
impl crate::IsEnum for Dio11 {}
#[doc = "Field `DIO11` reader - Enables and configures edge detection polarity for DIO11."]
pub type Dio11R = crate::FieldReader<Dio11>;
impl Dio11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio11 {
        match self.bits {
            0 => Dio11::Disable,
            1 => Dio11::Rise,
            2 => Dio11::Fall,
            3 => Dio11::RiseFall,
            _ => unreachable!(),
        }
    }
    #[doc = "Edge detection disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Dio11::Disable
    }
    #[doc = "Detects rising edge of input event"]
    #[inline(always)]
    pub fn is_rise(&self) -> bool {
        *self == Dio11::Rise
    }
    #[doc = "Detects falling edge of input event"]
    #[inline(always)]
    pub fn is_fall(&self) -> bool {
        *self == Dio11::Fall
    }
    #[doc = "Detects both rising and falling edge of input event"]
    #[inline(always)]
    pub fn is_rise_fall(&self) -> bool {
        *self == Dio11::RiseFall
    }
}
#[doc = "Field `DIO11` writer - Enables and configures edge detection polarity for DIO11."]
pub type Dio11W<'a, REG> = crate::FieldWriter<'a, REG, 2, Dio11, crate::Safe>;
impl<'a, REG> Dio11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Edge detection disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Dio11::Disable)
    }
    #[doc = "Detects rising edge of input event"]
    #[inline(always)]
    pub fn rise(self) -> &'a mut crate::W<REG> {
        self.variant(Dio11::Rise)
    }
    #[doc = "Detects falling edge of input event"]
    #[inline(always)]
    pub fn fall(self) -> &'a mut crate::W<REG> {
        self.variant(Dio11::Fall)
    }
    #[doc = "Detects both rising and falling edge of input event"]
    #[inline(always)]
    pub fn rise_fall(self) -> &'a mut crate::W<REG> {
        self.variant(Dio11::RiseFall)
    }
}
#[doc = "Enables and configures edge detection polarity for DIO12.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dio12 {
    #[doc = "0: Edge detection disabled"]
    Disable = 0,
    #[doc = "1: Detects rising edge of input event"]
    Rise = 1,
    #[doc = "2: Detects falling edge of input event"]
    Fall = 2,
    #[doc = "3: Detects both rising and falling edge of input event"]
    RiseFall = 3,
}
impl From<Dio12> for u8 {
    #[inline(always)]
    fn from(variant: Dio12) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dio12 {
    type Ux = u8;
}
impl crate::IsEnum for Dio12 {}
#[doc = "Field `DIO12` reader - Enables and configures edge detection polarity for DIO12."]
pub type Dio12R = crate::FieldReader<Dio12>;
impl Dio12R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio12 {
        match self.bits {
            0 => Dio12::Disable,
            1 => Dio12::Rise,
            2 => Dio12::Fall,
            3 => Dio12::RiseFall,
            _ => unreachable!(),
        }
    }
    #[doc = "Edge detection disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Dio12::Disable
    }
    #[doc = "Detects rising edge of input event"]
    #[inline(always)]
    pub fn is_rise(&self) -> bool {
        *self == Dio12::Rise
    }
    #[doc = "Detects falling edge of input event"]
    #[inline(always)]
    pub fn is_fall(&self) -> bool {
        *self == Dio12::Fall
    }
    #[doc = "Detects both rising and falling edge of input event"]
    #[inline(always)]
    pub fn is_rise_fall(&self) -> bool {
        *self == Dio12::RiseFall
    }
}
#[doc = "Field `DIO12` writer - Enables and configures edge detection polarity for DIO12."]
pub type Dio12W<'a, REG> = crate::FieldWriter<'a, REG, 2, Dio12, crate::Safe>;
impl<'a, REG> Dio12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Edge detection disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Dio12::Disable)
    }
    #[doc = "Detects rising edge of input event"]
    #[inline(always)]
    pub fn rise(self) -> &'a mut crate::W<REG> {
        self.variant(Dio12::Rise)
    }
    #[doc = "Detects falling edge of input event"]
    #[inline(always)]
    pub fn fall(self) -> &'a mut crate::W<REG> {
        self.variant(Dio12::Fall)
    }
    #[doc = "Detects both rising and falling edge of input event"]
    #[inline(always)]
    pub fn rise_fall(self) -> &'a mut crate::W<REG> {
        self.variant(Dio12::RiseFall)
    }
}
#[doc = "Enables and configures edge detection polarity for DIO13.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dio13 {
    #[doc = "0: Edge detection disabled"]
    Disable = 0,
    #[doc = "1: Detects rising edge of input event"]
    Rise = 1,
    #[doc = "2: Detects falling edge of input event"]
    Fall = 2,
    #[doc = "3: Detects both rising and falling edge of input event"]
    RiseFall = 3,
}
impl From<Dio13> for u8 {
    #[inline(always)]
    fn from(variant: Dio13) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dio13 {
    type Ux = u8;
}
impl crate::IsEnum for Dio13 {}
#[doc = "Field `DIO13` reader - Enables and configures edge detection polarity for DIO13."]
pub type Dio13R = crate::FieldReader<Dio13>;
impl Dio13R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio13 {
        match self.bits {
            0 => Dio13::Disable,
            1 => Dio13::Rise,
            2 => Dio13::Fall,
            3 => Dio13::RiseFall,
            _ => unreachable!(),
        }
    }
    #[doc = "Edge detection disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Dio13::Disable
    }
    #[doc = "Detects rising edge of input event"]
    #[inline(always)]
    pub fn is_rise(&self) -> bool {
        *self == Dio13::Rise
    }
    #[doc = "Detects falling edge of input event"]
    #[inline(always)]
    pub fn is_fall(&self) -> bool {
        *self == Dio13::Fall
    }
    #[doc = "Detects both rising and falling edge of input event"]
    #[inline(always)]
    pub fn is_rise_fall(&self) -> bool {
        *self == Dio13::RiseFall
    }
}
#[doc = "Field `DIO13` writer - Enables and configures edge detection polarity for DIO13."]
pub type Dio13W<'a, REG> = crate::FieldWriter<'a, REG, 2, Dio13, crate::Safe>;
impl<'a, REG> Dio13W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Edge detection disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Dio13::Disable)
    }
    #[doc = "Detects rising edge of input event"]
    #[inline(always)]
    pub fn rise(self) -> &'a mut crate::W<REG> {
        self.variant(Dio13::Rise)
    }
    #[doc = "Detects falling edge of input event"]
    #[inline(always)]
    pub fn fall(self) -> &'a mut crate::W<REG> {
        self.variant(Dio13::Fall)
    }
    #[doc = "Detects both rising and falling edge of input event"]
    #[inline(always)]
    pub fn rise_fall(self) -> &'a mut crate::W<REG> {
        self.variant(Dio13::RiseFall)
    }
}
#[doc = "Enables and configures edge detection polarity for DIO14.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dio14 {
    #[doc = "0: Edge detection disabled"]
    Disable = 0,
    #[doc = "1: Detects rising edge of input event"]
    Rise = 1,
    #[doc = "2: Detects falling edge of input event"]
    Fall = 2,
    #[doc = "3: Detects both rising and falling edge of input event"]
    RiseFall = 3,
}
impl From<Dio14> for u8 {
    #[inline(always)]
    fn from(variant: Dio14) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dio14 {
    type Ux = u8;
}
impl crate::IsEnum for Dio14 {}
#[doc = "Field `DIO14` reader - Enables and configures edge detection polarity for DIO14."]
pub type Dio14R = crate::FieldReader<Dio14>;
impl Dio14R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio14 {
        match self.bits {
            0 => Dio14::Disable,
            1 => Dio14::Rise,
            2 => Dio14::Fall,
            3 => Dio14::RiseFall,
            _ => unreachable!(),
        }
    }
    #[doc = "Edge detection disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Dio14::Disable
    }
    #[doc = "Detects rising edge of input event"]
    #[inline(always)]
    pub fn is_rise(&self) -> bool {
        *self == Dio14::Rise
    }
    #[doc = "Detects falling edge of input event"]
    #[inline(always)]
    pub fn is_fall(&self) -> bool {
        *self == Dio14::Fall
    }
    #[doc = "Detects both rising and falling edge of input event"]
    #[inline(always)]
    pub fn is_rise_fall(&self) -> bool {
        *self == Dio14::RiseFall
    }
}
#[doc = "Field `DIO14` writer - Enables and configures edge detection polarity for DIO14."]
pub type Dio14W<'a, REG> = crate::FieldWriter<'a, REG, 2, Dio14, crate::Safe>;
impl<'a, REG> Dio14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Edge detection disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Dio14::Disable)
    }
    #[doc = "Detects rising edge of input event"]
    #[inline(always)]
    pub fn rise(self) -> &'a mut crate::W<REG> {
        self.variant(Dio14::Rise)
    }
    #[doc = "Detects falling edge of input event"]
    #[inline(always)]
    pub fn fall(self) -> &'a mut crate::W<REG> {
        self.variant(Dio14::Fall)
    }
    #[doc = "Detects both rising and falling edge of input event"]
    #[inline(always)]
    pub fn rise_fall(self) -> &'a mut crate::W<REG> {
        self.variant(Dio14::RiseFall)
    }
}
#[doc = "Enables and configures edge detection polarity for DIO15.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dio15 {
    #[doc = "0: Edge detection disabled"]
    Disable = 0,
    #[doc = "1: Detects rising edge of input event"]
    Rise = 1,
    #[doc = "2: Detects falling edge of input event"]
    Fall = 2,
    #[doc = "3: Detects both rising and falling edge of input event"]
    RiseFall = 3,
}
impl From<Dio15> for u8 {
    #[inline(always)]
    fn from(variant: Dio15) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dio15 {
    type Ux = u8;
}
impl crate::IsEnum for Dio15 {}
#[doc = "Field `DIO15` reader - Enables and configures edge detection polarity for DIO15."]
pub type Dio15R = crate::FieldReader<Dio15>;
impl Dio15R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio15 {
        match self.bits {
            0 => Dio15::Disable,
            1 => Dio15::Rise,
            2 => Dio15::Fall,
            3 => Dio15::RiseFall,
            _ => unreachable!(),
        }
    }
    #[doc = "Edge detection disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Dio15::Disable
    }
    #[doc = "Detects rising edge of input event"]
    #[inline(always)]
    pub fn is_rise(&self) -> bool {
        *self == Dio15::Rise
    }
    #[doc = "Detects falling edge of input event"]
    #[inline(always)]
    pub fn is_fall(&self) -> bool {
        *self == Dio15::Fall
    }
    #[doc = "Detects both rising and falling edge of input event"]
    #[inline(always)]
    pub fn is_rise_fall(&self) -> bool {
        *self == Dio15::RiseFall
    }
}
#[doc = "Field `DIO15` writer - Enables and configures edge detection polarity for DIO15."]
pub type Dio15W<'a, REG> = crate::FieldWriter<'a, REG, 2, Dio15, crate::Safe>;
impl<'a, REG> Dio15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Edge detection disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Dio15::Disable)
    }
    #[doc = "Detects rising edge of input event"]
    #[inline(always)]
    pub fn rise(self) -> &'a mut crate::W<REG> {
        self.variant(Dio15::Rise)
    }
    #[doc = "Detects falling edge of input event"]
    #[inline(always)]
    pub fn fall(self) -> &'a mut crate::W<REG> {
        self.variant(Dio15::Fall)
    }
    #[doc = "Detects both rising and falling edge of input event"]
    #[inline(always)]
    pub fn rise_fall(self) -> &'a mut crate::W<REG> {
        self.variant(Dio15::RiseFall)
    }
}
impl R {
    #[doc = "Bits 0:1 - Enables and configures edge detection polarity for DIO0."]
    #[inline(always)]
    pub fn dio0(&self) -> Dio0R {
        Dio0R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Enables and configures edge detection polarity for DIO1."]
    #[inline(always)]
    pub fn dio1(&self) -> Dio1R {
        Dio1R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Enables and configures edge detection polarity for DIO2."]
    #[inline(always)]
    pub fn dio2(&self) -> Dio2R {
        Dio2R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Enables and configures edge detection polarity for DIO3."]
    #[inline(always)]
    pub fn dio3(&self) -> Dio3R {
        Dio3R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Enables and configures edge detection polarity for DIO4."]
    #[inline(always)]
    pub fn dio4(&self) -> Dio4R {
        Dio4R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Enables and configures edge detection polarity for DIO5."]
    #[inline(always)]
    pub fn dio5(&self) -> Dio5R {
        Dio5R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Enables and configures edge detection polarity for DIO6."]
    #[inline(always)]
    pub fn dio6(&self) -> Dio6R {
        Dio6R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Enables and configures edge detection polarity for DIO7."]
    #[inline(always)]
    pub fn dio7(&self) -> Dio7R {
        Dio7R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Enables and configures edge detection polarity for DIO8."]
    #[inline(always)]
    pub fn dio8(&self) -> Dio8R {
        Dio8R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Enables and configures edge detection polarity for DIO9."]
    #[inline(always)]
    pub fn dio9(&self) -> Dio9R {
        Dio9R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Enables and configures edge detection polarity for DIO10."]
    #[inline(always)]
    pub fn dio10(&self) -> Dio10R {
        Dio10R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Enables and configures edge detection polarity for DIO11."]
    #[inline(always)]
    pub fn dio11(&self) -> Dio11R {
        Dio11R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Enables and configures edge detection polarity for DIO12."]
    #[inline(always)]
    pub fn dio12(&self) -> Dio12R {
        Dio12R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - Enables and configures edge detection polarity for DIO13."]
    #[inline(always)]
    pub fn dio13(&self) -> Dio13R {
        Dio13R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Enables and configures edge detection polarity for DIO14."]
    #[inline(always)]
    pub fn dio14(&self) -> Dio14R {
        Dio14R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Enables and configures edge detection polarity for DIO15."]
    #[inline(always)]
    pub fn dio15(&self) -> Dio15R {
        Dio15R::new(((self.bits >> 30) & 3) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("POLARITY15_0")
            .field("dio0", &self.dio0())
            .field("dio1", &self.dio1())
            .field("dio2", &self.dio2())
            .field("dio3", &self.dio3())
            .field("dio4", &self.dio4())
            .field("dio5", &self.dio5())
            .field("dio6", &self.dio6())
            .field("dio7", &self.dio7())
            .field("dio8", &self.dio8())
            .field("dio9", &self.dio9())
            .field("dio10", &self.dio10())
            .field("dio11", &self.dio11())
            .field("dio12", &self.dio12())
            .field("dio13", &self.dio13())
            .field("dio14", &self.dio14())
            .field("dio15", &self.dio15())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - Enables and configures edge detection polarity for DIO0."]
    #[inline(always)]
    pub fn dio0(&mut self) -> Dio0W<'_, Polarity15_0Spec> {
        Dio0W::new(self, 0)
    }
    #[doc = "Bits 2:3 - Enables and configures edge detection polarity for DIO1."]
    #[inline(always)]
    pub fn dio1(&mut self) -> Dio1W<'_, Polarity15_0Spec> {
        Dio1W::new(self, 2)
    }
    #[doc = "Bits 4:5 - Enables and configures edge detection polarity for DIO2."]
    #[inline(always)]
    pub fn dio2(&mut self) -> Dio2W<'_, Polarity15_0Spec> {
        Dio2W::new(self, 4)
    }
    #[doc = "Bits 6:7 - Enables and configures edge detection polarity for DIO3."]
    #[inline(always)]
    pub fn dio3(&mut self) -> Dio3W<'_, Polarity15_0Spec> {
        Dio3W::new(self, 6)
    }
    #[doc = "Bits 8:9 - Enables and configures edge detection polarity for DIO4."]
    #[inline(always)]
    pub fn dio4(&mut self) -> Dio4W<'_, Polarity15_0Spec> {
        Dio4W::new(self, 8)
    }
    #[doc = "Bits 10:11 - Enables and configures edge detection polarity for DIO5."]
    #[inline(always)]
    pub fn dio5(&mut self) -> Dio5W<'_, Polarity15_0Spec> {
        Dio5W::new(self, 10)
    }
    #[doc = "Bits 12:13 - Enables and configures edge detection polarity for DIO6."]
    #[inline(always)]
    pub fn dio6(&mut self) -> Dio6W<'_, Polarity15_0Spec> {
        Dio6W::new(self, 12)
    }
    #[doc = "Bits 14:15 - Enables and configures edge detection polarity for DIO7."]
    #[inline(always)]
    pub fn dio7(&mut self) -> Dio7W<'_, Polarity15_0Spec> {
        Dio7W::new(self, 14)
    }
    #[doc = "Bits 16:17 - Enables and configures edge detection polarity for DIO8."]
    #[inline(always)]
    pub fn dio8(&mut self) -> Dio8W<'_, Polarity15_0Spec> {
        Dio8W::new(self, 16)
    }
    #[doc = "Bits 18:19 - Enables and configures edge detection polarity for DIO9."]
    #[inline(always)]
    pub fn dio9(&mut self) -> Dio9W<'_, Polarity15_0Spec> {
        Dio9W::new(self, 18)
    }
    #[doc = "Bits 20:21 - Enables and configures edge detection polarity for DIO10."]
    #[inline(always)]
    pub fn dio10(&mut self) -> Dio10W<'_, Polarity15_0Spec> {
        Dio10W::new(self, 20)
    }
    #[doc = "Bits 22:23 - Enables and configures edge detection polarity for DIO11."]
    #[inline(always)]
    pub fn dio11(&mut self) -> Dio11W<'_, Polarity15_0Spec> {
        Dio11W::new(self, 22)
    }
    #[doc = "Bits 24:25 - Enables and configures edge detection polarity for DIO12."]
    #[inline(always)]
    pub fn dio12(&mut self) -> Dio12W<'_, Polarity15_0Spec> {
        Dio12W::new(self, 24)
    }
    #[doc = "Bits 26:27 - Enables and configures edge detection polarity for DIO13."]
    #[inline(always)]
    pub fn dio13(&mut self) -> Dio13W<'_, Polarity15_0Spec> {
        Dio13W::new(self, 26)
    }
    #[doc = "Bits 28:29 - Enables and configures edge detection polarity for DIO14."]
    #[inline(always)]
    pub fn dio14(&mut self) -> Dio14W<'_, Polarity15_0Spec> {
        Dio14W::new(self, 28)
    }
    #[doc = "Bits 30:31 - Enables and configures edge detection polarity for DIO15."]
    #[inline(always)]
    pub fn dio15(&mut self) -> Dio15W<'_, Polarity15_0Spec> {
        Dio15W::new(self, 30)
    }
}
#[doc = "Polarity 15 to 0\n\nYou can [`read`](crate::Reg::read) this register and get [`polarity15_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`polarity15_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Polarity15_0Spec;
impl crate::RegisterSpec for Polarity15_0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`polarity15_0::R`](R) reader structure"]
impl crate::Readable for Polarity15_0Spec {}
#[doc = "`write(|w| ..)` method takes [`polarity15_0::W`](W) writer structure"]
impl crate::Writable for Polarity15_0Spec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets POLARITY15_0 to value 0"]
impl crate::Resettable for Polarity15_0Spec {}
