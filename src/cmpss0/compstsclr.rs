#[doc = "Register `COMPSTSCLR` reader"]
pub type R = crate::R<CompstsclrSpec>;
#[doc = "Register `COMPSTSCLR` writer"]
pub type W = crate::W<CompstsclrSpec>;
#[doc = "Field `HLATCHCLR` reader - High Comparator Latched Status Clear"]
pub type HlatchclrR = crate::BitReader;
#[doc = "Field `HLATCHCLR` writer - High Comparator Latched Status Clear"]
pub type HlatchclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSYNCCLREN` reader - High Comparator EPWMSYNCPER Clear Enable"]
pub type HsyncclrenR = crate::BitReader;
#[doc = "Field `HSYNCCLREN` writer - High Comparator EPWMSYNCPER Clear Enable"]
pub type HsyncclrenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LLATCHCLR` reader - Low Comparator Latched Status Clear"]
pub type LlatchclrR = crate::BitReader;
#[doc = "Field `LLATCHCLR` writer - Low Comparator Latched Status Clear"]
pub type LlatchclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSYNCCLREN` reader - Low Comparator EPWMSYNCPER Clear Enable"]
pub type LsyncclrenR = crate::BitReader;
#[doc = "Field `LSYNCCLREN` writer - Low Comparator EPWMSYNCPER Clear Enable"]
pub type LsyncclrenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - High Comparator Latched Status Clear"]
    #[inline(always)]
    pub fn hlatchclr(&self) -> HlatchclrR {
        HlatchclrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - High Comparator EPWMSYNCPER Clear Enable"]
    #[inline(always)]
    pub fn hsyncclren(&self) -> HsyncclrenR {
        HsyncclrenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 9 - Low Comparator Latched Status Clear"]
    #[inline(always)]
    pub fn llatchclr(&self) -> LlatchclrR {
        LlatchclrR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Low Comparator EPWMSYNCPER Clear Enable"]
    #[inline(always)]
    pub fn lsyncclren(&self) -> LsyncclrenR {
        LsyncclrenR::new(((self.bits >> 10) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("COMPSTSCLR")
            .field("hlatchclr", &self.hlatchclr())
            .field("hsyncclren", &self.hsyncclren())
            .field("llatchclr", &self.llatchclr())
            .field("lsyncclren", &self.lsyncclren())
            .finish()
    }
}
impl W {
    #[doc = "Bit 1 - High Comparator Latched Status Clear"]
    #[inline(always)]
    pub fn hlatchclr(&mut self) -> HlatchclrW<'_, CompstsclrSpec> {
        HlatchclrW::new(self, 1)
    }
    #[doc = "Bit 2 - High Comparator EPWMSYNCPER Clear Enable"]
    #[inline(always)]
    pub fn hsyncclren(&mut self) -> HsyncclrenW<'_, CompstsclrSpec> {
        HsyncclrenW::new(self, 2)
    }
    #[doc = "Bit 9 - Low Comparator Latched Status Clear"]
    #[inline(always)]
    pub fn llatchclr(&mut self) -> LlatchclrW<'_, CompstsclrSpec> {
        LlatchclrW::new(self, 9)
    }
    #[doc = "Bit 10 - Low Comparator EPWMSYNCPER Clear Enable"]
    #[inline(always)]
    pub fn lsyncclren(&mut self) -> LsyncclrenW<'_, CompstsclrSpec> {
        LsyncclrenW::new(self, 10)
    }
}
#[doc = "CMPSS Comparator Status Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`compstsclr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`compstsclr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CompstsclrSpec;
impl crate::RegisterSpec for CompstsclrSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`compstsclr::R`](R) reader structure"]
impl crate::Readable for CompstsclrSpec {}
#[doc = "`write(|w| ..)` method takes [`compstsclr::W`](W) writer structure"]
impl crate::Writable for CompstsclrSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets COMPSTSCLR to value 0"]
impl crate::Resettable for CompstsclrSpec {}
