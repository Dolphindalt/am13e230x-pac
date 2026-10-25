#[doc = "Register `FPC_WEPROTB0` reader"]
pub type R = crate::R<FpcWeprotb0Spec>;
#[doc = "Register `FPC_WEPROTB0` writer"]
pub type W = crate::W<FpcWeprotb0Spec>;
#[doc = "Field `WEPROT_PAGE` reader - Erase/program protection for higher sectors of Flash Bank0"]
pub type WeprotPageR = crate::FieldReader<u16>;
#[doc = "Field `WEPROT_PAGE` writer - Erase/program protection for higher sectors of Flash Bank0"]
pub type WeprotPageW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 4:15 - Erase/program protection for higher sectors of Flash Bank0"]
    #[inline(always)]
    pub fn weprot_page(&self) -> WeprotPageR {
        WeprotPageR::new(((self.bits >> 4) & 0x0fff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FPC_WEPROTB0")
            .field("weprot_page", &self.weprot_page())
            .finish()
    }
}
impl W {
    #[doc = "Bits 4:15 - Erase/program protection for higher sectors of Flash Bank0"]
    #[inline(always)]
    pub fn weprot_page(&mut self) -> WeprotPageW<'_, FpcWeprotb0Spec> {
        WeprotPageW::new(self, 4)
    }
}
#[doc = "Flash Write Protect Attribute Register B0\n\nYou can [`read`](crate::Reg::read) this register and get [`fpc_weprotb0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fpc_weprotb0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FpcWeprotb0Spec;
impl crate::RegisterSpec for FpcWeprotb0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fpc_weprotb0::R`](R) reader structure"]
impl crate::Readable for FpcWeprotb0Spec {}
#[doc = "`write(|w| ..)` method takes [`fpc_weprotb0::W`](W) writer structure"]
impl crate::Writable for FpcWeprotb0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FPC_WEPROTB0 to value 0"]
impl crate::Resettable for FpcWeprotb0Spec {}
