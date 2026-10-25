#[doc = "Register `AGGR_ENABLE_SET` reader"]
pub type R = crate::R<AggrEnableSetSpec>;
#[doc = "Register `AGGR_ENABLE_SET` writer"]
pub type W = crate::W<AggrEnableSetSpec>;
#[doc = "Field `ENABLE_PARITY_SET` reader - Enable Parity Errors Set"]
pub type EnableParitySetR = crate::BitReader;
#[doc = "Field `ENABLE_PARITY_SET` writer - Enable Parity Errors Set"]
pub type EnableParitySetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENABLE_TIMEOUT_SET` reader - Enable Timeout Errors Set"]
pub type EnableTimeoutSetR = crate::BitReader;
#[doc = "Field `ENABLE_TIMEOUT_SET` writer - Enable Timeout Errors Set"]
pub type EnableTimeoutSetW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable Parity Errors Set"]
    #[inline(always)]
    pub fn enable_parity_set(&self) -> EnableParitySetR {
        EnableParitySetR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable Timeout Errors Set"]
    #[inline(always)]
    pub fn enable_timeout_set(&self) -> EnableTimeoutSetR {
        EnableTimeoutSetR::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AGGR_ENABLE_SET")
            .field("enable_parity_set", &self.enable_parity_set())
            .field("enable_timeout_set", &self.enable_timeout_set())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Enable Parity Errors Set"]
    #[inline(always)]
    pub fn enable_parity_set(&mut self) -> EnableParitySetW<'_, AggrEnableSetSpec> {
        EnableParitySetW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable Timeout Errors Set"]
    #[inline(always)]
    pub fn enable_timeout_set(&mut self) -> EnableTimeoutSetW<'_, AggrEnableSetSpec> {
        EnableTimeoutSetW::new(self, 1)
    }
}
#[doc = "MCAN Error Aggregator Enable Set Register\n\nYou can [`read`](crate::Reg::read) this register and get [`aggr_enable_set::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aggr_enable_set::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AggrEnableSetSpec;
impl crate::RegisterSpec for AggrEnableSetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aggr_enable_set::R`](R) reader structure"]
impl crate::Readable for AggrEnableSetSpec {}
#[doc = "`write(|w| ..)` method takes [`aggr_enable_set::W`](W) writer structure"]
impl crate::Writable for AggrEnableSetSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets AGGR_ENABLE_SET to value 0"]
impl crate::Resettable for AggrEnableSetSpec {}
