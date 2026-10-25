#[doc = "Register `FPC_HDPEN_CONTROL` reader"]
pub type R = crate::R<FpcHdpenControlSpec>;
#[doc = "Register `FPC_HDPEN_CONTROL` writer"]
pub type W = crate::W<FpcHdpenControlSpec>;
#[doc = "Field `HDPEN` reader - HDP mode enable"]
pub type HdpenR = crate::BitReader;
#[doc = "Field `HDPEN` writer - HDP mode enable"]
pub type HdpenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - HDP mode enable"]
    #[inline(always)]
    pub fn hdpen(&self) -> HdpenR {
        HdpenR::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FPC_HDPEN_CONTROL")
            .field("hdpen", &self.hdpen())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - HDP mode enable"]
    #[inline(always)]
    pub fn hdpen(&mut self) -> HdpenW<'_, FpcHdpenControlSpec> {
        HdpenW::new(self, 0)
    }
}
#[doc = "Flash Hide Protection Enable Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fpc_hdpen_control::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fpc_hdpen_control::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FpcHdpenControlSpec;
impl crate::RegisterSpec for FpcHdpenControlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fpc_hdpen_control::R`](R) reader structure"]
impl crate::Readable for FpcHdpenControlSpec {}
#[doc = "`write(|w| ..)` method takes [`fpc_hdpen_control::W`](W) writer structure"]
impl crate::Writable for FpcHdpenControlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FPC_HDPEN_CONTROL to value 0"]
impl crate::Resettable for FpcHdpenControlSpec {}
