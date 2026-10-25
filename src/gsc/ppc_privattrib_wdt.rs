#[doc = "Register `PPC_PRIVATTRIB_WDT` reader"]
pub type R = crate::R<PpcPrivattribWdtSpec>;
#[doc = "Register `PPC_PRIVATTRIB_WDT` writer"]
pub type W = crate::W<PpcPrivattribWdtSpec>;
#[doc = "Field `PRIV_WWDT` reader - Privilege access for Windowed Watchdog"]
pub type PrivWwdtR = crate::BitReader;
#[doc = "Field `PRIV_WWDT` writer - Privilege access for Windowed Watchdog"]
pub type PrivWwdtW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Privilege access for Windowed Watchdog"]
    #[inline(always)]
    pub fn priv_wwdt(&self) -> PrivWwdtR {
        PrivWwdtR::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PPC_PRIVATTRIB_WDT")
            .field("priv_wwdt", &self.priv_wwdt())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Privilege access for Windowed Watchdog"]
    #[inline(always)]
    pub fn priv_wwdt(&mut self) -> PrivWwdtW<'_, PpcPrivattribWdtSpec> {
        PrivWwdtW::new(self, 0)
    }
}
#[doc = "Peripheral Privilege Access Watchdog\n\nYou can [`read`](crate::Reg::read) this register and get [`ppc_privattrib_wdt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppc_privattrib_wdt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PpcPrivattribWdtSpec;
impl crate::RegisterSpec for PpcPrivattribWdtSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ppc_privattrib_wdt::R`](R) reader structure"]
impl crate::Readable for PpcPrivattribWdtSpec {}
#[doc = "`write(|w| ..)` method takes [`ppc_privattrib_wdt::W`](W) writer structure"]
impl crate::Writable for PpcPrivattribWdtSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PPC_PRIVATTRIB_WDT to value 0"]
impl crate::Resettable for PpcPrivattribWdtSpec {}
