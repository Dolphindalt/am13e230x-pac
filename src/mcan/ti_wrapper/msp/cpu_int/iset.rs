#[doc = "Register `ISET` writer"]
pub type W = crate::W<IsetSpec>;
#[doc = "Field `INTL0` writer - MCAN Interrupt Line 0"]
pub type Intl0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTL1` writer - MCAN Interrupt Line 1"]
pub type Intl1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEC` writer - Message RAM SEC interrupt"]
pub type SecW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DED` writer - Message RAM DED interrupt"]
pub type DedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXT_TS_CNTR_OVFL` writer - External Timestamp Counter Overflow Iterrupt"]
pub type ExtTsCntrOvflW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WAKEUP` writer - Clock Stop Wake Up Interrupt"]
pub type WakeupW<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<IsetSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - MCAN Interrupt Line 0"]
    #[inline(always)]
    pub fn intl0(&mut self) -> Intl0W<'_, IsetSpec> {
        Intl0W::new(self, 0)
    }
    #[doc = "Bit 1 - MCAN Interrupt Line 1"]
    #[inline(always)]
    pub fn intl1(&mut self) -> Intl1W<'_, IsetSpec> {
        Intl1W::new(self, 1)
    }
    #[doc = "Bit 2 - Message RAM SEC interrupt"]
    #[inline(always)]
    pub fn sec(&mut self) -> SecW<'_, IsetSpec> {
        SecW::new(self, 2)
    }
    #[doc = "Bit 3 - Message RAM DED interrupt"]
    #[inline(always)]
    pub fn ded(&mut self) -> DedW<'_, IsetSpec> {
        DedW::new(self, 3)
    }
    #[doc = "Bit 4 - External Timestamp Counter Overflow Iterrupt"]
    #[inline(always)]
    pub fn ext_ts_cntr_ovfl(&mut self) -> ExtTsCntrOvflW<'_, IsetSpec> {
        ExtTsCntrOvflW::new(self, 4)
    }
    #[doc = "Bit 5 - Clock Stop Wake Up Interrupt"]
    #[inline(always)]
    pub fn wakeup(&mut self) -> WakeupW<'_, IsetSpec> {
        WakeupW::new(self, 5)
    }
}
#[doc = "Interrupt set\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iset::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IsetSpec;
impl crate::RegisterSpec for IsetSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`iset::W`](W) writer structure"]
impl crate::Writable for IsetSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets ISET to value 0"]
impl crate::Resettable for IsetSpec {}
