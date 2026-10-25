#[doc = "Register `OUTPUTXBARFLAGINVERT` reader"]
pub type R = crate::R<OutputxbarflaginvertSpec>;
#[doc = "Register `OUTPUTXBARFLAGINVERT` writer"]
pub type W = crate::W<OutputxbarflaginvertSpec>;
#[doc = "Field `XBAR1` reader - Invert XBAR Output Before Latch for XBAR1"]
pub type Xbar1R = crate::BitReader;
#[doc = "Field `XBAR1` writer - Invert XBAR Output Before Latch for XBAR1"]
pub type Xbar1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XBAR2` reader - Invert XBAR Output Before Latch for XBAR2"]
pub type Xbar2R = crate::BitReader;
#[doc = "Field `XBAR2` writer - Invert XBAR Output Before Latch for XBAR2"]
pub type Xbar2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XBAR3` reader - Invert XBAR Output Before Latch for XBAR3"]
pub type Xbar3R = crate::BitReader;
#[doc = "Field `XBAR3` writer - Invert XBAR Output Before Latch for XBAR3"]
pub type Xbar3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XBAR4` reader - Invert XBAR Output Before Latch for XBAR4"]
pub type Xbar4R = crate::BitReader;
#[doc = "Field `XBAR4` writer - Invert XBAR Output Before Latch for XBAR4"]
pub type Xbar4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XBAR5` reader - Invert XBAR Output Before Latch for XBAR5"]
pub type Xbar5R = crate::BitReader;
#[doc = "Field `XBAR5` writer - Invert XBAR Output Before Latch for XBAR5"]
pub type Xbar5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XBAR6` reader - Invert XBAR Output Before Latch for XBAR6"]
pub type Xbar6R = crate::BitReader;
#[doc = "Field `XBAR6` writer - Invert XBAR Output Before Latch for XBAR6"]
pub type Xbar6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XBAR7` reader - Invert XBAR Output Before Latch for XBAR7"]
pub type Xbar7R = crate::BitReader;
#[doc = "Field `XBAR7` writer - Invert XBAR Output Before Latch for XBAR7"]
pub type Xbar7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XBAR8` reader - Invert XBAR Output Before Latch for XBAR8"]
pub type Xbar8R = crate::BitReader;
#[doc = "Field `XBAR8` writer - Invert XBAR Output Before Latch for XBAR8"]
pub type Xbar8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XBAR9` reader - Invert XBAR Output Before Latch for XBAR9"]
pub type Xbar9R = crate::BitReader;
#[doc = "Field `XBAR9` writer - Invert XBAR Output Before Latch for XBAR9"]
pub type Xbar9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XBAR10` reader - Invert XBAR Output Before Latch for XBAR10"]
pub type Xbar10R = crate::BitReader;
#[doc = "Field `XBAR10` writer - Invert XBAR Output Before Latch for XBAR10"]
pub type Xbar10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XBAR11` reader - Invert XBAR Output Before Latch for XBAR11"]
pub type Xbar11R = crate::BitReader;
#[doc = "Field `XBAR11` writer - Invert XBAR Output Before Latch for XBAR11"]
pub type Xbar11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XBAR12` reader - Invert XBAR Output Before Latch for XBAR12"]
pub type Xbar12R = crate::BitReader;
#[doc = "Field `XBAR12` writer - Invert XBAR Output Before Latch for XBAR12"]
pub type Xbar12W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Invert XBAR Output Before Latch for XBAR1"]
    #[inline(always)]
    pub fn xbar1(&self) -> Xbar1R {
        Xbar1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Invert XBAR Output Before Latch for XBAR2"]
    #[inline(always)]
    pub fn xbar2(&self) -> Xbar2R {
        Xbar2R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Invert XBAR Output Before Latch for XBAR3"]
    #[inline(always)]
    pub fn xbar3(&self) -> Xbar3R {
        Xbar3R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Invert XBAR Output Before Latch for XBAR4"]
    #[inline(always)]
    pub fn xbar4(&self) -> Xbar4R {
        Xbar4R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Invert XBAR Output Before Latch for XBAR5"]
    #[inline(always)]
    pub fn xbar5(&self) -> Xbar5R {
        Xbar5R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Invert XBAR Output Before Latch for XBAR6"]
    #[inline(always)]
    pub fn xbar6(&self) -> Xbar6R {
        Xbar6R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Invert XBAR Output Before Latch for XBAR7"]
    #[inline(always)]
    pub fn xbar7(&self) -> Xbar7R {
        Xbar7R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Invert XBAR Output Before Latch for XBAR8"]
    #[inline(always)]
    pub fn xbar8(&self) -> Xbar8R {
        Xbar8R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Invert XBAR Output Before Latch for XBAR9"]
    #[inline(always)]
    pub fn xbar9(&self) -> Xbar9R {
        Xbar9R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Invert XBAR Output Before Latch for XBAR10"]
    #[inline(always)]
    pub fn xbar10(&self) -> Xbar10R {
        Xbar10R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Invert XBAR Output Before Latch for XBAR11"]
    #[inline(always)]
    pub fn xbar11(&self) -> Xbar11R {
        Xbar11R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Invert XBAR Output Before Latch for XBAR12"]
    #[inline(always)]
    pub fn xbar12(&self) -> Xbar12R {
        Xbar12R::new(((self.bits >> 11) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OUTPUTXBARFLAGINVERT")
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
    #[doc = "Bit 0 - Invert XBAR Output Before Latch for XBAR1"]
    #[inline(always)]
    pub fn xbar1(&mut self) -> Xbar1W<'_, OutputxbarflaginvertSpec> {
        Xbar1W::new(self, 0)
    }
    #[doc = "Bit 1 - Invert XBAR Output Before Latch for XBAR2"]
    #[inline(always)]
    pub fn xbar2(&mut self) -> Xbar2W<'_, OutputxbarflaginvertSpec> {
        Xbar2W::new(self, 1)
    }
    #[doc = "Bit 2 - Invert XBAR Output Before Latch for XBAR3"]
    #[inline(always)]
    pub fn xbar3(&mut self) -> Xbar3W<'_, OutputxbarflaginvertSpec> {
        Xbar3W::new(self, 2)
    }
    #[doc = "Bit 3 - Invert XBAR Output Before Latch for XBAR4"]
    #[inline(always)]
    pub fn xbar4(&mut self) -> Xbar4W<'_, OutputxbarflaginvertSpec> {
        Xbar4W::new(self, 3)
    }
    #[doc = "Bit 4 - Invert XBAR Output Before Latch for XBAR5"]
    #[inline(always)]
    pub fn xbar5(&mut self) -> Xbar5W<'_, OutputxbarflaginvertSpec> {
        Xbar5W::new(self, 4)
    }
    #[doc = "Bit 5 - Invert XBAR Output Before Latch for XBAR6"]
    #[inline(always)]
    pub fn xbar6(&mut self) -> Xbar6W<'_, OutputxbarflaginvertSpec> {
        Xbar6W::new(self, 5)
    }
    #[doc = "Bit 6 - Invert XBAR Output Before Latch for XBAR7"]
    #[inline(always)]
    pub fn xbar7(&mut self) -> Xbar7W<'_, OutputxbarflaginvertSpec> {
        Xbar7W::new(self, 6)
    }
    #[doc = "Bit 7 - Invert XBAR Output Before Latch for XBAR8"]
    #[inline(always)]
    pub fn xbar8(&mut self) -> Xbar8W<'_, OutputxbarflaginvertSpec> {
        Xbar8W::new(self, 7)
    }
    #[doc = "Bit 8 - Invert XBAR Output Before Latch for XBAR9"]
    #[inline(always)]
    pub fn xbar9(&mut self) -> Xbar9W<'_, OutputxbarflaginvertSpec> {
        Xbar9W::new(self, 8)
    }
    #[doc = "Bit 9 - Invert XBAR Output Before Latch for XBAR10"]
    #[inline(always)]
    pub fn xbar10(&mut self) -> Xbar10W<'_, OutputxbarflaginvertSpec> {
        Xbar10W::new(self, 9)
    }
    #[doc = "Bit 10 - Invert XBAR Output Before Latch for XBAR11"]
    #[inline(always)]
    pub fn xbar11(&mut self) -> Xbar11W<'_, OutputxbarflaginvertSpec> {
        Xbar11W::new(self, 10)
    }
    #[doc = "Bit 11 - Invert XBAR Output Before Latch for XBAR12"]
    #[inline(always)]
    pub fn xbar12(&mut self) -> Xbar12W<'_, OutputxbarflaginvertSpec> {
        Xbar12W::new(self, 11)
    }
}
#[doc = "Output Signal inversion register\n\nYou can [`read`](crate::Reg::read) this register and get [`outputxbarflaginvert::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`outputxbarflaginvert::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OutputxbarflaginvertSpec;
impl crate::RegisterSpec for OutputxbarflaginvertSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`outputxbarflaginvert::R`](R) reader structure"]
impl crate::Readable for OutputxbarflaginvertSpec {}
#[doc = "`write(|w| ..)` method takes [`outputxbarflaginvert::W`](W) writer structure"]
impl crate::Writable for OutputxbarflaginvertSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets OUTPUTXBARFLAGINVERT to value 0"]
impl crate::Resettable for OutputxbarflaginvertSpec {}
