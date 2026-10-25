#[doc = "Register `CTRIPHFILCTL` reader"]
pub type R = crate::R<CtriphfilctlSpec>;
#[doc = "Register `CTRIPHFILCTL` writer"]
pub type W = crate::W<CtriphfilctlSpec>;
#[doc = "Field `FILTINSEL` reader - Filter Input Select"]
pub type FiltinselR = crate::FieldReader;
#[doc = "Field `FILTINSEL` writer - Filter Input Select"]
pub type FiltinselW<'a, REG> = crate::FieldWriter<'a, REG, 3, u8, crate::Safe>;
#[doc = "Field `SAMPWIN` reader - Sample Window"]
pub type SampwinR = crate::FieldReader;
#[doc = "Field `SAMPWIN` writer - Sample Window"]
pub type SampwinW<'a, REG> = crate::FieldWriter<'a, REG, 6, u8, crate::Safe>;
#[doc = "Field `THRESH` reader - Majority Voting Threshold"]
pub type ThreshR = crate::FieldReader;
#[doc = "Field `THRESH` writer - Majority Voting Threshold"]
pub type ThreshW<'a, REG> = crate::FieldWriter<'a, REG, 6, u8, crate::Safe>;
#[doc = "Field `FILINIT` reader - Filter Initialization Bit"]
pub type FilinitR = crate::BitReader;
#[doc = "Field `FILINIT` writer - Filter Initialization Bit"]
pub type FilinitW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - Filter Input Select"]
    #[inline(always)]
    pub fn filtinsel(&self) -> FiltinselR {
        FiltinselR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:8 - Sample Window"]
    #[inline(always)]
    pub fn sampwin(&self) -> SampwinR {
        SampwinR::new(((self.bits >> 3) & 0x3f) as u8)
    }
    #[doc = "Bits 9:14 - Majority Voting Threshold"]
    #[inline(always)]
    pub fn thresh(&self) -> ThreshR {
        ThreshR::new(((self.bits >> 9) & 0x3f) as u8)
    }
    #[doc = "Bit 15 - Filter Initialization Bit"]
    #[inline(always)]
    pub fn filinit(&self) -> FilinitR {
        FilinitR::new(((self.bits >> 15) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRIPHFILCTL")
            .field("filtinsel", &self.filtinsel())
            .field("sampwin", &self.sampwin())
            .field("thresh", &self.thresh())
            .field("filinit", &self.filinit())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - Filter Input Select"]
    #[inline(always)]
    pub fn filtinsel(&mut self) -> FiltinselW<'_, CtriphfilctlSpec> {
        FiltinselW::new(self, 0)
    }
    #[doc = "Bits 3:8 - Sample Window"]
    #[inline(always)]
    pub fn sampwin(&mut self) -> SampwinW<'_, CtriphfilctlSpec> {
        SampwinW::new(self, 3)
    }
    #[doc = "Bits 9:14 - Majority Voting Threshold"]
    #[inline(always)]
    pub fn thresh(&mut self) -> ThreshW<'_, CtriphfilctlSpec> {
        ThreshW::new(self, 9)
    }
    #[doc = "Bit 15 - Filter Initialization Bit"]
    #[inline(always)]
    pub fn filinit(&mut self) -> FilinitW<'_, CtriphfilctlSpec> {
        FilinitW::new(self, 15)
    }
}
#[doc = "CTRIPH Filter Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctriphfilctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctriphfilctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtriphfilctlSpec;
impl crate::RegisterSpec for CtriphfilctlSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ctriphfilctl::R`](R) reader structure"]
impl crate::Readable for CtriphfilctlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctriphfilctl::W`](W) writer structure"]
impl crate::Writable for CtriphfilctlSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CTRIPHFILCTL to value 0"]
impl crate::Resettable for CtriphfilctlSpec {}
