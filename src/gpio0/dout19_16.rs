#[doc = "Register `DOUT19_16` writer"]
pub type W = crate::W<Dout19_16Spec>;
#[doc = "Field `DIO16` writer - This bit sets the value of the pin configured as DIO16 when the output is enabled through DOE31_0 register."]
pub type Dio16W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIO17` writer - This bit sets the value of the pin configured as DIO17 when the output is enabled through DOE31_0 register."]
pub type Dio17W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIO18` writer - This bit sets the value of the pin configured as DIO18 when the output is enabled through DOE31_0 register."]
pub type Dio18W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIO19` writer - This bit sets the value of the pin configured as DIO19 when the output is enabled through DOE31_0 register."]
pub type Dio19W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<Dout19_16Spec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - This bit sets the value of the pin configured as DIO16 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dio16(&mut self) -> Dio16W<'_, Dout19_16Spec> {
        Dio16W::new(self, 0)
    }
    #[doc = "Bit 8 - This bit sets the value of the pin configured as DIO17 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dio17(&mut self) -> Dio17W<'_, Dout19_16Spec> {
        Dio17W::new(self, 8)
    }
    #[doc = "Bit 16 - This bit sets the value of the pin configured as DIO18 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dio18(&mut self) -> Dio18W<'_, Dout19_16Spec> {
        Dio18W::new(self, 16)
    }
    #[doc = "Bit 24 - This bit sets the value of the pin configured as DIO19 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dio19(&mut self) -> Dio19W<'_, Dout19_16Spec> {
        Dio19W::new(self, 24)
    }
}
#[doc = "Data output 19 to 16\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dout19_16::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dout19_16Spec;
impl crate::RegisterSpec for Dout19_16Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`dout19_16::W`](W) writer structure"]
impl crate::Writable for Dout19_16Spec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets DOUT19_16 to value 0"]
impl crate::Resettable for Dout19_16Spec {}
