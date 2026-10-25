#[doc = "Register `INTFLAG` reader"]
pub type R = crate::R<IntflagSpec>;
#[doc = "Register `INTFLAG` writer"]
pub type W = crate::W<IntflagSpec>;
#[doc = "Field `INT` reader - Global Flag"]
pub type IntR = crate::BitReader;
#[doc = "Field `CBC` reader - CBC interrupt flag"]
pub type CbcR = crate::BitReader;
#[doc = "Field `OST` reader - OST interrupt flag"]
pub type OstR = crate::BitReader;
#[doc = "Field `ET1` reader - ET1 interrupt flag"]
pub type Et1R = crate::BitReader;
#[doc = "Field `ET2` reader - ET2 interrupt flag"]
pub type Et2R = crate::BitReader;
#[doc = "Field `CNT_OVF` reader - Counter overflow flag"]
pub type CntOvfR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Global Flag"]
    #[inline(always)]
    pub fn int(&self) -> IntR {
        IntR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CBC interrupt flag"]
    #[inline(always)]
    pub fn cbc(&self) -> CbcR {
        CbcR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - OST interrupt flag"]
    #[inline(always)]
    pub fn ost(&self) -> OstR {
        OstR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ET1 interrupt flag"]
    #[inline(always)]
    pub fn et1(&self) -> Et1R {
        Et1R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ET2 interrupt flag"]
    #[inline(always)]
    pub fn et2(&self) -> Et2R {
        Et2R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Counter overflow flag"]
    #[inline(always)]
    pub fn cnt_ovf(&self) -> CntOvfR {
        CntOvfR::new(((self.bits >> 5) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTFLAG")
            .field("int", &self.int())
            .field("cbc", &self.cbc())
            .field("ost", &self.ost())
            .field("et1", &self.et1())
            .field("et2", &self.et2())
            .field("cnt_ovf", &self.cnt_ovf())
            .finish()
    }
}
impl W {}
#[doc = "Interrupt flag\n\nYou can [`read`](crate::Reg::read) this register and get [`intflag::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intflag::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntflagSpec;
impl crate::RegisterSpec for IntflagSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intflag::R`](R) reader structure"]
impl crate::Readable for IntflagSpec {}
#[doc = "`write(|w| ..)` method takes [`intflag::W`](W) writer structure"]
impl crate::Writable for IntflagSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets INTFLAG to value 0"]
impl crate::Resettable for IntflagSpec {}
