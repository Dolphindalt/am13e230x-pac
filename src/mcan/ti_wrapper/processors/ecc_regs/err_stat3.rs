#[doc = "Register `ERR_STAT3` reader"]
pub type R = crate::R<ErrStat3Spec>;
#[doc = "Register `ERR_STAT3` writer"]
pub type W = crate::W<ErrStat3Spec>;
#[doc = "Field `WB_PEND` reader - Delayed Write Back Pending Status"]
pub type WbPendR = crate::BitReader;
#[doc = "Field `SVBUS_TIMEOUT` reader - Serial VBUS Timeout Flag"]
pub type SvbusTimeoutR = crate::BitReader;
#[doc = "Field `SVBUS_TIMEOUT` writer - Serial VBUS Timeout Flag"]
pub type SvbusTimeoutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLR_SVBUS_TIMEOUT` reader - Clear Serial VBUS Timeout"]
pub type ClrSvbusTimeoutR = crate::BitReader;
#[doc = "Field `CLR_SVBUS_TIMEOUT` writer - Clear Serial VBUS Timeout"]
pub type ClrSvbusTimeoutW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Delayed Write Back Pending Status"]
    #[inline(always)]
    pub fn wb_pend(&self) -> WbPendR {
        WbPendR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Serial VBUS Timeout Flag"]
    #[inline(always)]
    pub fn svbus_timeout(&self) -> SvbusTimeoutR {
        SvbusTimeoutR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 9 - Clear Serial VBUS Timeout"]
    #[inline(always)]
    pub fn clr_svbus_timeout(&self) -> ClrSvbusTimeoutR {
        ClrSvbusTimeoutR::new(((self.bits >> 9) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ERR_STAT3")
            .field("wb_pend", &self.wb_pend())
            .field("svbus_timeout", &self.svbus_timeout())
            .field("clr_svbus_timeout", &self.clr_svbus_timeout())
            .finish()
    }
}
impl W {
    #[doc = "Bit 1 - Serial VBUS Timeout Flag"]
    #[inline(always)]
    pub fn svbus_timeout(&mut self) -> SvbusTimeoutW<'_, ErrStat3Spec> {
        SvbusTimeoutW::new(self, 1)
    }
    #[doc = "Bit 9 - Clear Serial VBUS Timeout"]
    #[inline(always)]
    pub fn clr_svbus_timeout(&mut self) -> ClrSvbusTimeoutW<'_, ErrStat3Spec> {
        ClrSvbusTimeoutW::new(self, 9)
    }
}
#[doc = "MCAN ECC Error Status 3 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`err_stat3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`err_stat3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ErrStat3Spec;
impl crate::RegisterSpec for ErrStat3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`err_stat3::R`](R) reader structure"]
impl crate::Readable for ErrStat3Spec {}
#[doc = "`write(|w| ..)` method takes [`err_stat3::W`](W) writer structure"]
impl crate::Writable for ErrStat3Spec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets ERR_STAT3 to value 0"]
impl crate::Resettable for ErrStat3Spec {}
