#[doc = "Register `PPC_PRIVATTRIB_MCAN` reader"]
pub type R = crate::R<PpcPrivattribMcanSpec>;
#[doc = "Register `PPC_PRIVATTRIB_MCAN` writer"]
pub type W = crate::W<PpcPrivattribMcanSpec>;
#[doc = "Field `PRIV_MCAN` reader - Privilege access for MCAN"]
pub type PrivMcanR = crate::BitReader;
#[doc = "Field `PRIV_MCAN` writer - Privilege access for MCAN"]
pub type PrivMcanW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Privilege access for MCAN"]
    #[inline(always)]
    pub fn priv_mcan(&self) -> PrivMcanR {
        PrivMcanR::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PPC_PRIVATTRIB_MCAN")
            .field("priv_mcan", &self.priv_mcan())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Privilege access for MCAN"]
    #[inline(always)]
    pub fn priv_mcan(&mut self) -> PrivMcanW<'_, PpcPrivattribMcanSpec> {
        PrivMcanW::new(self, 0)
    }
}
#[doc = "Peripheral Privilege Access CAN\n\nYou can [`read`](crate::Reg::read) this register and get [`ppc_privattrib_mcan::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppc_privattrib_mcan::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PpcPrivattribMcanSpec;
impl crate::RegisterSpec for PpcPrivattribMcanSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ppc_privattrib_mcan::R`](R) reader structure"]
impl crate::Readable for PpcPrivattribMcanSpec {}
#[doc = "`write(|w| ..)` method takes [`ppc_privattrib_mcan::W`](W) writer structure"]
impl crate::Writable for PpcPrivattribMcanSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PPC_PRIVATTRIB_MCAN to value 0"]
impl crate::Resettable for PpcPrivattribMcanSpec {}
