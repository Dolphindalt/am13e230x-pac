#[doc = "Register `FPC_ATTRIBVIOLP_CONFIG` reader"]
pub type R = crate::R<FpcAttribviolpConfigSpec>;
#[doc = "Register `FPC_ATTRIBVIOLP_CONFIG` writer"]
pub type W = crate::W<FpcAttribviolpConfigSpec>;
#[doc = "Field `PRIVVIOL` reader - Privilege access to non-privilege Flash"]
pub type PrivviolR = crate::BitReader;
#[doc = "Field `PRIVVIOL` writer - Privilege access to non-privilege Flash"]
pub type PrivviolW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Privilege access to non-privilege Flash"]
    #[inline(always)]
    pub fn privviol(&self) -> PrivviolR {
        PrivviolR::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FPC_ATTRIBVIOLP_CONFIG")
            .field("privviol", &self.privviol())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Privilege access to non-privilege Flash"]
    #[inline(always)]
    pub fn privviol(&mut self) -> PrivviolW<'_, FpcAttribviolpConfigSpec> {
        PrivviolW::new(self, 0)
    }
}
#[doc = "Flash Privilege Access Attribute Violation Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fpc_attribviolp_config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fpc_attribviolp_config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FpcAttribviolpConfigSpec;
impl crate::RegisterSpec for FpcAttribviolpConfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fpc_attribviolp_config::R`](R) reader structure"]
impl crate::Readable for FpcAttribviolpConfigSpec {}
#[doc = "`write(|w| ..)` method takes [`fpc_attribviolp_config::W`](W) writer structure"]
impl crate::Writable for FpcAttribviolpConfigSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FPC_ATTRIBVIOLP_CONFIG to value 0"]
impl crate::Resettable for FpcAttribviolpConfigSpec {}
