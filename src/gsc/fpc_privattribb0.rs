#[doc = "Register `FPC_PRIVATTRIBB0` reader"]
pub type R = crate::R<FpcPrivattribb0Spec>;
#[doc = "Register `FPC_PRIVATTRIBB0` writer"]
pub type W = crate::W<FpcPrivattribb0Spec>;
#[doc = "Field `PRIV_PAGE` reader - Privilege access for higher sectors of Bank0"]
pub type PrivPageR = crate::FieldReader<u16>;
#[doc = "Field `PRIV_PAGE` writer - Privilege access for higher sectors of Bank0"]
pub type PrivPageW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 4:15 - Privilege access for higher sectors of Bank0"]
    #[inline(always)]
    pub fn priv_page(&self) -> PrivPageR {
        PrivPageR::new(((self.bits >> 4) & 0x0fff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FPC_PRIVATTRIBB0")
            .field("priv_page", &self.priv_page())
            .finish()
    }
}
impl W {
    #[doc = "Bits 4:15 - Privilege access for higher sectors of Bank0"]
    #[inline(always)]
    pub fn priv_page(&mut self) -> PrivPageW<'_, FpcPrivattribb0Spec> {
        PrivPageW::new(self, 4)
    }
}
#[doc = "Flash Privilege Attribute Register B0\n\nYou can [`read`](crate::Reg::read) this register and get [`fpc_privattribb0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fpc_privattribb0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FpcPrivattribb0Spec;
impl crate::RegisterSpec for FpcPrivattribb0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fpc_privattribb0::R`](R) reader structure"]
impl crate::Readable for FpcPrivattribb0Spec {}
#[doc = "`write(|w| ..)` method takes [`fpc_privattribb0::W`](W) writer structure"]
impl crate::Writable for FpcPrivattribb0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FPC_PRIVATTRIBB0 to value 0xfff0"]
impl crate::Resettable for FpcPrivattribb0Spec {
    const RESET_VALUE: u32 = 0xfff0;
}
