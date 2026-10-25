#[doc = "Register `ETCNT` reader"]
pub type R = crate::R<EtcntSpec>;
#[doc = "Register `ETCNT` writer"]
pub type W = crate::W<EtcntSpec>;
#[doc = "Field `ET1_CNT` reader - Event trigger1 COUNT"]
pub type Et1CntR = crate::FieldReader;
#[doc = "Field `ET2_CNT` reader - Event trigger2 COUNT"]
pub type Et2CntR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:2 - Event trigger1 COUNT"]
    #[inline(always)]
    pub fn et1_cnt(&self) -> Et1CntR {
        Et1CntR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:10 - Event trigger2 COUNT"]
    #[inline(always)]
    pub fn et2_cnt(&self) -> Et2CntR {
        Et2CntR::new(((self.bits >> 8) & 7) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ETCNT")
            .field("et1_cnt", &self.et1_cnt())
            .field("et2_cnt", &self.et2_cnt())
            .finish()
    }
}
impl W {}
#[doc = "Event trigger count\n\nYou can [`read`](crate::Reg::read) this register and get [`etcnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`etcnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EtcntSpec;
impl crate::RegisterSpec for EtcntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`etcnt::R`](R) reader structure"]
impl crate::Readable for EtcntSpec {}
#[doc = "`write(|w| ..)` method takes [`etcnt::W`](W) writer structure"]
impl crate::Writable for EtcntSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets ETCNT to value 0"]
impl crate::Resettable for EtcntSpec {}
