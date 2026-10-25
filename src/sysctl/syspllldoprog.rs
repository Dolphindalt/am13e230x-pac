#[doc = "Register `SYSPLLLDOPROG` reader"]
pub type R = crate::R<SyspllldoprogSpec>;
#[doc = "Register `SYSPLLLDOPROG` writer"]
pub type W = crate::W<SyspllldoprogSpec>;
#[doc = "Field `LDOVOUTPROGLOWV` reader - HPLL LDO Vout Prog"]
pub type LdovoutproglowvR = crate::FieldReader;
#[doc = "Field `LDOVOUTPROGLOWV` writer - HPLL LDO Vout Prog"]
pub type LdovoutproglowvW<'a, REG> = crate::FieldWriter<'a, REG, 3, u8, crate::Safe>;
impl R {
    #[doc = "Bits 0:2 - HPLL LDO Vout Prog"]
    #[inline(always)]
    pub fn ldovoutproglowv(&self) -> LdovoutproglowvR {
        LdovoutproglowvR::new((self.bits & 7) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SYSPLLLDOPROG")
            .field("ldovoutproglowv", &self.ldovoutproglowv())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - HPLL LDO Vout Prog"]
    #[inline(always)]
    pub fn ldovoutproglowv(&mut self) -> LdovoutproglowvW<'_, SyspllldoprogSpec> {
        LdovoutproglowvW::new(self, 0)
    }
}
#[doc = "SYSPLL LDO VOUT PROG (load from FACTORY region)\n\nYou can [`read`](crate::Reg::read) this register and get [`syspllldoprog::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syspllldoprog::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SyspllldoprogSpec;
impl crate::RegisterSpec for SyspllldoprogSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syspllldoprog::R`](R) reader structure"]
impl crate::Readable for SyspllldoprogSpec {}
#[doc = "`write(|w| ..)` method takes [`syspllldoprog::W`](W) writer structure"]
impl crate::Writable for SyspllldoprogSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets SYSPLLLDOPROG to value 0x04"]
impl crate::Resettable for SyspllldoprogSpec {
    const RESET_VALUE: u32 = 0x04;
}
