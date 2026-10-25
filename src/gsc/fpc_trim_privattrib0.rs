#[doc = "Register `FPC_TRIM_PRIVATTRIB0` reader"]
pub type R = crate::R<FpcTrimPrivattrib0Spec>;
#[doc = "Register `FPC_TRIM_PRIVATTRIB0` writer"]
pub type W = crate::W<FpcTrimPrivattrib0Spec>;
#[doc = "Field `PRIV_PAGE` reader - Privilege access for TRIM region of Bank0"]
pub type PrivPageR = crate::BitReader;
#[doc = "Field `PRIV_PAGE` writer - Privilege access for TRIM region of Bank0"]
pub type PrivPageW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Privilege access for TRIM region of Bank0"]
    #[inline(always)]
    pub fn priv_page(&self) -> PrivPageR {
        PrivPageR::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FPC_TRIM_PRIVATTRIB0")
            .field("priv_page", &self.priv_page())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Privilege access for TRIM region of Bank0"]
    #[inline(always)]
    pub fn priv_page(&mut self) -> PrivPageW<'_, FpcTrimPrivattrib0Spec> {
        PrivPageW::new(self, 0)
    }
}
#[doc = "Flash Trim Privilege Attribute Register B0\n\nYou can [`read`](crate::Reg::read) this register and get [`fpc_trim_privattrib0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fpc_trim_privattrib0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FpcTrimPrivattrib0Spec;
impl crate::RegisterSpec for FpcTrimPrivattrib0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fpc_trim_privattrib0::R`](R) reader structure"]
impl crate::Readable for FpcTrimPrivattrib0Spec {}
#[doc = "`write(|w| ..)` method takes [`fpc_trim_privattrib0::W`](W) writer structure"]
impl crate::Writable for FpcTrimPrivattrib0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FPC_TRIM_PRIVATTRIB0 to value 0x01"]
impl crate::Resettable for FpcTrimPrivattrib0Spec {
    const RESET_VALUE: u32 = 0x01;
}
