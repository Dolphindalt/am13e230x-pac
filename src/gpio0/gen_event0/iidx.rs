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
