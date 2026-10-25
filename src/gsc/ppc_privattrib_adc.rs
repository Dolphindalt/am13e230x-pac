#[doc = "Register `PPC_PRIVATTRIB_ADC` reader"]
pub type R = crate::R<PpcPrivattribAdcSpec>;
#[doc = "Register `PPC_PRIVATTRIB_ADC` writer"]
pub type W = crate::W<PpcPrivattribAdcSpec>;
#[doc = "Field `PRIV_ADC0` reader - Privilege access for ADC0"]
pub type PrivAdc0R = crate::BitReader;
#[doc = "Field `PRIV_ADC0` writer - Privilege access for ADC0"]
pub type PrivAdc0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRIV_ADC1` reader - Privilege access for ADC1"]
pub type PrivAdc1R = crate::BitReader;
#[doc = "Field `PRIV_ADC1` writer - Privilege access for ADC1"]
pub type PrivAdc1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRIV_ADC2` reader - Privilege access for ADC2"]
pub type PrivAdc2R = crate::BitReader;
#[doc = "Field `PRIV_ADC2` writer - Privilege access for ADC2"]
pub type PrivAdc2W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Privilege access for ADC0"]
    #[inline(always)]
    pub fn priv_adc0(&self) -> PrivAdc0R {
        PrivAdc0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Privilege access for ADC1"]
    #[inline(always)]
    pub fn priv_adc1(&self) -> PrivAdc1R {
        PrivAdc1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Privilege access for ADC2"]
    #[inline(always)]
    pub fn priv_adc2(&self) -> PrivAdc2R {
        PrivAdc2R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PPC_PRIVATTRIB_ADC")
            .field("priv_adc0", &self.priv_adc0())
            .field("priv_adc1", &self.priv_adc1())
            .field("priv_adc2", &self.priv_adc2())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Privilege access for ADC0"]
    #[inline(always)]
    pub fn priv_adc0(&mut self) -> PrivAdc0W<'_, PpcPrivattribAdcSpec> {
        PrivAdc0W::new(self, 0)
    }
    #[doc = "Bit 1 - Privilege access for ADC1"]
    #[inline(always)]
    pub fn priv_adc1(&mut self) -> PrivAdc1W<'_, PpcPrivattribAdcSpec> {
        PrivAdc1W::new(self, 1)
    }
    #[doc = "Bit 2 - Privilege access for ADC2"]
    #[inline(always)]
    pub fn priv_adc2(&mut self) -> PrivAdc2W<'_, PpcPrivattribAdcSpec> {
        PrivAdc2W::new(self, 2)
    }
}
#[doc = "Peripheral Privilege Access ADC\n\nYou can [`read`](crate::Reg::read) this register and get [`ppc_privattrib_adc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppc_privattrib_adc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PpcPrivattribAdcSpec;
impl crate::RegisterSpec for PpcPrivattribAdcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ppc_privattrib_adc::R`](R) reader structure"]
impl crate::Readable for PpcPrivattribAdcSpec {}
#[doc = "`write(|w| ..)` method takes [`ppc_privattrib_adc::W`](W) writer structure"]
impl crate::Writable for PpcPrivattribAdcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PPC_PRIVATTRIB_ADC to value 0"]
impl crate::Resettable for PpcPrivattribAdcSpec {}
