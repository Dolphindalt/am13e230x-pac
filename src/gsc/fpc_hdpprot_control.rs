#[doc = "Register `FPC_HDPPROT_CONTROL` reader"]
pub type R = crate::R<FpcHdpprotControlSpec>;
#[doc = "Register `FPC_HDPPROT_CONTROL` writer"]
pub type W = crate::W<FpcHdpprotControlSpec>;
#[doc = "Field `HDPPROT0ACCDIS` reader - HDP region-0 enable"]
pub type Hdpprot0accdisR = crate::BitReader;
#[doc = "Field `HDPPROT0ACCDIS` writer - HDP region-0 enable"]
pub type Hdpprot0accdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HDPPROT1ACCDIS` reader - HDP region-1 enable"]
pub type Hdpprot1accdisR = crate::BitReader;
#[doc = "Field `HDPPROT1ACCDIS` writer - HDP region-1 enable"]
pub type Hdpprot1accdisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - HDP region-0 enable"]
    #[inline(always)]
    pub fn hdpprot0accdis(&self) -> Hdpprot0accdisR {
        Hdpprot0accdisR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - HDP region-1 enable"]
    #[inline(always)]
    pub fn hdpprot1accdis(&self) -> Hdpprot1accdisR {
        Hdpprot1accdisR::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FPC_HDPPROT_CONTROL")
            .field("hdpprot0accdis", &self.hdpprot0accdis())
            .field("hdpprot1accdis", &self.hdpprot1accdis())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - HDP region-0 enable"]
    #[inline(always)]
    pub fn hdpprot0accdis(&mut self) -> Hdpprot0accdisW<'_, FpcHdpprotControlSpec> {
        Hdpprot0accdisW::new(self, 0)
    }
    #[doc = "Bit 1 - HDP region-1 enable"]
    #[inline(always)]
    pub fn hdpprot1accdis(&mut self) -> Hdpprot1accdisW<'_, FpcHdpprotControlSpec> {
        Hdpprot1accdisW::new(self, 1)
    }
}
#[doc = "Flash Hide Protection Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fpc_hdpprot_control::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fpc_hdpprot_control::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FpcHdpprotControlSpec;
impl crate::RegisterSpec for FpcHdpprotControlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fpc_hdpprot_control::R`](R) reader structure"]
impl crate::Readable for FpcHdpprotControlSpec {}
#[doc = "`write(|w| ..)` method takes [`fpc_hdpprot_control::W`](W) writer structure"]
impl crate::Writable for FpcHdpprotControlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FPC_HDPPROT_CONTROL to value 0"]
impl crate::Resettable for FpcHdpprotControlSpec {}
