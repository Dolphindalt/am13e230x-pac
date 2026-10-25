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
    Dio0 = 1,
    #[doc = "2: DIO1 interrupt"]
    Dio1 = 2,
    #[doc = "3: DIO2 interrupt"]
    Dio2 = 3,
    #[doc = "4: DIO3 interrupt"]
    Dio3 = 4,
    #[doc = "5: DIO4 interrupt"]
    Dio4 = 5,
    #[doc = "6: DIO5 interrupt"]
    Dio5 = 6,
    #[doc = "7: DIO6 interrupt"]
    Dio6 = 7,
    #[doc = "8: DIO7 interrupt"]
    Dio7 = 8,
    #[doc = "9: DIO8 interrupt"]
    Dio8 = 9,
    #[doc = "10: DIO9 interrupt"]
    Dio9 = 10,
    #[doc = "11: DIO10 interrupt"]
    Dio10 = 11,
    #[doc = "12: DIO11 interrupt"]
    Dio11 = 12,
    #[doc = "13: DIO12 interrupt"]
    Dio12 = 13,
    #[doc = "14: DIO13 interrupt"]
    Dio13 = 14,
    #[doc = "15: DIO14 interrupt"]
    Dio14 = 15,
    #[doc = "16: DIO15 interrupt"]
    Dio15 = 16,
    #[doc = "17: DIO16 interrupt"]
    Dio16 = 17,
    #[doc = "18: DIO17 interrupt"]
    Dio17 = 18,
    #[doc = "19: DIO18 interrupt"]
    Dio18 = 19,
    #[doc = "20: DIO19 interrupt"]
    Dio19 = 20,
    #[doc = "21: DIO20 interrupt"]
    Dio20 = 21,
    #[doc = "22: DIO21 interrupt"]
    Dio21 = 22,
    #[doc = "23: DIO22 interrupt"]
    Dio22 = 23,
    #[doc = "24: DIO23 interrupt"]
    Dio23 = 24,
    #[doc = "25: DIO24 interrupt"]
    Dio24 = 25,
    #[doc = "26: DIO25 interrupt"]
    Dio25 = 26,
    #[doc = "27: DIO26 interrupt"]
    Dio26 = 27,
    #[doc = "28: DIO27 interrupt"]
    Dio27 = 28,
    #[doc = "29: DIO28 interrupt"]
    Dio28 = 29,
    #[doc = "30: DIO29 interrupt"]
    Dio29 = 30,
    #[doc = "31: DIO30 interrupt"]
    Dio30 = 31,
    #[doc = "32: DIO31 interrupt"]
    Dio31 = 32,
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
            1 => Some(Stat::Dio0),
            2 => Some(Stat::Dio1),
            3 => Some(Stat::Dio2),
            4 => Some(Stat::Dio3),
            5 => Some(Stat::Dio4),
            6 => Some(Stat::Dio5),
            7 => Some(Stat::Dio6),
            8 => Some(Stat::Dio7),
            9 => Some(Stat::Dio8),
            10 => Some(Stat::Dio9),
            11 => Some(Stat::Dio10),
            12 => Some(Stat::Dio11),
            13 => Some(Stat::Dio12),
            14 => Some(Stat::Dio13),
            15 => Some(Stat::Dio14),
            16 => Some(Stat::Dio15),
            17 => Some(Stat::Dio16),
            18 => Some(Stat::Dio17),
            19 => Some(Stat::Dio18),
            20 => Some(Stat::Dio19),
            21 => Some(Stat::Dio20),
            22 => Some(Stat::Dio21),
            23 => Some(Stat::Dio22),
            24 => Some(Stat::Dio23),
            25 => Some(Stat::Dio24),
            26 => Some(Stat::Dio25),
            27 => Some(Stat::Dio26),
            28 => Some(Stat::Dio27),
            29 => Some(Stat::Dio28),
            30 => Some(Stat::Dio29),
            31 => Some(Stat::Dio30),
            32 => Some(Stat::Dio31),
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
    pub fn is_dio0(&self) -> bool {
        *self == Stat::Dio0
    }
    #[doc = "DIO1 interrupt"]
    #[inline(always)]
    pub fn is_dio1(&self) -> bool {
        *self == Stat::Dio1
    }
    #[doc = "DIO2 interrupt"]
    #[inline(always)]
    pub fn is_dio2(&self) -> bool {
        *self == Stat::Dio2
    }
    #[doc = "DIO3 interrupt"]
    #[inline(always)]
    pub fn is_dio3(&self) -> bool {
        *self == Stat::Dio3
    }
    #[doc = "DIO4 interrupt"]
    #[inline(always)]
    pub fn is_dio4(&self) -> bool {
        *self == Stat::Dio4
    }
    #[doc = "DIO5 interrupt"]
    #[inline(always)]
    pub fn is_dio5(&self) -> bool {
        *self == Stat::Dio5
    }
    #[doc = "DIO6 interrupt"]
    #[inline(always)]
    pub fn is_dio6(&self) -> bool {
        *self == Stat::Dio6
    }
    #[doc = "DIO7 interrupt"]
    #[inline(always)]
    pub fn is_dio7(&self) -> bool {
        *self == Stat::Dio7
    }
    #[doc = "DIO8 interrupt"]
    #[inline(always)]
    pub fn is_dio8(&self) -> bool {
        *self == Stat::Dio8
    }
    #[doc = "DIO9 interrupt"]
    #[inline(always)]
    pub fn is_dio9(&self) -> bool {
        *self == Stat::Dio9
    }
    #[doc = "DIO10 interrupt"]
    #[inline(always)]
    pub fn is_dio10(&self) -> bool {
        *self == Stat::Dio10
    }
    #[doc = "DIO11 interrupt"]
    #[inline(always)]
    pub fn is_dio11(&self) -> bool {
        *self == Stat::Dio11
    }
    #[doc = "DIO12 interrupt"]
    #[inline(always)]
    pub fn is_dio12(&self) -> bool {
        *self == Stat::Dio12
    }
    #[doc = "DIO13 interrupt"]
    #[inline(always)]
    pub fn is_dio13(&self) -> bool {
        *self == Stat::Dio13
    }
    #[doc = "DIO14 interrupt"]
    #[inline(always)]
    pub fn is_dio14(&self) -> bool {
        *self == Stat::Dio14
    }
    #[doc = "DIO15 interrupt"]
    #[inline(always)]
    pub fn is_dio15(&self) -> bool {
        *self == Stat::Dio15
    }
    #[doc = "DIO16 interrupt"]
    #[inline(always)]
    pub fn is_dio16(&self) -> bool {
        *self == Stat::Dio16
    }
    #[doc = "DIO17 interrupt"]
    #[inline(always)]
    pub fn is_dio17(&self) -> bool {
        *self == Stat::Dio17
    }
    #[doc = "DIO18 interrupt"]
    #[inline(always)]
    pub fn is_dio18(&self) -> bool {
        *self == Stat::Dio18
    }
    #[doc = "DIO19 interrupt"]
    #[inline(always)]
    pub fn is_dio19(&self) -> bool {
        *self == Stat::Dio19
    }
    #[doc = "DIO20 interrupt"]
    #[inline(always)]
    pub fn is_dio20(&self) -> bool {
        *self == Stat::Dio20
    }
    #[doc = "DIO21 interrupt"]
    #[inline(always)]
    pub fn is_dio21(&self) -> bool {
        *self == Stat::Dio21
    }
    #[doc = "DIO22 interrupt"]
    #[inline(always)]
    pub fn is_dio22(&self) -> bool {
        *self == Stat::Dio22
    }
    #[doc = "DIO23 interrupt"]
    #[inline(always)]
    pub fn is_dio23(&self) -> bool {
        *self == Stat::Dio23
    }
    #[doc = "DIO24 interrupt"]
    #[inline(always)]
    pub fn is_dio24(&self) -> bool {
        *self == Stat::Dio24
    }
    #[doc = "DIO25 interrupt"]
    #[inline(always)]
    pub fn is_dio25(&self) -> bool {
        *self == Stat::Dio25
    }
    #[doc = "DIO26 interrupt"]
    #[inline(always)]
    pub fn is_dio26(&self) -> bool {
        *self == Stat::Dio26
    }
    #[doc = "DIO27 interrupt"]
    #[inline(always)]
    pub fn is_dio27(&self) -> bool {
        *self == Stat::Dio27
    }
    #[doc = "DIO28 interrupt"]
    #[inline(always)]
    pub fn is_dio28(&self) -> bool {
        *self == Stat::Dio28
    }
    #[doc = "DIO29 interrupt"]
    #[inline(always)]
    pub fn is_dio29(&self) -> bool {
        *self == Stat::Dio29
    }
    #[doc = "DIO30 interrupt"]
    #[inline(always)]
    pub fn is_dio30(&self) -> bool {
        *self == Stat::Dio30
    }
    #[doc = "DIO31 interrupt"]
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
