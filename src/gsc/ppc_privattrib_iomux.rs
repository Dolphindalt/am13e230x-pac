#[doc = "Register `PPC_PRIVATTRIB_IOMUX` reader"]
pub type R = crate::R<PpcPrivattribIomuxSpec>;
#[doc = "Register `PPC_PRIVATTRIB_IOMUX` writer"]
pub type W = crate::W<PpcPrivattribIomuxSpec>;
#[doc = "Field `PRIV_IOMUX` reader - Privilege access for IOMUX"]
pub type PrivIomuxR = crate::BitReader;
#[doc = "Field `PRIV_IOMUX` writer - Privilege access for IOMUX"]
pub type PrivIomuxW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Privilege access for IOMUX"]
    #[inline(always)]
    pub fn priv_iomux(&self) -> PrivIomuxR {
        PrivIomuxR::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PPC_PRIVATTRIB_IOMUX")
            .field("priv_iomux", &self.priv_iomux())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Privilege access for IOMUX"]
    #[inline(always)]
    pub fn priv_iomux(&mut self) -> PrivIomuxW<'_, PpcPrivattribIomuxSpec> {
        PrivIomuxW::new(self, 0)
    }
}
#[doc = "Peripheral Privilege Access IOMUX\n\nYou can [`read`](crate::Reg::read) this register and get [`ppc_privattrib_iomux::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppc_privattrib_iomux::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PpcPrivattribIomuxSpec;
impl crate::RegisterSpec for PpcPrivattribIomuxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ppc_privattrib_iomux::R`](R) reader structure"]
impl crate::Readable for PpcPrivattribIomuxSpec {}
#[doc = "`write(|w| ..)` method takes [`ppc_privattrib_iomux::W`](W) writer structure"]
impl crate::Writable for PpcPrivattribIomuxSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PPC_PRIVATTRIB_IOMUX to value 0"]
impl crate::Resettable for PpcPrivattribIomuxSpec {}
