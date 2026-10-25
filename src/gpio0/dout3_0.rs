#[doc = "Register `DOUT3_0` writer"]
pub type W = crate::W<Dout3_0Spec>;
#[doc = "Field `DIO0` writer - This bit sets the value of the pin configured as DIO0 when the output is enabled through DOE31_0 register."]
pub type Dio0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIO1` writer - This bit sets the value of the pin configured as DIO1 when the output is enabled through DOE31_0 register."]
pub type Dio1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIO2` writer - This bit sets the value of the pin configured as DIO2 when the output is enabled through DOE31_0 register."]
pub type Dio2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIO3` writer - This bit sets the value of the pin configured as DIO3 when the output is enabled through DOE31_0 register."]
pub type Dio3W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<Dout3_0Spec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - This bit sets the value of the pin configured as DIO0 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dio0(&mut self) -> Dio0W<'_, Dout3_0Spec> {
        Dio0W::new(self, 0)
    }
    #[doc = "Bit 8 - This bit sets the value of the pin configured as DIO1 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dio1(&mut self) -> Dio1W<'_, Dout3_0Spec> {
        Dio1W::new(self, 8)
    }
    #[doc = "Bit 16 - This bit sets the value of the pin configured as DIO2 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dio2(&mut self) -> Dio2W<'_, Dout3_0Spec> {
        Dio2W::new(self, 16)
    }
    #[doc = "Bit 24 - This bit sets the value of the pin configured as DIO3 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dio3(&mut self) -> Dio3W<'_, Dout3_0Spec> {
        Dio3W::new(self, 24)
    }
}
#[doc = "Data output 3 to 0\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dout3_0::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dout3_0Spec;
impl crate::RegisterSpec for Dout3_0Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`dout3_0::W`](W) writer structure"]
impl crate::Writable for Dout3_0Spec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets DOUT3_0 to value 0"]
impl crate::Resettable for Dout3_0Spec {}
