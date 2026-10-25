#[doc = "Register `IMASK` reader"]
pub type R = crate::R<ImaskSpec>;
#[doc = "Register `IMASK` writer"]
pub type W = crate::W<ImaskSpec>;
#[doc = "Field `DMACH0` reader - DMA Channel 0 interrupt signal. Size counter reached zero (DMASZ=0)."]
pub type Dmach0R = crate::BitReader;
#[doc = "Field `DMACH0` writer - DMA Channel 0 interrupt signal. Size counter reached zero (DMASZ=0)."]
pub type Dmach0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMACH1` reader - DMA Channel 1 interrupt signal. Size counter reached zero (DMASZ=0)."]
pub type Dmach1R = crate::BitReader;
#[doc = "Field `DMACH1` writer - DMA Channel 1 interrupt signal. Size counter reached zero (DMASZ=0)."]
pub type Dmach1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMACH2` reader - DMA Channel 2 interrupt signal. Size counter reached zero (DMASZ=0)."]
pub type Dmach2R = crate::BitReader;
#[doc = "Field `DMACH2` writer - DMA Channel 2 interrupt signal. Size counter reached zero (DMASZ=0)."]
pub type Dmach2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMACH3` reader - DMA Channel 3 interrupt signal. Size counter reached zero (DMASZ=0)."]
pub type Dmach3R = crate::BitReader;
#[doc = "Field `DMACH3` writer - DMA Channel 3 interrupt signal. Size counter reached zero (DMASZ=0)."]
pub type Dmach3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMACH4` reader - DMA Channel 4 interrupt signal. Size counter reached zero (DMASZ=0)."]
pub type Dmach4R = crate::BitReader;
#[doc = "Field `DMACH4` writer - DMA Channel 4 interrupt signal. Size counter reached zero (DMASZ=0)."]
pub type Dmach4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMACH5` reader - DMA Channel 5 interrupt signal. Size counter reached zero (DMASZ=0)."]
pub type Dmach5R = crate::BitReader;
#[doc = "Field `DMACH5` writer - DMA Channel 5 interrupt signal. Size counter reached zero (DMASZ=0)."]
pub type Dmach5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMACH6` reader - DMA Channel 6 interrupt signal. Size counter reached zero (DMASZ=0)."]
pub type Dmach6R = crate::BitReader;
#[doc = "Field `DMACH6` writer - DMA Channel 6 interrupt signal. Size counter reached zero (DMASZ=0)."]
pub type Dmach6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMACH7` reader - DMA Channel 7 interrupt signal. Size counter reached zero (DMASZ=0)."]
pub type Dmach7R = crate::BitReader;
#[doc = "Field `DMACH7` writer - DMA Channel 7 interrupt signal. Size counter reached zero (DMASZ=0)."]
pub type Dmach7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMACH8` reader - DMA Channel 8 interrupt signal. Size counter reached zero (DMASZ=0)."]
pub type Dmach8R = crate::BitReader;
#[doc = "Field `DMACH8` writer - DMA Channel 8 interrupt signal. Size counter reached zero (DMASZ=0)."]
pub type Dmach8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMACH9` reader - DMA Channel 9 interrupt signal. Size counter reached zero (DMASZ=0)."]
pub type Dmach9R = crate::BitReader;
#[doc = "Field `DMACH9` writer - DMA Channel 9 interrupt signal. Size counter reached zero (DMASZ=0)."]
pub type Dmach9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMACH10` reader - DMA Channel 10 interrupt signal. Size counter reached zero (DMASZ=0)."]
pub type Dmach10R = crate::BitReader;
#[doc = "Field `DMACH10` writer - DMA Channel 10 interrupt signal. Size counter reached zero (DMASZ=0)."]
pub type Dmach10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMACH11` reader - DMA Channel 11 interrupt signal. Size counter reached zero (DMASZ=0)."]
pub type Dmach11R = crate::BitReader;
#[doc = "Field `DMACH11` writer - DMA Channel 11 interrupt signal. Size counter reached zero (DMASZ=0)."]
pub type Dmach11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PREIRQCH0` reader - Pre-IRQ for Channel 0. Size counter reached Pre-IRQ threshold."]
pub type Preirqch0R = crate::BitReader;
#[doc = "Field `PREIRQCH0` writer - Pre-IRQ for Channel 0. Size counter reached Pre-IRQ threshold."]
pub type Preirqch0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PREIRQCH1` reader - Pre-IRQ for Channel 1. Size counter reached Pre-IRQ threshold."]
pub type Preirqch1R = crate::BitReader;
#[doc = "Field `PREIRQCH1` writer - Pre-IRQ for Channel 1. Size counter reached Pre-IRQ threshold."]
pub type Preirqch1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PREIRQCH2` reader - Pre-IRQ for Channel 2. Size counter reached Pre-IRQ threshold."]
pub type Preirqch2R = crate::BitReader;
#[doc = "Field `PREIRQCH2` writer - Pre-IRQ for Channel 2. Size counter reached Pre-IRQ threshold."]
pub type Preirqch2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PREIRQCH3` reader - Pre-IRQ for Channel 3. Size counter reached Pre-IRQ threshold."]
pub type Preirqch3R = crate::BitReader;
#[doc = "Field `PREIRQCH3` writer - Pre-IRQ for Channel 3. Size counter reached Pre-IRQ threshold."]
pub type Preirqch3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PREIRQCH4` reader - Pre-IRQ for Channel 4. Size counter reached Pre-IRQ threshold."]
pub type Preirqch4R = crate::BitReader;
#[doc = "Field `PREIRQCH4` writer - Pre-IRQ for Channel 4. Size counter reached Pre-IRQ threshold."]
pub type Preirqch4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PREIRQCH5` reader - Pre-IRQ for Channel 5. Size counter reached Pre-IRQ threshold."]
pub type Preirqch5R = crate::BitReader;
#[doc = "Field `PREIRQCH5` writer - Pre-IRQ for Channel 5. Size counter reached Pre-IRQ threshold."]
pub type Preirqch5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDRERR` reader - DMA address error, SRC address not reachable."]
pub type AddrerrR = crate::BitReader;
#[doc = "Field `ADDRERR` writer - DMA address error, SRC address not reachable."]
pub type AddrerrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - DMA Channel 0 interrupt signal. Size counter reached zero (DMASZ=0)."]
    #[inline(always)]
    pub fn dmach0(&self) -> Dmach0R {
        Dmach0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMA Channel 1 interrupt signal. Size counter reached zero (DMASZ=0)."]
    #[inline(always)]
    pub fn dmach1(&self) -> Dmach1R {
        Dmach1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DMA Channel 2 interrupt signal. Size counter reached zero (DMASZ=0)."]
    #[inline(always)]
    pub fn dmach2(&self) -> Dmach2R {
        Dmach2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DMA Channel 3 interrupt signal. Size counter reached zero (DMASZ=0)."]
    #[inline(always)]
    pub fn dmach3(&self) -> Dmach3R {
        Dmach3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - DMA Channel 4 interrupt signal. Size counter reached zero (DMASZ=0)."]
    #[inline(always)]
    pub fn dmach4(&self) -> Dmach4R {
        Dmach4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - DMA Channel 5 interrupt signal. Size counter reached zero (DMASZ=0)."]
    #[inline(always)]
    pub fn dmach5(&self) -> Dmach5R {
        Dmach5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - DMA Channel 6 interrupt signal. Size counter reached zero (DMASZ=0)."]
    #[inline(always)]
    pub fn dmach6(&self) -> Dmach6R {
        Dmach6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - DMA Channel 7 interrupt signal. Size counter reached zero (DMASZ=0)."]
    #[inline(always)]
    pub fn dmach7(&self) -> Dmach7R {
        Dmach7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - DMA Channel 8 interrupt signal. Size counter reached zero (DMASZ=0)."]
    #[inline(always)]
    pub fn dmach8(&self) -> Dmach8R {
        Dmach8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - DMA Channel 9 interrupt signal. Size counter reached zero (DMASZ=0)."]
    #[inline(always)]
    pub fn dmach9(&self) -> Dmach9R {
        Dmach9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - DMA Channel 10 interrupt signal. Size counter reached zero (DMASZ=0)."]
    #[inline(always)]
    pub fn dmach10(&self) -> Dmach10R {
        Dmach10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - DMA Channel 11 interrupt signal. Size counter reached zero (DMASZ=0)."]
    #[inline(always)]
    pub fn dmach11(&self) -> Dmach11R {
        Dmach11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16 - Pre-IRQ for Channel 0. Size counter reached Pre-IRQ threshold."]
    #[inline(always)]
    pub fn preirqch0(&self) -> Preirqch0R {
        Preirqch0R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Pre-IRQ for Channel 1. Size counter reached Pre-IRQ threshold."]
    #[inline(always)]
    pub fn preirqch1(&self) -> Preirqch1R {
        Preirqch1R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Pre-IRQ for Channel 2. Size counter reached Pre-IRQ threshold."]
    #[inline(always)]
    pub fn preirqch2(&self) -> Preirqch2R {
        Preirqch2R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Pre-IRQ for Channel 3. Size counter reached Pre-IRQ threshold."]
    #[inline(always)]
    pub fn preirqch3(&self) -> Preirqch3R {
        Preirqch3R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Pre-IRQ for Channel 4. Size counter reached Pre-IRQ threshold."]
    #[inline(always)]
    pub fn preirqch4(&self) -> Preirqch4R {
        Preirqch4R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Pre-IRQ for Channel 5. Size counter reached Pre-IRQ threshold."]
    #[inline(always)]
    pub fn preirqch5(&self) -> Preirqch5R {
        Preirqch5R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 24 - DMA address error, SRC address not reachable."]
    #[inline(always)]
    pub fn addrerr(&self) -> AddrerrR {
        AddrerrR::new(((self.bits >> 24) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IMASK")
            .field("dmach0", &self.dmach0())
            .field("dmach1", &self.dmach1())
            .field("dmach2", &self.dmach2())
            .field("dmach3", &self.dmach3())
            .field("dmach4", &self.dmach4())
            .field("dmach5", &self.dmach5())
            .field("dmach6", &self.dmach6())
            .field("dmach7", &self.dmach7())
            .field("dmach8", &self.dmach8())
            .field("dmach9", &self.dmach9())
            .field("dmach10", &self.dmach10())
            .field("dmach11", &self.dmach11())
            .field("preirqch0", &self.preirqch0())
            .field("preirqch1", &self.preirqch1())
            .field("preirqch2", &self.preirqch2())
            .field("preirqch3", &self.preirqch3())
            .field("preirqch4", &self.preirqch4())
            .field("preirqch5", &self.preirqch5())
            .field("addrerr", &self.addrerr())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - DMA Channel 0 interrupt signal. Size counter reached zero (DMASZ=0)."]
    #[inline(always)]
    pub fn dmach0(&mut self) -> Dmach0W<'_, ImaskSpec> {
        Dmach0W::new(self, 0)
    }
    #[doc = "Bit 1 - DMA Channel 1 interrupt signal. Size counter reached zero (DMASZ=0)."]
    #[inline(always)]
    pub fn dmach1(&mut self) -> Dmach1W<'_, ImaskSpec> {
        Dmach1W::new(self, 1)
    }
    #[doc = "Bit 2 - DMA Channel 2 interrupt signal. Size counter reached zero (DMASZ=0)."]
    #[inline(always)]
    pub fn dmach2(&mut self) -> Dmach2W<'_, ImaskSpec> {
        Dmach2W::new(self, 2)
    }
    #[doc = "Bit 3 - DMA Channel 3 interrupt signal. Size counter reached zero (DMASZ=0)."]
    #[inline(always)]
    pub fn dmach3(&mut self) -> Dmach3W<'_, ImaskSpec> {
        Dmach3W::new(self, 3)
    }
    #[doc = "Bit 4 - DMA Channel 4 interrupt signal. Size counter reached zero (DMASZ=0)."]
    #[inline(always)]
    pub fn dmach4(&mut self) -> Dmach4W<'_, ImaskSpec> {
        Dmach4W::new(self, 4)
    }
    #[doc = "Bit 5 - DMA Channel 5 interrupt signal. Size counter reached zero (DMASZ=0)."]
    #[inline(always)]
    pub fn dmach5(&mut self) -> Dmach5W<'_, ImaskSpec> {
        Dmach5W::new(self, 5)
    }
    #[doc = "Bit 6 - DMA Channel 6 interrupt signal. Size counter reached zero (DMASZ=0)."]
    #[inline(always)]
    pub fn dmach6(&mut self) -> Dmach6W<'_, ImaskSpec> {
        Dmach6W::new(self, 6)
    }
    #[doc = "Bit 7 - DMA Channel 7 interrupt signal. Size counter reached zero (DMASZ=0)."]
    #[inline(always)]
    pub fn dmach7(&mut self) -> Dmach7W<'_, ImaskSpec> {
        Dmach7W::new(self, 7)
    }
    #[doc = "Bit 8 - DMA Channel 8 interrupt signal. Size counter reached zero (DMASZ=0)."]
    #[inline(always)]
    pub fn dmach8(&mut self) -> Dmach8W<'_, ImaskSpec> {
        Dmach8W::new(self, 8)
    }
    #[doc = "Bit 9 - DMA Channel 9 interrupt signal. Size counter reached zero (DMASZ=0)."]
    #[inline(always)]
    pub fn dmach9(&mut self) -> Dmach9W<'_, ImaskSpec> {
        Dmach9W::new(self, 9)
    }
    #[doc = "Bit 10 - DMA Channel 10 interrupt signal. Size counter reached zero (DMASZ=0)."]
    #[inline(always)]
    pub fn dmach10(&mut self) -> Dmach10W<'_, ImaskSpec> {
        Dmach10W::new(self, 10)
    }
    #[doc = "Bit 11 - DMA Channel 11 interrupt signal. Size counter reached zero (DMASZ=0)."]
    #[inline(always)]
    pub fn dmach11(&mut self) -> Dmach11W<'_, ImaskSpec> {
        Dmach11W::new(self, 11)
    }
    #[doc = "Bit 16 - Pre-IRQ for Channel 0. Size counter reached Pre-IRQ threshold."]
    #[inline(always)]
    pub fn preirqch0(&mut self) -> Preirqch0W<'_, ImaskSpec> {
        Preirqch0W::new(self, 16)
    }
    #[doc = "Bit 17 - Pre-IRQ for Channel 1. Size counter reached Pre-IRQ threshold."]
    #[inline(always)]
    pub fn preirqch1(&mut self) -> Preirqch1W<'_, ImaskSpec> {
        Preirqch1W::new(self, 17)
    }
    #[doc = "Bit 18 - Pre-IRQ for Channel 2. Size counter reached Pre-IRQ threshold."]
    #[inline(always)]
    pub fn preirqch2(&mut self) -> Preirqch2W<'_, ImaskSpec> {
        Preirqch2W::new(self, 18)
    }
    #[doc = "Bit 19 - Pre-IRQ for Channel 3. Size counter reached Pre-IRQ threshold."]
    #[inline(always)]
    pub fn preirqch3(&mut self) -> Preirqch3W<'_, ImaskSpec> {
        Preirqch3W::new(self, 19)
    }
    #[doc = "Bit 20 - Pre-IRQ for Channel 4. Size counter reached Pre-IRQ threshold."]
    #[inline(always)]
    pub fn preirqch4(&mut self) -> Preirqch4W<'_, ImaskSpec> {
        Preirqch4W::new(self, 20)
    }
    #[doc = "Bit 21 - Pre-IRQ for Channel 5. Size counter reached Pre-IRQ threshold."]
    #[inline(always)]
    pub fn preirqch5(&mut self) -> Preirqch5W<'_, ImaskSpec> {
        Preirqch5W::new(self, 21)
    }
    #[doc = "Bit 24 - DMA address error, SRC address not reachable."]
    #[inline(always)]
    pub fn addrerr(&mut self) -> AddrerrW<'_, ImaskSpec> {
        AddrerrW::new(self, 24)
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
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IMASK to value 0"]
impl crate::Resettable for ImaskSpec {}
