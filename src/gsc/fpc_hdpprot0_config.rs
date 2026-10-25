#[doc = "Register `FPC_HDPPROT0_CONFIG` reader"]
pub type R = crate::R<FpcHdpprot0ConfigSpec>;
#[doc = "Register `FPC_HDPPROT0_CONFIG` writer"]
pub type W = crate::W<FpcHdpprot0ConfigSpec>;
#[doc = "Field `HDPEND` writer - HDPPROT0 end sector number"]
pub type HdpendW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `HDPSTART` writer - HDPPROT0 start sector number"]
pub type HdpstartW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FPC_HDPPROT0_CONFIG").finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - HDPPROT0 end sector number"]
    #[inline(always)]
    pub fn hdpend(&mut self) -> HdpendW<'_, FpcHdpprot0ConfigSpec> {
        HdpendW::new(self, 0)
    }
    #[doc = "Bits 16:23 - HDPPROT0 start sector number"]
    #[inline(always)]
    pub fn hdpstart(&mut self) -> HdpstartW<'_, FpcHdpprot0ConfigSpec> {
        HdpstartW::new(self, 16)
    }
}
#[doc = "Flash Hide Protection for Bank0 Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fpc_hdpprot0_config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fpc_hdpprot0_config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FpcHdpprot0ConfigSpec;
impl crate::RegisterSpec for FpcHdpprot0ConfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fpc_hdpprot0_config::R`](R) reader structure"]
impl crate::Readable for FpcHdpprot0ConfigSpec {}
#[doc = "`write(|w| ..)` method takes [`fpc_hdpprot0_config::W`](W) writer structure"]
impl crate::Writable for FpcHdpprot0ConfigSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FPC_HDPPROT0_CONFIG to value 0x00ff_0000"]
impl crate::Resettable for FpcHdpprot0ConfigSpec {
    const RESET_VALUE: u32 = 0x00ff_0000;
}
