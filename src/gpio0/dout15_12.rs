#[doc = "Register `DOUT15_12` writer"]
pub type W = crate::W<Dout15_12Spec>;
#[doc = "Field `DIO12` writer - This bit sets the value of the pin configured as DIO12 when the output is enabled through DOE31_0 register."]
pub type Dio12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIO13` writer - This bit sets the value of the pin configured as DIO13 when the output is enabled through DOE31_0 register."]
pub type Dio13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIO14` writer - This bit sets the value of the pin configured as DIO14 when the output is enabled through DOE31_0 register."]
pub type Dio14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIO15` writer - This bit sets the value of the pin configured as DIO15 when the output is enabled through DOE31_0 register."]
pub type Dio15W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<Dout15_12Spec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - This bit sets the value of the pin configured as DIO12 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dio12(&mut self) -> Dio12W<'_, Dout15_12Spec> {
        Dio12W::new(self, 0)
    }
    #[doc = "Bit 8 - This bit sets the value of the pin configured as DIO13 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dio13(&mut self) -> Dio13W<'_, Dout15_12Spec> {
        Dio13W::new(self, 8)
    }
    #[doc = "Bit 16 - This bit sets the value of the pin configured as DIO14 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dio14(&mut self) -> Dio14W<'_, Dout15_12Spec> {
        Dio14W::new(self, 16)
    }
    #[doc = "Bit 24 - This bit sets the value of the pin configured as DIO15 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dio15(&mut self) -> Dio15W<'_, Dout15_12Spec> {
        Dio15W::new(self, 24)
    }
}
#[doc = "Data output 15 to 12\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dout15_12::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dout15_12Spec;
impl crate::RegisterSpec for Dout15_12Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`dout15_12::W`](W) writer structure"]
impl crate::Writable for Dout15_12Spec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets DOUT15_12 to value 0"]
impl crate::Resettable for Dout15_12Spec {}
