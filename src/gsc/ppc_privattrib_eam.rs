#[doc = "Register `PPC_PRIVATTRIB_EAM` reader"]
pub type R = crate::R<PpcPrivattribEamSpec>;
#[doc = "Register `PPC_PRIVATTRIB_EAM` writer"]
pub type W = crate::W<PpcPrivattribEamSpec>;
#[doc = "Field `PRIV_EAM` reader - Privilege access for EAM"]
pub type PrivEamR = crate::BitReader;
#[doc = "Field `PRIV_EAM` writer - Privilege access for EAM"]
pub type PrivEamW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Privilege access for EAM"]
    #[inline(always)]
    pub fn priv_eam(&self) -> PrivEamR {
        PrivEamR::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PPC_PRIVATTRIB_EAM")
            .field("priv_eam", &self.priv_eam())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Privilege access for EAM"]
    #[inline(always)]
    pub fn priv_eam(&mut self) -> PrivEamW<'_, PpcPrivattribEamSpec> {
        PrivEamW::new(self, 0)
    }
}
#[doc = "Peripheral Privilege Access EAM\n\nYou can [`read`](crate::Reg::read) this register and get [`ppc_privattrib_eam::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppc_privattrib_eam::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PpcPrivattribEamSpec;
impl crate::RegisterSpec for PpcPrivattribEamSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ppc_privattrib_eam::R`](R) reader structure"]
impl crate::Readable for PpcPrivattribEamSpec {}
#[doc = "`write(|w| ..)` method takes [`ppc_privattrib_eam::W`](W) writer structure"]
impl crate::Writable for PpcPrivattribEamSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PPC_PRIVATTRIB_EAM to value 0"]
impl crate::Resettable for PpcPrivattribEamSpec {}
