#[doc = "Register `PPC_PRIVATTRIB_WUC` reader"]
pub type R = crate::R<PpcPrivattribWucSpec>;
#[doc = "Register `PPC_PRIVATTRIB_WUC` writer"]
pub type W = crate::W<PpcPrivattribWucSpec>;
#[doc = "Field `PRIV_WUC` reader - Privilege access for WUC"]
pub type PrivWucR = crate::BitReader;
#[doc = "Field `PRIV_WUC` writer - Privilege access for WUC"]
pub type PrivWucW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Privilege access for WUC"]
    #[inline(always)]
    pub fn priv_wuc(&self) -> PrivWucR {
        PrivWucR::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PPC_PRIVATTRIB_WUC")
            .field("priv_wuc", &self.priv_wuc())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Privilege access for WUC"]
    #[inline(always)]
    pub fn priv_wuc(&mut self) -> PrivWucW<'_, PpcPrivattribWucSpec> {
        PrivWucW::new(self, 0)
    }
}
#[doc = "Peripheral Privilege Access WUC\n\nYou can [`read`](crate::Reg::read) this register and get [`ppc_privattrib_wuc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppc_privattrib_wuc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PpcPrivattribWucSpec;
impl crate::RegisterSpec for PpcPrivattribWucSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ppc_privattrib_wuc::R`](R) reader structure"]
impl crate::Readable for PpcPrivattribWucSpec {}
#[doc = "`write(|w| ..)` method takes [`ppc_privattrib_wuc::W`](W) writer structure"]
impl crate::Writable for PpcPrivattribWucSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PPC_PRIVATTRIB_WUC to value 0"]
impl crate::Resettable for PpcPrivattribWucSpec {}
