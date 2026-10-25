#[doc = "Register `RIS` reader"]
pub type R = crate::R<RisSpec>;
#[doc = "Field `INTL0` reader - MCAN Interrupt Line 0"]
pub type Intl0R = crate::BitReader;
#[doc = "Field `INTL1` reader - MCAN Interrupt Line 1"]
pub type Intl1R = crate::BitReader;
#[doc = "Field `SEC` reader - Message RAM SEC interrupt"]
pub type SecR = crate::BitReader;
#[doc = "Field `DED` reader - Message RAM DED interrupt"]
pub type DedR = crate::BitReader;
#[doc = "Field `EXT_TS_CNTR_OVFL` reader - External Timestamp Counter Overflow Interrupt"]
pub type ExtTsCntrOvflR = crate::BitReader;
#[doc = "Field `WAKEUP` reader - Clock Stop Wake Up Interrupt"]
pub type WakeupR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - MCAN Interrupt Line 0"]
    #[inline(always)]
    pub fn intl0(&self) -> Intl0R {
        Intl0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - MCAN Interrupt Line 1"]
    #[inline(always)]
    pub fn intl1(&self) -> Intl1R {
        Intl1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Message RAM SEC interrupt"]
    #[inline(always)]
    pub fn sec(&self) -> SecR {
        SecR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Message RAM DED interrupt"]
    #[inline(always)]
    pub fn ded(&self) -> DedR {
        DedR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - External Timestamp Counter Overflow Interrupt"]
    #[inline(always)]
    pub fn ext_ts_cntr_ovfl(&self) -> ExtTsCntrOvflR {
        ExtTsCntrOvflR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Clock Stop Wake Up Interrupt"]
    #[inline(always)]
    pub fn wakeup(&self) -> WakeupR {
        WakeupR::new(((self.bits >> 5) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RIS")
            .field("intl0", &self.intl0())
            .field("intl1", &self.intl1())
            .field("sec", &self.sec())
            .field("ded", &self.ded())
            .field("ext_ts_cntr_ovfl", &self.ext_ts_cntr_ovfl())
            .field("wakeup", &self.wakeup())
            .finish()
    }
}
#[doc = "Raw interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`ris::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RisSpec;
impl crate::RegisterSpec for RisSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ris::R`](R) reader structure"]
impl crate::Readable for RisSpec {}
#[doc = "`reset()` method sets RIS to value 0"]
impl crate::Resettable for RisSpec {}
