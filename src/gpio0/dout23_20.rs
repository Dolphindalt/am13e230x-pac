#[doc = "Register `DOUT23_20` writer"]
pub type W = crate::W<Dout23_20Spec>;
#[doc = "Field `DIO20` writer - This bit sets the value of the pin configured as DIO20 when the output is enabled through DOE31_0 register."]
pub type Dio20W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIO21` writer - This bit sets the value of the pin configured as DIO21 when the output is enabled through DOE31_0 register."]
pub type Dio21W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIO22` writer - This bit sets the value of the pin configured as DIO22 when the output is enabled through DOE31_0 register."]
pub type Dio22W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIO23` writer - This bit sets the value of the pin configured as DIO23 when the output is enabled through DOE31_0 register."]
pub type Dio23W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<Dout23_20Spec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - This bit sets the value of the pin configured as DIO20 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dio20(&mut self) -> Dio20W<'_, Dout23_20Spec> {
        Dio20W::new(self, 0)
    }
    #[doc = "Bit 8 - This bit sets the value of the pin configured as DIO21 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dio21(&mut self) -> Dio21W<'_, Dout23_20Spec> {
        Dio21W::new(self, 8)
    }
    #[doc = "Bit 16 - This bit sets the value of the pin configured as DIO22 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dio22(&mut self) -> Dio22W<'_, Dout23_20Spec> {
        Dio22W::new(self, 16)
    }
    #[doc = "Bit 24 - This bit sets the value of the pin configured as DIO23 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dio23(&mut self) -> Dio23W<'_, Dout23_20Spec> {
        Dio23W::new(self, 24)
    }
}
#[doc = "Data output 23 to 20\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dout23_20::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dout23_20Spec;
impl crate::RegisterSpec for Dout23_20Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`dout23_20::W`](W) writer structure"]
impl crate::Writable for Dout23_20Spec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets DOUT23_20 to value 0"]
impl crate::Resettable for Dout23_20Spec {}
