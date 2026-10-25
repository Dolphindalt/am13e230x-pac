#[doc = "Register `SPC_ATTRIBVIOLP_CONFIG` reader"]
pub type R = crate::R<SpcAttribviolpConfigSpec>;
#[doc = "Register `SPC_ATTRIBVIOLP_CONFIG` writer"]
pub type W = crate::W<SpcAttribviolpConfigSpec>;
#[doc = "Field `PRIVVIOL` reader - Privilege access to non-privilege SRAM"]
pub type PrivviolR = crate::BitReader;
#[doc = "Field `PRIVVIOL` writer - Privilege access to non-privilege SRAM"]
pub type PrivviolW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Privilege access to non-privilege SRAM"]
    #[inline(always)]
    pub fn privviol(&self) -> PrivviolR {
        PrivviolR::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPC_ATTRIBVIOLP_CONFIG")
            .field("privviol", &self.privviol())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Privilege access to non-privilege SRAM"]
    #[inline(always)]
    pub fn privviol(&mut self) -> PrivviolW<'_, SpcAttribviolpConfigSpec> {
        PrivviolW::new(self, 0)
    }
}
#[doc = "SRAM Privilege Access Attribute Violation Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`spc_attribviolp_config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spc_attribviolp_config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpcAttribviolpConfigSpec;
impl crate::RegisterSpec for SpcAttribviolpConfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spc_attribviolp_config::R`](R) reader structure"]
impl crate::Readable for SpcAttribviolpConfigSpec {}
#[doc = "`write(|w| ..)` method takes [`spc_attribviolp_config::W`](W) writer structure"]
impl crate::Writable for SpcAttribviolpConfigSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPC_ATTRIBVIOLP_CONFIG to value 0"]
impl crate::Resettable for SpcAttribviolpConfigSpec {}
