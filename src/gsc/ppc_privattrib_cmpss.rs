#[doc = "Register `PPC_PRIVATTRIB_CMPSS` reader"]
pub type R = crate::R<PpcPrivattribCmpssSpec>;
#[doc = "Register `PPC_PRIVATTRIB_CMPSS` writer"]
pub type W = crate::W<PpcPrivattribCmpssSpec>;
#[doc = "Field `PRIV_CMPSS0` reader - Privilege access for CMPSS0"]
pub type PrivCmpss0R = crate::BitReader;
#[doc = "Field `PRIV_CMPSS0` writer - Privilege access for CMPSS0"]
pub type PrivCmpss0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRIV_CMPSS1` reader - Privilege access for CMPSS1"]
pub type PrivCmpss1R = crate::BitReader;
#[doc = "Field `PRIV_CMPSS1` writer - Privilege access for CMPSS1"]
pub type PrivCmpss1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRIV_CMPSS2` reader - Privilege access for CMPSS2"]
pub type PrivCmpss2R = crate::BitReader;
#[doc = "Field `PRIV_CMPSS2` writer - Privilege access for CMPSS2"]
pub type PrivCmpss2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRIV_CMPSS3` reader - Privilege access for CMPSS3"]
pub type PrivCmpss3R = crate::BitReader;
#[doc = "Field `PRIV_CMPSS3` writer - Privilege access for CMPSS3"]
pub type PrivCmpss3W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Privilege access for CMPSS0"]
    #[inline(always)]
    pub fn priv_cmpss0(&self) -> PrivCmpss0R {
        PrivCmpss0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Privilege access for CMPSS1"]
    #[inline(always)]
    pub fn priv_cmpss1(&self) -> PrivCmpss1R {
        PrivCmpss1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Privilege access for CMPSS2"]
    #[inline(always)]
    pub fn priv_cmpss2(&self) -> PrivCmpss2R {
        PrivCmpss2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Privilege access for CMPSS3"]
    #[inline(always)]
    pub fn priv_cmpss3(&self) -> PrivCmpss3R {
        PrivCmpss3R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PPC_PRIVATTRIB_CMPSS")
            .field("priv_cmpss0", &self.priv_cmpss0())
            .field("priv_cmpss1", &self.priv_cmpss1())
            .field("priv_cmpss2", &self.priv_cmpss2())
            .field("priv_cmpss3", &self.priv_cmpss3())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Privilege access for CMPSS0"]
    #[inline(always)]
    pub fn priv_cmpss0(&mut self) -> PrivCmpss0W<'_, PpcPrivattribCmpssSpec> {
        PrivCmpss0W::new(self, 0)
    }
    #[doc = "Bit 1 - Privilege access for CMPSS1"]
    #[inline(always)]
    pub fn priv_cmpss1(&mut self) -> PrivCmpss1W<'_, PpcPrivattribCmpssSpec> {
        PrivCmpss1W::new(self, 1)
    }
    #[doc = "Bit 2 - Privilege access for CMPSS2"]
    #[inline(always)]
    pub fn priv_cmpss2(&mut self) -> PrivCmpss2W<'_, PpcPrivattribCmpssSpec> {
        PrivCmpss2W::new(self, 2)
    }
    #[doc = "Bit 3 - Privilege access for CMPSS3"]
    #[inline(always)]
    pub fn priv_cmpss3(&mut self) -> PrivCmpss3W<'_, PpcPrivattribCmpssSpec> {
        PrivCmpss3W::new(self, 3)
    }
}
#[doc = "Peripheral Privilege Access CMPSS\n\nYou can [`read`](crate::Reg::read) this register and get [`ppc_privattrib_cmpss::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppc_privattrib_cmpss::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PpcPrivattribCmpssSpec;
impl crate::RegisterSpec for PpcPrivattribCmpssSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ppc_privattrib_cmpss::R`](R) reader structure"]
impl crate::Readable for PpcPrivattribCmpssSpec {}
#[doc = "`write(|w| ..)` method takes [`ppc_privattrib_cmpss::W`](W) writer structure"]
impl crate::Writable for PpcPrivattribCmpssSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PPC_PRIVATTRIB_CMPSS to value 0"]
impl crate::Resettable for PpcPrivattribCmpssSpec {}
