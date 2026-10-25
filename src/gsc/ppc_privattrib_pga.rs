#[doc = "Register `PPC_PRIVATTRIB_PGA` reader"]
pub type R = crate::R<PpcPrivattribPgaSpec>;
#[doc = "Register `PPC_PRIVATTRIB_PGA` writer"]
pub type W = crate::W<PpcPrivattribPgaSpec>;
#[doc = "Field `PRIV_PGA0` reader - Privilege access for PGA0"]
pub type PrivPga0R = crate::BitReader;
#[doc = "Field `PRIV_PGA0` writer - Privilege access for PGA0"]
pub type PrivPga0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRIV_PGA1` reader - Privilege access for PGA1"]
pub type PrivPga1R = crate::BitReader;
#[doc = "Field `PRIV_PGA1` writer - Privilege access for PGA1"]
pub type PrivPga1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRIV_PGA2` reader - Privilege access for PGA2"]
pub type PrivPga2R = crate::BitReader;
#[doc = "Field `PRIV_PGA2` writer - Privilege access for PGA2"]
pub type PrivPga2W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Privilege access for PGA0"]
    #[inline(always)]
    pub fn priv_pga0(&self) -> PrivPga0R {
        PrivPga0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Privilege access for PGA1"]
    #[inline(always)]
    pub fn priv_pga1(&self) -> PrivPga1R {
        PrivPga1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Privilege access for PGA2"]
    #[inline(always)]
    pub fn priv_pga2(&self) -> PrivPga2R {
        PrivPga2R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PPC_PRIVATTRIB_PGA")
            .field("priv_pga0", &self.priv_pga0())
            .field("priv_pga1", &self.priv_pga1())
            .field("priv_pga2", &self.priv_pga2())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Privilege access for PGA0"]
    #[inline(always)]
    pub fn priv_pga0(&mut self) -> PrivPga0W<'_, PpcPrivattribPgaSpec> {
        PrivPga0W::new(self, 0)
    }
    #[doc = "Bit 1 - Privilege access for PGA1"]
    #[inline(always)]
    pub fn priv_pga1(&mut self) -> PrivPga1W<'_, PpcPrivattribPgaSpec> {
        PrivPga1W::new(self, 1)
    }
    #[doc = "Bit 2 - Privilege access for PGA2"]
    #[inline(always)]
    pub fn priv_pga2(&mut self) -> PrivPga2W<'_, PpcPrivattribPgaSpec> {
        PrivPga2W::new(self, 2)
    }
}
#[doc = "Peripheral Privilege Access PGA\n\nYou can [`read`](crate::Reg::read) this register and get [`ppc_privattrib_pga::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppc_privattrib_pga::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PpcPrivattribPgaSpec;
impl crate::RegisterSpec for PpcPrivattribPgaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ppc_privattrib_pga::R`](R) reader structure"]
impl crate::Readable for PpcPrivattribPgaSpec {}
#[doc = "`write(|w| ..)` method takes [`ppc_privattrib_pga::W`](W) writer structure"]
impl crate::Writable for PpcPrivattribPgaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PPC_PRIVATTRIB_PGA to value 0"]
impl crate::Resettable for PpcPrivattribPgaSpec {}
