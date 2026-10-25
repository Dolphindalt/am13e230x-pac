#[doc = "Register `PPC_PRIVATTRIB_DEBUGSS` reader"]
pub type R = crate::R<PpcPrivattribDebugssSpec>;
#[doc = "Register `PPC_PRIVATTRIB_DEBUGSS` writer"]
pub type W = crate::W<PpcPrivattribDebugssSpec>;
#[doc = "Field `PRIV_DEBUGSS` reader - Privilege access for DEBUGSS"]
pub type PrivDebugssR = crate::BitReader;
#[doc = "Field `PRIV_DEBUGSS` writer - Privilege access for DEBUGSS"]
pub type PrivDebugssW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Privilege access for DEBUGSS"]
    #[inline(always)]
    pub fn priv_debugss(&self) -> PrivDebugssR {
        PrivDebugssR::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PPC_PRIVATTRIB_DEBUGSS")
            .field("priv_debugss", &self.priv_debugss())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Privilege access for DEBUGSS"]
    #[inline(always)]
    pub fn priv_debugss(&mut self) -> PrivDebugssW<'_, PpcPrivattribDebugssSpec> {
        PrivDebugssW::new(self, 0)
    }
}
#[doc = "Peripheral Privilege Access DEBUGSS\n\nYou can [`read`](crate::Reg::read) this register and get [`ppc_privattrib_debugss::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppc_privattrib_debugss::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PpcPrivattribDebugssSpec;
impl crate::RegisterSpec for PpcPrivattribDebugssSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ppc_privattrib_debugss::R`](R) reader structure"]
impl crate::Readable for PpcPrivattribDebugssSpec {}
#[doc = "`write(|w| ..)` method takes [`ppc_privattrib_debugss::W`](W) writer structure"]
impl crate::Writable for PpcPrivattribDebugssSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PPC_PRIVATTRIB_DEBUGSS to value 0x01"]
impl crate::Resettable for PpcPrivattribDebugssSpec {
    const RESET_VALUE: u32 = 0x01;
}
