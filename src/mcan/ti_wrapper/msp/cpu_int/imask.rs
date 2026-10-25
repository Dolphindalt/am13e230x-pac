#[doc = "Register `IMASK` reader"]
pub type R = crate::R<ImaskSpec>;
#[doc = "Register `IMASK` writer"]
pub type W = crate::W<ImaskSpec>;
#[doc = "Field `INTL0` reader - MCAN Interrupt Line 0"]
pub type Intl0R = crate::BitReader;
#[doc = "Field `INTL0` writer - MCAN Interrupt Line 0"]
pub type Intl0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTL1` reader - MCAN Interrupt Line 1"]
pub type Intl1R = crate::BitReader;
#[doc = "Field `INTL1` writer - MCAN Interrupt Line 1"]
pub type Intl1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEC` reader - Message RAM SEC interrupt"]
pub type SecR = crate::BitReader;
#[doc = "Field `SEC` writer - Message RAM SEC interrupt"]
pub type SecW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DED` reader - Message RAM DED interrupt"]
pub type DedR = crate::BitReader;
#[doc = "Field `DED` writer - Message RAM DED interrupt"]
pub type DedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXT_TS_CNTR_OVFL` reader - External Timestamp Counter Overflow"]
pub type ExtTsCntrOvflR = crate::BitReader;
#[doc = "Field `EXT_TS_CNTR_OVFL` writer - External Timestamp Counter Overflow"]
pub type ExtTsCntrOvflW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WAKEUP` reader - Clock Stop Wake Up Interrupt"]
pub type WakeupR = crate::BitReader;
#[doc = "Field `WAKEUP` writer - Clock Stop Wake Up Interrupt"]
pub type WakeupW<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[doc = "Bit 4 - External Timestamp Counter Overflow"]
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
        f.debug_struct("IMASK")
            .field("intl0", &self.intl0())
            .field("intl1", &self.intl1())
            .field("sec", &self.sec())
            .field("ded", &self.ded())
            .field("ext_ts_cntr_ovfl", &self.ext_ts_cntr_ovfl())
            .field("wakeup", &self.wakeup())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - MCAN Interrupt Line 0"]
    #[inline(always)]
    pub fn intl0(&mut self) -> Intl0W<'_, ImaskSpec> {
        Intl0W::new(self, 0)
    }
    #[doc = "Bit 1 - MCAN Interrupt Line 1"]
    #[inline(always)]
    pub fn intl1(&mut self) -> Intl1W<'_, ImaskSpec> {
        Intl1W::new(self, 1)
    }
    #[doc = "Bit 2 - Message RAM SEC interrupt"]
    #[inline(always)]
    pub fn sec(&mut self) -> SecW<'_, ImaskSpec> {
        SecW::new(self, 2)
    }
    #[doc = "Bit 3 - Message RAM DED interrupt"]
    #[inline(always)]
    pub fn ded(&mut self) -> DedW<'_, ImaskSpec> {
        DedW::new(self, 3)
    }
    #[doc = "Bit 4 - External Timestamp Counter Overflow"]
    #[inline(always)]
    pub fn ext_ts_cntr_ovfl(&mut self) -> ExtTsCntrOvflW<'_, ImaskSpec> {
        ExtTsCntrOvflW::new(self, 4)
    }
    #[doc = "Bit 5 - Clock Stop Wake Up Interrupt"]
    #[inline(always)]
    pub fn wakeup(&mut self) -> WakeupW<'_, ImaskSpec> {
        WakeupW::new(self, 5)
    }
}
#[doc = "Interrupt mask\n\nYou can [`read`](crate::Reg::read) this register and get [`imask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`imask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ImaskSpec;
impl crate::RegisterSpec for ImaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`imask::R`](R) reader structure"]
impl crate::Readable for ImaskSpec {}
#[doc = "`write(|w| ..)` method takes [`imask::W`](W) writer structure"]
impl crate::Writable for ImaskSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets IMASK to value 0"]
impl crate::Resettable for ImaskSpec {}
