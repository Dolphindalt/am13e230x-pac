#[doc = "Register `FPC_PRIVATTRIBB1` reader"]
pub type R = crate::R<FpcPrivattribb1Spec>;
#[doc = "Register `FPC_PRIVATTRIBB1` writer"]
pub type W = crate::W<FpcPrivattribb1Spec>;
#[doc = "Field `PRIV_PAGE` reader - Privilege access for higher sectors of Bank1"]
pub type PrivPageR = crate::FieldReader<u16>;
#[doc = "Field `PRIV_PAGE` writer - Privilege access for higher sectors of Bank1"]
pub type PrivPageW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 4:15 - Privilege access for higher sectors of Bank1"]
    #[inline(always)]
    pub fn priv_page(&self) -> PrivPageR {
        PrivPageR::new(((self.bits >> 4) & 0x0fff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FPC_PRIVATTRIBB1")
            .field("priv_page", &self.priv_page())
            .finish()
    }
}
impl W {
    #[doc = "Bits 4:15 - Privilege access for higher sectors of Bank1"]
    #[inline(always)]
    pub fn priv_page(&mut self) -> PrivPageW<'_, FpcPrivattribb1Spec> {
        PrivPageW::new(self, 4)
    }
}
#[doc = "Flash Privilege Attribute Register B1\n\nYou can [`read`](crate::Reg::read) this register and get [`fpc_privattribb1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fpc_privattribb1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FpcPrivattribb1Spec;
impl crate::RegisterSpec for FpcPrivattribb1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fpc_privattribb1::R`](R) reader structure"]
impl crate::Readable for FpcPrivattribb1Spec {}
#[doc = "`write(|w| ..)` method takes [`fpc_privattribb1::W`](W) writer structure"]
impl crate::Writable for FpcPrivattribb1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FPC_PRIVATTRIBB1 to value 0xfff0"]
impl crate::Resettable for FpcPrivattribb1Spec {
    const RESET_VALUE: u32 = 0xfff0;
}
