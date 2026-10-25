#[doc = "Register `PPC_PRIVATTRIB_EPI` reader"]
pub type R = crate::R<PpcPrivattribEpiSpec>;
#[doc = "Register `PPC_PRIVATTRIB_EPI` writer"]
pub type W = crate::W<PpcPrivattribEpiSpec>;
#[doc = "Field `PRIV_EPI` reader - Privilege access for EPI"]
pub type PrivEpiR = crate::BitReader;
#[doc = "Field `PRIV_EPI` writer - Privilege access for EPI"]
pub type PrivEpiW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Privilege access for EPI"]
    #[inline(always)]
    pub fn priv_epi(&self) -> PrivEpiR {
        PrivEpiR::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PPC_PRIVATTRIB_EPI")
            .field("priv_epi", &self.priv_epi())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Privilege access for EPI"]
    #[inline(always)]
    pub fn priv_epi(&mut self) -> PrivEpiW<'_, PpcPrivattribEpiSpec> {
        PrivEpiW::new(self, 0)
    }
}
#[doc = "Peripheral Privilege Access EPI\n\nYou can [`read`](crate::Reg::read) this register and get [`ppc_privattrib_epi::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppc_privattrib_epi::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PpcPrivattribEpiSpec;
impl crate::RegisterSpec for PpcPrivattribEpiSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ppc_privattrib_epi::R`](R) reader structure"]
impl crate::Readable for PpcPrivattribEpiSpec {}
#[doc = "`write(|w| ..)` method takes [`ppc_privattrib_epi::W`](W) writer structure"]
impl crate::Writable for PpcPrivattribEpiSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PPC_PRIVATTRIB_EPI to value 0"]
impl crate::Resettable for PpcPrivattribEpiSpec {}
