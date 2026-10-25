#[doc = "Register `ECFLG` reader"]
pub type R = crate::R<EcflgSpec>;
#[doc = "Register `ECFLG` writer"]
pub type W = crate::W<EcflgSpec>;
#[doc = "Field `INT` reader - Global Flag"]
pub type IntR = crate::BitReader;
#[doc = "Field `CEVT1` reader - Capture Event 1 Interrupt Flag"]
pub type Cevt1R = crate::BitReader;
#[doc = "Field `CEVT2` reader - Capture Event 2 Interrupt Flag"]
pub type Cevt2R = crate::BitReader;
#[doc = "Field `CEVT3` reader - Capture Event 3 Interrupt Flag"]
pub type Cevt3R = crate::BitReader;
#[doc = "Field `CEVT4` reader - Capture Event 4 Interrupt Flag"]
pub type Cevt4R = crate::BitReader;
#[doc = "Field `CTROVF` reader - Counter Overflow Interrupt Flag"]
pub type CtrovfR = crate::BitReader;
#[doc = "Field `CTR_PRD` reader - Period Equal Interrupt Flag"]
pub type CtrPrdR = crate::BitReader;
#[doc = "Field `CTR_CMP` reader - Compare Equal Interrupt Flag"]
pub type CtrCmpR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Global Flag"]
    #[inline(always)]
    pub fn int(&self) -> IntR {
        IntR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Capture Event 1 Interrupt Flag"]
    #[inline(always)]
    pub fn cevt1(&self) -> Cevt1R {
        Cevt1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Capture Event 2 Interrupt Flag"]
    #[inline(always)]
    pub fn cevt2(&self) -> Cevt2R {
        Cevt2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Capture Event 3 Interrupt Flag"]
    #[inline(always)]
    pub fn cevt3(&self) -> Cevt3R {
        Cevt3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Capture Event 4 Interrupt Flag"]
    #[inline(always)]
    pub fn cevt4(&self) -> Cevt4R {
        Cevt4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Counter Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn ctrovf(&self) -> CtrovfR {
        CtrovfR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Period Equal Interrupt Flag"]
    #[inline(always)]
    pub fn ctr_prd(&self) -> CtrPrdR {
        CtrPrdR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Compare Equal Interrupt Flag"]
    #[inline(always)]
    pub fn ctr_cmp(&self) -> CtrCmpR {
        CtrCmpR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ECFLG")
            .field("int", &self.int())
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
impl W {}
#[doc = "Capture Interrupt Flag Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ecflg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ecflg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EcflgSpec;
impl crate::RegisterSpec for EcflgSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ecflg::R`](R) reader structure"]
impl crate::Readable for EcflgSpec {}
#[doc = "`write(|w| ..)` method takes [`ecflg::W`](W) writer structure"]
impl crate::Writable for EcflgSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets ECFLG to value 0"]
impl crate::Resettable for EcflgSpec {}
