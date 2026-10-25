#[doc = "Register `FPC_WEPROTTRIM1` reader"]
pub type R = crate::R<FpcWeprottrim1Spec>;
#[doc = "Register `FPC_WEPROTTRIM1` writer"]
pub type W = crate::W<FpcWeprottrim1Spec>;
#[doc = "Field `WEPROT_PAGE` reader - Erase/program protection for Flash TRIM region of Bank1"]
pub type WeprotPageR = crate::FieldReader;
#[doc = "Field `WEPROT_PAGE` writer - Erase/program protection for Flash TRIM region of Bank1"]
pub type WeprotPageW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - Erase/program protection for Flash TRIM region of Bank1"]
    #[inline(always)]
    pub fn weprot_page(&self) -> WeprotPageR {
        WeprotPageR::new((self.bits & 3) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FPC_WEPROTTRIM1")
            .field("weprot_page", &self.weprot_page())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - Erase/program protection for Flash TRIM region of Bank1"]
    #[inline(always)]
    pub fn weprot_page(&mut self) -> WeprotPageW<'_, FpcWeprottrim1Spec> {
        WeprotPageW::new(self, 0)
    }
}
#[doc = "Flash Trim Write Protect Attribute Register B1\n\nYou can [`read`](crate::Reg::read) this register and get [`fpc_weprottrim1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fpc_weprottrim1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FpcWeprottrim1Spec;
impl crate::RegisterSpec for FpcWeprottrim1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fpc_weprottrim1::R`](R) reader structure"]
impl crate::Readable for FpcWeprottrim1Spec {}
#[doc = "`write(|w| ..)` method takes [`fpc_weprottrim1::W`](W) writer structure"]
impl crate::Writable for FpcWeprottrim1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FPC_WEPROTTRIM1 to value 0"]
impl crate::Resettable for FpcWeprottrim1Spec {}
