#[doc = "Register `FPC_WEPROTNONMAIN0` reader"]
pub type R = crate::R<FpcWeprotnonmain0Spec>;
#[doc = "Register `FPC_WEPROTNONMAIN0` writer"]
pub type W = crate::W<FpcWeprotnonmain0Spec>;
#[doc = "Field `WEPROT_PAGE` reader - Erase/program protection for Flash NONMAIN region of Bank0"]
pub type WeprotPageR = crate::FieldReader;
#[doc = "Field `WEPROT_PAGE` writer - Erase/program protection for Flash NONMAIN region of Bank0"]
pub type WeprotPageW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - Erase/program protection for Flash NONMAIN region of Bank0"]
    #[inline(always)]
    pub fn weprot_page(&self) -> WeprotPageR {
        WeprotPageR::new((self.bits & 3) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FPC_WEPROTNONMAIN0")
            .field("weprot_page", &self.weprot_page())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - Erase/program protection for Flash NONMAIN region of Bank0"]
    #[inline(always)]
    pub fn weprot_page(&mut self) -> WeprotPageW<'_, FpcWeprotnonmain0Spec> {
        WeprotPageW::new(self, 0)
    }
}
#[doc = "Flash Non-Main Write Protect Attribute Register B0\n\nYou can [`read`](crate::Reg::read) this register and get [`fpc_weprotnonmain0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fpc_weprotnonmain0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FpcWeprotnonmain0Spec;
impl crate::RegisterSpec for FpcWeprotnonmain0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fpc_weprotnonmain0::R`](R) reader structure"]
impl crate::Readable for FpcWeprotnonmain0Spec {}
#[doc = "`write(|w| ..)` method takes [`fpc_weprotnonmain0::W`](W) writer structure"]
impl crate::Writable for FpcWeprotnonmain0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FPC_WEPROTNONMAIN0 to value 0"]
impl crate::Resettable for FpcWeprotnonmain0Spec {}
