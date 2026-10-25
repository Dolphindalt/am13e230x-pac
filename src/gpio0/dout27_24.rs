#[doc = "Register `DOUT27_24` writer"]
pub type W = crate::W<Dout27_24Spec>;
#[doc = "Field `DIO24` writer - This bit sets the value of the pin configured as DIO24 when the output is enabled through DOE31_0 register."]
pub type Dio24W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIO25` writer - This bit sets the value of the pin configured as DIO25 when the output is enabled through DOE31_0 register."]
pub type Dio25W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIO26` writer - This bit sets the value of the pin configured as DIO26 when the output is enabled through DOE31_0 register."]
pub type Dio26W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIO27` writer - This bit sets the value of the pin configured as DIO27 when the output is enabled through DOE31_0 register."]
pub type Dio27W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<Dout27_24Spec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - This bit sets the value of the pin configured as DIO24 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dio24(&mut self) -> Dio24W<'_, Dout27_24Spec> {
        Dio24W::new(self, 0)
    }
    #[doc = "Bit 8 - This bit sets the value of the pin configured as DIO25 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dio25(&mut self) -> Dio25W<'_, Dout27_24Spec> {
        Dio25W::new(self, 8)
    }
    #[doc = "Bit 16 - This bit sets the value of the pin configured as DIO26 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dio26(&mut self) -> Dio26W<'_, Dout27_24Spec> {
        Dio26W::new(self, 16)
    }
    #[doc = "Bit 24 - This bit sets the value of the pin configured as DIO27 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dio27(&mut self) -> Dio27W<'_, Dout27_24Spec> {
        Dio27W::new(self, 24)
    }
}
#[doc = "Data output 27 to 24\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dout27_24::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dout27_24Spec;
impl crate::RegisterSpec for Dout27_24Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`dout27_24::W`](W) writer structure"]
impl crate::Writable for Dout27_24Spec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets DOUT27_24 to value 0"]
impl crate::Resettable for Dout27_24Spec {}
