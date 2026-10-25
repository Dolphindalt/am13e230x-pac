#[doc = "Register `INTEN` reader"]
pub type R = crate::R<IntenSpec>;
#[doc = "Register `INTEN` writer"]
pub type W = crate::W<IntenSpec>;
#[doc = "Field `CBC` reader - Enable CBC interrupt"]
pub type CbcR = crate::BitReader;
#[doc = "Field `CBC` writer - Enable CBC interrupt"]
pub type CbcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OST` reader - Enable OST interrupt"]
pub type OstR = crate::BitReader;
#[doc = "Field `OST` writer - Enable OST interrupt"]
pub type OstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ET1` reader - Enable ET1 interrupt"]
pub type Et1R = crate::BitReader;
#[doc = "Field `ET1` writer - Enable ET1 interrupt"]
pub type Et1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ET2` reader - Enable ET2 interrupt"]
pub type Et2R = crate::BitReader;
#[doc = "Field `ET2` writer - Enable ET2 interrupt"]
pub type Et2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CNT_OVF` reader - Counter overflow interrupt"]
pub type CntOvfR = crate::BitReader;
#[doc = "Field `CNT_OVF` writer - Counter overflow interrupt"]
pub type CntOvfW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - Enable CBC interrupt"]
    #[inline(always)]
    pub fn cbc(&self) -> CbcR {
        CbcR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable OST interrupt"]
    #[inline(always)]
    pub fn ost(&self) -> OstR {
        OstR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable ET1 interrupt"]
    #[inline(always)]
    pub fn et1(&self) -> Et1R {
        Et1R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable ET2 interrupt"]
    #[inline(always)]
    pub fn et2(&self) -> Et2R {
        Et2R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Counter overflow interrupt"]
    #[inline(always)]
    pub fn cnt_ovf(&self) -> CntOvfR {
        CntOvfR::new(((self.bits >> 5) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTEN")
            .field("cbc", &self.cbc())
            .field("ost", &self.ost())
            .field("et1", &self.et1())
            .field("et2", &self.et2())
            .field("cnt_ovf", &self.cnt_ovf())
            .finish()
    }
}
impl W {
    #[doc = "Bit 1 - Enable CBC interrupt"]
    #[inline(always)]
    pub fn cbc(&mut self) -> CbcW<'_, IntenSpec> {
        CbcW::new(self, 1)
    }
    #[doc = "Bit 2 - Enable OST interrupt"]
    #[inline(always)]
    pub fn ost(&mut self) -> OstW<'_, IntenSpec> {
        OstW::new(self, 2)
    }
    #[doc = "Bit 3 - Enable ET1 interrupt"]
    #[inline(always)]
    pub fn et1(&mut self) -> Et1W<'_, IntenSpec> {
        Et1W::new(self, 3)
    }
    #[doc = "Bit 4 - Enable ET2 interrupt"]
    #[inline(always)]
    pub fn et2(&mut self) -> Et2W<'_, IntenSpec> {
        Et2W::new(self, 4)
    }
    #[doc = "Bit 5 - Counter overflow interrupt"]
    #[inline(always)]
    pub fn cnt_ovf(&mut self) -> CntOvfW<'_, IntenSpec> {
        CntOvfW::new(self, 5)
    }
}
#[doc = "Interrupt enable\n\nYou can [`read`](crate::Reg::read) this register and get [`inten::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inten::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntenSpec;
impl crate::RegisterSpec for IntenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`inten::R`](R) reader structure"]
impl crate::Readable for IntenSpec {}
#[doc = "`write(|w| ..)` method takes [`inten::W`](W) writer structure"]
impl crate::Writable for IntenSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets INTEN to value 0"]
impl crate::Resettable for IntenSpec {}
