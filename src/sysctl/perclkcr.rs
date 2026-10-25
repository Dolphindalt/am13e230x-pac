#[doc = "Register `PERCLKCR` reader"]
pub type R = crate::R<PerclkcrSpec>;
#[doc = "Register `PERCLKCR` writer"]
pub type W = crate::W<PerclkcrSpec>;
#[doc = "Field `TBCLKSYNC` reader - PWM Time Base Clock sync: When set PWM time bases of all the PWM modules belonging to the same CPU-Subsystem (as partitioned using their CPUSEL bits) start counting"]
pub type TbclksyncR = crate::BitReader;
#[doc = "Field `TBCLKSYNC` writer - PWM Time Base Clock sync: When set PWM time bases of all the PWM modules belonging to the same CPU-Subsystem (as partitioned using their CPUSEL bits) start counting"]
pub type TbclksyncW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - PWM Time Base Clock sync: When set PWM time bases of all the PWM modules belonging to the same CPU-Subsystem (as partitioned using their CPUSEL bits) start counting"]
    #[inline(always)]
    pub fn tbclksync(&self) -> TbclksyncR {
        TbclksyncR::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PERCLKCR")
            .field("tbclksync", &self.tbclksync())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - PWM Time Base Clock sync: When set PWM time bases of all the PWM modules belonging to the same CPU-Subsystem (as partitioned using their CPUSEL bits) start counting"]
    #[inline(always)]
    pub fn tbclksync(&mut self) -> TbclksyncW<'_, PerclkcrSpec> {
        TbclksyncW::new(self, 0)
    }
}
#[doc = "PWM Time Base Clock sync\n\nYou can [`read`](crate::Reg::read) this register and get [`perclkcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`perclkcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PerclkcrSpec;
impl crate::RegisterSpec for PerclkcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`perclkcr::R`](R) reader structure"]
impl crate::Readable for PerclkcrSpec {}
#[doc = "`write(|w| ..)` method takes [`perclkcr::W`](W) writer structure"]
impl crate::Writable for PerclkcrSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets PERCLKCR to value 0"]
impl crate::Resettable for PerclkcrSpec {}
