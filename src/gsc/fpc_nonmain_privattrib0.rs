#[doc = "Register `FPC_NONMAIN_PRIVATTRIB0` reader"]
pub type R = crate::R<FpcNonmainPrivattrib0Spec>;
#[doc = "Register `FPC_NONMAIN_PRIVATTRIB0` writer"]
pub type W = crate::W<FpcNonmainPrivattrib0Spec>;
#[doc = "Field `PRIV_PAGE` reader - Privilege access for Nonmain region of Bank0"]
pub type PrivPageR = crate::FieldReader;
#[doc = "Field `PRIV_PAGE` writer - Privilege access for Nonmain region of Bank0"]
pub type PrivPageW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - Privilege access for Nonmain region of Bank0"]
    #[inline(always)]
    pub fn priv_page(&self) -> PrivPageR {
        PrivPageR::new((self.bits & 3) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FPC_NONMAIN_PRIVATTRIB0")
            .field("priv_page", &self.priv_page())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - Privilege access for Nonmain region of Bank0"]
    #[inline(always)]
    pub fn priv_page(&mut self) -> PrivPageW<'_, FpcNonmainPrivattrib0Spec> {
        PrivPageW::new(self, 0)
    }
}
#[doc = "Flash Non-Main Privilege Attribute Register B0\n\nYou can [`read`](crate::Reg::read) this register and get [`fpc_nonmain_privattrib0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fpc_nonmain_privattrib0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FpcNonmainPrivattrib0Spec;
impl crate::RegisterSpec for FpcNonmainPrivattrib0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fpc_nonmain_privattrib0::R`](R) reader structure"]
impl crate::Readable for FpcNonmainPrivattrib0Spec {}
#[doc = "`write(|w| ..)` method takes [`fpc_nonmain_privattrib0::W`](W) writer structure"]
impl crate::Writable for FpcNonmainPrivattrib0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FPC_NONMAIN_PRIVATTRIB0 to value 0x03"]
impl crate::Resettable for FpcNonmainPrivattrib0Spec {
    const RESET_VALUE: u32 = 0x03;
}
