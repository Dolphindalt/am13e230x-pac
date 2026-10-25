#[doc = "Register `IIDX` reader"]
pub type R = crate::R<IidxSpec>;
#[doc = "Interrupt index status\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Stat {
    #[doc = "0: No interrupt pending"]
    NoIntr = 0,
    #[doc = "1: Interrupt Source: Zero event (Z)"]
    Z = 1,
    #[doc = "2: nterrupt Source: Load event (L)"]
    L = 2,
    #[doc = "5: Interrupt Source: Capture or compare down event (CCD0)"]
    Ccd0 = 5,
    #[doc = "6: Interrupt Source: Capture or compare down event (CCD1)"]
    Ccd1 = 6,
    #[doc = "7: Interrupt Source: Capture or compare down event (CCD2)"]
    Ccd2 = 7,
    #[doc = "8: Interrupt Source: Capture or compare down event (CCD3)"]
    Ccd3 = 8,
    #[doc = "9: Interrupt Source: Capture or compare up event (CCU0)"]
    Ccu0 = 9,
    #[doc = "10: Interrupt Source: Capture or compare up event (CCU1)"]
    Ccu1 = 10,
    #[doc = "11: Interrupt Source: Capture or compare up event (CCU2)"]
    Ccu2 = 11,
    #[doc = "12: Interrupt Source: Capture or compare up event (CCU3)"]
    Ccu3 = 12,
    #[doc = "13: Interrupt Source: Compare down event (CCD4)"]
    Ccd4 = 13,
    #[doc = "14: Interrupt Source: Compare down event (CCD5)"]
    Ccd5 = 14,
    #[doc = "15: Interrupt Source: Compare down event (CCU4)"]
    Ccu4 = 15,
    #[doc = "16: Interrupt Source: Compare down event (CCU5)"]
    Ccu5 = 16,
    #[doc = "25: Interrupt Source: Fault Event generated an interrupt. (F)"]
    F = 25,
    #[doc = "26: Interrupt Source: Trigger overflow (TOV)"]
    Tov = 26,
    #[doc = "27: Interrupt Source: Repeat Counter Zero (REPC)"]
    Repc = 27,
    #[doc = "28: Interrupt Source: Direction Change (DC)"]
    Dc = 28,
    #[doc = "29: Interrupt Source:QEI Incorrect state transition error (QEIERR)"]
    Qeierr = 29,
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
            1 => Some(Stat::Z),
            2 => Some(Stat::L),
            5 => Some(Stat::Ccd0),
            6 => Some(Stat::Ccd1),
            7 => Some(Stat::Ccd2),
            8 => Some(Stat::Ccd3),
            9 => Some(Stat::Ccu0),
            10 => Some(Stat::Ccu1),
            11 => Some(Stat::Ccu2),
            12 => Some(Stat::Ccu3),
            13 => Some(Stat::Ccd4),
            14 => Some(Stat::Ccd5),
            15 => Some(Stat::Ccu4),
            16 => Some(Stat::Ccu5),
            25 => Some(Stat::F),
            26 => Some(Stat::Tov),
            27 => Some(Stat::Repc),
            28 => Some(Stat::Dc),
            29 => Some(Stat::Qeierr),
            _ => None,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_no_intr(&self) -> bool {
        *self == Stat::NoIntr
    }
    #[doc = "Interrupt Source: Zero event (Z)"]
    #[inline(always)]
    pub fn is_z(&self) -> bool {
        *self == Stat::Z
    }
    #[doc = "nterrupt Source: Load event (L)"]
    #[inline(always)]
    pub fn is_l(&self) -> bool {
        *self == Stat::L
    }
    #[doc = "Interrupt Source: Capture or compare down event (CCD0)"]
    #[inline(always)]
    pub fn is_ccd0(&self) -> bool {
        *self == Stat::Ccd0
    }
    #[doc = "Interrupt Source: Capture or compare down event (CCD1)"]
    #[inline(always)]
    pub fn is_ccd1(&self) -> bool {
        *self == Stat::Ccd1
    }
    #[doc = "Interrupt Source: Capture or compare down event (CCD2)"]
    #[inline(always)]
    pub fn is_ccd2(&self) -> bool {
        *self == Stat::Ccd2
    }
    #[doc = "Interrupt Source: Capture or compare down event (CCD3)"]
    #[inline(always)]
    pub fn is_ccd3(&self) -> bool {
        *self == Stat::Ccd3
    }
    #[doc = "Interrupt Source: Capture or compare up event (CCU0)"]
    #[inline(always)]
    pub fn is_ccu0(&self) -> bool {
        *self == Stat::Ccu0
    }
    #[doc = "Interrupt Source: Capture or compare up event (CCU1)"]
    #[inline(always)]
    pub fn is_ccu1(&self) -> bool {
        *self == Stat::Ccu1
    }
    #[doc = "Interrupt Source: Capture or compare up event (CCU2)"]
    #[inline(always)]
    pub fn is_ccu2(&self) -> bool {
        *self == Stat::Ccu2
    }
    #[doc = "Interrupt Source: Capture or compare up event (CCU3)"]
    #[inline(always)]
    pub fn is_ccu3(&self) -> bool {
        *self == Stat::Ccu3
    }
    #[doc = "Interrupt Source: Compare down event (CCD4)"]
    #[inline(always)]
    pub fn is_ccd4(&self) -> bool {
        *self == Stat::Ccd4
    }
    #[doc = "Interrupt Source: Compare down event (CCD5)"]
    #[inline(always)]
    pub fn is_ccd5(&self) -> bool {
        *self == Stat::Ccd5
    }
    #[doc = "Interrupt Source: Compare down event (CCU4)"]
    #[inline(always)]
    pub fn is_ccu4(&self) -> bool {
        *self == Stat::Ccu4
    }
    #[doc = "Interrupt Source: Compare down event (CCU5)"]
    #[inline(always)]
    pub fn is_ccu5(&self) -> bool {
        *self == Stat::Ccu5
    }
    #[doc = "Interrupt Source: Fault Event generated an interrupt. (F)"]
    #[inline(always)]
    pub fn is_f(&self) -> bool {
        *self == Stat::F
    }
    #[doc = "Interrupt Source: Trigger overflow (TOV)"]
    #[inline(always)]
    pub fn is_tov(&self) -> bool {
        *self == Stat::Tov
    }
    #[doc = "Interrupt Source: Repeat Counter Zero (REPC)"]
    #[inline(always)]
    pub fn is_repc(&self) -> bool {
        *self == Stat::Repc
    }
    #[doc = "Interrupt Source: Direction Change (DC)"]
    #[inline(always)]
    pub fn is_dc(&self) -> bool {
        *self == Stat::Dc
    }
    #[doc = "Interrupt Source:QEI Incorrect state transition error (QEIERR)"]
    #[inline(always)]
    pub fn is_qeierr(&self) -> bool {
        *self == Stat::Qeierr
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
