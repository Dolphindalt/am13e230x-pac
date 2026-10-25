#[doc = "Register `SOCCLR` reader"]
pub type R = crate::R<SocclrSpec>;
#[doc = "Register `SOCCLR` writer"]
pub type W = crate::W<SocclrSpec>;
#[doc = "Field `SOCA` reader - SOCA Clear"]
pub type SocaR = crate::BitReader;
#[doc = "Field `SOCA` writer - SOCA Clear"]
pub type SocaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOCB` reader - SOCB Clear"]
pub type SocbR = crate::BitReader;
#[doc = "Field `SOCB` writer - SOCB Clear"]
pub type SocbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOCC` reader - SOCC Clear"]
pub type SoccR = crate::BitReader;
#[doc = "Field `SOCC` writer - SOCC Clear"]
pub type SoccW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOCD` reader - SOCD Clear"]
pub type SocdR = crate::BitReader;
#[doc = "Field `SOCD` writer - SOCD Clear"]
pub type SocdW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SOCA Clear"]
    #[inline(always)]
    pub fn soca(&self) -> SocaR {
        SocaR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SOCB Clear"]
    #[inline(always)]
    pub fn socb(&self) -> SocbR {
        SocbR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SOCC Clear"]
    #[inline(always)]
    pub fn socc(&self) -> SoccR {
        SoccR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SOCD Clear"]
    #[inline(always)]
    pub fn socd(&self) -> SocdR {
        SocdR::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SOCCLR")
            .field("soca", &self.soca())
            .field("socb", &self.socb())
            .field("socc", &self.socc())
            .field("socd", &self.socd())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - SOCA Clear"]
    #[inline(always)]
    pub fn soca(&mut self) -> SocaW<'_, SocclrSpec> {
        SocaW::new(self, 0)
    }
    #[doc = "Bit 1 - SOCB Clear"]
    #[inline(always)]
    pub fn socb(&mut self) -> SocbW<'_, SocclrSpec> {
        SocbW::new(self, 1)
    }
    #[doc = "Bit 2 - SOCC Clear"]
    #[inline(always)]
    pub fn socc(&mut self) -> SoccW<'_, SocclrSpec> {
        SoccW::new(self, 2)
    }
    #[doc = "Bit 3 - SOCD Clear"]
    #[inline(always)]
    pub fn socd(&mut self) -> SocdW<'_, SocclrSpec> {
        SocdW::new(self, 3)
    }
}
#[doc = "Start of conversion clear\n\nYou can [`read`](crate::Reg::read) this register and get [`socclr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`socclr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SocclrSpec;
impl crate::RegisterSpec for SocclrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`socclr::R`](R) reader structure"]
impl crate::Readable for SocclrSpec {}
#[doc = "`write(|w| ..)` method takes [`socclr::W`](W) writer structure"]
impl crate::Writable for SocclrSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets SOCCLR to value 0"]
impl crate::Resettable for SocclrSpec {}
