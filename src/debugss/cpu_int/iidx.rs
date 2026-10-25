#[doc = "Register `IIDX` reader"]
pub type R = crate::R<IidxSpec>;
#[doc = "Interrupt index status\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Stat {
    #[doc = "0: No pending interrupt request"]
    NoIntr = 0,
    #[doc = "1: TX interrupt"]
    Txifg = 1,
    #[doc = "2: RX interrupt"]
    Rxifg = 2,
    #[doc = "3: Power-up interrupt. A debug session has started."]
    Pwrup = 3,
    #[doc = "4: Power-up interrupt. A debug session has started."]
    Pwrdwn = 4,
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
            1 => Some(Stat::Txifg),
            2 => Some(Stat::Rxifg),
            3 => Some(Stat::Pwrup),
            4 => Some(Stat::Pwrdwn),
            _ => None,
        }
    }
    #[doc = "No pending interrupt request"]
    #[inline(always)]
    pub fn is_no_intr(&self) -> bool {
        *self == Stat::NoIntr
    }
    #[doc = "TX interrupt"]
    #[inline(always)]
    pub fn is_txifg(&self) -> bool {
        *self == Stat::Txifg
    }
    #[doc = "RX interrupt"]
    #[inline(always)]
    pub fn is_rxifg(&self) -> bool {
        *self == Stat::Rxifg
    }
    #[doc = "Power-up interrupt. A debug session has started."]
    #[inline(always)]
    pub fn is_pwrup(&self) -> bool {
        *self == Stat::Pwrup
    }
    #[doc = "Power-up interrupt. A debug session has started."]
    #[inline(always)]
    pub fn is_pwrdwn(&self) -> bool {
        *self == Stat::Pwrdwn
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
