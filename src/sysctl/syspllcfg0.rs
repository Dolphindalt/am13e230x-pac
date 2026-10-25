#[doc = "Register `SYSPLLCFG0` reader"]
pub type R = crate::R<Syspllcfg0Spec>;
#[doc = "Register `SYSPLLCFG0` writer"]
pub type W = crate::W<Syspllcfg0Spec>;
#[doc = "SYSPLLREF selects the system PLL (SYSPLL) reference clock source.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Syspllref {
    #[doc = "0: SYSPLL reference is SYSOSC"]
    Sysosc = 0,
    #[doc = "1: SYSPLL reference is HFCLK"]
    Hfclk = 1,
}
impl From<Syspllref> for bool {
    #[inline(always)]
    fn from(variant: Syspllref) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYSPLLREF` reader - SYSPLLREF selects the system PLL (SYSPLL) reference clock source."]
pub type SyspllrefR = crate::BitReader<Syspllref>;
impl SyspllrefR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Syspllref {
        match self.bits {
            false => Syspllref::Sysosc,
            true => Syspllref::Hfclk,
        }
    }
    #[doc = "SYSPLL reference is SYSOSC"]
    #[inline(always)]
    pub fn is_sysosc(&self) -> bool {
        *self == Syspllref::Sysosc
    }
    #[doc = "SYSPLL reference is HFCLK"]
    #[inline(always)]
    pub fn is_hfclk(&self) -> bool {
        *self == Syspllref::Hfclk
    }
}
#[doc = "Field `SYSPLLREF` writer - SYSPLLREF selects the system PLL (SYSPLL) reference clock source."]
pub type SyspllrefW<'a, REG> = crate::BitWriter<'a, REG, Syspllref>;
impl<'a, REG> SyspllrefW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SYSPLL reference is SYSOSC"]
    #[inline(always)]
    pub fn sysosc(self) -> &'a mut crate::W<REG> {
        self.variant(Syspllref::Sysosc)
    }
    #[doc = "SYSPLL reference is HFCLK"]
    #[inline(always)]
    pub fn hfclk(self) -> &'a mut crate::W<REG> {
        self.variant(Syspllref::Hfclk)
    }
}
#[doc = "ENABLECLK0 enables or disables the SYSPLLCLK0 output.\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enableclk0 {
    #[doc = "0: SYSPLLCLK0 is disabled"]
    Disable = 0,
    #[doc = "1: SYSPLLCLK0 is enabled"]
    Enable = 1,
}
impl From<Enableclk0> for bool {
    #[inline(always)]
    fn from(variant: Enableclk0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENABLECLK0` reader - ENABLECLK0 enables or disables the SYSPLLCLK0 output."]
pub type Enableclk0R = crate::BitReader<Enableclk0>;
impl Enableclk0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enableclk0 {
        match self.bits {
            false => Enableclk0::Disable,
            true => Enableclk0::Enable,
        }
    }
    #[doc = "SYSPLLCLK0 is disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Enableclk0::Disable
    }
    #[doc = "SYSPLLCLK0 is enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Enableclk0::Enable
    }
}
#[doc = "Field `ENABLECLK0` writer - ENABLECLK0 enables or disables the SYSPLLCLK0 output."]
pub type Enableclk0W<'a, REG> = crate::BitWriter<'a, REG, Enableclk0>;
impl<'a, REG> Enableclk0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SYSPLLCLK0 is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Enableclk0::Disable)
    }
    #[doc = "SYSPLLCLK0 is enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Enableclk0::Enable)
    }
}
#[doc = "ENABLECLK1 enables or disables the SYSPLLCLK1 output.\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enableclk1 {
    #[doc = "0: SYSPLLCLK1 is disabled"]
    Disable = 0,
    #[doc = "1: SYSPLLCLK1 is enabled"]
    Enable = 1,
}
impl From<Enableclk1> for bool {
    #[inline(always)]
    fn from(variant: Enableclk1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENABLECLK1` reader - ENABLECLK1 enables or disables the SYSPLLCLK1 output."]
pub type Enableclk1R = crate::BitReader<Enableclk1>;
impl Enableclk1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enableclk1 {
        match self.bits {
            false => Enableclk1::Disable,
            true => Enableclk1::Enable,
        }
    }
    #[doc = "SYSPLLCLK1 is disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Enableclk1::Disable
    }
    #[doc = "SYSPLLCLK1 is enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Enableclk1::Enable
    }
}
#[doc = "Field `ENABLECLK1` writer - ENABLECLK1 enables or disables the SYSPLLCLK1 output."]
pub type Enableclk1W<'a, REG> = crate::BitWriter<'a, REG, Enableclk1>;
impl<'a, REG> Enableclk1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SYSPLLCLK1 is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Enableclk1::Disable)
    }
    #[doc = "SYSPLLCLK1 is enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Enableclk1::Enable)
    }
}
#[doc = "RDIVCLK0 sets the final divider for the SYSPLLCLK0 output.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rdivclk0 {
    #[doc = "0: SYSPLLCLK0 is divided by 2"]
    Clk0div2 = 0,
    #[doc = "1: SYSPLLCLK0 is divided by 4"]
    Clk0div4 = 1,
    #[doc = "2: SYSPLLCLK0 is divided by 6"]
    Clk0div6 = 2,
    #[doc = "3: SYSPLLCLK0 is divided by 8"]
    Clk0div8 = 3,
    #[doc = "4: SYSPLLCLK0 is divided by 10"]
    Clk0div10 = 4,
    #[doc = "5: SYSPLLCLK0 is divided by 12"]
    Clk0div12 = 5,
    #[doc = "6: SYSPLLCLK0 is divided by 14"]
    Clk0div14 = 6,
    #[doc = "7: SYSPLLCLK0 is divided by 16"]
    Clk0div16 = 7,
    #[doc = "8: SYSPLLCLK0 is divided by 18"]
    Clk0div18 = 8,
    #[doc = "9: SYSPLLCLK0 is divided by 20"]
    Clk0div20 = 9,
    #[doc = "10: SYSPLLCLK0 is divided by 22"]
    Clk0div22 = 10,
    #[doc = "11: SYSPLLCLK0 is divided by 24"]
    Clk0div24 = 11,
    #[doc = "12: SYSPLLCLK0 is divided by 26"]
    Clk0div26 = 12,
    #[doc = "13: SYSPLLCLK0 is divided by 28"]
    Clk0div28 = 13,
    #[doc = "14: SYSPLLCLK0 is divided by 30"]
    Clk0div30 = 14,
    #[doc = "15: SYSPLLCLK0 is divided by 32"]
    Clk0div32 = 15,
}
impl From<Rdivclk0> for u8 {
    #[inline(always)]
    fn from(variant: Rdivclk0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rdivclk0 {
    type Ux = u8;
}
impl crate::IsEnum for Rdivclk0 {}
#[doc = "Field `RDIVCLK0` reader - RDIVCLK0 sets the final divider for the SYSPLLCLK0 output."]
pub type Rdivclk0R = crate::FieldReader<Rdivclk0>;
impl Rdivclk0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rdivclk0 {
        match self.bits {
            0 => Rdivclk0::Clk0div2,
            1 => Rdivclk0::Clk0div4,
            2 => Rdivclk0::Clk0div6,
            3 => Rdivclk0::Clk0div8,
            4 => Rdivclk0::Clk0div10,
            5 => Rdivclk0::Clk0div12,
            6 => Rdivclk0::Clk0div14,
            7 => Rdivclk0::Clk0div16,
            8 => Rdivclk0::Clk0div18,
            9 => Rdivclk0::Clk0div20,
            10 => Rdivclk0::Clk0div22,
            11 => Rdivclk0::Clk0div24,
            12 => Rdivclk0::Clk0div26,
            13 => Rdivclk0::Clk0div28,
            14 => Rdivclk0::Clk0div30,
            15 => Rdivclk0::Clk0div32,
            _ => unreachable!(),
        }
    }
    #[doc = "SYSPLLCLK0 is divided by 2"]
    #[inline(always)]
    pub fn is_clk0div2(&self) -> bool {
        *self == Rdivclk0::Clk0div2
    }
    #[doc = "SYSPLLCLK0 is divided by 4"]
    #[inline(always)]
    pub fn is_clk0div4(&self) -> bool {
        *self == Rdivclk0::Clk0div4
    }
    #[doc = "SYSPLLCLK0 is divided by 6"]
    #[inline(always)]
    pub fn is_clk0div6(&self) -> bool {
        *self == Rdivclk0::Clk0div6
    }
    #[doc = "SYSPLLCLK0 is divided by 8"]
    #[inline(always)]
    pub fn is_clk0div8(&self) -> bool {
        *self == Rdivclk0::Clk0div8
    }
    #[doc = "SYSPLLCLK0 is divided by 10"]
    #[inline(always)]
    pub fn is_clk0div10(&self) -> bool {
        *self == Rdivclk0::Clk0div10
    }
    #[doc = "SYSPLLCLK0 is divided by 12"]
    #[inline(always)]
    pub fn is_clk0div12(&self) -> bool {
        *self == Rdivclk0::Clk0div12
    }
    #[doc = "SYSPLLCLK0 is divided by 14"]
    #[inline(always)]
    pub fn is_clk0div14(&self) -> bool {
        *self == Rdivclk0::Clk0div14
    }
    #[doc = "SYSPLLCLK0 is divided by 16"]
    #[inline(always)]
    pub fn is_clk0div16(&self) -> bool {
        *self == Rdivclk0::Clk0div16
    }
    #[doc = "SYSPLLCLK0 is divided by 18"]
    #[inline(always)]
    pub fn is_clk0div18(&self) -> bool {
        *self == Rdivclk0::Clk0div18
    }
    #[doc = "SYSPLLCLK0 is divided by 20"]
    #[inline(always)]
    pub fn is_clk0div20(&self) -> bool {
        *self == Rdivclk0::Clk0div20
    }
    #[doc = "SYSPLLCLK0 is divided by 22"]
    #[inline(always)]
    pub fn is_clk0div22(&self) -> bool {
        *self == Rdivclk0::Clk0div22
    }
    #[doc = "SYSPLLCLK0 is divided by 24"]
    #[inline(always)]
    pub fn is_clk0div24(&self) -> bool {
        *self == Rdivclk0::Clk0div24
    }
    #[doc = "SYSPLLCLK0 is divided by 26"]
    #[inline(always)]
    pub fn is_clk0div26(&self) -> bool {
        *self == Rdivclk0::Clk0div26
    }
    #[doc = "SYSPLLCLK0 is divided by 28"]
    #[inline(always)]
    pub fn is_clk0div28(&self) -> bool {
        *self == Rdivclk0::Clk0div28
    }
    #[doc = "SYSPLLCLK0 is divided by 30"]
    #[inline(always)]
    pub fn is_clk0div30(&self) -> bool {
        *self == Rdivclk0::Clk0div30
    }
    #[doc = "SYSPLLCLK0 is divided by 32"]
    #[inline(always)]
    pub fn is_clk0div32(&self) -> bool {
        *self == Rdivclk0::Clk0div32
    }
}
#[doc = "Field `RDIVCLK0` writer - RDIVCLK0 sets the final divider for the SYSPLLCLK0 output."]
pub type Rdivclk0W<'a, REG> = crate::FieldWriter<'a, REG, 4, Rdivclk0, crate::Safe>;
impl<'a, REG> Rdivclk0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "SYSPLLCLK0 is divided by 2"]
    #[inline(always)]
    pub fn clk0div2(self) -> &'a mut crate::W<REG> {
        self.variant(Rdivclk0::Clk0div2)
    }
    #[doc = "SYSPLLCLK0 is divided by 4"]
    #[inline(always)]
    pub fn clk0div4(self) -> &'a mut crate::W<REG> {
        self.variant(Rdivclk0::Clk0div4)
    }
    #[doc = "SYSPLLCLK0 is divided by 6"]
    #[inline(always)]
    pub fn clk0div6(self) -> &'a mut crate::W<REG> {
        self.variant(Rdivclk0::Clk0div6)
    }
    #[doc = "SYSPLLCLK0 is divided by 8"]
    #[inline(always)]
    pub fn clk0div8(self) -> &'a mut crate::W<REG> {
        self.variant(Rdivclk0::Clk0div8)
    }
    #[doc = "SYSPLLCLK0 is divided by 10"]
    #[inline(always)]
    pub fn clk0div10(self) -> &'a mut crate::W<REG> {
        self.variant(Rdivclk0::Clk0div10)
    }
    #[doc = "SYSPLLCLK0 is divided by 12"]
    #[inline(always)]
    pub fn clk0div12(self) -> &'a mut crate::W<REG> {
        self.variant(Rdivclk0::Clk0div12)
    }
    #[doc = "SYSPLLCLK0 is divided by 14"]
    #[inline(always)]
    pub fn clk0div14(self) -> &'a mut crate::W<REG> {
        self.variant(Rdivclk0::Clk0div14)
    }
    #[doc = "SYSPLLCLK0 is divided by 16"]
    #[inline(always)]
    pub fn clk0div16(self) -> &'a mut crate::W<REG> {
        self.variant(Rdivclk0::Clk0div16)
    }
    #[doc = "SYSPLLCLK0 is divided by 18"]
    #[inline(always)]
    pub fn clk0div18(self) -> &'a mut crate::W<REG> {
        self.variant(Rdivclk0::Clk0div18)
    }
    #[doc = "SYSPLLCLK0 is divided by 20"]
    #[inline(always)]
    pub fn clk0div20(self) -> &'a mut crate::W<REG> {
        self.variant(Rdivclk0::Clk0div20)
    }
    #[doc = "SYSPLLCLK0 is divided by 22"]
    #[inline(always)]
    pub fn clk0div22(self) -> &'a mut crate::W<REG> {
        self.variant(Rdivclk0::Clk0div22)
    }
    #[doc = "SYSPLLCLK0 is divided by 24"]
    #[inline(always)]
    pub fn clk0div24(self) -> &'a mut crate::W<REG> {
        self.variant(Rdivclk0::Clk0div24)
    }
    #[doc = "SYSPLLCLK0 is divided by 26"]
    #[inline(always)]
    pub fn clk0div26(self) -> &'a mut crate::W<REG> {
        self.variant(Rdivclk0::Clk0div26)
    }
    #[doc = "SYSPLLCLK0 is divided by 28"]
    #[inline(always)]
    pub fn clk0div28(self) -> &'a mut crate::W<REG> {
        self.variant(Rdivclk0::Clk0div28)
    }
    #[doc = "SYSPLLCLK0 is divided by 30"]
    #[inline(always)]
    pub fn clk0div30(self) -> &'a mut crate::W<REG> {
        self.variant(Rdivclk0::Clk0div30)
    }
    #[doc = "SYSPLLCLK0 is divided by 32"]
    #[inline(always)]
    pub fn clk0div32(self) -> &'a mut crate::W<REG> {
        self.variant(Rdivclk0::Clk0div32)
    }
}
#[doc = "RDIVCLK1 sets the final divider for the SYSPLLCLK1 output.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rdivclk1 {
    #[doc = "0: SYSPLLCLK1 is divided by 2"]
    Clk1div2 = 0,
    #[doc = "1: SYSPLLCLK1 is divided by 4"]
    Clk1div4 = 1,
    #[doc = "2: SYSPLLCLK1 is divided by 6"]
    Clk1div6 = 2,
    #[doc = "3: SYSPLLCLK1 is divided by 8"]
    Clk1div8 = 3,
    #[doc = "4: SYSPLLCLK1 is divided by 10"]
    Clk1div10 = 4,
    #[doc = "5: SYSPLLCLK1 is divided by 12"]
    Clk1div12 = 5,
    #[doc = "6: SYSPLLCLK1 is divided by 14"]
    Clk1div14 = 6,
    #[doc = "7: SYSPLLCLK1 is divided by 16"]
    Clk1div16 = 7,
    #[doc = "8: SYSPLLCLK1 is divided by 18"]
    Clk1div18 = 8,
    #[doc = "9: SYSPLLCLK1 is divided by 20"]
    Clk1div20 = 9,
    #[doc = "10: SYSPLLCLK1 is divided by 22"]
    Clk1div22 = 10,
    #[doc = "11: SYSPLLCLK1 is divided by 24"]
    Clk1div24 = 11,
    #[doc = "12: SYSPLLCLK1 is divided by 26"]
    Clk1div26 = 12,
    #[doc = "13: SYSPLLCLK1 is divided by 28"]
    Clk1div28 = 13,
    #[doc = "14: SYSPLLCLK1 is divided by 30"]
    Clk1div30 = 14,
    #[doc = "15: SYSPLLCLK1 is divided by 32"]
    Clk1div32 = 15,
}
impl From<Rdivclk1> for u8 {
    #[inline(always)]
    fn from(variant: Rdivclk1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rdivclk1 {
    type Ux = u8;
}
impl crate::IsEnum for Rdivclk1 {}
#[doc = "Field `RDIVCLK1` reader - RDIVCLK1 sets the final divider for the SYSPLLCLK1 output."]
pub type Rdivclk1R = crate::FieldReader<Rdivclk1>;
impl Rdivclk1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rdivclk1 {
        match self.bits {
            0 => Rdivclk1::Clk1div2,
            1 => Rdivclk1::Clk1div4,
            2 => Rdivclk1::Clk1div6,
            3 => Rdivclk1::Clk1div8,
            4 => Rdivclk1::Clk1div10,
            5 => Rdivclk1::Clk1div12,
            6 => Rdivclk1::Clk1div14,
            7 => Rdivclk1::Clk1div16,
            8 => Rdivclk1::Clk1div18,
            9 => Rdivclk1::Clk1div20,
            10 => Rdivclk1::Clk1div22,
            11 => Rdivclk1::Clk1div24,
            12 => Rdivclk1::Clk1div26,
            13 => Rdivclk1::Clk1div28,
            14 => Rdivclk1::Clk1div30,
            15 => Rdivclk1::Clk1div32,
            _ => unreachable!(),
        }
    }
    #[doc = "SYSPLLCLK1 is divided by 2"]
    #[inline(always)]
    pub fn is_clk1div2(&self) -> bool {
        *self == Rdivclk1::Clk1div2
    }
    #[doc = "SYSPLLCLK1 is divided by 4"]
    #[inline(always)]
    pub fn is_clk1div4(&self) -> bool {
        *self == Rdivclk1::Clk1div4
    }
    #[doc = "SYSPLLCLK1 is divided by 6"]
    #[inline(always)]
    pub fn is_clk1div6(&self) -> bool {
        *self == Rdivclk1::Clk1div6
    }
    #[doc = "SYSPLLCLK1 is divided by 8"]
    #[inline(always)]
    pub fn is_clk1div8(&self) -> bool {
        *self == Rdivclk1::Clk1div8
    }
    #[doc = "SYSPLLCLK1 is divided by 10"]
    #[inline(always)]
    pub fn is_clk1div10(&self) -> bool {
        *self == Rdivclk1::Clk1div10
    }
    #[doc = "SYSPLLCLK1 is divided by 12"]
    #[inline(always)]
    pub fn is_clk1div12(&self) -> bool {
        *self == Rdivclk1::Clk1div12
    }
    #[doc = "SYSPLLCLK1 is divided by 14"]
    #[inline(always)]
    pub fn is_clk1div14(&self) -> bool {
        *self == Rdivclk1::Clk1div14
    }
    #[doc = "SYSPLLCLK1 is divided by 16"]
    #[inline(always)]
    pub fn is_clk1div16(&self) -> bool {
        *self == Rdivclk1::Clk1div16
    }
    #[doc = "SYSPLLCLK1 is divided by 18"]
    #[inline(always)]
    pub fn is_clk1div18(&self) -> bool {
        *self == Rdivclk1::Clk1div18
    }
    #[doc = "SYSPLLCLK1 is divided by 20"]
    #[inline(always)]
    pub fn is_clk1div20(&self) -> bool {
        *self == Rdivclk1::Clk1div20
    }
    #[doc = "SYSPLLCLK1 is divided by 22"]
    #[inline(always)]
    pub fn is_clk1div22(&self) -> bool {
        *self == Rdivclk1::Clk1div22
    }
    #[doc = "SYSPLLCLK1 is divided by 24"]
    #[inline(always)]
    pub fn is_clk1div24(&self) -> bool {
        *self == Rdivclk1::Clk1div24
    }
    #[doc = "SYSPLLCLK1 is divided by 26"]
    #[inline(always)]
    pub fn is_clk1div26(&self) -> bool {
        *self == Rdivclk1::Clk1div26
    }
    #[doc = "SYSPLLCLK1 is divided by 28"]
    #[inline(always)]
    pub fn is_clk1div28(&self) -> bool {
        *self == Rdivclk1::Clk1div28
    }
    #[doc = "SYSPLLCLK1 is divided by 30"]
    #[inline(always)]
    pub fn is_clk1div30(&self) -> bool {
        *self == Rdivclk1::Clk1div30
    }
    #[doc = "SYSPLLCLK1 is divided by 32"]
    #[inline(always)]
    pub fn is_clk1div32(&self) -> bool {
        *self == Rdivclk1::Clk1div32
    }
}
#[doc = "Field `RDIVCLK1` writer - RDIVCLK1 sets the final divider for the SYSPLLCLK1 output."]
pub type Rdivclk1W<'a, REG> = crate::FieldWriter<'a, REG, 4, Rdivclk1, crate::Safe>;
impl<'a, REG> Rdivclk1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "SYSPLLCLK1 is divided by 2"]
    #[inline(always)]
    pub fn clk1div2(self) -> &'a mut crate::W<REG> {
        self.variant(Rdivclk1::Clk1div2)
    }
    #[doc = "SYSPLLCLK1 is divided by 4"]
    #[inline(always)]
    pub fn clk1div4(self) -> &'a mut crate::W<REG> {
        self.variant(Rdivclk1::Clk1div4)
    }
    #[doc = "SYSPLLCLK1 is divided by 6"]
    #[inline(always)]
    pub fn clk1div6(self) -> &'a mut crate::W<REG> {
        self.variant(Rdivclk1::Clk1div6)
    }
    #[doc = "SYSPLLCLK1 is divided by 8"]
    #[inline(always)]
    pub fn clk1div8(self) -> &'a mut crate::W<REG> {
        self.variant(Rdivclk1::Clk1div8)
    }
    #[doc = "SYSPLLCLK1 is divided by 10"]
    #[inline(always)]
    pub fn clk1div10(self) -> &'a mut crate::W<REG> {
        self.variant(Rdivclk1::Clk1div10)
    }
    #[doc = "SYSPLLCLK1 is divided by 12"]
    #[inline(always)]
    pub fn clk1div12(self) -> &'a mut crate::W<REG> {
        self.variant(Rdivclk1::Clk1div12)
    }
    #[doc = "SYSPLLCLK1 is divided by 14"]
    #[inline(always)]
    pub fn clk1div14(self) -> &'a mut crate::W<REG> {
        self.variant(Rdivclk1::Clk1div14)
    }
    #[doc = "SYSPLLCLK1 is divided by 16"]
    #[inline(always)]
    pub fn clk1div16(self) -> &'a mut crate::W<REG> {
        self.variant(Rdivclk1::Clk1div16)
    }
    #[doc = "SYSPLLCLK1 is divided by 18"]
    #[inline(always)]
    pub fn clk1div18(self) -> &'a mut crate::W<REG> {
        self.variant(Rdivclk1::Clk1div18)
    }
    #[doc = "SYSPLLCLK1 is divided by 20"]
    #[inline(always)]
    pub fn clk1div20(self) -> &'a mut crate::W<REG> {
        self.variant(Rdivclk1::Clk1div20)
    }
    #[doc = "SYSPLLCLK1 is divided by 22"]
    #[inline(always)]
    pub fn clk1div22(self) -> &'a mut crate::W<REG> {
        self.variant(Rdivclk1::Clk1div22)
    }
    #[doc = "SYSPLLCLK1 is divided by 24"]
    #[inline(always)]
    pub fn clk1div24(self) -> &'a mut crate::W<REG> {
        self.variant(Rdivclk1::Clk1div24)
    }
    #[doc = "SYSPLLCLK1 is divided by 26"]
    #[inline(always)]
    pub fn clk1div26(self) -> &'a mut crate::W<REG> {
        self.variant(Rdivclk1::Clk1div26)
    }
    #[doc = "SYSPLLCLK1 is divided by 28"]
    #[inline(always)]
    pub fn clk1div28(self) -> &'a mut crate::W<REG> {
        self.variant(Rdivclk1::Clk1div28)
    }
    #[doc = "SYSPLLCLK1 is divided by 30"]
    #[inline(always)]
    pub fn clk1div30(self) -> &'a mut crate::W<REG> {
        self.variant(Rdivclk1::Clk1div30)
    }
    #[doc = "SYSPLLCLK1 is divided by 32"]
    #[inline(always)]
    pub fn clk1div32(self) -> &'a mut crate::W<REG> {
        self.variant(Rdivclk1::Clk1div32)
    }
}
impl R {
    #[doc = "Bit 0 - SYSPLLREF selects the system PLL (SYSPLL) reference clock source."]
    #[inline(always)]
    pub fn syspllref(&self) -> SyspllrefR {
        SyspllrefR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - ENABLECLK0 enables or disables the SYSPLLCLK0 output."]
    #[inline(always)]
    pub fn enableclk0(&self) -> Enableclk0R {
        Enableclk0R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ENABLECLK1 enables or disables the SYSPLLCLK1 output."]
    #[inline(always)]
    pub fn enableclk1(&self) -> Enableclk1R {
        Enableclk1R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 8:11 - RDIVCLK0 sets the final divider for the SYSPLLCLK0 output."]
    #[inline(always)]
    pub fn rdivclk0(&self) -> Rdivclk0R {
        Rdivclk0R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - RDIVCLK1 sets the final divider for the SYSPLLCLK1 output."]
    #[inline(always)]
    pub fn rdivclk1(&self) -> Rdivclk1R {
        Rdivclk1R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SYSPLLCFG0")
            .field("enableclk0", &self.enableclk0())
            .field("enableclk1", &self.enableclk1())
            .field("rdivclk1", &self.rdivclk1())
            .field("rdivclk0", &self.rdivclk0())
            .field("syspllref", &self.syspllref())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - SYSPLLREF selects the system PLL (SYSPLL) reference clock source."]
    #[inline(always)]
    pub fn syspllref(&mut self) -> SyspllrefW<'_, Syspllcfg0Spec> {
        SyspllrefW::new(self, 0)
    }
    #[doc = "Bit 4 - ENABLECLK0 enables or disables the SYSPLLCLK0 output."]
    #[inline(always)]
    pub fn enableclk0(&mut self) -> Enableclk0W<'_, Syspllcfg0Spec> {
        Enableclk0W::new(self, 4)
    }
    #[doc = "Bit 5 - ENABLECLK1 enables or disables the SYSPLLCLK1 output."]
    #[inline(always)]
    pub fn enableclk1(&mut self) -> Enableclk1W<'_, Syspllcfg0Spec> {
        Enableclk1W::new(self, 5)
    }
    #[doc = "Bits 8:11 - RDIVCLK0 sets the final divider for the SYSPLLCLK0 output."]
    #[inline(always)]
    pub fn rdivclk0(&mut self) -> Rdivclk0W<'_, Syspllcfg0Spec> {
        Rdivclk0W::new(self, 8)
    }
    #[doc = "Bits 12:15 - RDIVCLK1 sets the final divider for the SYSPLLCLK1 output."]
    #[inline(always)]
    pub fn rdivclk1(&mut self) -> Rdivclk1W<'_, Syspllcfg0Spec> {
        Rdivclk1W::new(self, 12)
    }
}
#[doc = "SYSPLL reference and output configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`syspllcfg0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syspllcfg0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Syspllcfg0Spec;
impl crate::RegisterSpec for Syspllcfg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syspllcfg0::R`](R) reader structure"]
impl crate::Readable for Syspllcfg0Spec {}
#[doc = "`write(|w| ..)` method takes [`syspllcfg0::W`](W) writer structure"]
impl crate::Writable for Syspllcfg0Spec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets SYSPLLCFG0 to value 0x70"]
impl crate::Resettable for Syspllcfg0Spec {
    const RESET_VALUE: u32 = 0x70;
}
