#[doc = "Register `PPC_PRIVATTRIB_SYSCTL` reader"]
pub type R = crate::R<PpcPrivattribSysctlSpec>;
#[doc = "Register `PPC_PRIVATTRIB_SYSCTL` writer"]
pub type W = crate::W<PpcPrivattribSysctlSpec>;
#[doc = "Field `PRIV_SYSCTL` reader - Privilege access for SYSCTL"]
pub type PrivSysctlR = crate::BitReader;
#[doc = "Field `PRIV_SYSCTL` writer - Privilege access for SYSCTL"]
pub type PrivSysctlW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Privilege access for SYSCTL"]
    #[inline(always)]
    pub fn priv_sysctl(&self) -> PrivSysctlR {
        PrivSysctlR::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PPC_PRIVATTRIB_SYSCTL")
            .field("priv_sysctl", &self.priv_sysctl())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Privilege access for SYSCTL"]
    #[inline(always)]
    pub fn priv_sysctl(&mut self) -> PrivSysctlW<'_, PpcPrivattribSysctlSpec> {
        PrivSysctlW::new(self, 0)
    }
}
#[doc = "Peripheral Privilege Access System Control\n\nYou can [`read`](crate::Reg::read) this register and get [`ppc_privattrib_sysctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppc_privattrib_sysctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PpcPrivattribSysctlSpec;
impl crate::RegisterSpec for PpcPrivattribSysctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ppc_privattrib_sysctl::R`](R) reader structure"]
impl crate::Readable for PpcPrivattribSysctlSpec {}
#[doc = "`write(|w| ..)` method takes [`ppc_privattrib_sysctl::W`](W) writer structure"]
impl crate::Writable for PpcPrivattribSysctlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PPC_PRIVATTRIB_SYSCTL to value 0x01"]
impl crate::Resettable for PpcPrivattribSysctlSpec {
    const RESET_VALUE: u32 = 0x01;
}
