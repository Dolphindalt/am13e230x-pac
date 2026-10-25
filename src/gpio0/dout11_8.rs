#[doc = "Register `DOUT11_8` writer"]
pub type W = crate::W<Dout11_8Spec>;
#[doc = "Field `DIO8` writer - This bit sets the value of the pin configured as DIO8 when the output is enabled through DOE31_0 register."]
pub type Dio8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIO9` writer - This bit sets the value of the pin configured as DIO9 when the output is enabled through DOE31_0 register."]
pub type Dio9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIO10` writer - This bit sets the value of the pin configured as DIO10 when the output is enabled through DOE31_0 register."]
pub type Dio10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIO11` writer - This bit sets the value of the pin configured as DIO11 when the output is enabled through DOE31_0 register."]
pub type Dio11W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<Dout11_8Spec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - This bit sets the value of the pin configured as DIO8 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dio8(&mut self) -> Dio8W<'_, Dout11_8Spec> {
        Dio8W::new(self, 0)
    }
    #[doc = "Bit 8 - This bit sets the value of the pin configured as DIO9 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dio9(&mut self) -> Dio9W<'_, Dout11_8Spec> {
        Dio9W::new(self, 8)
    }
    #[doc = "Bit 16 - This bit sets the value of the pin configured as DIO10 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dio10(&mut self) -> Dio10W<'_, Dout11_8Spec> {
        Dio10W::new(self, 16)
    }
    #[doc = "Bit 24 - This bit sets the value of the pin configured as DIO11 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dio11(&mut self) -> Dio11W<'_, Dout11_8Spec> {
        Dio11W::new(self, 24)
    }
}
#[doc = "Data output 11 to 8\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dout11_8::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dout11_8Spec;
impl crate::RegisterSpec for Dout11_8Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`dout11_8::W`](W) writer structure"]
impl crate::Writable for Dout11_8Spec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets DOUT11_8 to value 0"]
impl crate::Resettable for Dout11_8Spec {}
