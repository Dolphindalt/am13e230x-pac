#[doc = "Register `IIDX` reader"]
pub type R = crate::R<IidxSpec>;
#[doc = "Interrupt index status\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Stat {
    #[doc = "0: No bit is set means there is no pending interrupt request"]
    NoIntr = 0,
    #[doc = "1: DMA Channel 0 size counter reached zero (DMASZ=0)."]
    Dmach0 = 1,
    #[doc = "2: DMA Channel 1 size counter reached zero (DMASZ=0)."]
    Dmach1 = 2,
    #[doc = "3: DMA Channel 2 size counter reached zero (DMASZ=0)."]
    Dmach2 = 3,
    #[doc = "4: DMA Channel 3 size counter reached zero (DMASZ=0)."]
    Dmach3 = 4,
    #[doc = "5: DMA Channel 4 size counter reached zero (DMASZ=0)."]
    Dmach4 = 5,
    #[doc = "6: DMA Channel 5 size counter reached zero (DMASZ=0)."]
    Dmach5 = 6,
    #[doc = "7: DMA Channel 6 size counter reached zero (DMASZ=0)."]
    Dmach6 = 7,
    #[doc = "8: DMA Channel 7 size counter reached zero (DMASZ=0)."]
    Dmach7 = 8,
    #[doc = "9: DMA Channel 8 size counter reached zero (DMASZ=0)."]
    Dmach8 = 9,
    #[doc = "10: DMA Channel 9 size counter reached zero (DMASZ=0)."]
    Dmach9 = 10,
    #[doc = "11: DMA Channel 10 size counter reached zero (DMASZ=0)."]
    Dmach10 = 11,
    #[doc = "12: DMA Channel 11 size counter reached zero (DMASZ=0)."]
    Dmach11 = 12,
    #[doc = "13: DMA Channel 12 size counter reached zero (DMASZ=0)."]
    Dmach12 = 13,
    #[doc = "14: DMA Channel 13 size counter reached zero (DMASZ=0)."]
    Dmach13 = 14,
    #[doc = "15: DMA Channel 14 size counter reached zero (DMASZ=0)."]
    Dmach14 = 15,
    #[doc = "16: DMA Channel 15 size counter reached zero (DMASZ=0)."]
    Dmach15 = 16,
    #[doc = "17: PRE-IRQ event for DMA Channel 0."]
    Preirqch0 = 17,
    #[doc = "18: PRE-IRQ event for DMA Channel 1."]
    Preirqch1 = 18,
    #[doc = "19: PRE-IRQ event for DMA Channel 2."]
    Preirqch2 = 19,
    #[doc = "20: PRE-IRQ event for DMA Channel 3."]
    Preirqch3 = 20,
    #[doc = "21: PRE-IRQ event for DMA Channel 4."]
    Preirqch4 = 21,
    #[doc = "22: PRE-IRQ event for DMA Channel 5."]
    Preirqch5 = 22,
    #[doc = "23: PRE-IRQ event for DMA Channel 6."]
    Preirqch6 = 23,
    #[doc = "24: PRE-IRQ event for DMA Channel 7."]
    Preirqch7 = 24,
    #[doc = "25: DMA address error, SRC address not reachable."]
    Addrerr = 25,
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
            1 => Some(Stat::Dmach0),
            2 => Some(Stat::Dmach1),
            3 => Some(Stat::Dmach2),
            4 => Some(Stat::Dmach3),
            5 => Some(Stat::Dmach4),
            6 => Some(Stat::Dmach5),
            7 => Some(Stat::Dmach6),
            8 => Some(Stat::Dmach7),
            9 => Some(Stat::Dmach8),
            10 => Some(Stat::Dmach9),
            11 => Some(Stat::Dmach10),
            12 => Some(Stat::Dmach11),
            13 => Some(Stat::Dmach12),
            14 => Some(Stat::Dmach13),
            15 => Some(Stat::Dmach14),
            16 => Some(Stat::Dmach15),
            17 => Some(Stat::Preirqch0),
            18 => Some(Stat::Preirqch1),
            19 => Some(Stat::Preirqch2),
            20 => Some(Stat::Preirqch3),
            21 => Some(Stat::Preirqch4),
            22 => Some(Stat::Preirqch5),
            23 => Some(Stat::Preirqch6),
            24 => Some(Stat::Preirqch7),
            25 => Some(Stat::Addrerr),
            _ => None,
        }
    }
    #[doc = "No bit is set means there is no pending interrupt request"]
    #[inline(always)]
    pub fn is_no_intr(&self) -> bool {
        *self == Stat::NoIntr
    }
    #[doc = "DMA Channel 0 size counter reached zero (DMASZ=0)."]
    #[inline(always)]
    pub fn is_dmach0(&self) -> bool {
        *self == Stat::Dmach0
    }
    #[doc = "DMA Channel 1 size counter reached zero (DMASZ=0)."]
    #[inline(always)]
    pub fn is_dmach1(&self) -> bool {
        *self == Stat::Dmach1
    }
    #[doc = "DMA Channel 2 size counter reached zero (DMASZ=0)."]
    #[inline(always)]
    pub fn is_dmach2(&self) -> bool {
        *self == Stat::Dmach2
    }
    #[doc = "DMA Channel 3 size counter reached zero (DMASZ=0)."]
    #[inline(always)]
    pub fn is_dmach3(&self) -> bool {
        *self == Stat::Dmach3
    }
    #[doc = "DMA Channel 4 size counter reached zero (DMASZ=0)."]
    #[inline(always)]
    pub fn is_dmach4(&self) -> bool {
        *self == Stat::Dmach4
    }
    #[doc = "DMA Channel 5 size counter reached zero (DMASZ=0)."]
    #[inline(always)]
    pub fn is_dmach5(&self) -> bool {
        *self == Stat::Dmach5
    }
    #[doc = "DMA Channel 6 size counter reached zero (DMASZ=0)."]
    #[inline(always)]
    pub fn is_dmach6(&self) -> bool {
        *self == Stat::Dmach6
    }
    #[doc = "DMA Channel 7 size counter reached zero (DMASZ=0)."]
    #[inline(always)]
    pub fn is_dmach7(&self) -> bool {
        *self == Stat::Dmach7
    }
    #[doc = "DMA Channel 8 size counter reached zero (DMASZ=0)."]
    #[inline(always)]
    pub fn is_dmach8(&self) -> bool {
        *self == Stat::Dmach8
    }
    #[doc = "DMA Channel 9 size counter reached zero (DMASZ=0)."]
    #[inline(always)]
    pub fn is_dmach9(&self) -> bool {
        *self == Stat::Dmach9
    }
    #[doc = "DMA Channel 10 size counter reached zero (DMASZ=0)."]
    #[inline(always)]
    pub fn is_dmach10(&self) -> bool {
        *self == Stat::Dmach10
    }
    #[doc = "DMA Channel 11 size counter reached zero (DMASZ=0)."]
    #[inline(always)]
    pub fn is_dmach11(&self) -> bool {
        *self == Stat::Dmach11
    }
    #[doc = "DMA Channel 12 size counter reached zero (DMASZ=0)."]
    #[inline(always)]
    pub fn is_dmach12(&self) -> bool {
        *self == Stat::Dmach12
    }
    #[doc = "DMA Channel 13 size counter reached zero (DMASZ=0)."]
    #[inline(always)]
    pub fn is_dmach13(&self) -> bool {
        *self == Stat::Dmach13
    }
    #[doc = "DMA Channel 14 size counter reached zero (DMASZ=0)."]
    #[inline(always)]
    pub fn is_dmach14(&self) -> bool {
        *self == Stat::Dmach14
    }
    #[doc = "DMA Channel 15 size counter reached zero (DMASZ=0)."]
    #[inline(always)]
    pub fn is_dmach15(&self) -> bool {
        *self == Stat::Dmach15
    }
    #[doc = "PRE-IRQ event for DMA Channel 0."]
    #[inline(always)]
    pub fn is_preirqch0(&self) -> bool {
        *self == Stat::Preirqch0
    }
    #[doc = "PRE-IRQ event for DMA Channel 1."]
    #[inline(always)]
    pub fn is_preirqch1(&self) -> bool {
        *self == Stat::Preirqch1
    }
    #[doc = "PRE-IRQ event for DMA Channel 2."]
    #[inline(always)]
    pub fn is_preirqch2(&self) -> bool {
        *self == Stat::Preirqch2
    }
    #[doc = "PRE-IRQ event for DMA Channel 3."]
    #[inline(always)]
    pub fn is_preirqch3(&self) -> bool {
        *self == Stat::Preirqch3
    }
    #[doc = "PRE-IRQ event for DMA Channel 4."]
    #[inline(always)]
    pub fn is_preirqch4(&self) -> bool {
        *self == Stat::Preirqch4
    }
    #[doc = "PRE-IRQ event for DMA Channel 5."]
    #[inline(always)]
    pub fn is_preirqch5(&self) -> bool {
        *self == Stat::Preirqch5
    }
    #[doc = "PRE-IRQ event for DMA Channel 6."]
    #[inline(always)]
    pub fn is_preirqch6(&self) -> bool {
        *self == Stat::Preirqch6
    }
    #[doc = "PRE-IRQ event for DMA Channel 7."]
    #[inline(always)]
    pub fn is_preirqch7(&self) -> bool {
        *self == Stat::Preirqch7
    }
    #[doc = "DMA address error, SRC address not reachable."]
    #[inline(always)]
    pub fn is_addrerr(&self) -> bool {
        *self == Stat::Addrerr
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
