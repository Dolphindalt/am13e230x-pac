#[doc = "Register `OUTPUTXBAROUTLATCH` reader"]
pub type R = crate::R<OutputxbaroutlatchSpec>;
#[doc = "Register `OUTPUTXBAROUTLATCH` writer"]
pub type W = crate::W<OutputxbaroutlatchSpec>;
#[doc = "Field `XBAR1` reader - XBAR Output Latch Select for XBAR1"]
pub type Xbar1R = crate::BitReader;
#[doc = "Field `XBAR1` writer - XBAR Output Latch Select for XBAR1"]
pub type Xbar1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XBAR2` reader - XBAR Output Latch Select for XBAR2"]
pub type Xbar2R = crate::BitReader;
#[doc = "Field `XBAR2` writer - XBAR Output Latch Select for XBAR2"]
pub type Xbar2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XBAR3` reader - XBAR Output Latch Select for XBAR3"]
pub type Xbar3R = crate::BitReader;
#[doc = "Field `XBAR3` writer - XBAR Output Latch Select for XBAR3"]
pub type Xbar3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XBAR4` reader - XBAR Output Latch Select for XBAR4"]
pub type Xbar4R = crate::BitReader;
#[doc = "Field `XBAR4` writer - XBAR Output Latch Select for XBAR4"]
pub type Xbar4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XBAR5` reader - XBAR Output Latch Select for XBAR5"]
pub type Xbar5R = crate::BitReader;
#[doc = "Field `XBAR5` writer - XBAR Output Latch Select for XBAR5"]
pub type Xbar5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XBAR6` reader - XBAR Output Latch Select for XBAR6"]
pub type Xbar6R = crate::BitReader;
#[doc = "Field `XBAR6` writer - XBAR Output Latch Select for XBAR6"]
pub type Xbar6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XBAR7` reader - XBAR Output Latch Select for XBAR7"]
pub type Xbar7R = crate::BitReader;
#[doc = "Field `XBAR7` writer - XBAR Output Latch Select for XBAR7"]
pub type Xbar7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XBAR8` reader - XBAR Output Latch Select for XBAR8"]
pub type Xbar8R = crate::BitReader;
#[doc = "Field `XBAR8` writer - XBAR Output Latch Select for XBAR8"]
pub type Xbar8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XBAR9` reader - XBAR Output Latch Select for XBAR9"]
pub type Xbar9R = crate::BitReader;
#[doc = "Field `XBAR9` writer - XBAR Output Latch Select for XBAR9"]
pub type Xbar9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XBAR10` reader - XBAR Output Latch Select for XBAR10"]
pub type Xbar10R = crate::BitReader;
#[doc = "Field `XBAR10` writer - XBAR Output Latch Select for XBAR10"]
pub type Xbar10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XBAR11` reader - XBAR Output Latch Select for XBAR11"]
pub type Xbar11R = crate::BitReader;
#[doc = "Field `XBAR11` writer - XBAR Output Latch Select for XBAR11"]
pub type Xbar11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XBAR12` reader - XBAR Output Latch Select for XBAR12"]
pub type Xbar12R = crate::BitReader;
#[doc = "Field `XBAR12` writer - XBAR Output Latch Select for XBAR12"]
pub type Xbar12W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - XBAR Output Latch Select for XBAR1"]
    #[inline(always)]
    pub fn xbar1(&self) -> Xbar1R {
        Xbar1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - XBAR Output Latch Select for XBAR2"]
    #[inline(always)]
    pub fn xbar2(&self) -> Xbar2R {
        Xbar2R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - XBAR Output Latch Select for XBAR3"]
    #[inline(always)]
    pub fn xbar3(&self) -> Xbar3R {
        Xbar3R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - XBAR Output Latch Select for XBAR4"]
    #[inline(always)]
    pub fn xbar4(&self) -> Xbar4R {
        Xbar4R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - XBAR Output Latch Select for XBAR5"]
    #[inline(always)]
    pub fn xbar5(&self) -> Xbar5R {
        Xbar5R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - XBAR Output Latch Select for XBAR6"]
    #[inline(always)]
    pub fn xbar6(&self) -> Xbar6R {
        Xbar6R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - XBAR Output Latch Select for XBAR7"]
    #[inline(always)]
    pub fn xbar7(&self) -> Xbar7R {
        Xbar7R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - XBAR Output Latch Select for XBAR8"]
    #[inline(always)]
    pub fn xbar8(&self) -> Xbar8R {
        Xbar8R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - XBAR Output Latch Select for XBAR9"]
    #[inline(always)]
    pub fn xbar9(&self) -> Xbar9R {
        Xbar9R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - XBAR Output Latch Select for XBAR10"]
    #[inline(always)]
    pub fn xbar10(&self) -> Xbar10R {
        Xbar10R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - XBAR Output Latch Select for XBAR11"]
    #[inline(always)]
    pub fn xbar11(&self) -> Xbar11R {
        Xbar11R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - XBAR Output Latch Select for XBAR12"]
    #[inline(always)]
    pub fn xbar12(&self) -> Xbar12R {
        Xbar12R::new(((self.bits >> 11) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OUTPUTXBAROUTLATCH")
            .field("xbar1", &self.xbar1())
            .field("xbar2", &self.xbar2())
            .field("xbar3", &self.xbar3())
            .field("xbar4", &self.xbar4())
            .field("xbar5", &self.xbar5())
            .field("xbar6", &self.xbar6())
            .field("xbar7", &self.xbar7())
            .field("xbar8", &self.xbar8())
            .field("xbar9", &self.xbar9())
            .field("xbar10", &self.xbar10())
            .field("xbar11", &self.xbar11())
            .field("xbar12", &self.xbar12())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - XBAR Output Latch Select for XBAR1"]
    #[inline(always)]
    pub fn xbar1(&mut self) -> Xbar1W<'_, OutputxbaroutlatchSpec> {
        Xbar1W::new(self, 0)
    }
    #[doc = "Bit 1 - XBAR Output Latch Select for XBAR2"]
    #[inline(always)]
    pub fn xbar2(&mut self) -> Xbar2W<'_, OutputxbaroutlatchSpec> {
        Xbar2W::new(self, 1)
    }
    #[doc = "Bit 2 - XBAR Output Latch Select for XBAR3"]
    #[inline(always)]
    pub fn xbar3(&mut self) -> Xbar3W<'_, OutputxbaroutlatchSpec> {
        Xbar3W::new(self, 2)
    }
    #[doc = "Bit 3 - XBAR Output Latch Select for XBAR4"]
    #[inline(always)]
    pub fn xbar4(&mut self) -> Xbar4W<'_, OutputxbaroutlatchSpec> {
        Xbar4W::new(self, 3)
    }
    #[doc = "Bit 4 - XBAR Output Latch Select for XBAR5"]
    #[inline(always)]
    pub fn xbar5(&mut self) -> Xbar5W<'_, OutputxbaroutlatchSpec> {
        Xbar5W::new(self, 4)
    }
    #[doc = "Bit 5 - XBAR Output Latch Select for XBAR6"]
    #[inline(always)]
    pub fn xbar6(&mut self) -> Xbar6W<'_, OutputxbaroutlatchSpec> {
        Xbar6W::new(self, 5)
    }
    #[doc = "Bit 6 - XBAR Output Latch Select for XBAR7"]
    #[inline(always)]
    pub fn xbar7(&mut self) -> Xbar7W<'_, OutputxbaroutlatchSpec> {
        Xbar7W::new(self, 6)
    }
    #[doc = "Bit 7 - XBAR Output Latch Select for XBAR8"]
    #[inline(always)]
    pub fn xbar8(&mut self) -> Xbar8W<'_, OutputxbaroutlatchSpec> {
        Xbar8W::new(self, 7)
    }
    #[doc = "Bit 8 - XBAR Output Latch Select for XBAR9"]
    #[inline(always)]
    pub fn xbar9(&mut self) -> Xbar9W<'_, OutputxbaroutlatchSpec> {
        Xbar9W::new(self, 8)
    }
    #[doc = "Bit 9 - XBAR Output Latch Select for XBAR10"]
    #[inline(always)]
    pub fn xbar10(&mut self) -> Xbar10W<'_, OutputxbaroutlatchSpec> {
        Xbar10W::new(self, 9)
    }
    #[doc = "Bit 10 - XBAR Output Latch Select for XBAR11"]
    #[inline(always)]
    pub fn xbar11(&mut self) -> Xbar11W<'_, OutputxbaroutlatchSpec> {
        Xbar11W::new(self, 10)
    }
    #[doc = "Bit 11 - XBAR Output Latch Select for XBAR12"]
    #[inline(always)]
    pub fn xbar12(&mut self) -> Xbar12W<'_, OutputxbaroutlatchSpec> {
        Xbar12W::new(self, 11)
    }
}
#[doc = "Output Signal Select Latch\n\nYou can [`read`](crate::Reg::read) this register and get [`outputxbaroutlatch::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`outputxbaroutlatch::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OutputxbaroutlatchSpec;
impl crate::RegisterSpec for OutputxbaroutlatchSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`outputxbaroutlatch::R`](R) reader structure"]
impl crate::Readable for OutputxbaroutlatchSpec {}
#[doc = "`write(|w| ..)` method takes [`outputxbaroutlatch::W`](W) writer structure"]
impl crate::Writable for OutputxbaroutlatchSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets OUTPUTXBAROUTLATCH to value 0"]
impl crate::Resettable for OutputxbaroutlatchSpec {}
