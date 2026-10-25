#[doc = "Register `IIDX` reader"]
pub type R = crate::R<IidxSpec>;
#[doc = "The SYSCTL interrupt index (IIDX) register generates a value corresponding to the highest priority pending interrupt source. This value may be used as an address offset for fast, deterministic handling in the interrupt service routine. A read of the IIDX register will clear the corresponding interrupt status in the RIS and MIS registers.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Stat {
    #[doc = "0: No interrupt pending"]
    NoIntr = 0,
    #[doc = "1: LFOSCGOOD interrupt pending"]
    Lfoscgood = 1,
    #[doc = "2: `10`"]
    Anaclkerr = 2,
    #[doc = "3: `11`"]
    Flashsec = 3,
    #[doc = "4: `100`"]
    Sramsec = 4,
    #[doc = "5: `101`"]
    Hfclkgood = 5,
    #[doc = "6: `110`"]
    Syspllgood = 6,
    #[doc = "7: `111`"]
    Hsclkgood = 7,
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
#[doc = "Field `STAT` reader - The SYSCTL interrupt index (IIDX) register generates a value corresponding to the highest priority pending interrupt source. This value may be used as an address offset for fast, deterministic handling in the interrupt service routine. A read of the IIDX register will clear the corresponding interrupt status in the RIS and MIS registers."]
pub type StatR = crate::FieldReader<Stat>;
impl StatR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Stat {
        match self.bits {
            0 => Stat::NoIntr,
            1 => Stat::Lfoscgood,
            2 => Stat::Anaclkerr,
            3 => Stat::Flashsec,
            4 => Stat::Sramsec,
            5 => Stat::Hfclkgood,
            6 => Stat::Syspllgood,
            7 => Stat::Hsclkgood,
            _ => unreachable!(),
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_no_intr(&self) -> bool {
        *self == Stat::NoIntr
    }
    #[doc = "LFOSCGOOD interrupt pending"]
    #[inline(always)]
    pub fn is_lfoscgood(&self) -> bool {
        *self == Stat::Lfoscgood
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_anaclkerr(&self) -> bool {
        *self == Stat::Anaclkerr
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_flashsec(&self) -> bool {
        *self == Stat::Flashsec
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn is_sramsec(&self) -> bool {
        *self == Stat::Sramsec
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn is_hfclkgood(&self) -> bool {
        *self == Stat::Hfclkgood
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn is_syspllgood(&self) -> bool {
        *self == Stat::Syspllgood
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn is_hsclkgood(&self) -> bool {
        *self == Stat::Hsclkgood
    }
}
impl R {
    #[doc = "Bits 0:2 - The SYSCTL interrupt index (IIDX) register generates a value corresponding to the highest priority pending interrupt source. This value may be used as an address offset for fast, deterministic handling in the interrupt service routine. A read of the IIDX register will clear the corresponding interrupt status in the RIS and MIS registers."]
    #[inline(always)]
    pub fn stat(&self) -> StatR {
        StatR::new((self.bits & 7) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IIDX").field("stat", &self.stat()).finish()
    }
}
#[doc = "SYSCTL interrupt index\n\nYou can [`read`](crate::Reg::read) this register and get [`iidx::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IidxSpec;
impl crate::RegisterSpec for IidxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iidx::R`](R) reader structure"]
impl crate::Readable for IidxSpec {}
#[doc = "`reset()` method sets IIDX to value 0"]
impl crate::Resettable for IidxSpec {}
