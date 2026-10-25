#[doc = "Register `FPC_NONMAIN_PRIVATTRIB1` reader"]
pub type R = crate::R<FpcNonmainPrivattrib1Spec>;
#[doc = "Register `FPC_NONMAIN_PRIVATTRIB1` writer"]
pub type W = crate::W<FpcNonmainPrivattrib1Spec>;
#[doc = "Field `PRIV_PAGE` reader - Privilege access for Nonmain region of Bank1"]
pub type PrivPageR = crate::FieldReader;
#[doc = "Field `PRIV_PAGE` writer - Privilege access for Nonmain region of Bank1"]
pub type PrivPageW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - Privilege access for Nonmain region of Bank1"]
    #[inline(always)]
    pub fn priv_page(&self) -> PrivPageR {
        PrivPageR::new((self.bits & 3) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FPC_NONMAIN_PRIVATTRIB1")
            .field("priv_page", &self.priv_page())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - Privilege access for Nonmain region of Bank1"]
    #[inline(always)]
    pub fn priv_page(&mut self) -> PrivPageW<'_, FpcNonmainPrivattrib1Spec> {
        PrivPageW::new(self, 0)
    }
}
#[doc = "Flash Non-Main Privilege Attribute Register B1\n\nYou can [`read`](crate::Reg::read) this register and get [`fpc_nonmain_privattrib1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fpc_nonmain_privattrib1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FpcNonmainPrivattrib1Spec;
impl crate::RegisterSpec for FpcNonmainPrivattrib1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fpc_nonmain_privattrib1::R`](R) reader structure"]
impl crate::Readable for FpcNonmainPrivattrib1Spec {}
#[doc = "`write(|w| ..)` method takes [`fpc_nonmain_privattrib1::W`](W) writer structure"]
impl crate::Writable for FpcNonmainPrivattrib1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FPC_NONMAIN_PRIVATTRIB1 to value 0x03"]
impl crate::Resettable for FpcNonmainPrivattrib1Spec {
    const RESET_VALUE: u32 = 0x03;
}
