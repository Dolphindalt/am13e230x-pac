#[doc = "Register `TI_WRAPPER_REGS_CTRL` reader"]
pub type R = crate::R<TiWrapperRegsCtrlSpec>;
#[doc = "Register `TI_WRAPPER_REGS_CTRL` writer"]
pub type W = crate::W<TiWrapperRegsCtrlSpec>;
#[doc = "Field `DBGSUSP_FREE` reader - Debug Suspend Free"]
pub type DbgsuspFreeR = crate::BitReader;
#[doc = "Field `DBGSUSP_FREE` writer - Debug Suspend Free"]
pub type DbgsuspFreeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WAKEUPREQEN` reader - Wakeup Request Enable"]
pub type WakeupreqenR = crate::BitReader;
#[doc = "Field `WAKEUPREQEN` writer - Wakeup Request Enable"]
pub type WakeupreqenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUTOWAKEUP` reader - Automatic Wakeup Enable"]
pub type AutowakeupR = crate::BitReader;
#[doc = "Field `AUTOWAKEUP` writer - Automatic Wakeup Enable"]
pub type AutowakeupW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXT_TS_CNTR_EN` reader - External Timestamp Counter Enable"]
pub type ExtTsCntrEnR = crate::BitReader;
#[doc = "Field `EXT_TS_CNTR_EN` writer - External Timestamp Counter Enable"]
pub type ExtTsCntrEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 3 - Debug Suspend Free"]
    #[inline(always)]
    pub fn dbgsusp_free(&self) -> DbgsuspFreeR {
        DbgsuspFreeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Wakeup Request Enable"]
    #[inline(always)]
    pub fn wakeupreqen(&self) -> WakeupreqenR {
        WakeupreqenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Automatic Wakeup Enable"]
    #[inline(always)]
    pub fn autowakeup(&self) -> AutowakeupR {
        AutowakeupR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - External Timestamp Counter Enable"]
    #[inline(always)]
    pub fn ext_ts_cntr_en(&self) -> ExtTsCntrEnR {
        ExtTsCntrEnR::new(((self.bits >> 6) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TI_WRAPPER_REGS_CTRL")
            .field("dbgsusp_free", &self.dbgsusp_free())
            .field("wakeupreqen", &self.wakeupreqen())
            .field("autowakeup", &self.autowakeup())
            .field("ext_ts_cntr_en", &self.ext_ts_cntr_en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 3 - Debug Suspend Free"]
    #[inline(always)]
    pub fn dbgsusp_free(&mut self) -> DbgsuspFreeW<'_, TiWrapperRegsCtrlSpec> {
        DbgsuspFreeW::new(self, 3)
    }
    #[doc = "Bit 4 - Wakeup Request Enable"]
    #[inline(always)]
    pub fn wakeupreqen(&mut self) -> WakeupreqenW<'_, TiWrapperRegsCtrlSpec> {
        WakeupreqenW::new(self, 4)
    }
    #[doc = "Bit 5 - Automatic Wakeup Enable"]
    #[inline(always)]
    pub fn autowakeup(&mut self) -> AutowakeupW<'_, TiWrapperRegsCtrlSpec> {
        AutowakeupW::new(self, 5)
    }
    #[doc = "Bit 6 - External Timestamp Counter Enable"]
    #[inline(always)]
    pub fn ext_ts_cntr_en(&mut self) -> ExtTsCntrEnW<'_, TiWrapperRegsCtrlSpec> {
        ExtTsCntrEnW::new(self, 6)
    }
}
#[doc = "MCAN Subsystem Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ti_wrapper_regs_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ti_wrapper_regs_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TiWrapperRegsCtrlSpec;
impl crate::RegisterSpec for TiWrapperRegsCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ti_wrapper_regs_ctrl::R`](R) reader structure"]
impl crate::Readable for TiWrapperRegsCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ti_wrapper_regs_ctrl::W`](W) writer structure"]
impl crate::Writable for TiWrapperRegsCtrlSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets TI_WRAPPER_REGS_CTRL to value 0x08"]
impl crate::Resettable for TiWrapperRegsCtrlSpec {
    const RESET_VALUE: u32 = 0x08;
}
