#[doc = "Register `IIDX` reader"]
pub type R = crate::R<IidxSpec>;
#[doc = "Interrupt index status\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Stat {
    #[doc = "0: No bit is set means there is no pending interrupt request"]
    NoIntr = 0,
    #[doc = "1: DIO0 interrupt"]
    Dio16 = 1,
    #[doc = "2: DIO1 interrupt"]
    Dio17 = 2,
    #[doc = "3: DIO2 interrupt"]
    Dio18 = 3,
    #[doc = "4: DIO3 interrupt"]
    Dio19 = 4,
    #[doc = "5: DIO4 interrupt"]
    Dio20 = 5,
    #[doc = "6: DIO5 interrupt"]
    Dio21 = 6,
    #[doc = "7: DIO6 interrupt"]
    Dio22 = 7,
    #[doc = "8: DIO7 interrupt"]
    Dio23 = 8,
    #[doc = "9: DIO8 interrupt"]
    Dio24 = 9,
    #[doc = "10: DIO9 interrupt"]
    Dio25 = 10,
    #[doc = "11: DIO10 interrupt"]
    Dio26 = 11,
    #[doc = "12: DIO11 interrupt"]
    Dio27 = 12,
    #[doc = "13: DIO12 interrupt"]
    Dio28 = 13,
    #[doc = "14: DIO13 interrupt"]
    Dio29 = 14,
    #[doc = "15: DIO14 interrupt"]
    Dio30 = 15,
    #[doc = "16: DIO15 interrupt"]
    Dio31 = 16,
}
impl From<Stat> for u8 {
    #[inline(always)]
    fn from(variant: Stat) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Stat {
    type Ux = u8;
}
impl crate::IsEnum for Stat {}
#[doc = "Field `STAT` reader - Interrupt index status"]
pub type StatR = crate::FieldReader<Stat>;
impl StatR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Stat> {
        match self.bits {
            0 => Some(Stat::NoIntr),
            1 => Some(Stat::Dio16),
            2 => Some(Stat::Dio17),
            3 => Some(Stat::Dio18),
            4 => Some(Stat::Dio19),
            5 => Some(Stat::Dio20),
            6 => Some(Stat::Dio21),
            7 => Some(Stat::Dio22),
            8 => Some(Stat::Dio23),
            9 => Some(Stat::Dio24),
            10 => Some(Stat::Dio25),
            11 => Some(Stat::Dio26),
            12 => Some(Stat::Dio27),
            13 => Some(Stat::Dio28),
            14 => Some(Stat::Dio29),
            15 => Some(Stat::Dio30),
            16 => Some(Stat::Dio31),
            _ => None,
        }
    }
    #[doc = "No bit is set means there is no pending interrupt request"]
    #[inline(always)]
    pub fn is_no_intr(&self) -> bool {
        *self == Stat::NoIntr
    }
    #[doc = "DIO0 interrupt"]
    #[inline(always)]
    pub fn is_dio16(&self) -> bool {
        *self == Stat::Dio16
    }
    #[doc = "DIO1 interrupt"]
    #[inline(always)]
    pub fn is_dio17(&self) -> bool {
        *self == Stat::Dio17
    }
    #[doc = "DIO2 interrupt"]
    #[inline(always)]
    pub fn is_dio18(&self) -> bool {
        *self == Stat::Dio18
    }
    #[doc = "DIO3 interrupt"]
    #[inline(always)]
    pub fn is_dio19(&self) -> bool {
        *self == Stat::Dio19
    }
    #[doc = "DIO4 interrupt"]
    #[inline(always)]
    pub fn is_dio20(&self) -> bool {
        *self == Stat::Dio20
    }
    #[doc = "DIO5 interrupt"]
    #[inline(always)]
    pub fn is_dio21(&self) -> bool {
        *self == Stat::Dio21
    }
    #[doc = "DIO6 interrupt"]
    #[inline(always)]
    pub fn is_dio22(&self) -> bool {
        *self == Stat::Dio22
    }
    #[doc = "DIO7 interrupt"]
    #[inline(always)]
    pub fn is_dio23(&self) -> bool {
        *self == Stat::Dio23
    }
    #[doc = "DIO8 interrupt"]
    #[inline(always)]
    pub fn is_dio24(&self) -> bool {
        *self == Stat::Dio24
    }
    #[doc = "DIO9 interrupt"]
    #[inline(always)]
    pub fn is_dio25(&self) -> bool {
        *self == Stat::Dio25
    }
    #[doc = "DIO10 interrupt"]
    #[inline(always)]
    pub fn is_dio26(&self) -> bool {
        *self == Stat::Dio26
    }
    #[doc = "DIO11 interrupt"]
    #[inline(always)]
    pub fn is_dio27(&self) -> bool {
        *self == Stat::Dio27
    }
    #[doc = "DIO12 interrupt"]
    #[inline(always)]
    pub fn is_dio28(&self) -> bool {
        *self == Stat::Dio28
    }
    #[doc = "DIO13 interrupt"]
    #[inline(always)]
    pub fn is_dio29(&self) -> bool {
        *self == Stat::Dio29
    }
    #[doc = "DIO14 interrupt"]
    #[inline(always)]
    pub fn is_dio30(&self) -> bool {
        *self == Stat::Dio30
    }
    #[doc = "DIO15 interrupt"]
    #[inline(always)]
    pub fn is_dio31(&self) -> bool {
        *self == Stat::Dio31
    }
}
impl R {
    #[doc = "Bits 0:7 - Interrupt index status"]
    #[inline(always)]
    pub fn stat(&self) -> StatR {
        StatR::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IIDX").field("stat", &self.stat()).finish()
    }
}
#[doc = "Interrupt index\n\nYou can [`read`](crate::Reg::read) this register and get [`iidx::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IidxSpec;
impl crate::RegisterSpec for IidxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iidx::R`](R) reader structure"]
impl crate::Readable for IidxSpec {}
#[doc = "`reset()` method sets IIDX to value 0"]
impl crate::Resettable for IidxSpec {}
