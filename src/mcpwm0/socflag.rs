#[doc = "Register `SOCFLAG` reader"]
pub type R = crate::R<SocflagSpec>;
#[doc = "Register `SOCFLAG` writer"]
pub type W = crate::W<SocflagSpec>;
#[doc = "Field `SOCA` reader - SOCA Flag"]
pub type SocaR = crate::BitReader;
#[doc = "Field `SOCB` reader - SOCB Flag"]
pub type SocbR = crate::BitReader;
#[doc = "Field `SOCC` reader - SOCC Flag"]
pub type SoccR = crate::BitReader;
#[doc = "Field `SOCD` reader - SOCD Flag"]
pub type SocdR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - SOCA Flag"]
    #[inline(always)]
    pub fn soca(&self) -> SocaR {
        SocaR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SOCB Flag"]
    #[inline(always)]
    pub fn socb(&self) -> SocbR {
        SocbR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SOCC Flag"]
    #[inline(always)]
    pub fn socc(&self) -> SoccR {
        SoccR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SOCD Flag"]
    #[inline(always)]
    pub fn socd(&self) -> SocdR {
        SocdR::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SOCFLAG")
            .field("soca", &self.soca())
            .field("socb", &self.socb())
            .field("socc", &self.socc())
            .field("socd", &self.socd())
            .finish()
    }
}
impl W {}
#[doc = "Start of conversion flag\n\nYou can [`read`](crate::Reg::read) this register and get [`socflag::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`socflag::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SocflagSpec;
impl crate::RegisterSpec for SocflagSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`socflag::R`](R) reader structure"]
impl crate::Readable for SocflagSpec {}
#[doc = "`write(|w| ..)` method takes [`socflag::W`](W) writer structure"]
impl crate::Writable for SocflagSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets SOCFLAG to value 0"]
impl crate::Resettable for SocflagSpec {}
