#[doc = "Register `IMASK` reader"]
pub type R = crate::R<ImaskSpec>;
#[doc = "Register `IMASK` writer"]
pub type W = crate::W<ImaskSpec>;
#[doc = "Field `Z` reader - Zero Event mask"]
pub type ZR = crate::BitReader;
#[doc = "Field `Z` writer - Zero Event mask"]
pub type ZW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L` reader - Load Event mask"]
pub type LR = crate::BitReader;
#[doc = "Field `L` writer - Load Event mask"]
pub type LW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCD0` reader - CCD0 Event Mask"]
pub type Ccd0R = crate::BitReader;
#[doc = "Field `CCD0` writer - CCD0 Event Mask"]
pub type Ccd0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCD1` reader - CCD1 Event Mask"]
pub type Ccd1R = crate::BitReader;
#[doc = "Field `CCD1` writer - CCD1 Event Mask"]
pub type Ccd1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCD2` reader - CCD2 Event Mask"]
pub type Ccd2R = crate::BitReader;
#[doc = "Field `CCD2` writer - CCD2 Event Mask"]
pub type Ccd2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCD3` reader - CCD3 Event Mask"]
pub type Ccd3R = crate::BitReader;
#[doc = "Field `CCD3` writer - CCD3 Event Mask"]
pub type Ccd3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCU0` reader - CCU0 Event Mask"]
pub type Ccu0R = crate::BitReader;
#[doc = "Field `CCU0` writer - CCU0 Event Mask"]
pub type Ccu0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCU1` reader - CCU1 Event Mask"]
pub type Ccu1R = crate::BitReader;
#[doc = "Field `CCU1` writer - CCU1 Event Mask"]
pub type Ccu1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCU2` reader - CCU2 Event Mask"]
pub type Ccu2R = crate::BitReader;
#[doc = "Field `CCU2` writer - CCU2 Event Mask"]
pub type Ccu2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCU3` reader - CCU3 Event Mask"]
pub type Ccu3R = crate::BitReader;
#[doc = "Field `CCU3` writer - CCU3 Event Mask"]
pub type Ccu3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCD4` reader - CCD4 Event Mask"]
pub type Ccd4R = crate::BitReader;
#[doc = "Field `CCD4` writer - CCD4 Event Mask"]
pub type Ccd4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCD5` reader - CCD5 Event Mask"]
pub type Ccd5R = crate::BitReader;
#[doc = "Field `CCD5` writer - CCD5 Event Mask"]
pub type Ccd5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCU4` reader - CCU4 Event Mask"]
pub type Ccu4R = crate::BitReader;
#[doc = "Field `CCU4` writer - CCU4 Event Mask"]
pub type Ccu4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCU5` reader - CCU5 Event Mask"]
pub type Ccu5R = crate::BitReader;
#[doc = "Field `CCU5` writer - CCU5 Event Mask"]
pub type Ccu5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `F` reader - Fault Event mask"]
pub type FR = crate::BitReader;
#[doc = "Field `F` writer - Fault Event mask"]
pub type FW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOV` reader - Trigger Overflow Event mask"]
pub type TovR = crate::BitReader;
#[doc = "Field `TOV` writer - Trigger Overflow Event mask"]
pub type TovW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REPC` reader - Repeat Counter Zero Event mask"]
pub type RepcR = crate::BitReader;
#[doc = "Field `REPC` writer - Repeat Counter Zero Event mask"]
pub type RepcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DC` reader - Direction Change Event mask"]
pub type DcR = crate::BitReader;
#[doc = "Field `DC` writer - Direction Change Event mask"]
pub type DcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `QEIERR` reader - Direction Change Event mask"]
pub type QeierrR = crate::BitReader;
#[doc = "Field `QEIERR` writer - Direction Change Event mask"]
pub type QeierrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Zero Event mask"]
    #[inline(always)]
    pub fn z(&self) -> ZR {
        ZR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Load Event mask"]
    #[inline(always)]
    pub fn l(&self) -> LR {
        LR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - CCD0 Event Mask"]
    #[inline(always)]
    pub fn ccd0(&self) -> Ccd0R {
        Ccd0R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CCD1 Event Mask"]
    #[inline(always)]
    pub fn ccd1(&self) -> Ccd1R {
        Ccd1R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - CCD2 Event Mask"]
    #[inline(always)]
    pub fn ccd2(&self) -> Ccd2R {
        Ccd2R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - CCD3 Event Mask"]
    #[inline(always)]
    pub fn ccd3(&self) -> Ccd3R {
        Ccd3R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - CCU0 Event Mask"]
    #[inline(always)]
    pub fn ccu0(&self) -> Ccu0R {
        Ccu0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CCU1 Event Mask"]
    #[inline(always)]
    pub fn ccu1(&self) -> Ccu1R {
        Ccu1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - CCU2 Event Mask"]
    #[inline(always)]
    pub fn ccu2(&self) -> Ccu2R {
        Ccu2R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - CCU3 Event Mask"]
    #[inline(always)]
    pub fn ccu3(&self) -> Ccu3R {
        Ccu3R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - CCD4 Event Mask"]
    #[inline(always)]
    pub fn ccd4(&self) -> Ccd4R {
        Ccd4R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - CCD5 Event Mask"]
    #[inline(always)]
    pub fn ccd5(&self) -> Ccd5R {
        Ccd5R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - CCU4 Event Mask"]
    #[inline(always)]
    pub fn ccu4(&self) -> Ccu4R {
        Ccu4R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - CCU5 Event Mask"]
    #[inline(always)]
    pub fn ccu5(&self) -> Ccu5R {
        Ccu5R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 24 - Fault Event mask"]
    #[inline(always)]
    pub fn f(&self) -> FR {
        FR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Trigger Overflow Event mask"]
    #[inline(always)]
    pub fn tov(&self) -> TovR {
        TovR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Repeat Counter Zero Event mask"]
    #[inline(always)]
    pub fn repc(&self) -> RepcR {
        RepcR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Direction Change Event mask"]
    #[inline(always)]
    pub fn dc(&self) -> DcR {
        DcR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Direction Change Event mask"]
    #[inline(always)]
    pub fn qeierr(&self) -> QeierrR {
        QeierrR::new(((self.bits >> 28) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IMASK")
            .field("z", &self.z())
            .field("l", &self.l())
            .field("ccd0", &self.ccd0())
            .field("ccd1", &self.ccd1())
            .field("ccu0", &self.ccu0())
            .field("ccu1", &self.ccu1())
            .field("f", &self.f())
            .field("tov", &self.tov())
            .field("dc", &self.dc())
            .field("qeierr", &self.qeierr())
            .field("ccd2", &self.ccd2())
            .field("ccd3", &self.ccd3())
            .field("ccu2", &self.ccu2())
            .field("ccu3", &self.ccu3())
            .field("ccd4", &self.ccd4())
            .field("ccd5", &self.ccd5())
            .field("ccu4", &self.ccu4())
            .field("ccu5", &self.ccu5())
            .field("repc", &self.repc())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Zero Event mask"]
    #[inline(always)]
    pub fn z(&mut self) -> ZW<'_, ImaskSpec> {
        ZW::new(self, 0)
    }
    #[doc = "Bit 1 - Load Event mask"]
    #[inline(always)]
    pub fn l(&mut self) -> LW<'_, ImaskSpec> {
        LW::new(self, 1)
    }
    #[doc = "Bit 4 - CCD0 Event Mask"]
    #[inline(always)]
    pub fn ccd0(&mut self) -> Ccd0W<'_, ImaskSpec> {
        Ccd0W::new(self, 4)
    }
    #[doc = "Bit 5 - CCD1 Event Mask"]
    #[inline(always)]
    pub fn ccd1(&mut self) -> Ccd1W<'_, ImaskSpec> {
        Ccd1W::new(self, 5)
    }
    #[doc = "Bit 6 - CCD2 Event Mask"]
    #[inline(always)]
    pub fn ccd2(&mut self) -> Ccd2W<'_, ImaskSpec> {
        Ccd2W::new(self, 6)
    }
    #[doc = "Bit 7 - CCD3 Event Mask"]
    #[inline(always)]
    pub fn ccd3(&mut self) -> Ccd3W<'_, ImaskSpec> {
        Ccd3W::new(self, 7)
    }
    #[doc = "Bit 8 - CCU0 Event Mask"]
    #[inline(always)]
    pub fn ccu0(&mut self) -> Ccu0W<'_, ImaskSpec> {
        Ccu0W::new(self, 8)
    }
    #[doc = "Bit 9 - CCU1 Event Mask"]
    #[inline(always)]
    pub fn ccu1(&mut self) -> Ccu1W<'_, ImaskSpec> {
        Ccu1W::new(self, 9)
    }
    #[doc = "Bit 10 - CCU2 Event Mask"]
    #[inline(always)]
    pub fn ccu2(&mut self) -> Ccu2W<'_, ImaskSpec> {
        Ccu2W::new(self, 10)
    }
    #[doc = "Bit 11 - CCU3 Event Mask"]
    #[inline(always)]
    pub fn ccu3(&mut self) -> Ccu3W<'_, ImaskSpec> {
        Ccu3W::new(self, 11)
    }
    #[doc = "Bit 12 - CCD4 Event Mask"]
    #[inline(always)]
    pub fn ccd4(&mut self) -> Ccd4W<'_, ImaskSpec> {
        Ccd4W::new(self, 12)
    }
    #[doc = "Bit 13 - CCD5 Event Mask"]
    #[inline(always)]
    pub fn ccd5(&mut self) -> Ccd5W<'_, ImaskSpec> {
        Ccd5W::new(self, 13)
    }
    #[doc = "Bit 14 - CCU4 Event Mask"]
    #[inline(always)]
    pub fn ccu4(&mut self) -> Ccu4W<'_, ImaskSpec> {
        Ccu4W::new(self, 14)
    }
    #[doc = "Bit 15 - CCU5 Event Mask"]
    #[inline(always)]
    pub fn ccu5(&mut self) -> Ccu5W<'_, ImaskSpec> {
        Ccu5W::new(self, 15)
    }
    #[doc = "Bit 24 - Fault Event mask"]
    #[inline(always)]
    pub fn f(&mut self) -> FW<'_, ImaskSpec> {
        FW::new(self, 24)
    }
    #[doc = "Bit 25 - Trigger Overflow Event mask"]
    #[inline(always)]
    pub fn tov(&mut self) -> TovW<'_, ImaskSpec> {
        TovW::new(self, 25)
    }
    #[doc = "Bit 26 - Repeat Counter Zero Event mask"]
    #[inline(always)]
    pub fn repc(&mut self) -> RepcW<'_, ImaskSpec> {
        RepcW::new(self, 26)
    }
    #[doc = "Bit 27 - Direction Change Event mask"]
    #[inline(always)]
    pub fn dc(&mut self) -> DcW<'_, ImaskSpec> {
        DcW::new(self, 27)
    }
    #[doc = "Bit 28 - Direction Change Event mask"]
    #[inline(always)]
    pub fn qeierr(&mut self) -> QeierrW<'_, ImaskSpec> {
        QeierrW::new(self, 28)
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
