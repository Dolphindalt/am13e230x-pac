#[doc = "Register `FPC_HDPPROT1_CONFIG` reader"]
pub type R = crate::R<FpcHdpprot1ConfigSpec>;
#[doc = "Register `FPC_HDPPROT1_CONFIG` writer"]
pub type W = crate::W<FpcHdpprot1ConfigSpec>;
#[doc = "Field `HDPEND` writer - HDPPROT1 end sector number"]
pub type HdpendW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `HDPSTART` writer - HDPPROT1 start sector number"]
pub type HdpstartW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FPC_HDPPROT1_CONFIG").finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - HDPPROT1 end sector number"]
    #[inline(always)]
    pub fn hdpend(&mut self) -> HdpendW<'_, FpcHdpprot1ConfigSpec> {
        HdpendW::new(self, 0)
    }
    #[doc = "Bits 16:23 - HDPPROT1 start sector number"]
    #[inline(always)]
    pub fn hdpstart(&mut self) -> HdpstartW<'_, FpcHdpprot1ConfigSpec> {
        HdpstartW::new(self, 16)
    }
}
#[doc = "Flash Hide Protection for Bank1 Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fpc_hdpprot1_config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fpc_hdpprot1_config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FpcHdpprot1ConfigSpec;
impl crate::RegisterSpec for FpcHdpprot1ConfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fpc_hdpprot1_config::R`](R) reader structure"]
impl crate::Readable for FpcHdpprot1ConfigSpec {}
#[doc = "`write(|w| ..)` method takes [`fpc_hdpprot1_config::W`](W) writer structure"]
impl crate::Writable for FpcHdpprot1ConfigSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FPC_HDPPROT1_CONFIG to value 0x00ff_0000"]
impl crate::Resettable for FpcHdpprot1ConfigSpec {
    const RESET_VALUE: u32 = 0x00ff_0000;
}
