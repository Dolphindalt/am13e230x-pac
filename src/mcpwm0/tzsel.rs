#[doc = "Register `TZSEL` reader"]
pub type R = crate::R<TzselSpec>;
#[doc = "Register `TZSEL` writer"]
pub type W = crate::W<TzselSpec>;
#[doc = "Field `CBC1` reader - CBC TZ1 select"]
pub type Cbc1R = crate::BitReader;
#[doc = "Field `CBC1` writer - CBC TZ1 select"]
pub type Cbc1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CBC2` reader - CBC TZ2 select"]
pub type Cbc2R = crate::BitReader;
#[doc = "Field `CBC2` writer - CBC TZ2 select"]
pub type Cbc2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CBC3` reader - CBC TZ3 select"]
pub type Cbc3R = crate::BitReader;
#[doc = "Field `CBC3` writer - CBC TZ3 select"]
pub type Cbc3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CBC4` reader - CBC TZ4 select"]
pub type Cbc4R = crate::BitReader;
#[doc = "Field `CBC4` writer - CBC TZ4 select"]
pub type Cbc4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CBC5` reader - CBC TZ5 select"]
pub type Cbc5R = crate::BitReader;
#[doc = "Field `CBC5` writer - CBC TZ5 select"]
pub type Cbc5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CBC6` reader - CBC TZ6 select"]
pub type Cbc6R = crate::BitReader;
#[doc = "Field `CBC6` writer - CBC TZ6 select"]
pub type Cbc6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CBC7` reader - CBC TZ7 select"]
pub type Cbc7R = crate::BitReader;
#[doc = "Field `CBC7` writer - CBC TZ7 select"]
pub type Cbc7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CBC8` reader - CBC TZ8 select"]
pub type Cbc8R = crate::BitReader;
#[doc = "Field `CBC8` writer - CBC TZ8 select"]
pub type Cbc8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OST1` reader - OST TZ1 select"]
pub type Ost1R = crate::BitReader;
#[doc = "Field `OST1` writer - OST TZ1 select"]
pub type Ost1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OST2` reader - OST TZ2 select"]
pub type Ost2R = crate::BitReader;
#[doc = "Field `OST2` writer - OST TZ2 select"]
pub type Ost2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OST3` reader - OST TZ3 select"]
pub type Ost3R = crate::BitReader;
#[doc = "Field `OST3` writer - OST TZ3 select"]
pub type Ost3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OST4` reader - OST TZ4 select"]
pub type Ost4R = crate::BitReader;
#[doc = "Field `OST4` writer - OST TZ4 select"]
pub type Ost4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OST5` reader - OST TZ5 select"]
pub type Ost5R = crate::BitReader;
#[doc = "Field `OST5` writer - OST TZ5 select"]
pub type Ost5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OST6` reader - OST TZ6 select"]
pub type Ost6R = crate::BitReader;
#[doc = "Field `OST6` writer - OST TZ6 select"]
pub type Ost6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OST7` reader - OST TZ7 select"]
pub type Ost7R = crate::BitReader;
#[doc = "Field `OST7` writer - OST TZ7 select"]
pub type Ost7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OST8` reader - OST TZ8 select"]
pub type Ost8R = crate::BitReader;
#[doc = "Field `OST8` writer - OST TZ8 select"]
pub type Ost8W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - CBC TZ1 select"]
    #[inline(always)]
    pub fn cbc1(&self) -> Cbc1R {
        Cbc1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CBC TZ2 select"]
    #[inline(always)]
    pub fn cbc2(&self) -> Cbc2R {
        Cbc2R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CBC TZ3 select"]
    #[inline(always)]
    pub fn cbc3(&self) -> Cbc3R {
        Cbc3R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CBC TZ4 select"]
    #[inline(always)]
    pub fn cbc4(&self) -> Cbc4R {
        Cbc4R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CBC TZ5 select"]
    #[inline(always)]
    pub fn cbc5(&self) -> Cbc5R {
        Cbc5R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CBC TZ6 select"]
    #[inline(always)]
    pub fn cbc6(&self) -> Cbc6R {
        Cbc6R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - CBC TZ7 select"]
    #[inline(always)]
    pub fn cbc7(&self) -> Cbc7R {
        Cbc7R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - CBC TZ8 select"]
    #[inline(always)]
    pub fn cbc8(&self) -> Cbc8R {
        Cbc8R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 16 - OST TZ1 select"]
    #[inline(always)]
    pub fn ost1(&self) -> Ost1R {
        Ost1R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - OST TZ2 select"]
    #[inline(always)]
    pub fn ost2(&self) -> Ost2R {
        Ost2R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - OST TZ3 select"]
    #[inline(always)]
    pub fn ost3(&self) -> Ost3R {
        Ost3R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - OST TZ4 select"]
    #[inline(always)]
    pub fn ost4(&self) -> Ost4R {
        Ost4R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - OST TZ5 select"]
    #[inline(always)]
    pub fn ost5(&self) -> Ost5R {
        Ost5R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - OST TZ6 select"]
    #[inline(always)]
    pub fn ost6(&self) -> Ost6R {
        Ost6R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - OST TZ7 select"]
    #[inline(always)]
    pub fn ost7(&self) -> Ost7R {
        Ost7R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - OST TZ8 select"]
    #[inline(always)]
    pub fn ost8(&self) -> Ost8R {
        Ost8R::new(((self.bits >> 23) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TZSEL")
            .field("cbc1", &self.cbc1())
            .field("cbc2", &self.cbc2())
            .field("cbc3", &self.cbc3())
            .field("cbc4", &self.cbc4())
            .field("cbc5", &self.cbc5())
            .field("cbc6", &self.cbc6())
            .field("cbc7", &self.cbc7())
            .field("cbc8", &self.cbc8())
            .field("ost1", &self.ost1())
            .field("ost2", &self.ost2())
            .field("ost3", &self.ost3())
            .field("ost4", &self.ost4())
            .field("ost5", &self.ost5())
            .field("ost6", &self.ost6())
            .field("ost7", &self.ost7())
            .field("ost8", &self.ost8())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - CBC TZ1 select"]
    #[inline(always)]
    pub fn cbc1(&mut self) -> Cbc1W<'_, TzselSpec> {
        Cbc1W::new(self, 0)
    }
    #[doc = "Bit 1 - CBC TZ2 select"]
    #[inline(always)]
    pub fn cbc2(&mut self) -> Cbc2W<'_, TzselSpec> {
        Cbc2W::new(self, 1)
    }
    #[doc = "Bit 2 - CBC TZ3 select"]
    #[inline(always)]
    pub fn cbc3(&mut self) -> Cbc3W<'_, TzselSpec> {
        Cbc3W::new(self, 2)
    }
    #[doc = "Bit 3 - CBC TZ4 select"]
    #[inline(always)]
    pub fn cbc4(&mut self) -> Cbc4W<'_, TzselSpec> {
        Cbc4W::new(self, 3)
    }
    #[doc = "Bit 4 - CBC TZ5 select"]
    #[inline(always)]
    pub fn cbc5(&mut self) -> Cbc5W<'_, TzselSpec> {
        Cbc5W::new(self, 4)
    }
    #[doc = "Bit 5 - CBC TZ6 select"]
    #[inline(always)]
    pub fn cbc6(&mut self) -> Cbc6W<'_, TzselSpec> {
        Cbc6W::new(self, 5)
    }
    #[doc = "Bit 6 - CBC TZ7 select"]
    #[inline(always)]
    pub fn cbc7(&mut self) -> Cbc7W<'_, TzselSpec> {
        Cbc7W::new(self, 6)
    }
    #[doc = "Bit 7 - CBC TZ8 select"]
    #[inline(always)]
    pub fn cbc8(&mut self) -> Cbc8W<'_, TzselSpec> {
        Cbc8W::new(self, 7)
    }
    #[doc = "Bit 16 - OST TZ1 select"]
    #[inline(always)]
    pub fn ost1(&mut self) -> Ost1W<'_, TzselSpec> {
        Ost1W::new(self, 16)
    }
    #[doc = "Bit 17 - OST TZ2 select"]
    #[inline(always)]
    pub fn ost2(&mut self) -> Ost2W<'_, TzselSpec> {
        Ost2W::new(self, 17)
    }
    #[doc = "Bit 18 - OST TZ3 select"]
    #[inline(always)]
    pub fn ost3(&mut self) -> Ost3W<'_, TzselSpec> {
        Ost3W::new(self, 18)
    }
    #[doc = "Bit 19 - OST TZ4 select"]
    #[inline(always)]
    pub fn ost4(&mut self) -> Ost4W<'_, TzselSpec> {
        Ost4W::new(self, 19)
    }
    #[doc = "Bit 20 - OST TZ5 select"]
    #[inline(always)]
    pub fn ost5(&mut self) -> Ost5W<'_, TzselSpec> {
        Ost5W::new(self, 20)
    }
    #[doc = "Bit 21 - OST TZ6 select"]
    #[inline(always)]
    pub fn ost6(&mut self) -> Ost6W<'_, TzselSpec> {
        Ost6W::new(self, 21)
    }
    #[doc = "Bit 22 - OST TZ7 select"]
    #[inline(always)]
    pub fn ost7(&mut self) -> Ost7W<'_, TzselSpec> {
        Ost7W::new(self, 22)
    }
    #[doc = "Bit 23 - OST TZ8 select"]
    #[inline(always)]
    pub fn ost8(&mut self) -> Ost8W<'_, TzselSpec> {
        Ost8W::new(self, 23)
    }
}
#[doc = "Trip Zone selection\n\nYou can [`read`](crate::Reg::read) this register and get [`tzsel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzsel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TzselSpec;
impl crate::RegisterSpec for TzselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzsel::R`](R) reader structure"]
impl crate::Readable for TzselSpec {}
#[doc = "`write(|w| ..)` method takes [`tzsel::W`](W) writer structure"]
impl crate::Writable for TzselSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets TZSEL to value 0"]
impl crate::Resettable for TzselSpec {}
