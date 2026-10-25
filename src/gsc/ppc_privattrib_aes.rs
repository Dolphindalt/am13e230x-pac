#[doc = "Register `PPC_PRIVATTRIB_AES` reader"]
pub type R = crate::R<PpcPrivattribAesSpec>;
#[doc = "Register `PPC_PRIVATTRIB_AES` writer"]
pub type W = crate::W<PpcPrivattribAesSpec>;
#[doc = "Field `PRIV_AES` reader - Privilege access for AES"]
pub type PrivAesR = crate::BitReader;
#[doc = "Field `PRIV_AES` writer - Privilege access for AES"]
pub type PrivAesW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Privilege access for AES"]
    #[inline(always)]
    pub fn priv_aes(&self) -> PrivAesR {
        PrivAesR::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PPC_PRIVATTRIB_AES")
            .field("priv_aes", &self.priv_aes())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Privilege access for AES"]
    #[inline(always)]
    pub fn priv_aes(&mut self) -> PrivAesW<'_, PpcPrivattribAesSpec> {
        PrivAesW::new(self, 0)
    }
}
#[doc = "Peripheral Privilege Access AES\n\nYou can [`read`](crate::Reg::read) this register and get [`ppc_privattrib_aes::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppc_privattrib_aes::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PpcPrivattribAesSpec;
impl crate::RegisterSpec for PpcPrivattribAesSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ppc_privattrib_aes::R`](R) reader structure"]
impl crate::Readable for PpcPrivattribAesSpec {}
#[doc = "`write(|w| ..)` method takes [`ppc_privattrib_aes::W`](W) writer structure"]
impl crate::Writable for PpcPrivattribAesSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PPC_PRIVATTRIB_AES to value 0"]
impl crate::Resettable for PpcPrivattribAesSpec {}
