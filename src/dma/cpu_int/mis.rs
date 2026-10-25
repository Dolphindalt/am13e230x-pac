#[doc = "Register `MIS` reader"]
pub type R = crate::R<MisSpec>;
#[doc = "Field `DMACH0` reader - DMA Channel 0 interrupt signals that size counter reached zero (DMASZ=0)."]
pub type Dmach0R = crate::BitReader;
#[doc = "Field `DMACH1` reader - DMA Channel 1 interrupt signals that size counter reached zero (DMASZ=0)."]
pub type Dmach1R = crate::BitReader;
#[doc = "Field `DMACH2` reader - DMA Channel 2 interrupt signals that size counter reached zero (DMASZ=0)."]
pub type Dmach2R = crate::BitReader;
#[doc = "Field `DMACH3` reader - DMA Channel 3 interrupt signals that size counter reached zero (DMASZ=0)."]
pub type Dmach3R = crate::BitReader;
#[doc = "Field `DMACH4` reader - DMA Channel 4 interrupt signals that size counter reached zero (DMASZ=0)."]
pub type Dmach4R = crate::BitReader;
#[doc = "Field `DMACH5` reader - DMA Channel 5 interrupt signals that size counter reached zero (DMASZ=0)."]
pub type Dmach5R = crate::BitReader;
#[doc = "Field `DMACH6` reader - DMA Channel 6 interrupt signals that size counter reached zero (DMASZ=0)."]
pub type Dmach6R = crate::BitReader;
#[doc = "Field `DMACH7` reader - DMA Channel 7 interrupt signals that size counter reached zero (DMASZ=0)."]
pub type Dmach7R = crate::BitReader;
#[doc = "Field `DMACH8` reader - DMA Channel 8 interrupt signals that size counter reached zero (DMASZ=0)."]
pub type Dmach8R = crate::BitReader;
#[doc = "Field `DMACH9` reader - DMA Channel 9 interrupt signals that size counter reached zero (DMASZ=0)."]
pub type Dmach9R = crate::BitReader;
#[doc = "Field `DMACH10` reader - DMA Channel 10 interrupt signals that size counter reached zero (DMASZ=0)."]
pub type Dmach10R = crate::BitReader;
#[doc = "Field `DMACH11` reader - DMA Channel 11 interrupt signals that size counter reached zero (DMASZ=0)."]
pub type Dmach11R = crate::BitReader;
#[doc = "Field `PREIRQCH0` reader - Pre-IRQ for Channel 0. Size counter reached Pre-IRQ threshold."]
pub type Preirqch0R = crate::BitReader;
#[doc = "Field `PREIRQCH1` reader - Pre-IRQ for Channel 1. Size counter reached Pre-IRQ threshold."]
pub type Preirqch1R = crate::BitReader;
#[doc = "Field `PREIRQCH2` reader - Pre-IRQ for Channel 2. Size counter reached Pre-IRQ threshold."]
pub type Preirqch2R = crate::BitReader;
#[doc = "Field `PREIRQCH3` reader - Pre-IRQ for Channel 3. Size counter reached Pre-IRQ threshold."]
pub type Preirqch3R = crate::BitReader;
#[doc = "Field `PREIRQCH4` reader - Pre-IRQ for Channel 4. Size counter reached Pre-IRQ threshold."]
pub type Preirqch4R = crate::BitReader;
#[doc = "Field `PREIRQCH5` reader - Pre-IRQ for Channel 5. Size counter reached Pre-IRQ threshold."]
pub type Preirqch5R = crate::BitReader;
#[doc = "Field `ADDRERR` reader - DMA address error, SRC address not reachable."]
pub type AddrerrR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - DMA Channel 0 interrupt signals that size counter reached zero (DMASZ=0)."]
    #[inline(always)]
    pub fn dmach0(&self) -> Dmach0R {
        Dmach0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMA Channel 1 interrupt signals that size counter reached zero (DMASZ=0)."]
    #[inline(always)]
    pub fn dmach1(&self) -> Dmach1R {
        Dmach1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DMA Channel 2 interrupt signals that size counter reached zero (DMASZ=0)."]
    #[inline(always)]
    pub fn dmach2(&self) -> Dmach2R {
        Dmach2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DMA Channel 3 interrupt signals that size counter reached zero (DMASZ=0)."]
    #[inline(always)]
    pub fn dmach3(&self) -> Dmach3R {
        Dmach3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - DMA Channel 4 interrupt signals that size counter reached zero (DMASZ=0)."]
    #[inline(always)]
    pub fn dmach4(&self) -> Dmach4R {
        Dmach4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - DMA Channel 5 interrupt signals that size counter reached zero (DMASZ=0)."]
    #[inline(always)]
    pub fn dmach5(&self) -> Dmach5R {
        Dmach5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - DMA Channel 6 interrupt signals that size counter reached zero (DMASZ=0)."]
    #[inline(always)]
    pub fn dmach6(&self) -> Dmach6R {
        Dmach6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - DMA Channel 7 interrupt signals that size counter reached zero (DMASZ=0)."]
    #[inline(always)]
    pub fn dmach7(&self) -> Dmach7R {
        Dmach7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - DMA Channel 8 interrupt signals that size counter reached zero (DMASZ=0)."]
    #[inline(always)]
    pub fn dmach8(&self) -> Dmach8R {
        Dmach8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - DMA Channel 9 interrupt signals that size counter reached zero (DMASZ=0)."]
    #[inline(always)]
    pub fn dmach9(&self) -> Dmach9R {
        Dmach9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - DMA Channel 10 interrupt signals that size counter reached zero (DMASZ=0)."]
    #[inline(always)]
    pub fn dmach10(&self) -> Dmach10R {
        Dmach10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - DMA Channel 11 interrupt signals that size counter reached zero (DMASZ=0)."]
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
        f.debug_struct("MIS")
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
#[doc = "Masked interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`mis::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MisSpec;
impl crate::RegisterSpec for MisSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mis::R`](R) reader structure"]
impl crate::Readable for MisSpec {}
#[doc = "`reset()` method sets MIS to value 0"]
impl crate::Resettable for MisSpec {}
