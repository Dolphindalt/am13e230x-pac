#[doc = "Register `INTFRC` reader"]
pub type R = crate::R<IntfrcSpec>;
#[doc = "Register `INTFRC` writer"]
pub type W = crate::W<IntfrcSpec>;
#[doc = "Field `CBC` reader - Force CBC interrupt flag"]
pub type CbcR = crate::BitReader;
#[doc = "Field `CBC` writer - Force CBC interrupt flag"]
pub type CbcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OST` reader - Force OST interrupt flag"]
pub type OstR = crate::BitReader;
#[doc = "Field `OST` writer - Force OST interrupt flag"]
pub type OstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ET1` reader - Force ET1 interrupt flag"]
pub type Et1R = crate::BitReader;
#[doc = "Field `ET1` writer - Force ET1 interrupt flag"]
pub type Et1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ET2` reader - Force ET2 interrupt flag"]
pub type Et2R = crate::BitReader;
#[doc = "Field `ET2` writer - Force ET2 interrupt flag"]
pub type Et2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CNT_OVF` reader - Counter overflow flag"]
pub type CntOvfR = crate::BitReader;
#[doc = "Field `CNT_OVF` writer - Counter overflow flag"]
pub type CntOvfW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - Force CBC interrupt flag"]
    #[inline(always)]
    pub fn cbc(&self) -> CbcR {
        CbcR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Force OST interrupt flag"]
    #[inline(always)]
    pub fn ost(&self) -> OstR {
        OstR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Force ET1 interrupt flag"]
    #[inline(always)]
    pub fn et1(&self) -> Et1R {
        Et1R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Force ET2 interrupt flag"]
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
        f.debug_struct("INTFRC")
            .field("cbc", &self.cbc())
            .field("ost", &self.ost())
            .field("et1", &self.et1())
            .field("et2", &self.et2())
            .field("cnt_ovf", &self.cnt_ovf())
            .finish()
    }
}
impl W {
    #[doc = "Bit 1 - Force CBC interrupt flag"]
    #[inline(always)]
    pub fn cbc(&mut self) -> CbcW<'_, IntfrcSpec> {
        CbcW::new(self, 1)
    }
    #[doc = "Bit 2 - Force OST interrupt flag"]
    #[inline(always)]
    pub fn ost(&mut self) -> OstW<'_, IntfrcSpec> {
        OstW::new(self, 2)
    }
    #[doc = "Bit 3 - Force ET1 interrupt flag"]
    #[inline(always)]
    pub fn et1(&mut self) -> Et1W<'_, IntfrcSpec> {
        Et1W::new(self, 3)
    }
    #[doc = "Bit 4 - Force ET2 interrupt flag"]
    #[inline(always)]
    pub fn et2(&mut self) -> Et2W<'_, IntfrcSpec> {
        Et2W::new(self, 4)
    }
    #[doc = "Bit 5 - Counter overflow flag"]
    #[inline(always)]
    pub fn cnt_ovf(&mut self) -> CntOvfW<'_, IntfrcSpec> {
        CntOvfW::new(self, 5)
    }
}
#[doc = "Interrupt force\n\nYou can [`read`](crate::Reg::read) this register and get [`intfrc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intfrc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntfrcSpec;
impl crate::RegisterSpec for IntfrcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intfrc::R`](R) reader structure"]
impl crate::Readable for IntfrcSpec {}
#[doc = "`write(|w| ..)` method takes [`intfrc::W`](W) writer structure"]
impl crate::Writable for IntfrcSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets INTFRC to value 0"]
impl crate::Resettable for IntfrcSpec {}
