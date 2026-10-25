#[doc = "Register `PPC_PRIVATTRIB_FRIREGS` reader"]
pub type R = crate::R<PpcPrivattribFriregsSpec>;
#[doc = "Register `PPC_PRIVATTRIB_FRIREGS` writer"]
pub type W = crate::W<PpcPrivattribFriregsSpec>;
#[doc = "Field `PRIV_FRIREGS` reader - Privilege access for FRIREGS"]
pub type PrivFriregsR = crate::BitReader;
#[doc = "Field `PRIV_FRIREGS` writer - Privilege access for FRIREGS"]
pub type PrivFriregsW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Privilege access for FRIREGS"]
    #[inline(always)]
    pub fn priv_friregs(&self) -> PrivFriregsR {
        PrivFriregsR::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PPC_PRIVATTRIB_FRIREGS")
            .field("priv_friregs", &self.priv_friregs())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Privilege access for FRIREGS"]
    #[inline(always)]
    pub fn priv_friregs(&mut self) -> PrivFriregsW<'_, PpcPrivattribFriregsSpec> {
        PrivFriregsW::new(self, 0)
    }
}
#[doc = "Peripheral Privilege Access FRIREGS\n\nYou can [`read`](crate::Reg::read) this register and get [`ppc_privattrib_friregs::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppc_privattrib_friregs::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PpcPrivattribFriregsSpec;
impl crate::RegisterSpec for PpcPrivattribFriregsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ppc_privattrib_friregs::R`](R) reader structure"]
impl crate::Readable for PpcPrivattribFriregsSpec {}
#[doc = "`write(|w| ..)` method takes [`ppc_privattrib_friregs::W`](W) writer structure"]
impl crate::Writable for PpcPrivattribFriregsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PPC_PRIVATTRIB_FRIREGS to value 0"]
impl crate::Resettable for PpcPrivattribFriregsSpec {}
