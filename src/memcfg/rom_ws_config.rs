#[doc = "Register `ROM_WS_CONFIG` reader"]
pub type R = crate::R<RomWsConfigSpec>;
#[doc = "Register `ROM_WS_CONFIG` writer"]
pub type W = crate::W<RomWsConfigSpec>;
#[doc = "Field `ROM_WS_ENABLE` reader - ROM wait state 1 enable"]
pub type RomWsEnableR = crate::BitReader;
#[doc = "Field `ROM_WS_ENABLE` writer - ROM wait state 1 enable"]
pub type RomWsEnableW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - ROM wait state 1 enable"]
    #[inline(always)]
    pub fn rom_ws_enable(&self) -> RomWsEnableR {
        RomWsEnableR::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ROM_WS_CONFIG")
            .field("rom_ws_enable", &self.rom_ws_enable())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - ROM wait state 1 enable"]
    #[inline(always)]
    pub fn rom_ws_enable(&mut self) -> RomWsEnableW<'_, RomWsConfigSpec> {
        RomWsEnableW::new(self, 0)
    }
}
#[doc = "ROM wait state configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`rom_ws_config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rom_ws_config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RomWsConfigSpec;
impl crate::RegisterSpec for RomWsConfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rom_ws_config::R`](R) reader structure"]
impl crate::Readable for RomWsConfigSpec {}
#[doc = "`write(|w| ..)` method takes [`rom_ws_config::W`](W) writer structure"]
impl crate::Writable for RomWsConfigSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets ROM_WS_CONFIG to value 0"]
impl crate::Resettable for RomWsConfigSpec {}
