#[doc = "Register `CTL0` reader"]
pub type R = crate::R<Ctl0Spec>;
#[doc = "Register `CTL0` writer"]
pub type W = crate::W<Ctl0Spec>;
#[doc = "Input feature map type,\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ifmaptype {
    #[doc = "0: `0`"]
    Unsigned16bit = 0,
    #[doc = "1: Unsigned 8 bit data type"]
    Unsigned8bit = 1,
    #[doc = "2: Unsigned 4 bit data type"]
    Unsigned4bit = 2,
    #[doc = "3: Binary data type"]
    Binary = 3,
    #[doc = "8: `1000`"]
    Signed16bit = 8,
    #[doc = "9: Signed 8 bit data type"]
    Signed8bit = 9,
    #[doc = "10: Signed 4 bit data type"]
    Signed4bit = 10,
}
impl From<Ifmaptype> for u8 {
    #[inline(always)]
    fn from(variant: Ifmaptype) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ifmaptype {
    type Ux = u8;
}
impl crate::IsEnum for Ifmaptype {}
#[doc = "Field `IFMAPTYPE` reader - Input feature map type,"]
pub type IfmaptypeR = crate::FieldReader<Ifmaptype>;
impl IfmaptypeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ifmaptype> {
        match self.bits {
            0 => Some(Ifmaptype::Unsigned16bit),
            1 => Some(Ifmaptype::Unsigned8bit),
            2 => Some(Ifmaptype::Unsigned4bit),
            3 => Some(Ifmaptype::Binary),
            8 => Some(Ifmaptype::Signed16bit),
            9 => Some(Ifmaptype::Signed8bit),
            10 => Some(Ifmaptype::Signed4bit),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_unsigned_16bit(&self) -> bool {
        *self == Ifmaptype::Unsigned16bit
    }
    #[doc = "Unsigned 8 bit data type"]
    #[inline(always)]
    pub fn is_unsigned_8bit(&self) -> bool {
        *self == Ifmaptype::Unsigned8bit
    }
    #[doc = "Unsigned 4 bit data type"]
    #[inline(always)]
    pub fn is_unsigned_4bit(&self) -> bool {
        *self == Ifmaptype::Unsigned4bit
    }
    #[doc = "Binary data type"]
    #[inline(always)]
    pub fn is_binary(&self) -> bool {
        *self == Ifmaptype::Binary
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn is_signed_16bit(&self) -> bool {
        *self == Ifmaptype::Signed16bit
    }
    #[doc = "Signed 8 bit data type"]
    #[inline(always)]
    pub fn is_signed_8bit(&self) -> bool {
        *self == Ifmaptype::Signed8bit
    }
    #[doc = "Signed 4 bit data type"]
    #[inline(always)]
    pub fn is_signed_4bit(&self) -> bool {
        *self == Ifmaptype::Signed4bit
    }
}
#[doc = "Field `IFMAPTYPE` writer - Input feature map type,"]
pub type IfmaptypeW<'a, REG> = crate::FieldWriter<'a, REG, 4, Ifmaptype>;
impl<'a, REG> IfmaptypeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn unsigned_16bit(self) -> &'a mut crate::W<REG> {
        self.variant(Ifmaptype::Unsigned16bit)
    }
    #[doc = "Unsigned 8 bit data type"]
    #[inline(always)]
    pub fn unsigned_8bit(self) -> &'a mut crate::W<REG> {
        self.variant(Ifmaptype::Unsigned8bit)
    }
    #[doc = "Unsigned 4 bit data type"]
    #[inline(always)]
    pub fn unsigned_4bit(self) -> &'a mut crate::W<REG> {
        self.variant(Ifmaptype::Unsigned4bit)
    }
    #[doc = "Binary data type"]
    #[inline(always)]
    pub fn binary(self) -> &'a mut crate::W<REG> {
        self.variant(Ifmaptype::Binary)
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn signed_16bit(self) -> &'a mut crate::W<REG> {
        self.variant(Ifmaptype::Signed16bit)
    }
    #[doc = "Signed 8 bit data type"]
    #[inline(always)]
    pub fn signed_8bit(self) -> &'a mut crate::W<REG> {
        self.variant(Ifmaptype::Signed8bit)
    }
    #[doc = "Signed 4 bit data type"]
    #[inline(always)]
    pub fn signed_4bit(self) -> &'a mut crate::W<REG> {
        self.variant(Ifmaptype::Signed4bit)
    }
}
#[doc = "Output feature map type.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ofmaptype {
    #[doc = "0: `0`"]
    Unsigned16bit = 0,
    #[doc = "1: Unsigned 8 bit data type"]
    Unsigned8bit = 1,
    #[doc = "2: Unsigned 4 bit data type"]
    Unsigned4bit = 2,
    #[doc = "3: Binary data type"]
    Binary = 3,
    #[doc = "8: `1000`"]
    Signed16bit = 8,
    #[doc = "9: Signed 8 bit data type"]
    Signed8bit = 9,
    #[doc = "10: Signed 4 bit data type"]
    Signed4bit = 10,
}
impl From<Ofmaptype> for u8 {
    #[inline(always)]
    fn from(variant: Ofmaptype) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ofmaptype {
    type Ux = u8;
}
impl crate::IsEnum for Ofmaptype {}
#[doc = "Field `OFMAPTYPE` reader - Output feature map type."]
pub type OfmaptypeR = crate::FieldReader<Ofmaptype>;
impl OfmaptypeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ofmaptype> {
        match self.bits {
            0 => Some(Ofmaptype::Unsigned16bit),
            1 => Some(Ofmaptype::Unsigned8bit),
            2 => Some(Ofmaptype::Unsigned4bit),
            3 => Some(Ofmaptype::Binary),
            8 => Some(Ofmaptype::Signed16bit),
            9 => Some(Ofmaptype::Signed8bit),
            10 => Some(Ofmaptype::Signed4bit),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_unsigned_16bit(&self) -> bool {
        *self == Ofmaptype::Unsigned16bit
    }
    #[doc = "Unsigned 8 bit data type"]
    #[inline(always)]
    pub fn is_unsigned_8bit(&self) -> bool {
        *self == Ofmaptype::Unsigned8bit
    }
    #[doc = "Unsigned 4 bit data type"]
    #[inline(always)]
    pub fn is_unsigned_4bit(&self) -> bool {
        *self == Ofmaptype::Unsigned4bit
    }
    #[doc = "Binary data type"]
    #[inline(always)]
    pub fn is_binary(&self) -> bool {
        *self == Ofmaptype::Binary
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn is_signed_16bit(&self) -> bool {
        *self == Ofmaptype::Signed16bit
    }
    #[doc = "Signed 8 bit data type"]
    #[inline(always)]
    pub fn is_signed_8bit(&self) -> bool {
        *self == Ofmaptype::Signed8bit
    }
    #[doc = "Signed 4 bit data type"]
    #[inline(always)]
    pub fn is_signed_4bit(&self) -> bool {
        *self == Ofmaptype::Signed4bit
    }
}
#[doc = "Field `OFMAPTYPE` writer - Output feature map type."]
pub type OfmaptypeW<'a, REG> = crate::FieldWriter<'a, REG, 4, Ofmaptype>;
impl<'a, REG> OfmaptypeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn unsigned_16bit(self) -> &'a mut crate::W<REG> {
        self.variant(Ofmaptype::Unsigned16bit)
    }
    #[doc = "Unsigned 8 bit data type"]
    #[inline(always)]
    pub fn unsigned_8bit(self) -> &'a mut crate::W<REG> {
        self.variant(Ofmaptype::Unsigned8bit)
    }
    #[doc = "Unsigned 4 bit data type"]
    #[inline(always)]
    pub fn unsigned_4bit(self) -> &'a mut crate::W<REG> {
        self.variant(Ofmaptype::Unsigned4bit)
    }
    #[doc = "Binary data type"]
    #[inline(always)]
    pub fn binary(self) -> &'a mut crate::W<REG> {
        self.variant(Ofmaptype::Binary)
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn signed_16bit(self) -> &'a mut crate::W<REG> {
        self.variant(Ofmaptype::Signed16bit)
    }
    #[doc = "Signed 8 bit data type"]
    #[inline(always)]
    pub fn signed_8bit(self) -> &'a mut crate::W<REG> {
        self.variant(Ofmaptype::Signed8bit)
    }
    #[doc = "Signed 4 bit data type"]
    #[inline(always)]
    pub fn signed_4bit(self) -> &'a mut crate::W<REG> {
        self.variant(Ofmaptype::Signed4bit)
    }
}
#[doc = "Weight Type\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Wtype {
    #[doc = "0: Ternary weights: +1, -1, 0"]
    Ternary = 0,
    #[doc = "1: Binary data type, +1 or -1"]
    Binary = 1,
    #[doc = "2: Unsigned 8 bit data type"]
    Signed2bit = 2,
    #[doc = "3: Signed 4 bit data type"]
    Signed4bit = 3,
    #[doc = "4: Signed 8 bit data type"]
    Signed8bit = 4,
}
impl From<Wtype> for u8 {
    #[inline(always)]
    fn from(variant: Wtype) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Wtype {
    type Ux = u8;
}
impl crate::IsEnum for Wtype {}
#[doc = "Field `WTYPE` reader - Weight Type"]
pub type WtypeR = crate::FieldReader<Wtype>;
impl WtypeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Wtype> {
        match self.bits {
            0 => Some(Wtype::Ternary),
            1 => Some(Wtype::Binary),
            2 => Some(Wtype::Signed2bit),
            3 => Some(Wtype::Signed4bit),
            4 => Some(Wtype::Signed8bit),
            _ => None,
        }
    }
    #[doc = "Ternary weights: +1, -1, 0"]
    #[inline(always)]
    pub fn is_ternary(&self) -> bool {
        *self == Wtype::Ternary
    }
    #[doc = "Binary data type, +1 or -1"]
    #[inline(always)]
    pub fn is_binary(&self) -> bool {
        *self == Wtype::Binary
    }
    #[doc = "Unsigned 8 bit data type"]
    #[inline(always)]
    pub fn is_signed_2bit(&self) -> bool {
        *self == Wtype::Signed2bit
    }
    #[doc = "Signed 4 bit data type"]
    #[inline(always)]
    pub fn is_signed_4bit(&self) -> bool {
        *self == Wtype::Signed4bit
    }
    #[doc = "Signed 8 bit data type"]
    #[inline(always)]
    pub fn is_signed_8bit(&self) -> bool {
        *self == Wtype::Signed8bit
    }
}
#[doc = "Field `WTYPE` writer - Weight Type"]
pub type WtypeW<'a, REG> = crate::FieldWriter<'a, REG, 4, Wtype>;
impl<'a, REG> WtypeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Ternary weights: +1, -1, 0"]
    #[inline(always)]
    pub fn ternary(self) -> &'a mut crate::W<REG> {
        self.variant(Wtype::Ternary)
    }
    #[doc = "Binary data type, +1 or -1"]
    #[inline(always)]
    pub fn binary(self) -> &'a mut crate::W<REG> {
        self.variant(Wtype::Binary)
    }
    #[doc = "Unsigned 8 bit data type"]
    #[inline(always)]
    pub fn signed_2bit(self) -> &'a mut crate::W<REG> {
        self.variant(Wtype::Signed2bit)
    }
    #[doc = "Signed 4 bit data type"]
    #[inline(always)]
    pub fn signed_4bit(self) -> &'a mut crate::W<REG> {
        self.variant(Wtype::Signed4bit)
    }
    #[doc = "Signed 8 bit data type"]
    #[inline(always)]
    pub fn signed_8bit(self) -> &'a mut crate::W<REG> {
        self.variant(Wtype::Signed8bit)
    }
}
#[doc = "Layer type\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Layertype {
    #[doc = "0: `0`"]
    Fconv = 0,
    #[doc = "2: `10`"]
    Dw = 2,
    #[doc = "3: `11`"]
    Pw = 3,
    #[doc = "5: `101`"]
    Conv = 5,
    #[doc = "6: `110`"]
    Fc = 6,
    #[doc = "7: `111`"]
    Avgpool = 7,
    #[doc = "8: `1000`"]
    Maxpool = 8,
    #[doc = "16: `10000`"]
    FconvResidual = 16,
    #[doc = "18: `10010`"]
    DwResidual = 18,
    #[doc = "19: `10011`"]
    PwResidual = 19,
    #[doc = "21: `10101`"]
    ConvResidual = 21,
    #[doc = "22: `10110`"]
    FcResidual = 22,
    #[doc = "23: `10111`"]
    AvgpoolResidual = 23,
    #[doc = "24: `11000`"]
    MaxpoolResidual = 24,
}
impl From<Layertype> for u8 {
    #[inline(always)]
    fn from(variant: Layertype) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Layertype {
    type Ux = u8;
}
impl crate::IsEnum for Layertype {}
#[doc = "Field `LAYERTYPE` reader - Layer type"]
pub type LayertypeR = crate::FieldReader<Layertype>;
impl LayertypeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Layertype> {
        match self.bits {
            0 => Some(Layertype::Fconv),
            2 => Some(Layertype::Dw),
            3 => Some(Layertype::Pw),
            5 => Some(Layertype::Conv),
            6 => Some(Layertype::Fc),
            7 => Some(Layertype::Avgpool),
            8 => Some(Layertype::Maxpool),
            16 => Some(Layertype::FconvResidual),
            18 => Some(Layertype::DwResidual),
            19 => Some(Layertype::PwResidual),
            21 => Some(Layertype::ConvResidual),
            22 => Some(Layertype::FcResidual),
            23 => Some(Layertype::AvgpoolResidual),
            24 => Some(Layertype::MaxpoolResidual),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_fconv(&self) -> bool {
        *self == Layertype::Fconv
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_dw(&self) -> bool {
        *self == Layertype::Dw
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_pw(&self) -> bool {
        *self == Layertype::Pw
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn is_conv(&self) -> bool {
        *self == Layertype::Conv
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn is_fc(&self) -> bool {
        *self == Layertype::Fc
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn is_avgpool(&self) -> bool {
        *self == Layertype::Avgpool
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn is_maxpool(&self) -> bool {
        *self == Layertype::Maxpool
    }
    #[doc = "`10000`"]
    #[inline(always)]
    pub fn is_fconv_residual(&self) -> bool {
        *self == Layertype::FconvResidual
    }
    #[doc = "`10010`"]
    #[inline(always)]
    pub fn is_dw_residual(&self) -> bool {
        *self == Layertype::DwResidual
    }
    #[doc = "`10011`"]
    #[inline(always)]
    pub fn is_pw_residual(&self) -> bool {
        *self == Layertype::PwResidual
    }
    #[doc = "`10101`"]
    #[inline(always)]
    pub fn is_conv_residual(&self) -> bool {
        *self == Layertype::ConvResidual
    }
    #[doc = "`10110`"]
    #[inline(always)]
    pub fn is_fc_residual(&self) -> bool {
        *self == Layertype::FcResidual
    }
    #[doc = "`10111`"]
    #[inline(always)]
    pub fn is_avgpool_residual(&self) -> bool {
        *self == Layertype::AvgpoolResidual
    }
    #[doc = "`11000`"]
    #[inline(always)]
    pub fn is_maxpool_residual(&self) -> bool {
        *self == Layertype::MaxpoolResidual
    }
}
#[doc = "Field `LAYERTYPE` writer - Layer type"]
pub type LayertypeW<'a, REG> = crate::FieldWriter<'a, REG, 5, Layertype>;
impl<'a, REG> LayertypeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn fconv(self) -> &'a mut crate::W<REG> {
        self.variant(Layertype::Fconv)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn dw(self) -> &'a mut crate::W<REG> {
        self.variant(Layertype::Dw)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn pw(self) -> &'a mut crate::W<REG> {
        self.variant(Layertype::Pw)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn conv(self) -> &'a mut crate::W<REG> {
        self.variant(Layertype::Conv)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn fc(self) -> &'a mut crate::W<REG> {
        self.variant(Layertype::Fc)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn avgpool(self) -> &'a mut crate::W<REG> {
        self.variant(Layertype::Avgpool)
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn maxpool(self) -> &'a mut crate::W<REG> {
        self.variant(Layertype::Maxpool)
    }
    #[doc = "`10000`"]
    #[inline(always)]
    pub fn fconv_residual(self) -> &'a mut crate::W<REG> {
        self.variant(Layertype::FconvResidual)
    }
    #[doc = "`10010`"]
    #[inline(always)]
    pub fn dw_residual(self) -> &'a mut crate::W<REG> {
        self.variant(Layertype::DwResidual)
    }
    #[doc = "`10011`"]
    #[inline(always)]
    pub fn pw_residual(self) -> &'a mut crate::W<REG> {
        self.variant(Layertype::PwResidual)
    }
    #[doc = "`10101`"]
    #[inline(always)]
    pub fn conv_residual(self) -> &'a mut crate::W<REG> {
        self.variant(Layertype::ConvResidual)
    }
    #[doc = "`10110`"]
    #[inline(always)]
    pub fn fc_residual(self) -> &'a mut crate::W<REG> {
        self.variant(Layertype::FcResidual)
    }
    #[doc = "`10111`"]
    #[inline(always)]
    pub fn avgpool_residual(self) -> &'a mut crate::W<REG> {
        self.variant(Layertype::AvgpoolResidual)
    }
    #[doc = "`11000`"]
    #[inline(always)]
    pub fn maxpool_residual(self) -> &'a mut crate::W<REG> {
        self.variant(Layertype::MaxpoolResidual)
    }
}
#[doc = "Residual Input feature map type,\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Resifmaptype {
    #[doc = "0: `0`"]
    Unsigned16bit = 0,
    #[doc = "1: Unsigned 8 bit data type"]
    Unsigned8bit = 1,
    #[doc = "2: Unsigned 4 bit data type"]
    Unsigned4bit = 2,
    #[doc = "8: `1000`"]
    Signed16bit = 8,
    #[doc = "9: Signed 8 bit data type"]
    Signed8bit = 9,
    #[doc = "10: Signed 4 bit data type"]
    Signed4bit = 10,
}
impl From<Resifmaptype> for u8 {
    #[inline(always)]
    fn from(variant: Resifmaptype) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Resifmaptype {
    type Ux = u8;
}
impl crate::IsEnum for Resifmaptype {}
#[doc = "Field `RESIFMAPTYPE` reader - Residual Input feature map type,"]
pub type ResifmaptypeR = crate::FieldReader<Resifmaptype>;
impl ResifmaptypeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Resifmaptype> {
        match self.bits {
            0 => Some(Resifmaptype::Unsigned16bit),
            1 => Some(Resifmaptype::Unsigned8bit),
            2 => Some(Resifmaptype::Unsigned4bit),
            8 => Some(Resifmaptype::Signed16bit),
            9 => Some(Resifmaptype::Signed8bit),
            10 => Some(Resifmaptype::Signed4bit),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_unsigned_16bit(&self) -> bool {
        *self == Resifmaptype::Unsigned16bit
    }
    #[doc = "Unsigned 8 bit data type"]
    #[inline(always)]
    pub fn is_unsigned_8bit(&self) -> bool {
        *self == Resifmaptype::Unsigned8bit
    }
    #[doc = "Unsigned 4 bit data type"]
    #[inline(always)]
    pub fn is_unsigned_4bit(&self) -> bool {
        *self == Resifmaptype::Unsigned4bit
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn is_signed_16bit(&self) -> bool {
        *self == Resifmaptype::Signed16bit
    }
    #[doc = "Signed 8 bit data type"]
    #[inline(always)]
    pub fn is_signed_8bit(&self) -> bool {
        *self == Resifmaptype::Signed8bit
    }
    #[doc = "Signed 4 bit data type"]
    #[inline(always)]
    pub fn is_signed_4bit(&self) -> bool {
        *self == Resifmaptype::Signed4bit
    }
}
#[doc = "Field `RESIFMAPTYPE` writer - Residual Input feature map type,"]
pub type ResifmaptypeW<'a, REG> = crate::FieldWriter<'a, REG, 4, Resifmaptype>;
impl<'a, REG> ResifmaptypeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn unsigned_16bit(self) -> &'a mut crate::W<REG> {
        self.variant(Resifmaptype::Unsigned16bit)
    }
    #[doc = "Unsigned 8 bit data type"]
    #[inline(always)]
    pub fn unsigned_8bit(self) -> &'a mut crate::W<REG> {
        self.variant(Resifmaptype::Unsigned8bit)
    }
    #[doc = "Unsigned 4 bit data type"]
    #[inline(always)]
    pub fn unsigned_4bit(self) -> &'a mut crate::W<REG> {
        self.variant(Resifmaptype::Unsigned4bit)
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn signed_16bit(self) -> &'a mut crate::W<REG> {
        self.variant(Resifmaptype::Signed16bit)
    }
    #[doc = "Signed 8 bit data type"]
    #[inline(always)]
    pub fn signed_8bit(self) -> &'a mut crate::W<REG> {
        self.variant(Resifmaptype::Signed8bit)
    }
    #[doc = "Signed 4 bit data type"]
    #[inline(always)]
    pub fn signed_4bit(self) -> &'a mut crate::W<REG> {
        self.variant(Resifmaptype::Signed4bit)
    }
}
impl R {
    #[doc = "Bits 0:3 - Input feature map type,"]
    #[inline(always)]
    pub fn ifmaptype(&self) -> IfmaptypeR {
        IfmaptypeR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Output feature map type."]
    #[inline(always)]
    pub fn ofmaptype(&self) -> OfmaptypeR {
        OfmaptypeR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Weight Type"]
    #[inline(always)]
    pub fn wtype(&self) -> WtypeR {
        WtypeR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 20:24 - Layer type"]
    #[inline(always)]
    pub fn layertype(&self) -> LayertypeR {
        LayertypeR::new(((self.bits >> 20) & 0x1f) as u8)
    }
    #[doc = "Bits 26:29 - Residual Input feature map type,"]
    #[inline(always)]
    pub fn resifmaptype(&self) -> ResifmaptypeR {
        ResifmaptypeR::new(((self.bits >> 26) & 0x0f) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTL0")
            .field("wtype", &self.wtype())
            .field("ifmaptype", &self.ifmaptype())
            .field("ofmaptype", &self.ofmaptype())
            .field("layertype", &self.layertype())
            .field("resifmaptype", &self.resifmaptype())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - Input feature map type,"]
    #[inline(always)]
    pub fn ifmaptype(&mut self) -> IfmaptypeW<'_, Ctl0Spec> {
        IfmaptypeW::new(self, 0)
    }
    #[doc = "Bits 4:7 - Output feature map type."]
    #[inline(always)]
    pub fn ofmaptype(&mut self) -> OfmaptypeW<'_, Ctl0Spec> {
        OfmaptypeW::new(self, 4)
    }
    #[doc = "Bits 12:15 - Weight Type"]
    #[inline(always)]
    pub fn wtype(&mut self) -> WtypeW<'_, Ctl0Spec> {
        WtypeW::new(self, 12)
    }
    #[doc = "Bits 20:24 - Layer type"]
    #[inline(always)]
    pub fn layertype(&mut self) -> LayertypeW<'_, Ctl0Spec> {
        LayertypeW::new(self, 20)
    }
    #[doc = "Bits 26:29 - Residual Input feature map type,"]
    #[inline(always)]
    pub fn resifmaptype(&mut self) -> ResifmaptypeW<'_, Ctl0Spec> {
        ResifmaptypeW::new(self, 26)
    }
}
#[doc = "CTL0 register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ctl0Spec;
impl crate::RegisterSpec for Ctl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl0::R`](R) reader structure"]
impl crate::Readable for Ctl0Spec {}
#[doc = "`write(|w| ..)` method takes [`ctl0::W`](W) writer structure"]
impl crate::Writable for Ctl0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTL0 to value 0"]
impl crate::Resettable for Ctl0Spec {}
