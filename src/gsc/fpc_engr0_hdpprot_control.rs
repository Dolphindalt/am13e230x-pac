#[doc = "Register `FPC_ENGR0_HDPPROT_CONTROL` reader"]
pub type R = crate::R<FpcEngr0HdpprotControlSpec>;
#[doc = "Register `FPC_ENGR0_HDPPROT_CONTROL` writer"]
pub type W = crate::W<FpcEngr0HdpprotControlSpec>;
#[doc = "Field `HDPROTACCDIS` reader - HDP enable for ENGR region of Bank0"]
pub type HdprotaccdisR = crate::BitReader;
#[doc = "Field `HDPROTACCDIS` writer - HDP enable for ENGR region of Bank0"]
pub type HdprotaccdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HDPROTACCDIS1` reader - Redundant"]
pub type Hdprotaccdis1R = crate::BitReader;
#[doc = "Field `HDPROTACCDIS1` writer - Redundant"]
pub type Hdprotaccdis1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - HDP enable for ENGR region of Bank0"]
    #[inline(always)]
    pub fn hdprotaccdis(&self) -> HdprotaccdisR {
        HdprotaccdisR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Redundant"]
    #[inline(always)]
    pub fn hdprotaccdis1(&self) -> Hdprotaccdis1R {
        Hdprotaccdis1R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FPC_ENGR0_HDPPROT_CONTROL")
            .field("hdprotaccdis", &self.hdprotaccdis())
            .field("hdprotaccdis1", &self.hdprotaccdis1())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - HDP enable for ENGR region of Bank0"]
    #[inline(always)]
    pub fn hdprotaccdis(&mut self) -> HdprotaccdisW<'_, FpcEngr0HdpprotControlSpec> {
        HdprotaccdisW::new(self, 0)
    }
    #[doc = "Bit 1 - Redundant"]
    #[inline(always)]
    pub fn hdprotaccdis1(&mut self) -> Hdprotaccdis1W<'_, FpcEngr0HdpprotControlSpec> {
        Hdprotaccdis1W::new(self, 1)
    }
}
#[doc = "Flash ENGR Bank0 Hide Protection Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fpc_engr0_hdpprot_control::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fpc_engr0_hdpprot_control::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FpcEngr0HdpprotControlSpec;
impl crate::RegisterSpec for FpcEngr0HdpprotControlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fpc_engr0_hdpprot_control::R`](R) reader structure"]
impl crate::Readable for FpcEngr0HdpprotControlSpec {}
#[doc = "`write(|w| ..)` method takes [`fpc_engr0_hdpprot_control::W`](W) writer structure"]
impl crate::Writable for FpcEngr0HdpprotControlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FPC_ENGR0_HDPPROT_CONTROL to value 0"]
impl crate::Resettable for FpcEngr0HdpprotControlSpec {}
