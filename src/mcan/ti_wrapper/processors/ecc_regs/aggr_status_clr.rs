#[doc = "Register `AGGR_STATUS_CLR` reader"]
pub type R = crate::R<AggrStatusClrSpec>;
#[doc = "Register `AGGR_STATUS_CLR` writer"]
pub type W = crate::W<AggrStatusClrSpec>;
#[doc = "Field `AGGR_PARITY_ERR` reader - Aggregator Parity Error Status"]
pub type AggrParityErrR = crate::FieldReader;
#[doc = "Field `AGGR_PARITY_ERR` writer - Aggregator Parity Error Status"]
pub type AggrParityErrW<'a, REG> = crate::FieldWriter<'a, REG, 2, u8, crate::Safe>;
#[doc = "Field `SVBUS_TIMEOUT` reader - Aggregator Serial VBUS Timeout Error Status"]
pub type SvbusTimeoutR = crate::FieldReader;
#[doc = "Field `SVBUS_TIMEOUT` writer - Aggregator Serial VBUS Timeout Error Status"]
pub type SvbusTimeoutW<'a, REG> = crate::FieldWriter<'a, REG, 2, u8, crate::Safe>;
impl R {
    #[doc = "Bits 0:1 - Aggregator Parity Error Status"]
    #[inline(always)]
    pub fn aggr_parity_err(&self) -> AggrParityErrR {
        AggrParityErrR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Aggregator Serial VBUS Timeout Error Status"]
    #[inline(always)]
    pub fn svbus_timeout(&self) -> SvbusTimeoutR {
        SvbusTimeoutR::new(((self.bits >> 2) & 3) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AGGR_STATUS_CLR")
            .field("aggr_parity_err", &self.aggr_parity_err())
            .field("svbus_timeout", &self.svbus_timeout())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - Aggregator Parity Error Status"]
    #[inline(always)]
    pub fn aggr_parity_err(&mut self) -> AggrParityErrW<'_, AggrStatusClrSpec> {
        AggrParityErrW::new(self, 0)
    }
    #[doc = "Bits 2:3 - Aggregator Serial VBUS Timeout Error Status"]
    #[inline(always)]
    pub fn svbus_timeout(&mut self) -> SvbusTimeoutW<'_, AggrStatusClrSpec> {
        SvbusTimeoutW::new(self, 2)
    }
}
#[doc = "MCAN Error Aggregator Status Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`aggr_status_clr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aggr_status_clr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AggrStatusClrSpec;
impl crate::RegisterSpec for AggrStatusClrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aggr_status_clr::R`](R) reader structure"]
impl crate::Readable for AggrStatusClrSpec {}
#[doc = "`write(|w| ..)` method takes [`aggr_status_clr::W`](W) writer structure"]
impl crate::Writable for AggrStatusClrSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets AGGR_STATUS_CLR to value 0"]
impl crate::Resettable for AggrStatusClrSpec {}
