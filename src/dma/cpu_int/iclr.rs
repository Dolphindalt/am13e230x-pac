#[doc = "Register `ICLR` writer"]
pub type W = crate::W<IclrSpec>;
#[doc = "Field `DMACH0` writer - DMA Channel 0 interrupt signals that size counter reached zero (DMASZ=0)."]
pub type Dmach0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMACH1` writer - DMA Channel 1 interrupt signals that size counter reached zero (DMASZ=0)."]
pub type Dmach1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMACH2` writer - DMA Channel 2 interrupt signals that size counter reached zero (DMASZ=0)."]
pub type Dmach2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMACH3` writer - DMA Channel 3 interrupt signals that size counter reached zero (DMASZ=0)."]
pub type Dmach3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMACH4` writer - DMA Channel 4 interrupt signals that size counter reached zero (DMASZ=0)."]
pub type Dmach4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMACH5` writer - DMA Channel 5 interrupt signals that size counter reached zero (DMASZ=0)."]
pub type Dmach5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMACH6` writer - DMA Channel 6 interrupt signals that size counter reached zero (DMASZ=0)."]
pub type Dmach6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMACH7` writer - DMA Channel 7 interrupt signals that size counter reached zero (DMASZ=0)."]
pub type Dmach7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMACH8` writer - DMA Channel 8 interrupt signals that size counter reached zero (DMASZ=0)."]
pub type Dmach8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMACH9` writer - DMA Channel 9 interrupt signals that size counter reached zero (DMASZ=0)."]
pub type Dmach9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMACH10` writer - DMA Channel 10 interrupt signals that size counter reached zero (DMASZ=0)."]
pub type Dmach10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMACH11` writer - DMA Channel 11 interrupt signals that size counter reached zero (DMASZ=0)."]
pub type Dmach11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PREIRQCH0` writer - Pre-IRQ for Channel 0. Size counter reached Pre-IRQ threshold."]
pub type Preirqch0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PREIRQCH1` writer - Pre-IRQ for Channel 1. Size counter reached Pre-IRQ threshold."]
pub type Preirqch1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PREIRQCH2` writer - Pre-IRQ for Channel 2. Size counter reached Pre-IRQ threshold."]
pub type Preirqch2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PREIRQCH3` writer - Pre-IRQ for Channel 3. Size counter reached Pre-IRQ threshold."]
pub type Preirqch3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PREIRQCH4` writer - Pre-IRQ for Channel 4. Size counter reached Pre-IRQ threshold."]
pub type Preirqch4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PREIRQCH5` writer - Pre-IRQ for Channel 5. Size counter reached Pre-IRQ threshold."]
pub type Preirqch5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDRERR` writer - DMA address error, SRC address not reachable."]
pub type AddrerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<IclrSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - DMA Channel 0 interrupt signals that size counter reached zero (DMASZ=0)."]
    #[inline(always)]
    pub fn dmach0(&mut self) -> Dmach0W<'_, IclrSpec> {
        Dmach0W::new(self, 0)
    }
    #[doc = "Bit 1 - DMA Channel 1 interrupt signals that size counter reached zero (DMASZ=0)."]
    #[inline(always)]
    pub fn dmach1(&mut self) -> Dmach1W<'_, IclrSpec> {
        Dmach1W::new(self, 1)
    }
    #[doc = "Bit 2 - DMA Channel 2 interrupt signals that size counter reached zero (DMASZ=0)."]
    #[inline(always)]
    pub fn dmach2(&mut self) -> Dmach2W<'_, IclrSpec> {
        Dmach2W::new(self, 2)
    }
    #[doc = "Bit 3 - DMA Channel 3 interrupt signals that size counter reached zero (DMASZ=0)."]
    #[inline(always)]
    pub fn dmach3(&mut self) -> Dmach3W<'_, IclrSpec> {
        Dmach3W::new(self, 3)
    }
    #[doc = "Bit 4 - DMA Channel 4 interrupt signals that size counter reached zero (DMASZ=0)."]
    #[inline(always)]
    pub fn dmach4(&mut self) -> Dmach4W<'_, IclrSpec> {
        Dmach4W::new(self, 4)
    }
    #[doc = "Bit 5 - DMA Channel 5 interrupt signals that size counter reached zero (DMASZ=0)."]
    #[inline(always)]
    pub fn dmach5(&mut self) -> Dmach5W<'_, IclrSpec> {
        Dmach5W::new(self, 5)
    }
    #[doc = "Bit 6 - DMA Channel 6 interrupt signals that size counter reached zero (DMASZ=0)."]
    #[inline(always)]
    pub fn dmach6(&mut self) -> Dmach6W<'_, IclrSpec> {
        Dmach6W::new(self, 6)
    }
    #[doc = "Bit 7 - DMA Channel 7 interrupt signals that size counter reached zero (DMASZ=0)."]
    #[inline(always)]
    pub fn dmach7(&mut self) -> Dmach7W<'_, IclrSpec> {
        Dmach7W::new(self, 7)
    }
    #[doc = "Bit 8 - DMA Channel 8 interrupt signals that size counter reached zero (DMASZ=0)."]
    #[inline(always)]
    pub fn dmach8(&mut self) -> Dmach8W<'_, IclrSpec> {
        Dmach8W::new(self, 8)
    }
    #[doc = "Bit 9 - DMA Channel 9 interrupt signals that size counter reached zero (DMASZ=0)."]
    #[inline(always)]
    pub fn dmach9(&mut self) -> Dmach9W<'_, IclrSpec> {
        Dmach9W::new(self, 9)
    }
    #[doc = "Bit 10 - DMA Channel 10 interrupt signals that size counter reached zero (DMASZ=0)."]
    #[inline(always)]
    pub fn dmach10(&mut self) -> Dmach10W<'_, IclrSpec> {
        Dmach10W::new(self, 10)
    }
    #[doc = "Bit 11 - DMA Channel 11 interrupt signals that size counter reached zero (DMASZ=0)."]
    #[inline(always)]
    pub fn dmach11(&mut self) -> Dmach11W<'_, IclrSpec> {
        Dmach11W::new(self, 11)
    }
    #[doc = "Bit 16 - Pre-IRQ for Channel 0. Size counter reached Pre-IRQ threshold."]
    #[inline(always)]
    pub fn preirqch0(&mut self) -> Preirqch0W<'_, IclrSpec> {
        Preirqch0W::new(self, 16)
    }
    #[doc = "Bit 17 - Pre-IRQ for Channel 1. Size counter reached Pre-IRQ threshold."]
    #[inline(always)]
    pub fn preirqch1(&mut self) -> Preirqch1W<'_, IclrSpec> {
        Preirqch1W::new(self, 17)
    }
    #[doc = "Bit 18 - Pre-IRQ for Channel 2. Size counter reached Pre-IRQ threshold."]
    #[inline(always)]
    pub fn preirqch2(&mut self) -> Preirqch2W<'_, IclrSpec> {
        Preirqch2W::new(self, 18)
    }
    #[doc = "Bit 19 - Pre-IRQ for Channel 3. Size counter reached Pre-IRQ threshold."]
    #[inline(always)]
    pub fn preirqch3(&mut self) -> Preirqch3W<'_, IclrSpec> {
        Preirqch3W::new(self, 19)
    }
    #[doc = "Bit 20 - Pre-IRQ for Channel 4. Size counter reached Pre-IRQ threshold."]
    #[inline(always)]
    pub fn preirqch4(&mut self) -> Preirqch4W<'_, IclrSpec> {
        Preirqch4W::new(self, 20)
    }
    #[doc = "Bit 21 - Pre-IRQ for Channel 5. Size counter reached Pre-IRQ threshold."]
    #[inline(always)]
    pub fn preirqch5(&mut self) -> Preirqch5W<'_, IclrSpec> {
        Preirqch5W::new(self, 21)
    }
    #[doc = "Bit 24 - DMA address error, SRC address not reachable."]
    #[inline(always)]
    pub fn addrerr(&mut self) -> AddrerrW<'_, IclrSpec> {
        AddrerrW::new(self, 24)
    }
}
#[doc = "Interrupt clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IclrSpec;
impl crate::RegisterSpec for IclrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`iclr::W`](W) writer structure"]
impl crate::Writable for IclrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ICLR to value 0"]
impl crate::Resettable for IclrSpec {}
