#[doc = "Register `TZCBCOSTFLAG` reader"]
pub type R = crate::R<TzcbcostflagSpec>;
#[doc = "Register `TZCBCOSTFLAG` writer"]
pub type W = crate::W<TzcbcostflagSpec>;
#[doc = "Field `CBC1` reader - CBC TZ1 Flag"]
pub type Cbc1R = crate::BitReader;
#[doc = "Field `CBC2` reader - CBC TZ2 Flag"]
pub type Cbc2R = crate::BitReader;
#[doc = "Field `CBC3` reader - CBC TZ3 Flag"]
pub type Cbc3R = crate::BitReader;
#[doc = "Field `CBC4` reader - CBC TZ4 Flag"]
pub type Cbc4R = crate::BitReader;
#[doc = "Field `CBC5` reader - CBC TZ5 Flag"]
pub type Cbc5R = crate::BitReader;
#[doc = "Field `CBC6` reader - CBC TZ6 Flag"]
pub type Cbc6R = crate::BitReader;
#[doc = "Field `CBC7` reader - CBC TZ7 Flag"]
pub type Cbc7R = crate::BitReader;
#[doc = "Field `CBC8` reader - OST TZ8 Flag"]
pub type Cbc8R = crate::BitReader;
#[doc = "Field `OST1` reader - OST TZ1 Flag"]
pub type Ost1R = crate::BitReader;
#[doc = "Field `OST2` reader - OST TZ2 Flag"]
pub type Ost2R = crate::BitReader;
#[doc = "Field `OST3` reader - OST TZ3 Flag"]
pub type Ost3R = crate::BitReader;
#[doc = "Field `OST4` reader - OST TZ4 Flag"]
pub type Ost4R = crate::BitReader;
#[doc = "Field `OST5` reader - OST TZ5 Flag"]
pub type Ost5R = crate::BitReader;
#[doc = "Field `OST6` reader - OST TZ6 Flag"]
pub type Ost6R = crate::BitReader;
#[doc = "Field `OST7` reader - OST TZ7 Flag"]
pub type Ost7R = crate::BitReader;
#[doc = "Field `OST8` reader - OST TZ8 Flag"]
pub type Ost8R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - CBC TZ1 Flag"]
    #[inline(always)]
    pub fn cbc1(&self) -> Cbc1R {
        Cbc1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CBC TZ2 Flag"]
    #[inline(always)]
    pub fn cbc2(&self) -> Cbc2R {
        Cbc2R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CBC TZ3 Flag"]
    #[inline(always)]
    pub fn cbc3(&self) -> Cbc3R {
        Cbc3R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CBC TZ4 Flag"]
    #[inline(always)]
    pub fn cbc4(&self) -> Cbc4R {
        Cbc4R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CBC TZ5 Flag"]
    #[inline(always)]
    pub fn cbc5(&self) -> Cbc5R {
        Cbc5R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CBC TZ6 Flag"]
    #[inline(always)]
    pub fn cbc6(&self) -> Cbc6R {
        Cbc6R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - CBC TZ7 Flag"]
    #[inline(always)]
    pub fn cbc7(&self) -> Cbc7R {
        Cbc7R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - OST TZ8 Flag"]
    #[inline(always)]
    pub fn cbc8(&self) -> Cbc8R {
        Cbc8R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 16 - OST TZ1 Flag"]
    #[inline(always)]
    pub fn ost1(&self) -> Ost1R {
        Ost1R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - OST TZ2 Flag"]
    #[inline(always)]
    pub fn ost2(&self) -> Ost2R {
        Ost2R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - OST TZ3 Flag"]
    #[inline(always)]
    pub fn ost3(&self) -> Ost3R {
        Ost3R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - OST TZ4 Flag"]
    #[inline(always)]
    pub fn ost4(&self) -> Ost4R {
        Ost4R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - OST TZ5 Flag"]
    #[inline(always)]
    pub fn ost5(&self) -> Ost5R {
        Ost5R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - OST TZ6 Flag"]
    #[inline(always)]
    pub fn ost6(&self) -> Ost6R {
        Ost6R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - OST TZ7 Flag"]
    #[inline(always)]
    pub fn ost7(&self) -> Ost7R {
        Ost7R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - OST TZ8 Flag"]
    #[inline(always)]
    pub fn ost8(&self) -> Ost8R {
        Ost8R::new(((self.bits >> 23) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TZCBCOSTFLAG")
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
impl W {}
#[doc = "Trip zone CBCOST flag\n\nYou can [`read`](crate::Reg::read) this register and get [`tzcbcostflag::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzcbcostflag::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TzcbcostflagSpec;
impl crate::RegisterSpec for TzcbcostflagSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzcbcostflag::R`](R) reader structure"]
impl crate::Readable for TzcbcostflagSpec {}
#[doc = "`write(|w| ..)` method takes [`tzcbcostflag::W`](W) writer structure"]
impl crate::Writable for TzcbcostflagSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets TZCBCOSTFLAG to value 0"]
impl crate::Resettable for TzcbcostflagSpec {}
