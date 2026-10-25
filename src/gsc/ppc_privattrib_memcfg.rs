#[doc = "Register `PPC_PRIVATTRIB_MEMCFG` reader"]
pub type R = crate::R<PpcPrivattribMemcfgSpec>;
#[doc = "Register `PPC_PRIVATTRIB_MEMCFG` writer"]
pub type W = crate::W<PpcPrivattribMemcfgSpec>;
#[doc = "Field `PRIV_MEMCFG` reader - Privilege access for MEMCFG"]
pub type PrivMemcfgR = crate::BitReader;
#[doc = "Field `PRIV_MEMCFG` writer - Privilege access for MEMCFG"]
pub type PrivMemcfgW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Privilege access for MEMCFG"]
    #[inline(always)]
    pub fn priv_memcfg(&self) -> PrivMemcfgR {
        PrivMemcfgR::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PPC_PRIVATTRIB_MEMCFG")
            .field("priv_memcfg", &self.priv_memcfg())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Privilege access for MEMCFG"]
    #[inline(always)]
    pub fn priv_memcfg(&mut self) -> PrivMemcfgW<'_, PpcPrivattribMemcfgSpec> {
        PrivMemcfgW::new(self, 0)
    }
}
#[doc = "Peripheral Privilege Access MEMCFG\n\nYou can [`read`](crate::Reg::read) this register and get [`ppc_privattrib_memcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppc_privattrib_memcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PpcPrivattribMemcfgSpec;
impl crate::RegisterSpec for PpcPrivattribMemcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ppc_privattrib_memcfg::R`](R) reader structure"]
impl crate::Readable for PpcPrivattribMemcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`ppc_privattrib_memcfg::W`](W) writer structure"]
impl crate::Writable for PpcPrivattribMemcfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PPC_PRIVATTRIB_MEMCFG to value 0"]
impl crate::Resettable for PpcPrivattribMemcfgSpec {}
