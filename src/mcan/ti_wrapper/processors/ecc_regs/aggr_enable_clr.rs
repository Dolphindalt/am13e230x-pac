#[doc = "Register `AGGR_ENABLE_CLR` reader"]
pub type R = crate::R<AggrEnableClrSpec>;
#[doc = "Register `AGGR_ENABLE_CLR` writer"]
pub type W = crate::W<AggrEnableClrSpec>;
#[doc = "Field `ENABLE_PARITY_CLR` reader - Enable Parity Errors Clear"]
pub type EnableParityClrR = crate::BitReader;
#[doc = "Field `ENABLE_PARITY_CLR` writer - Enable Parity Errors Clear"]
pub type EnableParityClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENABLE_TIMEOUT_CLR` reader - Enable Timeout Errors Clear"]
pub type EnableTimeoutClrR = crate::BitReader;
#[doc = "Field `ENABLE_TIMEOUT_CLR` writer - Enable Timeout Errors Clear"]
pub type EnableTimeoutClrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable Parity Errors Clear"]
    #[inline(always)]
    pub fn enable_parity_clr(&self) -> EnableParityClrR {
        EnableParityClrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable Timeout Errors Clear"]
    #[inline(always)]
    pub fn enable_timeout_clr(&self) -> EnableTimeoutClrR {
        EnableTimeoutClrR::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AGGR_ENABLE_CLR")
            .field("enable_parity_clr", &self.enable_parity_clr())
            .field("enable_timeout_clr", &self.enable_timeout_clr())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Enable Parity Errors Clear"]
    #[inline(always)]
    pub fn enable_parity_clr(&mut self) -> EnableParityClrW<'_, AggrEnableClrSpec> {
        EnableParityClrW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable Timeout Errors Clear"]
    #[inline(always)]
    pub fn enable_timeout_clr(&mut self) -> EnableTimeoutClrW<'_, AggrEnableClrSpec> {
        EnableTimeoutClrW::new(self, 1)
    }
}
#[doc = "MCAN Error Aggregator Enable Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`aggr_enable_clr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aggr_enable_clr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AggrEnableClrSpec;
impl crate::RegisterSpec for AggrEnableClrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aggr_enable_clr::R`](R) reader structure"]
impl crate::Readable for AggrEnableClrSpec {}
#[doc = "`write(|w| ..)` method takes [`aggr_enable_clr::W`](W) writer structure"]
impl crate::Writable for AggrEnableClrSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets AGGR_ENABLE_CLR to value 0"]
impl crate::Resettable for AggrEnableClrSpec {}
