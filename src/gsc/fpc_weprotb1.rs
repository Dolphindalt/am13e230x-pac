#[doc = "Register `FPC_WEPROTB1` reader"]
pub type R = crate::R<FpcWeprotb1Spec>;
#[doc = "Register `FPC_WEPROTB1` writer"]
pub type W = crate::W<FpcWeprotb1Spec>;
#[doc = "Field `WEPROT_PAGE` reader - Erase/program protection for higher sectors of Flash Bank1"]
pub type WeprotPageR = crate::FieldReader<u16>;
#[doc = "Field `WEPROT_PAGE` writer - Erase/program protection for higher sectors of Flash Bank1"]
pub type WeprotPageW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 4:15 - Erase/program protection for higher sectors of Flash Bank1"]
    #[inline(always)]
    pub fn weprot_page(&self) -> WeprotPageR {
        WeprotPageR::new(((self.bits >> 4) & 0x0fff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FPC_WEPROTB1")
            .field("weprot_page", &self.weprot_page())
            .finish()
    }
}
impl W {
    #[doc = "Bits 4:15 - Erase/program protection for higher sectors of Flash Bank1"]
    #[inline(always)]
    pub fn weprot_page(&mut self) -> WeprotPageW<'_, FpcWeprotb1Spec> {
        WeprotPageW::new(self, 4)
    }
}
#[doc = "Flash Write Protect Attribute Register B1\n\nYou can [`read`](crate::Reg::read) this register and get [`fpc_weprotb1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fpc_weprotb1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FpcWeprotb1Spec;
impl crate::RegisterSpec for FpcWeprotb1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fpc_weprotb1::R`](R) reader structure"]
impl crate::Readable for FpcWeprotb1Spec {}
#[doc = "`write(|w| ..)` method takes [`fpc_weprotb1::W`](W) writer structure"]
impl crate::Writable for FpcWeprotb1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FPC_WEPROTB1 to value 0"]
impl crate::Resettable for FpcWeprotb1Spec {}
