#[doc = "Register `DOUT7_4` writer"]
pub type W = crate::W<Dout7_4Spec>;
#[doc = "Field `DIO4` writer - This bit sets the value of the pin configured as DIO4 when the output is enabled through DOE31_0 register."]
pub type Dio4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIO5` writer - This bit sets the value of the pin configured as DIO5 when the output is enabled through DOE31_0 register."]
pub type Dio5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIO6` writer - This bit sets the value of the pin configured as DIO6 when the output is enabled through DOE31_0 register."]
pub type Dio6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIO7` writer - This bit sets the value of the pin configured as DIO7 when the output is enabled through DOE31_0 register."]
pub type Dio7W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<Dout7_4Spec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - This bit sets the value of the pin configured as DIO4 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dio4(&mut self) -> Dio4W<'_, Dout7_4Spec> {
        Dio4W::new(self, 0)
    }
    #[doc = "Bit 8 - This bit sets the value of the pin configured as DIO5 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dio5(&mut self) -> Dio5W<'_, Dout7_4Spec> {
        Dio5W::new(self, 8)
    }
    #[doc = "Bit 16 - This bit sets the value of the pin configured as DIO6 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dio6(&mut self) -> Dio6W<'_, Dout7_4Spec> {
        Dio6W::new(self, 16)
    }
    #[doc = "Bit 24 - This bit sets the value of the pin configured as DIO7 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dio7(&mut self) -> Dio7W<'_, Dout7_4Spec> {
        Dio7W::new(self, 24)
    }
}
#[doc = "Data output 7 to 4\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dout7_4::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dout7_4Spec;
impl crate::RegisterSpec for Dout7_4Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`dout7_4::W`](W) writer structure"]
impl crate::Writable for Dout7_4Spec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets DOUT7_4 to value 0"]
impl crate::Resettable for Dout7_4Spec {}
