#[doc = "Register `PPC_PRIVATTRIB_GSC` reader"]
pub type R = crate::R<PpcPrivattribGscSpec>;
#[doc = "Register `PPC_PRIVATTRIB_GSC` writer"]
pub type W = crate::W<PpcPrivattribGscSpec>;
#[doc = "Field `PRIV_GSC` reader - Privilege access for GSC"]
pub type PrivGscR = crate::BitReader;
#[doc = "Field `PRIV_GSC` writer - Privilege access for GSC"]
pub type PrivGscW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Privilege access for GSC"]
    #[inline(always)]
    pub fn priv_gsc(&self) -> PrivGscR {
        PrivGscR::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PPC_PRIVATTRIB_GSC")
            .field("priv_gsc", &self.priv_gsc())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Privilege access for GSC"]
    #[inline(always)]
    pub fn priv_gsc(&mut self) -> PrivGscW<'_, PpcPrivattribGscSpec> {
        PrivGscW::new(self, 0)
    }
}
#[doc = "Peripheral Privilege Access GSC\n\nYou can [`read`](crate::Reg::read) this register and get [`ppc_privattrib_gsc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppc_privattrib_gsc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PpcPrivattribGscSpec;
impl crate::RegisterSpec for PpcPrivattribGscSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ppc_privattrib_gsc::R`](R) reader structure"]
impl crate::Readable for PpcPrivattribGscSpec {}
#[doc = "`write(|w| ..)` method takes [`ppc_privattrib_gsc::W`](W) writer structure"]
impl crate::Writable for PpcPrivattribGscSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PPC_PRIVATTRIB_GSC to value 0x01"]
impl crate::Resettable for PpcPrivattribGscSpec {
    const RESET_VALUE: u32 = 0x01;
}
