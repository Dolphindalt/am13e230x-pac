#[doc = "Register `PPC_PRIVATTRIB_GPIO` reader"]
pub type R = crate::R<PpcPrivattribGpioSpec>;
#[doc = "Register `PPC_PRIVATTRIB_GPIO` writer"]
pub type W = crate::W<PpcPrivattribGpioSpec>;
#[doc = "Field `PRIV_GPIO0` reader - Privilege access for GPIO0"]
pub type PrivGpio0R = crate::BitReader;
#[doc = "Field `PRIV_GPIO0` writer - Privilege access for GPIO0"]
pub type PrivGpio0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRIV_GPIO1` reader - Privilege access for GPIO1"]
pub type PrivGpio1R = crate::BitReader;
#[doc = "Field `PRIV_GPIO1` writer - Privilege access for GPIO1"]
pub type PrivGpio1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRIV_GPIO2` reader - Privilege access for GPIO2"]
pub type PrivGpio2R = crate::BitReader;
#[doc = "Field `PRIV_GPIO2` writer - Privilege access for GPIO2"]
pub type PrivGpio2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRIV_GPIO3` reader - Privilege access for GPIO3"]
pub type PrivGpio3R = crate::BitReader;
#[doc = "Field `PRIV_GPIO3` writer - Privilege access for GPIO3"]
pub type PrivGpio3W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Privilege access for GPIO0"]
    #[inline(always)]
    pub fn priv_gpio0(&self) -> PrivGpio0R {
        PrivGpio0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Privilege access for GPIO1"]
    #[inline(always)]
    pub fn priv_gpio1(&self) -> PrivGpio1R {
        PrivGpio1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Privilege access for GPIO2"]
    #[inline(always)]
    pub fn priv_gpio2(&self) -> PrivGpio2R {
        PrivGpio2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Privilege access for GPIO3"]
    #[inline(always)]
    pub fn priv_gpio3(&self) -> PrivGpio3R {
        PrivGpio3R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PPC_PRIVATTRIB_GPIO")
            .field("priv_gpio0", &self.priv_gpio0())
            .field("priv_gpio1", &self.priv_gpio1())
            .field("priv_gpio2", &self.priv_gpio2())
            .field("priv_gpio3", &self.priv_gpio3())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Privilege access for GPIO0"]
    #[inline(always)]
    pub fn priv_gpio0(&mut self) -> PrivGpio0W<'_, PpcPrivattribGpioSpec> {
        PrivGpio0W::new(self, 0)
    }
    #[doc = "Bit 1 - Privilege access for GPIO1"]
    #[inline(always)]
    pub fn priv_gpio1(&mut self) -> PrivGpio1W<'_, PpcPrivattribGpioSpec> {
        PrivGpio1W::new(self, 1)
    }
    #[doc = "Bit 2 - Privilege access for GPIO2"]
    #[inline(always)]
    pub fn priv_gpio2(&mut self) -> PrivGpio2W<'_, PpcPrivattribGpioSpec> {
        PrivGpio2W::new(self, 2)
    }
    #[doc = "Bit 3 - Privilege access for GPIO3"]
    #[inline(always)]
    pub fn priv_gpio3(&mut self) -> PrivGpio3W<'_, PpcPrivattribGpioSpec> {
        PrivGpio3W::new(self, 3)
    }
}
#[doc = "Peripheral Privilege Access GPIO\n\nYou can [`read`](crate::Reg::read) this register and get [`ppc_privattrib_gpio::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppc_privattrib_gpio::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PpcPrivattribGpioSpec;
impl crate::RegisterSpec for PpcPrivattribGpioSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ppc_privattrib_gpio::R`](R) reader structure"]
impl crate::Readable for PpcPrivattribGpioSpec {}
#[doc = "`write(|w| ..)` method takes [`ppc_privattrib_gpio::W`](W) writer structure"]
impl crate::Writable for PpcPrivattribGpioSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PPC_PRIVATTRIB_GPIO to value 0"]
impl crate::Resettable for PpcPrivattribGpioSpec {}
