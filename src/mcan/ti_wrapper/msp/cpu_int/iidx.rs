#[doc = "Register `IIDX` reader"]
pub type R = crate::R<IidxSpec>;
#[doc = "Interrupt index status\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Stat {
    #[doc = "0: No interrupt pending."]
    NoIntr = 0,
    #[doc = "1: MCAN Interrupt Line 0 interrupt pending."]
    Intl0 = 1,
    #[doc = "2: MCAN Interrupt Line 1 interrupt pending."]
    Intl1 = 2,
    #[doc = "3: Message RAM SEC (Single Error Correction) interrupt pending."]
    Sec = 3,
    #[doc = "4: Message RAM DED (Double Error Detection) interrupt pending."]
    Ded = 4,
    #[doc = "5: External Timestamp Counter Overflow interrupt pending."]
    ExtTsCntrOvfl = 5,
    #[doc = "6: Clock Stop Wake Up interrupt pending."]
    Wakeup = 6,
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
            1 => Some(Stat::Intl0),
            2 => Some(Stat::Intl1),
            3 => Some(Stat::Sec),
            4 => Some(Stat::Ded),
            5 => Some(Stat::ExtTsCntrOvfl),
            6 => Some(Stat::Wakeup),
            _ => None,
        }
    }
    #[doc = "No interrupt pending."]
    #[inline(always)]
    pub fn is_no_intr(&self) -> bool {
        *self == Stat::NoIntr
    }
    #[doc = "MCAN Interrupt Line 0 interrupt pending."]
    #[inline(always)]
    pub fn is_intl0(&self) -> bool {
        *self == Stat::Intl0
    }
    #[doc = "MCAN Interrupt Line 1 interrupt pending."]
    #[inline(always)]
    pub fn is_intl1(&self) -> bool {
        *self == Stat::Intl1
    }
    #[doc = "Message RAM SEC (Single Error Correction) interrupt pending."]
    #[inline(always)]
    pub fn is_sec(&self) -> bool {
        *self == Stat::Sec
    }
    #[doc = "Message RAM DED (Double Error Detection) interrupt pending."]
    #[inline(always)]
    pub fn is_ded(&self) -> bool {
        *self == Stat::Ded
    }
    #[doc = "External Timestamp Counter Overflow interrupt pending."]
    #[inline(always)]
    pub fn is_ext_ts_cntr_ovfl(&self) -> bool {
        *self == Stat::ExtTsCntrOvfl
    }
    #[doc = "Clock Stop Wake Up interrupt pending."]
    #[inline(always)]
    pub fn is_wakeup(&self) -> bool {
        *self == Stat::Wakeup
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
#[doc = "Interrupt Index Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iidx::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IidxSpec;
impl crate::RegisterSpec for IidxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iidx::R`](R) reader structure"]
impl crate::Readable for IidxSpec {}
#[doc = "`reset()` method sets IIDX to value 0"]
impl crate::Resettable for IidxSpec {}
