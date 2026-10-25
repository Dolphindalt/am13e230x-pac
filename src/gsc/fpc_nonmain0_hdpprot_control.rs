#[doc = "Register `FPC_NONMAIN0_HDPPROT_CONTROL` reader"]
pub type R = crate::R<FpcNonmain0HdpprotControlSpec>;
#[doc = "Register `FPC_NONMAIN0_HDPPROT_CONTROL` writer"]
pub type W = crate::W<FpcNonmain0HdpprotControlSpec>;
#[doc = "Field `HDPROTACCDIS` reader - HDP enable for NONMAIN region of Bank0"]
pub type HdprotaccdisR = crate::FieldReader;
#[doc = "Field `HDPROTACCDIS` writer - HDP enable for NONMAIN region of Bank0"]
pub type HdprotaccdisW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - HDP enable for NONMAIN region of Bank0"]
    #[inline(always)]
    pub fn hdprotaccdis(&self) -> HdprotaccdisR {
        HdprotaccdisR::new((self.bits & 3) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FPC_NONMAIN0_HDPPROT_CONTROL")
            .field("hdprotaccdis", &self.hdprotaccdis())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - HDP enable for NONMAIN region of Bank0"]
    #[inline(always)]
    pub fn hdprotaccdis(&mut self) -> HdprotaccdisW<'_, FpcNonmain0HdpprotControlSpec> {
        HdprotaccdisW::new(self, 0)
    }
}
#[doc = "Flash NONMAIN Bank0 Hide Protection Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fpc_nonmain0_hdpprot_control::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fpc_nonmain0_hdpprot_control::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FpcNonmain0HdpprotControlSpec;
impl crate::RegisterSpec for FpcNonmain0HdpprotControlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fpc_nonmain0_hdpprot_control::R`](R) reader structure"]
impl crate::Readable for FpcNonmain0HdpprotControlSpec {}
#[doc = "`write(|w| ..)` method takes [`fpc_nonmain0_hdpprot_control::W`](W) writer structure"]
impl crate::Writable for FpcNonmain0HdpprotControlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FPC_NONMAIN0_HDPPROT_CONTROL to value 0"]
impl crate::Resettable for FpcNonmain0HdpprotControlSpec {}
