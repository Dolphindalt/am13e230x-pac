#[doc = "Register `TI_WRAPPER_ECC_REGS_CTRL` reader"]
pub type R = crate::R<TiWrapperEccRegsCtrlSpec>;
#[doc = "Register `TI_WRAPPER_ECC_REGS_CTRL` writer"]
pub type W = crate::W<TiWrapperEccRegsCtrlSpec>;
#[doc = "Field `ECC_ENABLE` reader - Enable ECC Generation"]
pub type EccEnableR = crate::BitReader;
#[doc = "Field `ECC_ENABLE` writer - Enable ECC Generation"]
pub type EccEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECC_CHECK` reader - Enable ECC Check"]
pub type EccCheckR = crate::BitReader;
#[doc = "Field `ECC_CHECK` writer - Enable ECC Check"]
pub type EccCheckW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENABLE_RMW` reader - Enable Read-Modify-Write"]
pub type EnableRmwR = crate::BitReader;
#[doc = "Field `ENABLE_RMW` writer - Enable Read-Modify-Write"]
pub type EnableRmwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_SEC` reader - Force Single Bit Error Corrected Error"]
pub type ForceSecR = crate::BitReader;
#[doc = "Field `FORCE_SEC` writer - Force Single Bit Error Corrected Error"]
pub type ForceSecW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_DED` reader - Force Double Bit Error Detected Error"]
pub type ForceDedR = crate::BitReader;
#[doc = "Field `FORCE_DED` writer - Force Double Bit Error Detected Error"]
pub type ForceDedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_N_ROW` reader - Force Next Single/Double Bit Error"]
pub type ForceNRowR = crate::BitReader;
#[doc = "Field `FORCE_N_ROW` writer - Force Next Single/Double Bit Error"]
pub type ForceNRowW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERROR_ONCE` reader - Force Error Only Once Enable"]
pub type ErrorOnceR = crate::BitReader;
#[doc = "Field `ERROR_ONCE` writer - Force Error Only Once Enable"]
pub type ErrorOnceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHECK_SVBUS_TIMEOUT` reader - SVBUS Timeout Enable"]
pub type CheckSvbusTimeoutR = crate::BitReader;
#[doc = "Field `CHECK_SVBUS_TIMEOUT` writer - SVBUS Timeout Enable"]
pub type CheckSvbusTimeoutW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable ECC Generation"]
    #[inline(always)]
    pub fn ecc_enable(&self) -> EccEnableR {
        EccEnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable ECC Check"]
    #[inline(always)]
    pub fn ecc_check(&self) -> EccCheckR {
        EccCheckR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable Read-Modify-Write"]
    #[inline(always)]
    pub fn enable_rmw(&self) -> EnableRmwR {
        EnableRmwR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Force Single Bit Error Corrected Error"]
    #[inline(always)]
    pub fn force_sec(&self) -> ForceSecR {
        ForceSecR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Force Double Bit Error Detected Error"]
    #[inline(always)]
    pub fn force_ded(&self) -> ForceDedR {
        ForceDedR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Force Next Single/Double Bit Error"]
    #[inline(always)]
    pub fn force_n_row(&self) -> ForceNRowR {
        ForceNRowR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Force Error Only Once Enable"]
    #[inline(always)]
    pub fn error_once(&self) -> ErrorOnceR {
        ErrorOnceR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - SVBUS Timeout Enable"]
    #[inline(always)]
    pub fn check_svbus_timeout(&self) -> CheckSvbusTimeoutR {
        CheckSvbusTimeoutR::new(((self.bits >> 8) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TI_WRAPPER_ECC_REGS_CTRL")
            .field("ecc_enable", &self.ecc_enable())
            .field("ecc_check", &self.ecc_check())
            .field("enable_rmw", &self.enable_rmw())
            .field("force_sec", &self.force_sec())
            .field("force_ded", &self.force_ded())
            .field("force_n_row", &self.force_n_row())
            .field("error_once", &self.error_once())
            .field("check_svbus_timeout", &self.check_svbus_timeout())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Enable ECC Generation"]
    #[inline(always)]
    pub fn ecc_enable(&mut self) -> EccEnableW<'_, TiWrapperEccRegsCtrlSpec> {
        EccEnableW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable ECC Check"]
    #[inline(always)]
    pub fn ecc_check(&mut self) -> EccCheckW<'_, TiWrapperEccRegsCtrlSpec> {
        EccCheckW::new(self, 1)
    }
    #[doc = "Bit 2 - Enable Read-Modify-Write"]
    #[inline(always)]
    pub fn enable_rmw(&mut self) -> EnableRmwW<'_, TiWrapperEccRegsCtrlSpec> {
        EnableRmwW::new(self, 2)
    }
    #[doc = "Bit 3 - Force Single Bit Error Corrected Error"]
    #[inline(always)]
    pub fn force_sec(&mut self) -> ForceSecW<'_, TiWrapperEccRegsCtrlSpec> {
        ForceSecW::new(self, 3)
    }
    #[doc = "Bit 4 - Force Double Bit Error Detected Error"]
    #[inline(always)]
    pub fn force_ded(&mut self) -> ForceDedW<'_, TiWrapperEccRegsCtrlSpec> {
        ForceDedW::new(self, 4)
    }
    #[doc = "Bit 5 - Force Next Single/Double Bit Error"]
    #[inline(always)]
    pub fn force_n_row(&mut self) -> ForceNRowW<'_, TiWrapperEccRegsCtrlSpec> {
        ForceNRowW::new(self, 5)
    }
    #[doc = "Bit 6 - Force Error Only Once Enable"]
    #[inline(always)]
    pub fn error_once(&mut self) -> ErrorOnceW<'_, TiWrapperEccRegsCtrlSpec> {
        ErrorOnceW::new(self, 6)
    }
    #[doc = "Bit 8 - SVBUS Timeout Enable"]
    #[inline(always)]
    pub fn check_svbus_timeout(&mut self) -> CheckSvbusTimeoutW<'_, TiWrapperEccRegsCtrlSpec> {
        CheckSvbusTimeoutW::new(self, 8)
    }
}
#[doc = "MCAN ECC Control\n\nYou can [`read`](crate::Reg::read) this register and get [`ti_wrapper_ecc_regs_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ti_wrapper_ecc_regs_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TiWrapperEccRegsCtrlSpec;
impl crate::RegisterSpec for TiWrapperEccRegsCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ti_wrapper_ecc_regs_ctrl::R`](R) reader structure"]
impl crate::Readable for TiWrapperEccRegsCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ti_wrapper_ecc_regs_ctrl::W`](W) writer structure"]
impl crate::Writable for TiWrapperEccRegsCtrlSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets TI_WRAPPER_ECC_REGS_CTRL to value 0x0187"]
impl crate::Resettable for TiWrapperEccRegsCtrlSpec {
    const RESET_VALUE: u32 = 0x0187;
}
