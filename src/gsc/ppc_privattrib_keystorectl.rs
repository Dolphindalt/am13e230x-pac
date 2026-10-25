#[doc = "Register `PPC_PRIVATTRIB_KEYSTORECTL` reader"]
pub type R = crate::R<PpcPrivattribKeystorectlSpec>;
#[doc = "Register `PPC_PRIVATTRIB_KEYSTORECTL` writer"]
pub type W = crate::W<PpcPrivattribKeystorectlSpec>;
#[doc = "Field `PRIV_KEYSTORECTL` reader - Privilege access for KEYSTORECTL"]
pub type PrivKeystorectlR = crate::BitReader;
#[doc = "Field `PRIV_KEYSTORECTL` writer - Privilege access for KEYSTORECTL"]
pub type PrivKeystorectlW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Privilege access for KEYSTORECTL"]
    #[inline(always)]
    pub fn priv_keystorectl(&self) -> PrivKeystorectlR {
        PrivKeystorectlR::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PPC_PRIVATTRIB_KEYSTORECTL")
            .field("priv_keystorectl", &self.priv_keystorectl())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Privilege access for KEYSTORECTL"]
    #[inline(always)]
    pub fn priv_keystorectl(&mut self) -> PrivKeystorectlW<'_, PpcPrivattribKeystorectlSpec> {
        PrivKeystorectlW::new(self, 0)
    }
}
#[doc = "Peripheral Privilege Access KEYSTORECTL\n\nYou can [`read`](crate::Reg::read) this register and get [`ppc_privattrib_keystorectl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppc_privattrib_keystorectl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PpcPrivattribKeystorectlSpec;
impl crate::RegisterSpec for PpcPrivattribKeystorectlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ppc_privattrib_keystorectl::R`](R) reader structure"]
impl crate::Readable for PpcPrivattribKeystorectlSpec {}
#[doc = "`write(|w| ..)` method takes [`ppc_privattrib_keystorectl::W`](W) writer structure"]
impl crate::Writable for PpcPrivattribKeystorectlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PPC_PRIVATTRIB_KEYSTORECTL to value 0"]
impl crate::Resettable for PpcPrivattribKeystorectlSpec {}
