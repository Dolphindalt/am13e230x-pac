#[doc = "Register `PPC_PRIVATTRIB_UNICOMM` reader"]
pub type R = crate::R<PpcPrivattribUnicommSpec>;
#[doc = "Register `PPC_PRIVATTRIB_UNICOMM` writer"]
pub type W = crate::W<PpcPrivattribUnicommSpec>;
#[doc = "Field `PRIV_S0U0` reader - Privilege access for UniComm S0U0"]
pub type PrivS0u0R = crate::BitReader;
#[doc = "Field `PRIV_S0U0` writer - Privilege access for UniComm S0U0"]
pub type PrivS0u0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRIV_S0U1` reader - Privilege access for UniComm S0U1"]
pub type PrivS0u1R = crate::BitReader;
#[doc = "Field `PRIV_S0U1` writer - Privilege access for UniComm S0U1"]
pub type PrivS0u1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRIV_S0U2` reader - Privilege access for UniComm S0U2"]
pub type PrivS0u2R = crate::BitReader;
#[doc = "Field `PRIV_S0U2` writer - Privilege access for UniComm S0U2"]
pub type PrivS0u2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRIV_S1U3` reader - Privilege access for UniComm S1U3"]
pub type PrivS1u3R = crate::BitReader;
#[doc = "Field `PRIV_S1U3` writer - Privilege access for UniComm S1U3"]
pub type PrivS1u3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRIV_S1U4` reader - Privilege access for UniComm S1U4"]
pub type PrivS1u4R = crate::BitReader;
#[doc = "Field `PRIV_S1U4` writer - Privilege access for UniComm S1U4"]
pub type PrivS1u4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRIV_S1U5` reader - Privilege access for UniComm S1U5"]
pub type PrivS1u5R = crate::BitReader;
#[doc = "Field `PRIV_S1U5` writer - Privilege access for UniComm S1U5"]
pub type PrivS1u5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRIV_S0_COMMON` reader - Privilege access for UniComm S0 common region"]
pub type PrivS0CommonR = crate::BitReader;
#[doc = "Field `PRIV_S0_COMMON` writer - Privilege access for UniComm S0 common region"]
pub type PrivS0CommonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRIV_S1_COMMON` reader - Privilege access for UniComm S1 common region"]
pub type PrivS1CommonR = crate::BitReader;
#[doc = "Field `PRIV_S1_COMMON` writer - Privilege access for UniComm S1 common region"]
pub type PrivS1CommonW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Privilege access for UniComm S0U0"]
    #[inline(always)]
    pub fn priv_s0u0(&self) -> PrivS0u0R {
        PrivS0u0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Privilege access for UniComm S0U1"]
    #[inline(always)]
    pub fn priv_s0u1(&self) -> PrivS0u1R {
        PrivS0u1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Privilege access for UniComm S0U2"]
    #[inline(always)]
    pub fn priv_s0u2(&self) -> PrivS0u2R {
        PrivS0u2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Privilege access for UniComm S1U3"]
    #[inline(always)]
    pub fn priv_s1u3(&self) -> PrivS1u3R {
        PrivS1u3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Privilege access for UniComm S1U4"]
    #[inline(always)]
    pub fn priv_s1u4(&self) -> PrivS1u4R {
        PrivS1u4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Privilege access for UniComm S1U5"]
    #[inline(always)]
    pub fn priv_s1u5(&self) -> PrivS1u5R {
        PrivS1u5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 24 - Privilege access for UniComm S0 common region"]
    #[inline(always)]
    pub fn priv_s0_common(&self) -> PrivS0CommonR {
        PrivS0CommonR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Privilege access for UniComm S1 common region"]
    #[inline(always)]
    pub fn priv_s1_common(&self) -> PrivS1CommonR {
        PrivS1CommonR::new(((self.bits >> 25) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PPC_PRIVATTRIB_UNICOMM")
            .field("priv_s0u0", &self.priv_s0u0())
            .field("priv_s0u1", &self.priv_s0u1())
            .field("priv_s0u2", &self.priv_s0u2())
            .field("priv_s1u3", &self.priv_s1u3())
            .field("priv_s1u4", &self.priv_s1u4())
            .field("priv_s1u5", &self.priv_s1u5())
            .field("priv_s0_common", &self.priv_s0_common())
            .field("priv_s1_common", &self.priv_s1_common())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Privilege access for UniComm S0U0"]
    #[inline(always)]
    pub fn priv_s0u0(&mut self) -> PrivS0u0W<'_, PpcPrivattribUnicommSpec> {
        PrivS0u0W::new(self, 0)
    }
    #[doc = "Bit 1 - Privilege access for UniComm S0U1"]
    #[inline(always)]
    pub fn priv_s0u1(&mut self) -> PrivS0u1W<'_, PpcPrivattribUnicommSpec> {
        PrivS0u1W::new(self, 1)
    }
    #[doc = "Bit 2 - Privilege access for UniComm S0U2"]
    #[inline(always)]
    pub fn priv_s0u2(&mut self) -> PrivS0u2W<'_, PpcPrivattribUnicommSpec> {
        PrivS0u2W::new(self, 2)
    }
    #[doc = "Bit 3 - Privilege access for UniComm S1U3"]
    #[inline(always)]
    pub fn priv_s1u3(&mut self) -> PrivS1u3W<'_, PpcPrivattribUnicommSpec> {
        PrivS1u3W::new(self, 3)
    }
    #[doc = "Bit 4 - Privilege access for UniComm S1U4"]
    #[inline(always)]
    pub fn priv_s1u4(&mut self) -> PrivS1u4W<'_, PpcPrivattribUnicommSpec> {
        PrivS1u4W::new(self, 4)
    }
    #[doc = "Bit 5 - Privilege access for UniComm S1U5"]
    #[inline(always)]
    pub fn priv_s1u5(&mut self) -> PrivS1u5W<'_, PpcPrivattribUnicommSpec> {
        PrivS1u5W::new(self, 5)
    }
    #[doc = "Bit 24 - Privilege access for UniComm S0 common region"]
    #[inline(always)]
    pub fn priv_s0_common(&mut self) -> PrivS0CommonW<'_, PpcPrivattribUnicommSpec> {
        PrivS0CommonW::new(self, 24)
    }
    #[doc = "Bit 25 - Privilege access for UniComm S1 common region"]
    #[inline(always)]
    pub fn priv_s1_common(&mut self) -> PrivS1CommonW<'_, PpcPrivattribUnicommSpec> {
        PrivS1CommonW::new(self, 25)
    }
}
#[doc = "Peripheral Privilege Access UniComm\n\nYou can [`read`](crate::Reg::read) this register and get [`ppc_privattrib_unicomm::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppc_privattrib_unicomm::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PpcPrivattribUnicommSpec;
impl crate::RegisterSpec for PpcPrivattribUnicommSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ppc_privattrib_unicomm::R`](R) reader structure"]
impl crate::Readable for PpcPrivattribUnicommSpec {}
#[doc = "`write(|w| ..)` method takes [`ppc_privattrib_unicomm::W`](W) writer structure"]
impl crate::Writable for PpcPrivattribUnicommSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PPC_PRIVATTRIB_UNICOMM to value 0"]
impl crate::Resettable for PpcPrivattribUnicommSpec {}
