#[doc = "Register `PPC_PRIVATTRIB_COPROC` reader"]
pub type R = crate::R<PpcPrivattribCoprocSpec>;
#[doc = "Register `PPC_PRIVATTRIB_COPROC` writer"]
pub type W = crate::W<PpcPrivattribCoprocSpec>;
#[doc = "Field `PRIV_TINIE` reader - Privilege access for TINIE"]
pub type PrivTinieR = crate::BitReader;
#[doc = "Field `PRIV_TINIE` writer - Privilege access for TINIE"]
pub type PrivTinieW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Privilege access for TINIE"]
    #[inline(always)]
    pub fn priv_tinie(&self) -> PrivTinieR {
        PrivTinieR::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PPC_PRIVATTRIB_COPROC")
            .field("priv_tinie", &self.priv_tinie())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Privilege access for TINIE"]
    #[inline(always)]
    pub fn priv_tinie(&mut self) -> PrivTinieW<'_, PpcPrivattribCoprocSpec> {
        PrivTinieW::new(self, 0)
    }
}
#[doc = "Peripheral Privilege Access Co-processor\n\nYou can [`read`](crate::Reg::read) this register and get [`ppc_privattrib_coproc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppc_privattrib_coproc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PpcPrivattribCoprocSpec;
impl crate::RegisterSpec for PpcPrivattribCoprocSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ppc_privattrib_coproc::R`](R) reader structure"]
impl crate::Readable for PpcPrivattribCoprocSpec {}
#[doc = "`write(|w| ..)` method takes [`ppc_privattrib_coproc::W`](W) writer structure"]
impl crate::Writable for PpcPrivattribCoprocSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PPC_PRIVATTRIB_COPROC to value 0"]
impl crate::Resettable for PpcPrivattribCoprocSpec {}
