#[doc = "Register `ISET` writer"]
pub type W = crate::W<IsetSpec>;
#[doc = "Field `Z` writer - Zero event SET"]
pub type ZW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L` writer - Load event SET"]
pub type LW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCD0` writer - Capture or compare down event SET"]
pub type Ccd0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCD1` writer - Capture or compare down event SET"]
pub type Ccd1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCD2` writer - Capture or compare down event SET"]
pub type Ccd2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCD3` writer - Capture or compare down event SET"]
pub type Ccd3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCU0` writer - Capture or compare up event SET"]
pub type Ccu0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCU1` writer - Capture or compare up event SET"]
pub type Ccu1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCU2` writer - Capture or compare up event SET"]
pub type Ccu2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCU3` writer - Capture or compare up event SET"]
pub type Ccu3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCD4` writer - Compare down event 4 SET"]
pub type Ccd4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCD5` writer - Compare down event 5 SET"]
pub type Ccd5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCU4` writer - Compare up event 4 SET"]
pub type Ccu4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCU5` writer - Compare up event 5 SET"]
pub type Ccu5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `F` writer - Fault event SET"]
pub type FW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOV` writer - Trigger Overflow event SET"]
pub type TovW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REPC` writer - Repeat Counter Zero event SET"]
pub type RepcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DC` writer - Direction Change event SET"]
pub type DcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `QEIERR` writer - QEIERR event SET"]
pub type QeierrW<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<IsetSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Zero event SET"]
    #[inline(always)]
    pub fn z(&mut self) -> ZW<'_, IsetSpec> {
        ZW::new(self, 0)
    }
    #[doc = "Bit 1 - Load event SET"]
    #[inline(always)]
    pub fn l(&mut self) -> LW<'_, IsetSpec> {
        LW::new(self, 1)
    }
    #[doc = "Bit 4 - Capture or compare down event SET"]
    #[inline(always)]
    pub fn ccd0(&mut self) -> Ccd0W<'_, IsetSpec> {
        Ccd0W::new(self, 4)
    }
    #[doc = "Bit 5 - Capture or compare down event SET"]
    #[inline(always)]
    pub fn ccd1(&mut self) -> Ccd1W<'_, IsetSpec> {
        Ccd1W::new(self, 5)
    }
    #[doc = "Bit 6 - Capture or compare down event SET"]
    #[inline(always)]
    pub fn ccd2(&mut self) -> Ccd2W<'_, IsetSpec> {
        Ccd2W::new(self, 6)
    }
    #[doc = "Bit 7 - Capture or compare down event SET"]
    #[inline(always)]
    pub fn ccd3(&mut self) -> Ccd3W<'_, IsetSpec> {
        Ccd3W::new(self, 7)
    }
    #[doc = "Bit 8 - Capture or compare up event SET"]
    #[inline(always)]
    pub fn ccu0(&mut self) -> Ccu0W<'_, IsetSpec> {
        Ccu0W::new(self, 8)
    }
    #[doc = "Bit 9 - Capture or compare up event SET"]
    #[inline(always)]
    pub fn ccu1(&mut self) -> Ccu1W<'_, IsetSpec> {
        Ccu1W::new(self, 9)
    }
    #[doc = "Bit 10 - Capture or compare up event SET"]
    #[inline(always)]
    pub fn ccu2(&mut self) -> Ccu2W<'_, IsetSpec> {
        Ccu2W::new(self, 10)
    }
    #[doc = "Bit 11 - Capture or compare up event SET"]
    #[inline(always)]
    pub fn ccu3(&mut self) -> Ccu3W<'_, IsetSpec> {
        Ccu3W::new(self, 11)
    }
    #[doc = "Bit 12 - Compare down event 4 SET"]
    #[inline(always)]
    pub fn ccd4(&mut self) -> Ccd4W<'_, IsetSpec> {
        Ccd4W::new(self, 12)
    }
    #[doc = "Bit 13 - Compare down event 5 SET"]
    #[inline(always)]
    pub fn ccd5(&mut self) -> Ccd5W<'_, IsetSpec> {
        Ccd5W::new(self, 13)
    }
    #[doc = "Bit 14 - Compare up event 4 SET"]
    #[inline(always)]
    pub fn ccu4(&mut self) -> Ccu4W<'_, IsetSpec> {
        Ccu4W::new(self, 14)
    }
    #[doc = "Bit 15 - Compare up event 5 SET"]
    #[inline(always)]
    pub fn ccu5(&mut self) -> Ccu5W<'_, IsetSpec> {
        Ccu5W::new(self, 15)
    }
    #[doc = "Bit 24 - Fault event SET"]
    #[inline(always)]
    pub fn f(&mut self) -> FW<'_, IsetSpec> {
        FW::new(self, 24)
    }
    #[doc = "Bit 25 - Trigger Overflow event SET"]
    #[inline(always)]
    pub fn tov(&mut self) -> TovW<'_, IsetSpec> {
        TovW::new(self, 25)
    }
    #[doc = "Bit 26 - Repeat Counter Zero event SET"]
    #[inline(always)]
    pub fn repc(&mut self) -> RepcW<'_, IsetSpec> {
        RepcW::new(self, 26)
    }
    #[doc = "Bit 27 - Direction Change event SET"]
    #[inline(always)]
    pub fn dc(&mut self) -> DcW<'_, IsetSpec> {
        DcW::new(self, 27)
    }
    #[doc = "Bit 28 - QEIERR event SET"]
    #[inline(always)]
    pub fn qeierr(&mut self) -> QeierrW<'_, IsetSpec> {
        QeierrW::new(self, 28)
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
