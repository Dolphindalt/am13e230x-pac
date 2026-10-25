#[doc = "Register `BSLPIN_INVOKE` reader"]
pub type R = crate::R<BslpinInvokeSpec>;
#[doc = "Field `BSL_PAD` reader - BSL Invocation Pin Number"]
pub type BslPadR = crate::FieldReader;
#[doc = "Field `GPIO_LEVEL` reader - GPIO Level Configuration for BSL Invocation"]
pub type GpioLevelR = crate::BitReader;
#[doc = "Field `GPIO_PIN_SEL` reader - GPIO Pin Number in GPIO Module"]
pub type GpioPinSelR = crate::FieldReader;
#[doc = "Field `GPIO_REG_SEL` reader - GPIO Module Selection"]
pub type GpioRegSelR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:6 - BSL Invocation Pin Number"]
    #[inline(always)]
    pub fn bsl_pad(&self) -> BslPadR {
        BslPadR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - GPIO Level Configuration for BSL Invocation"]
    #[inline(always)]
    pub fn gpio_level(&self) -> GpioLevelR {
        GpioLevelR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:12 - GPIO Pin Number in GPIO Module"]
    #[inline(always)]
    pub fn gpio_pin_sel(&self) -> GpioPinSelR {
        GpioPinSelR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 13:14 - GPIO Module Selection"]
    #[inline(always)]
    pub fn gpio_reg_sel(&self) -> GpioRegSelR {
        GpioRegSelR::new(((self.bits >> 13) & 3) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BSLPIN_INVOKE")
            .field("bsl_pad", &self.bsl_pad())
            .field("gpio_level", &self.gpio_level())
            .field("gpio_pin_sel", &self.gpio_pin_sel())
            .field("gpio_reg_sel", &self.gpio_reg_sel())
            .finish()
    }
}
#[doc = "BSL Pin Invocation Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`bslpin_invoke::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BslpinInvokeSpec;
impl crate::RegisterSpec for BslpinInvokeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bslpin_invoke::R`](R) reader structure"]
impl crate::Readable for BslpinInvokeSpec {}
#[doc = "`reset()` method sets BSLPIN_INVOKE to value 0"]
impl crate::Resettable for BslpinInvokeSpec {}
