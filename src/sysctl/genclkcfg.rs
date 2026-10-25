#[doc = "Register `GENCLKCFG` reader"]
pub type R = crate::R<GenclkcfgSpec>;
#[doc = "Register `GENCLKCFG` writer"]
pub type W = crate::W<GenclkcfgSpec>;
#[doc = "EXCLKSRC selects the source for the CLK_OUT external clock output block. ULPCLK and MFPCLK require the CLK_OUT divider (EXCLKDIVEN) to be enabled\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Exclksrc {
    #[doc = "0: CLK_OUT is SYSOSC"]
    Sysosc = 0,
    #[doc = "1: CLK_OUT is ULPCLK (EXCLKDIVEN must be enabled)"]
    Ulpclk = 1,
    #[doc = "2: CLK_OUT is LFCLK"]
    Lfclk = 2,
    #[doc = "3: CLK_OUT is MFPCLK (EXCLKDIVEN must be enabled)"]
    Mfpclk = 3,
    #[doc = "4: CLK_OUT is HFCLK"]
    Hfclk = 4,
    #[doc = "5: CLK_OUT is SYSPLLCLK1 (SYSPLLCLK1 must be <=48MHz)"]
    Syspllout1 = 5,
}
impl From<Exclksrc> for u8 {
    #[inline(always)]
    fn from(variant: Exclksrc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Exclksrc {
    type Ux = u8;
}
impl crate::IsEnum for Exclksrc {}
#[doc = "Field `EXCLKSRC` reader - EXCLKSRC selects the source for the CLK_OUT external clock output block. ULPCLK and MFPCLK require the CLK_OUT divider (EXCLKDIVEN) to be enabled"]
pub type ExclksrcR = crate::FieldReader<Exclksrc>;
impl ExclksrcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Exclksrc> {
        match self.bits {
            0 => Some(Exclksrc::Sysosc),
            1 => Some(Exclksrc::Ulpclk),
            2 => Some(Exclksrc::Lfclk),
            3 => Some(Exclksrc::Mfpclk),
            4 => Some(Exclksrc::Hfclk),
            5 => Some(Exclksrc::Syspllout1),
            _ => None,
        }
    }
    #[doc = "CLK_OUT is SYSOSC"]
    #[inline(always)]
    pub fn is_sysosc(&self) -> bool {
        *self == Exclksrc::Sysosc
    }
    #[doc = "CLK_OUT is ULPCLK (EXCLKDIVEN must be enabled)"]
    #[inline(always)]
    pub fn is_ulpclk(&self) -> bool {
        *self == Exclksrc::Ulpclk
    }
    #[doc = "CLK_OUT is LFCLK"]
    #[inline(always)]
    pub fn is_lfclk(&self) -> bool {
        *self == Exclksrc::Lfclk
    }
    #[doc = "CLK_OUT is MFPCLK (EXCLKDIVEN must be enabled)"]
    #[inline(always)]
    pub fn is_mfpclk(&self) -> bool {
        *self == Exclksrc::Mfpclk
    }
    #[doc = "CLK_OUT is HFCLK"]
    #[inline(always)]
    pub fn is_hfclk(&self) -> bool {
        *self == Exclksrc::Hfclk
    }
    #[doc = "CLK_OUT is SYSPLLCLK1 (SYSPLLCLK1 must be <=48MHz)"]
    #[inline(always)]
    pub fn is_syspllout1(&self) -> bool {
        *self == Exclksrc::Syspllout1
    }
}
#[doc = "Field `EXCLKSRC` writer - EXCLKSRC selects the source for the CLK_OUT external clock output block. ULPCLK and MFPCLK require the CLK_OUT divider (EXCLKDIVEN) to be enabled"]
pub type ExclksrcW<'a, REG> = crate::FieldWriter<'a, REG, 3, Exclksrc, crate::Safe>;
impl<'a, REG> ExclksrcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CLK_OUT is SYSOSC"]
    #[inline(always)]
    pub fn sysosc(self) -> &'a mut crate::W<REG> {
        self.variant(Exclksrc::Sysosc)
    }
    #[doc = "CLK_OUT is ULPCLK (EXCLKDIVEN must be enabled)"]
    #[inline(always)]
    pub fn ulpclk(self) -> &'a mut crate::W<REG> {
        self.variant(Exclksrc::Ulpclk)
    }
    #[doc = "CLK_OUT is LFCLK"]
    #[inline(always)]
    pub fn lfclk(self) -> &'a mut crate::W<REG> {
        self.variant(Exclksrc::Lfclk)
    }
    #[doc = "CLK_OUT is MFPCLK (EXCLKDIVEN must be enabled)"]
    #[inline(always)]
    pub fn mfpclk(self) -> &'a mut crate::W<REG> {
        self.variant(Exclksrc::Mfpclk)
    }
    #[doc = "CLK_OUT is HFCLK"]
    #[inline(always)]
    pub fn hfclk(self) -> &'a mut crate::W<REG> {
        self.variant(Exclksrc::Hfclk)
    }
    #[doc = "CLK_OUT is SYSPLLCLK1 (SYSPLLCLK1 must be <=48MHz)"]
    #[inline(always)]
    pub fn syspllout1(self) -> &'a mut crate::W<REG> {
        self.variant(Exclksrc::Syspllout1)
    }
}
#[doc = "EXCLKDIVVAL selects the divider value for the divider in the CLK_OUT external clock output block.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Exclkdivval {
    #[doc = "0: CLK_OUT source is divided by 2"]
    Div2 = 0,
    #[doc = "1: CLK_OUT source is divided by 4"]
    Div4 = 1,
    #[doc = "2: CLK_OUT source is divided by 6"]
    Div6 = 2,
    #[doc = "3: CLK_OUT source is divided by 8"]
    Div8 = 3,
    #[doc = "4: CLK_OUT source is divided by 10"]
    Div10 = 4,
    #[doc = "5: CLK_OUT source is divided by 12"]
    Div12 = 5,
    #[doc = "6: CLK_OUT source is divided by 14"]
    Div14 = 6,
    #[doc = "7: CLK_OUT source is divided by 16"]
    Div16 = 7,
}
impl From<Exclkdivval> for u8 {
    #[inline(always)]
    fn from(variant: Exclkdivval) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Exclkdivval {
    type Ux = u8;
}
impl crate::IsEnum for Exclkdivval {}
#[doc = "Field `EXCLKDIVVAL` reader - EXCLKDIVVAL selects the divider value for the divider in the CLK_OUT external clock output block."]
pub type ExclkdivvalR = crate::FieldReader<Exclkdivval>;
impl ExclkdivvalR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Exclkdivval {
        match self.bits {
            0 => Exclkdivval::Div2,
            1 => Exclkdivval::Div4,
            2 => Exclkdivval::Div6,
            3 => Exclkdivval::Div8,
            4 => Exclkdivval::Div10,
            5 => Exclkdivval::Div12,
            6 => Exclkdivval::Div14,
            7 => Exclkdivval::Div16,
            _ => unreachable!(),
        }
    }
    #[doc = "CLK_OUT source is divided by 2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == Exclkdivval::Div2
    }
    #[doc = "CLK_OUT source is divided by 4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == Exclkdivval::Div4
    }
    #[doc = "CLK_OUT source is divided by 6"]
    #[inline(always)]
    pub fn is_div6(&self) -> bool {
        *self == Exclkdivval::Div6
    }
    #[doc = "CLK_OUT source is divided by 8"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == Exclkdivval::Div8
    }
    #[doc = "CLK_OUT source is divided by 10"]
    #[inline(always)]
    pub fn is_div10(&self) -> bool {
        *self == Exclkdivval::Div10
    }
    #[doc = "CLK_OUT source is divided by 12"]
    #[inline(always)]
    pub fn is_div12(&self) -> bool {
        *self == Exclkdivval::Div12
    }
    #[doc = "CLK_OUT source is divided by 14"]
    #[inline(always)]
    pub fn is_div14(&self) -> bool {
        *self == Exclkdivval::Div14
    }
    #[doc = "CLK_OUT source is divided by 16"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == Exclkdivval::Div16
    }
}
#[doc = "Field `EXCLKDIVVAL` writer - EXCLKDIVVAL selects the divider value for the divider in the CLK_OUT external clock output block."]
pub type ExclkdivvalW<'a, REG> = crate::FieldWriter<'a, REG, 3, Exclkdivval, crate::Safe>;
impl<'a, REG> ExclkdivvalW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CLK_OUT source is divided by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(Exclkdivval::Div2)
    }
    #[doc = "CLK_OUT source is divided by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(Exclkdivval::Div4)
    }
    #[doc = "CLK_OUT source is divided by 6"]
    #[inline(always)]
    pub fn div6(self) -> &'a mut crate::W<REG> {
        self.variant(Exclkdivval::Div6)
    }
    #[doc = "CLK_OUT source is divided by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(Exclkdivval::Div8)
    }
    #[doc = "CLK_OUT source is divided by 10"]
    #[inline(always)]
    pub fn div10(self) -> &'a mut crate::W<REG> {
        self.variant(Exclkdivval::Div10)
    }
    #[doc = "CLK_OUT source is divided by 12"]
    #[inline(always)]
    pub fn div12(self) -> &'a mut crate::W<REG> {
        self.variant(Exclkdivval::Div12)
    }
    #[doc = "CLK_OUT source is divided by 14"]
    #[inline(always)]
    pub fn div14(self) -> &'a mut crate::W<REG> {
        self.variant(Exclkdivval::Div14)
    }
    #[doc = "CLK_OUT source is divided by 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(Exclkdivval::Div16)
    }
}
#[doc = "EXCLKDIVEN enables or disables the divider function of the CLK_OUT external clock output block.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Exclkdiven {
    #[doc = "0: CLock divider is disabled (passthrough, EXCLKDIVVAL is not applied)"]
    Passthru = 0,
    #[doc = "1: Clock divider is enabled (EXCLKDIVVAL is applied)"]
    Enable = 1,
}
impl From<Exclkdiven> for bool {
    #[inline(always)]
    fn from(variant: Exclkdiven) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EXCLKDIVEN` reader - EXCLKDIVEN enables or disables the divider function of the CLK_OUT external clock output block."]
pub type ExclkdivenR = crate::BitReader<Exclkdiven>;
impl ExclkdivenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Exclkdiven {
        match self.bits {
            false => Exclkdiven::Passthru,
            true => Exclkdiven::Enable,
        }
    }
    #[doc = "CLock divider is disabled (passthrough, EXCLKDIVVAL is not applied)"]
    #[inline(always)]
    pub fn is_passthru(&self) -> bool {
        *self == Exclkdiven::Passthru
    }
    #[doc = "Clock divider is enabled (EXCLKDIVVAL is applied)"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Exclkdiven::Enable
    }
}
#[doc = "Field `EXCLKDIVEN` writer - EXCLKDIVEN enables or disables the divider function of the CLK_OUT external clock output block."]
pub type ExclkdivenW<'a, REG> = crate::BitWriter<'a, REG, Exclkdiven>;
impl<'a, REG> ExclkdivenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLock divider is disabled (passthrough, EXCLKDIVVAL is not applied)"]
    #[inline(always)]
    pub fn passthru(self) -> &'a mut crate::W<REG> {
        self.variant(Exclkdiven::Passthru)
    }
    #[doc = "Clock divider is enabled (EXCLKDIVVAL is applied)"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Exclkdiven::Enable)
    }
}
#[doc = "CANCLKSRC selects the CANCLK source.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Canclksrc {
    #[doc = "0: CANCLK source is HFCLK"]
    Hfclk = 0,
    #[doc = "1: CANCLK source is SYSPLLCLK"]
    Syspllout = 1,
}
impl From<Canclksrc> for bool {
    #[inline(always)]
    fn from(variant: Canclksrc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CANCLKSRC` reader - CANCLKSRC selects the CANCLK source."]
pub type CanclksrcR = crate::BitReader<Canclksrc>;
impl CanclksrcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Canclksrc {
        match self.bits {
            false => Canclksrc::Hfclk,
            true => Canclksrc::Syspllout,
        }
    }
    #[doc = "CANCLK source is HFCLK"]
    #[inline(always)]
    pub fn is_hfclk(&self) -> bool {
        *self == Canclksrc::Hfclk
    }
    #[doc = "CANCLK source is SYSPLLCLK"]
    #[inline(always)]
    pub fn is_syspllout(&self) -> bool {
        *self == Canclksrc::Syspllout
    }
}
#[doc = "Field `CANCLKSRC` writer - CANCLKSRC selects the CANCLK source."]
pub type CanclksrcW<'a, REG> = crate::BitWriter<'a, REG, Canclksrc>;
impl<'a, REG> CanclksrcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CANCLK source is HFCLK"]
    #[inline(always)]
    pub fn hfclk(self) -> &'a mut crate::W<REG> {
        self.variant(Canclksrc::Hfclk)
    }
    #[doc = "CANCLK source is SYSPLLCLK"]
    #[inline(always)]
    pub fn syspllout(self) -> &'a mut crate::W<REG> {
        self.variant(Canclksrc::Syspllout)
    }
}
#[doc = "FCCSELCLK selectes the frequency clock counter (FCC) clock source.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fccselclk {
    #[doc = "0: FCC clock is MCLK/4"]
    Mclkby4 = 0,
    #[doc = "1: FCC clock is SYSOSC"]
    Sysosc = 1,
    #[doc = "2: FCC clock is HFCLK"]
    Hfclk = 2,
    #[doc = "3: FCC clock is the CLK_OUT selection"]
    Extclk = 3,
    #[doc = "4: FCC clock is SYSPLLCLK0"]
    Syspllclk0 = 4,
    #[doc = "5: FCC clock is SYSPLLCLK1"]
    Syspllclk1 = 5,
    #[doc = "7: FCC clock is the FCCIN external input"]
    Fccin = 7,
}
impl From<Fccselclk> for u8 {
    #[inline(always)]
    fn from(variant: Fccselclk) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fccselclk {
    type Ux = u8;
}
impl crate::IsEnum for Fccselclk {}
#[doc = "Field `FCCSELCLK` reader - FCCSELCLK selectes the frequency clock counter (FCC) clock source."]
pub type FccselclkR = crate::FieldReader<Fccselclk>;
impl FccselclkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Fccselclk> {
        match self.bits {
            0 => Some(Fccselclk::Mclkby4),
            1 => Some(Fccselclk::Sysosc),
            2 => Some(Fccselclk::Hfclk),
            3 => Some(Fccselclk::Extclk),
            4 => Some(Fccselclk::Syspllclk0),
            5 => Some(Fccselclk::Syspllclk1),
            7 => Some(Fccselclk::Fccin),
            _ => None,
        }
    }
    #[doc = "FCC clock is MCLK/4"]
    #[inline(always)]
    pub fn is_mclkby4(&self) -> bool {
        *self == Fccselclk::Mclkby4
    }
    #[doc = "FCC clock is SYSOSC"]
    #[inline(always)]
    pub fn is_sysosc(&self) -> bool {
        *self == Fccselclk::Sysosc
    }
    #[doc = "FCC clock is HFCLK"]
    #[inline(always)]
    pub fn is_hfclk(&self) -> bool {
        *self == Fccselclk::Hfclk
    }
    #[doc = "FCC clock is the CLK_OUT selection"]
    #[inline(always)]
    pub fn is_extclk(&self) -> bool {
        *self == Fccselclk::Extclk
    }
    #[doc = "FCC clock is SYSPLLCLK0"]
    #[inline(always)]
    pub fn is_syspllclk0(&self) -> bool {
        *self == Fccselclk::Syspllclk0
    }
    #[doc = "FCC clock is SYSPLLCLK1"]
    #[inline(always)]
    pub fn is_syspllclk1(&self) -> bool {
        *self == Fccselclk::Syspllclk1
    }
    #[doc = "FCC clock is the FCCIN external input"]
    #[inline(always)]
    pub fn is_fccin(&self) -> bool {
        *self == Fccselclk::Fccin
    }
}
#[doc = "Field `FCCSELCLK` writer - FCCSELCLK selectes the frequency clock counter (FCC) clock source."]
pub type FccselclkW<'a, REG> = crate::FieldWriter<'a, REG, 4, Fccselclk, crate::Safe>;
impl<'a, REG> FccselclkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "FCC clock is MCLK/4"]
    #[inline(always)]
    pub fn mclkby4(self) -> &'a mut crate::W<REG> {
        self.variant(Fccselclk::Mclkby4)
    }
    #[doc = "FCC clock is SYSOSC"]
    #[inline(always)]
    pub fn sysosc(self) -> &'a mut crate::W<REG> {
        self.variant(Fccselclk::Sysosc)
    }
    #[doc = "FCC clock is HFCLK"]
    #[inline(always)]
    pub fn hfclk(self) -> &'a mut crate::W<REG> {
        self.variant(Fccselclk::Hfclk)
    }
    #[doc = "FCC clock is the CLK_OUT selection"]
    #[inline(always)]
    pub fn extclk(self) -> &'a mut crate::W<REG> {
        self.variant(Fccselclk::Extclk)
    }
    #[doc = "FCC clock is SYSPLLCLK0"]
    #[inline(always)]
    pub fn syspllclk0(self) -> &'a mut crate::W<REG> {
        self.variant(Fccselclk::Syspllclk0)
    }
    #[doc = "FCC clock is SYSPLLCLK1"]
    #[inline(always)]
    pub fn syspllclk1(self) -> &'a mut crate::W<REG> {
        self.variant(Fccselclk::Syspllclk1)
    }
    #[doc = "FCC clock is the FCCIN external input"]
    #[inline(always)]
    pub fn fccin(self) -> &'a mut crate::W<REG> {
        self.variant(Fccselclk::Fccin)
    }
}
#[doc = "FCCTRIGSRC selects the frequency clock counter (FCC) trigger source.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fcctrigsrc {
    #[doc = "0: FCC trigger is the external pin"]
    Extpin = 0,
    #[doc = "1: FCC trigger is the LFCLK"]
    Lfclk = 1,
}
impl From<Fcctrigsrc> for bool {
    #[inline(always)]
    fn from(variant: Fcctrigsrc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FCCTRIGSRC` reader - FCCTRIGSRC selects the frequency clock counter (FCC) trigger source."]
pub type FcctrigsrcR = crate::BitReader<Fcctrigsrc>;
impl FcctrigsrcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fcctrigsrc {
        match self.bits {
            false => Fcctrigsrc::Extpin,
            true => Fcctrigsrc::Lfclk,
        }
    }
    #[doc = "FCC trigger is the external pin"]
    #[inline(always)]
    pub fn is_extpin(&self) -> bool {
        *self == Fcctrigsrc::Extpin
    }
    #[doc = "FCC trigger is the LFCLK"]
    #[inline(always)]
    pub fn is_lfclk(&self) -> bool {
        *self == Fcctrigsrc::Lfclk
    }
}
#[doc = "Field `FCCTRIGSRC` writer - FCCTRIGSRC selects the frequency clock counter (FCC) trigger source."]
pub type FcctrigsrcW<'a, REG> = crate::BitWriter<'a, REG, Fcctrigsrc>;
impl<'a, REG> FcctrigsrcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "FCC trigger is the external pin"]
    #[inline(always)]
    pub fn extpin(self) -> &'a mut crate::W<REG> {
        self.variant(Fcctrigsrc::Extpin)
    }
    #[doc = "FCC trigger is the LFCLK"]
    #[inline(always)]
    pub fn lfclk(self) -> &'a mut crate::W<REG> {
        self.variant(Fcctrigsrc::Lfclk)
    }
}
#[doc = "FCCLVLTRIG selects the frequency clock counter (FCC) trigger mode.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fcclvltrig {
    #[doc = "0: Rising edge to rising edge triggered"]
    Rise2rise = 0,
    #[doc = "1: Level triggered"]
    Level = 1,
}
impl From<Fcclvltrig> for bool {
    #[inline(always)]
    fn from(variant: Fcclvltrig) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FCCLVLTRIG` reader - FCCLVLTRIG selects the frequency clock counter (FCC) trigger mode."]
pub type FcclvltrigR = crate::BitReader<Fcclvltrig>;
impl FcclvltrigR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fcclvltrig {
        match self.bits {
            false => Fcclvltrig::Rise2rise,
            true => Fcclvltrig::Level,
        }
    }
    #[doc = "Rising edge to rising edge triggered"]
    #[inline(always)]
    pub fn is_rise2rise(&self) -> bool {
        *self == Fcclvltrig::Rise2rise
    }
    #[doc = "Level triggered"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Fcclvltrig::Level
    }
}
#[doc = "Field `FCCLVLTRIG` writer - FCCLVLTRIG selects the frequency clock counter (FCC) trigger mode."]
pub type FcclvltrigW<'a, REG> = crate::BitWriter<'a, REG, Fcclvltrig>;
impl<'a, REG> FcclvltrigW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Rising edge to rising edge triggered"]
    #[inline(always)]
    pub fn rise2rise(self) -> &'a mut crate::W<REG> {
        self.variant(Fcclvltrig::Rise2rise)
    }
    #[doc = "Level triggered"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Fcclvltrig::Level)
    }
}
#[doc = "Field `FCCTRIGCNT` reader - FCCTRIGCNT specifies the number of trigger clock periods in the trigger window. FCCTRIGCNT=0h (one trigger clock period) up to 1Fh (32 trigger clock periods) may be specified."]
pub type FcctrigcntR = crate::FieldReader;
#[doc = "Field `FCCTRIGCNT` writer - FCCTRIGCNT specifies the number of trigger clock periods in the trigger window. FCCTRIGCNT=0h (one trigger clock period) up to 1Fh (32 trigger clock periods) may be specified."]
pub type FcctrigcntW<'a, REG> = crate::FieldWriter<'a, REG, 5, u8, crate::Safe>;
impl R {
    #[doc = "Bits 0:2 - EXCLKSRC selects the source for the CLK_OUT external clock output block. ULPCLK and MFPCLK require the CLK_OUT divider (EXCLKDIVEN) to be enabled"]
    #[inline(always)]
    pub fn exclksrc(&self) -> ExclksrcR {
        ExclksrcR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - EXCLKDIVVAL selects the divider value for the divider in the CLK_OUT external clock output block."]
    #[inline(always)]
    pub fn exclkdivval(&self) -> ExclkdivvalR {
        ExclkdivvalR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - EXCLKDIVEN enables or disables the divider function of the CLK_OUT external clock output block."]
    #[inline(always)]
    pub fn exclkdiven(&self) -> ExclkdivenR {
        ExclkdivenR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - CANCLKSRC selects the CANCLK source."]
    #[inline(always)]
    pub fn canclksrc(&self) -> CanclksrcR {
        CanclksrcR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:19 - FCCSELCLK selectes the frequency clock counter (FCC) clock source."]
    #[inline(always)]
    pub fn fccselclk(&self) -> FccselclkR {
        FccselclkR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 20 - FCCTRIGSRC selects the frequency clock counter (FCC) trigger source."]
    #[inline(always)]
    pub fn fcctrigsrc(&self) -> FcctrigsrcR {
        FcctrigsrcR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - FCCLVLTRIG selects the frequency clock counter (FCC) trigger mode."]
    #[inline(always)]
    pub fn fcclvltrig(&self) -> FcclvltrigR {
        FcclvltrigR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 24:28 - FCCTRIGCNT specifies the number of trigger clock periods in the trigger window. FCCTRIGCNT=0h (one trigger clock period) up to 1Fh (32 trigger clock periods) may be specified."]
    #[inline(always)]
    pub fn fcctrigcnt(&self) -> FcctrigcntR {
        FcctrigcntR::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GENCLKCFG")
            .field("canclksrc", &self.canclksrc())
            .field("fcctrigcnt", &self.fcctrigcnt())
            .field("fccselclk", &self.fccselclk())
            .field("fcctrigsrc", &self.fcctrigsrc())
            .field("fcclvltrig", &self.fcclvltrig())
            .field("exclksrc", &self.exclksrc())
            .field("exclkdivval", &self.exclkdivval())
            .field("exclkdiven", &self.exclkdiven())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - EXCLKSRC selects the source for the CLK_OUT external clock output block. ULPCLK and MFPCLK require the CLK_OUT divider (EXCLKDIVEN) to be enabled"]
    #[inline(always)]
    pub fn exclksrc(&mut self) -> ExclksrcW<'_, GenclkcfgSpec> {
        ExclksrcW::new(self, 0)
    }
    #[doc = "Bits 4:6 - EXCLKDIVVAL selects the divider value for the divider in the CLK_OUT external clock output block."]
    #[inline(always)]
    pub fn exclkdivval(&mut self) -> ExclkdivvalW<'_, GenclkcfgSpec> {
        ExclkdivvalW::new(self, 4)
    }
    #[doc = "Bit 7 - EXCLKDIVEN enables or disables the divider function of the CLK_OUT external clock output block."]
    #[inline(always)]
    pub fn exclkdiven(&mut self) -> ExclkdivenW<'_, GenclkcfgSpec> {
        ExclkdivenW::new(self, 7)
    }
    #[doc = "Bit 8 - CANCLKSRC selects the CANCLK source."]
    #[inline(always)]
    pub fn canclksrc(&mut self) -> CanclksrcW<'_, GenclkcfgSpec> {
        CanclksrcW::new(self, 8)
    }
    #[doc = "Bits 16:19 - FCCSELCLK selectes the frequency clock counter (FCC) clock source."]
    #[inline(always)]
    pub fn fccselclk(&mut self) -> FccselclkW<'_, GenclkcfgSpec> {
        FccselclkW::new(self, 16)
    }
    #[doc = "Bit 20 - FCCTRIGSRC selects the frequency clock counter (FCC) trigger source."]
    #[inline(always)]
    pub fn fcctrigsrc(&mut self) -> FcctrigsrcW<'_, GenclkcfgSpec> {
        FcctrigsrcW::new(self, 20)
    }
    #[doc = "Bit 21 - FCCLVLTRIG selects the frequency clock counter (FCC) trigger mode."]
    #[inline(always)]
    pub fn fcclvltrig(&mut self) -> FcclvltrigW<'_, GenclkcfgSpec> {
        FcclvltrigW::new(self, 21)
    }
    #[doc = "Bits 24:28 - FCCTRIGCNT specifies the number of trigger clock periods in the trigger window. FCCTRIGCNT=0h (one trigger clock period) up to 1Fh (32 trigger clock periods) may be specified."]
    #[inline(always)]
    pub fn fcctrigcnt(&mut self) -> FcctrigcntW<'_, GenclkcfgSpec> {
        FcctrigcntW::new(self, 24)
    }
}
#[doc = "General clock configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`genclkcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`genclkcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GenclkcfgSpec;
impl crate::RegisterSpec for GenclkcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`genclkcfg::R`](R) reader structure"]
impl crate::Readable for GenclkcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`genclkcfg::W`](W) writer structure"]
impl crate::Writable for GenclkcfgSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets GENCLKCFG to value 0"]
impl crate::Resettable for GenclkcfgSpec {}
