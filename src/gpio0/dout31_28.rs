#[doc = "Register `DOUT31_28` writer"]
pub type W = crate::W<Dout31_28Spec>;
#[doc = "Field `DIO28` writer - This bit sets the value of the pin configured as DIO28 when the output is enabled through DOE31_0 register."]
pub type Dio28W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIO29` writer - This bit sets the value of the pin configured as DIO29 when the output is enabled through DOE31_0 register."]
pub type Dio29W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIO30` writer - This bit sets the value of the pin configured as DIO30 when the output is enabled through DOE31_0 register."]
pub type Dio30W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIO31` writer - This bit sets the value of the pin configured as DIO31 when the output is enabled through DOE31_0 register."]
pub type Dio31W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<Dout31_28Spec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - This bit sets the value of the pin configured as DIO28 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dio28(&mut self) -> Dio28W<'_, Dout31_28Spec> {
        Dio28W::new(self, 0)
    }
    #[doc = "Bit 8 - This bit sets the value of the pin configured as DIO29 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dio29(&mut self) -> Dio29W<'_, Dout31_28Spec> {
        Dio29W::new(self, 8)
    }
    #[doc = "Bit 16 - This bit sets the value of the pin configured as DIO30 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dio30(&mut self) -> Dio30W<'_, Dout31_28Spec> {
        Dio30W::new(self, 16)
    }
    #[doc = "Bit 24 - This bit sets the value of the pin configured as DIO31 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dio31(&mut self) -> Dio31W<'_, Dout31_28Spec> {
        Dio31W::new(self, 24)
    }
}
#[doc = "Data output 31 to 28\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dout31_28::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dout31_28Spec;
impl crate::RegisterSpec for Dout31_28Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`dout31_28::W`](W) writer structure"]
impl crate::Writable for Dout31_28Spec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets DOUT31_28 to value 0"]
impl crate::Resettable for Dout31_28Spec {}
