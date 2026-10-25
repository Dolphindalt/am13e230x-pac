#[doc = "Register `ECFRC` reader"]
pub type R = crate::R<EcfrcSpec>;
#[doc = "Register `ECFRC` writer"]
pub type W = crate::W<EcfrcSpec>;
#[doc = "Field `CEVT1` reader - Capture Event 1 Force Interrupt"]
pub type Cevt1R = crate::BitReader;
#[doc = "Field `CEVT1` writer - Capture Event 1 Force Interrupt"]
pub type Cevt1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CEVT2` reader - Capture Event 2 Force Interrupt"]
pub type Cevt2R = crate::BitReader;
#[doc = "Field `CEVT2` writer - Capture Event 2 Force Interrupt"]
pub type Cevt2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CEVT3` reader - Capture Event 3 Force Interrupt"]
pub type Cevt3R = crate::BitReader;
#[doc = "Field `CEVT3` writer - Capture Event 3 Force Interrupt"]
pub type Cevt3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CEVT4` reader - Capture Event 4 Force Interrupt"]
pub type Cevt4R = crate::BitReader;
#[doc = "Field `CEVT4` writer - Capture Event 4 Force Interrupt"]
pub type Cevt4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTROVF` reader - Counter Overflow Force Interrupt"]
pub type CtrovfR = crate::BitReader;
#[doc = "Field `CTROVF` writer - Counter Overflow Force Interrupt"]
pub type CtrovfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTR_PRD` reader - Period Equal Force Interrupt"]
pub type CtrPrdR = crate::BitReader;
#[doc = "Field `CTR_PRD` writer - Period Equal Force Interrupt"]
pub type CtrPrdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTR_CMP` reader - Compare Equal Force Interrupt"]
pub type CtrCmpR = crate::BitReader;
#[doc = "Field `CTR_CMP` writer - Compare Equal Force Interrupt"]
pub type CtrCmpW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - Capture Event 1 Force Interrupt"]
    #[inline(always)]
    pub fn cevt1(&self) -> Cevt1R {
        Cevt1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Capture Event 2 Force Interrupt"]
    #[inline(always)]
    pub fn cevt2(&self) -> Cevt2R {
        Cevt2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Capture Event 3 Force Interrupt"]
    #[inline(always)]
    pub fn cevt3(&self) -> Cevt3R {
        Cevt3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Capture Event 4 Force Interrupt"]
    #[inline(always)]
    pub fn cevt4(&self) -> Cevt4R {
        Cevt4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Counter Overflow Force Interrupt"]
    #[inline(always)]
    pub fn ctrovf(&self) -> CtrovfR {
        CtrovfR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Period Equal Force Interrupt"]
    #[inline(always)]
    pub fn ctr_prd(&self) -> CtrPrdR {
        CtrPrdR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Compare Equal Force Interrupt"]
    #[inline(always)]
    pub fn ctr_cmp(&self) -> CtrCmpR {
        CtrCmpR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ECFRC")
            .field("cevt1", &self.cevt1())
            .field("cevt2", &self.cevt2())
            .field("cevt3", &self.cevt3())
            .field("cevt4", &self.cevt4())
            .field("ctrovf", &self.ctrovf())
            .field("ctr_prd", &self.ctr_prd())
            .field("ctr_cmp", &self.ctr_cmp())
            .finish()
    }
}
impl W {
    #[doc = "Bit 1 - Capture Event 1 Force Interrupt"]
    #[inline(always)]
    pub fn cevt1(&mut self) -> Cevt1W<'_, EcfrcSpec> {
        Cevt1W::new(self, 1)
    }
    #[doc = "Bit 2 - Capture Event 2 Force Interrupt"]
    #[inline(always)]
    pub fn cevt2(&mut self) -> Cevt2W<'_, EcfrcSpec> {
        Cevt2W::new(self, 2)
    }
    #[doc = "Bit 3 - Capture Event 3 Force Interrupt"]
    #[inline(always)]
    pub fn cevt3(&mut self) -> Cevt3W<'_, EcfrcSpec> {
        Cevt3W::new(self, 3)
    }
    #[doc = "Bit 4 - Capture Event 4 Force Interrupt"]
    #[inline(always)]
    pub fn cevt4(&mut self) -> Cevt4W<'_, EcfrcSpec> {
        Cevt4W::new(self, 4)
    }
    #[doc = "Bit 5 - Counter Overflow Force Interrupt"]
    #[inline(always)]
    pub fn ctrovf(&mut self) -> CtrovfW<'_, EcfrcSpec> {
        CtrovfW::new(self, 5)
    }
    #[doc = "Bit 6 - Period Equal Force Interrupt"]
    #[inline(always)]
    pub fn ctr_prd(&mut self) -> CtrPrdW<'_, EcfrcSpec> {
        CtrPrdW::new(self, 6)
    }
    #[doc = "Bit 7 - Compare Equal Force Interrupt"]
    #[inline(always)]
    pub fn ctr_cmp(&mut self) -> CtrCmpW<'_, EcfrcSpec> {
        CtrCmpW::new(self, 7)
    }
}
#[doc = "Capture Interrupt Force Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ecfrc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ecfrc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EcfrcSpec;
impl crate::RegisterSpec for EcfrcSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ecfrc::R`](R) reader structure"]
impl crate::Readable for EcfrcSpec {}
#[doc = "`write(|w| ..)` method takes [`ecfrc::W`](W) writer structure"]
impl crate::Writable for EcfrcSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets ECFRC to value 0"]
impl crate::Resettable for EcfrcSpec {}
