#[doc = "Register `NMIIIDX` reader"]
pub type R = crate::R<NmiiidxSpec>;
#[doc = "The NMI interrupt index (NMIIIDX) register generates a value corresponding to the highest priority pending NMI source. This value may be used as an address offset for fast, deterministic handling in the NMI service routine. A read of the NMIIIDX register will clear the corresponding interrupt status in the NMIRIS register.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Stat {
    #[doc = "0: No NMI pending"]
    NoIntr = 0,
    #[doc = "1: Reserved"]
    Borlvl = 1,
    #[doc = "2: `10`"]
    Wwdt0 = 2,
    #[doc = "3: `11`"]
    Security = 3,
    #[doc = "4: `100`"]
    Flashded = 4,
    #[doc = "5: `101`"]
    Srampar = 5,
    #[doc = "6: `110`"]
    Tmurompar = 6,
    #[doc = "7: `111`"]
    Sysmemacc = 7,
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
#[doc = "Field `STAT` reader - The NMI interrupt index (NMIIIDX) register generates a value corresponding to the highest priority pending NMI source. This value may be used as an address offset for fast, deterministic handling in the NMI service routine. A read of the NMIIIDX register will clear the corresponding interrupt status in the NMIRIS register."]
pub type StatR = crate::FieldReader<Stat>;
impl StatR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Stat {
        match self.bits {
            0 => Stat::NoIntr,
            1 => Stat::Borlvl,
            2 => Stat::Wwdt0,
            3 => Stat::Security,
            4 => Stat::Flashded,
            5 => Stat::Srampar,
            6 => Stat::Tmurompar,
            7 => Stat::Sysmemacc,
            _ => unreachable!(),
        }
    }
    #[doc = "No NMI pending"]
    #[inline(always)]
    pub fn is_no_intr(&self) -> bool {
        *self == Stat::NoIntr
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn is_borlvl(&self) -> bool {
        *self == Stat::Borlvl
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_wwdt0(&self) -> bool {
        *self == Stat::Wwdt0
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_security(&self) -> bool {
        *self == Stat::Security
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn is_flashded(&self) -> bool {
        *self == Stat::Flashded
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn is_srampar(&self) -> bool {
        *self == Stat::Srampar
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn is_tmurompar(&self) -> bool {
        *self == Stat::Tmurompar
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn is_sysmemacc(&self) -> bool {
        *self == Stat::Sysmemacc
    }
}
impl R {
    #[doc = "Bits 0:2 - The NMI interrupt index (NMIIIDX) register generates a value corresponding to the highest priority pending NMI source. This value may be used as an address offset for fast, deterministic handling in the NMI service routine. A read of the NMIIIDX register will clear the corresponding interrupt status in the NMIRIS register."]
    #[inline(always)]
    pub fn stat(&self) -> StatR {
        StatR::new((self.bits & 7) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NMIIIDX")
            .field("stat", &self.stat())
            .finish()
    }
}
#[doc = "NMI interrupt index\n\nYou can [`read`](crate::Reg::read) this register and get [`nmiiidx::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NmiiidxSpec;
impl crate::RegisterSpec for NmiiidxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nmiiidx::R`](R) reader structure"]
impl crate::Readable for NmiiidxSpec {}
#[doc = "`reset()` method sets NMIIIDX to value 0"]
impl crate::Resettable for NmiiidxSpec {}
