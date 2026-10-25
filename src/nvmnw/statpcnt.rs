#[doc = "Register `STATPCNT` reader"]
pub type R = crate::R<StatpcntSpec>;
#[doc = "Field `PULSECNT` reader - Current Pulse Counter Value"]
pub type PulsecntR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:11 - Current Pulse Counter Value"]
    #[inline(always)]
    pub fn pulsecnt(&self) -> PulsecntR {
        PulsecntR::new((self.bits & 0x0fff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STATPCNT")
            .field("pulsecnt", &self.pulsecnt())
            .finish()
    }
}
#[doc = "Pulse Count Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`statpcnt::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatpcntSpec;
impl crate::RegisterSpec for StatpcntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`statpcnt::R`](R) reader structure"]
impl crate::Readable for StatpcntSpec {}
#[doc = "`reset()` method sets STATPCNT to value 0"]
impl crate::Resettable for StatpcntSpec {}
