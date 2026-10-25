#[doc = "Register `INTCLR` reader"]
pub type R = crate::R<IntclrSpec>;
#[doc = "Register `INTCLR` writer"]
pub type W = crate::W<IntclrSpec>;
#[doc = "Field `INT` reader - Clear Global Flag"]
pub type IntR = crate::BitReader;
#[doc = "Field `INT` writer - Clear Global Flag"]
pub type IntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CBC` reader - Clear CBC interrupt flag"]
pub type CbcR = crate::BitReader;
#[doc = "Field `CBC` writer - Clear CBC interrupt flag"]
pub type CbcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OST` reader - Clear OST interrupt flag"]
pub type OstR = crate::BitReader;
#[doc = "Field `OST` writer - Clear OST interrupt flag"]
pub type OstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ET1` reader - Clear ET1 interrupt flag"]
pub type Et1R = crate::BitReader;
#[doc = "Field `ET1` writer - Clear ET1 interrupt flag"]
pub type Et1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ET2` reader - Clear ET2 interrupt flag"]
pub type Et2R = crate::BitReader;
#[doc = "Field `ET2` writer - Clear ET2 interrupt flag"]
pub type Et2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CNT_OVF` reader - Counter overflow flag"]
pub type CntOvfR = crate::BitReader;
#[doc = "Field `CNT_OVF` writer - Counter overflow flag"]
pub type CntOvfW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Clear Global Flag"]
    #[inline(always)]
    pub fn int(&self) -> IntR {
        IntR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Clear CBC interrupt flag"]
    #[inline(always)]
    pub fn cbc(&self) -> CbcR {
        CbcR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Clear OST interrupt flag"]
    #[inline(always)]
    pub fn ost(&self) -> OstR {
        OstR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Clear ET1 interrupt flag"]
    #[inline(always)]
    pub fn et1(&self) -> Et1R {
        Et1R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Clear ET2 interrupt flag"]
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
        f.debug_struct("INTCLR")
            .field("int", &self.int())
            .field("cbc", &self.cbc())
            .field("ost", &self.ost())
            .field("et1", &self.et1())
            .field("et2", &self.et2())
            .field("cnt_ovf", &self.cnt_ovf())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Clear Global Flag"]
    #[inline(always)]
    pub fn int(&mut self) -> IntW<'_, IntclrSpec> {
        IntW::new(self, 0)
    }
    #[doc = "Bit 1 - Clear CBC interrupt flag"]
    #[inline(always)]
    pub fn cbc(&mut self) -> CbcW<'_, IntclrSpec> {
        CbcW::new(self, 1)
    }
    #[doc = "Bit 2 - Clear OST interrupt flag"]
    #[inline(always)]
    pub fn ost(&mut self) -> OstW<'_, IntclrSpec> {
        OstW::new(self, 2)
    }
    #[doc = "Bit 3 - Clear ET1 interrupt flag"]
    #[inline(always)]
    pub fn et1(&mut self) -> Et1W<'_, IntclrSpec> {
        Et1W::new(self, 3)
    }
    #[doc = "Bit 4 - Clear ET2 interrupt flag"]
    #[inline(always)]
    pub fn et2(&mut self) -> Et2W<'_, IntclrSpec> {
        Et2W::new(self, 4)
    }
    #[doc = "Bit 5 - Counter overflow flag"]
    #[inline(always)]
    pub fn cnt_ovf(&mut self) -> CntOvfW<'_, IntclrSpec> {
        CntOvfW::new(self, 5)
    }
}
#[doc = "Interrupt clear\n\nYou can [`read`](crate::Reg::read) this register and get [`intclr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intclr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntclrSpec;
impl crate::RegisterSpec for IntclrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intclr::R`](R) reader structure"]
impl crate::Readable for IntclrSpec {}
#[doc = "`write(|w| ..)` method takes [`intclr::W`](W) writer structure"]
impl crate::Writable for IntclrSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets INTCLR to value 0"]
impl crate::Resettable for IntclrSpec {}
