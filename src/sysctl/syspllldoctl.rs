#[doc = "Register `SYSPLLLDOCTL` reader"]
pub type R = crate::R<SyspllldoctlSpec>;
#[doc = "Register `SYSPLLLDOCTL` writer"]
pub type W = crate::W<SyspllldoctlSpec>;
#[doc = "Field `LDOCTLLOWV` reader - LDO Configurability"]
pub type LdoctllowvR = crate::FieldReader<u16>;
#[doc = "Field `LDOCTLLOWV` writer - LDO Configurability"]
pub type LdoctllowvW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16, crate::Safe>;
impl R {
    #[doc = "Bits 0:15 - LDO Configurability"]
    #[inline(always)]
    pub fn ldoctllowv(&self) -> LdoctllowvR {
        LdoctllowvR::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SYSPLLLDOCTL")
            .field("ldoctllowv", &self.ldoctllowv())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - LDO Configurability"]
    #[inline(always)]
    pub fn ldoctllowv(&mut self) -> LdoctllowvW<'_, SyspllldoctlSpec> {
        LdoctllowvW::new(self, 0)
    }
}
#[doc = "SYSPLL LDO CTL (load from FACTORY region)\n\nYou can [`read`](crate::Reg::read) this register and get [`syspllldoctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syspllldoctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SyspllldoctlSpec;
impl crate::RegisterSpec for SyspllldoctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syspllldoctl::R`](R) reader structure"]
impl crate::Readable for SyspllldoctlSpec {}
#[doc = "`write(|w| ..)` method takes [`syspllldoctl::W`](W) writer structure"]
impl crate::Writable for SyspllldoctlSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets SYSPLLLDOCTL to value 0"]
impl crate::Resettable for SyspllldoctlSpec {}
