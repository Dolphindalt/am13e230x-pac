#[doc = "Register `ANAREFCTL` reader"]
pub type R = crate::R<AnarefctlSpec>;
#[doc = "Register `ANAREFCTL` writer"]
pub type W = crate::W<AnarefctlSpec>;
#[doc = "Field `ANAREFSEL` reader - Analog reference mode select. This bit selects whether the VREFHI pin uses internal reference mode (the device drives a voltage onto the VREFHI pin) or external reference mode (the system is expected to drive a voltage into the VREFHI pin). 0 - Internal reference mode 1 - External reference mode"]
pub type AnarefselR = crate::BitReader;
#[doc = "Field `ANAREFSEL` writer - Analog reference mode select. This bit selects whether the VREFHI pin uses internal reference mode (the device drives a voltage onto the VREFHI pin) or external reference mode (the system is expected to drive a voltage into the VREFHI pin). 0 - Internal reference mode 1 - External reference mode"]
pub type AnarefselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ANAREF2P5SEL` reader - Analog reference 2.5V source select. In internal reference mode, this bit selects which voltage the internal reference buffer drives onto the VREFHI pin. The buffer can drive either 1.65V onto the pin, resulting in a reference range of 0 to 3.3V, or the buffer can drive 2.5V onto the pin, resulting in a reference range of 0 to 2.5V. If switching between these two modes, the user must allow adequate time for the external capacitor to charge to the new voltage before using the ADC or buffered DAC. 0 - Internal 1.65V reference mode (3.3V reference range) 1 - Internal 2.5V reference mode (2.5V reference range)"]
pub type Anaref2p5selR = crate::BitReader;
#[doc = "Field `ANAREF2P5SEL` writer - Analog reference 2.5V source select. In internal reference mode, this bit selects which voltage the internal reference buffer drives onto the VREFHI pin. The buffer can drive either 1.65V onto the pin, resulting in a reference range of 0 to 3.3V, or the buffer can drive 2.5V onto the pin, resulting in a reference range of 0 to 2.5V. If switching between these two modes, the user must allow adequate time for the external capacitor to charge to the new voltage before using the ADC or buffered DAC. 0 - Internal 1.65V reference mode (3.3V reference range) 1 - Internal 2.5V reference mode (2.5V reference range)"]
pub type Anaref2p5selW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Analog reference mode select. This bit selects whether the VREFHI pin uses internal reference mode (the device drives a voltage onto the VREFHI pin) or external reference mode (the system is expected to drive a voltage into the VREFHI pin). 0 - Internal reference mode 1 - External reference mode"]
    #[inline(always)]
    pub fn anarefsel(&self) -> AnarefselR {
        AnarefselR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - Analog reference 2.5V source select. In internal reference mode, this bit selects which voltage the internal reference buffer drives onto the VREFHI pin. The buffer can drive either 1.65V onto the pin, resulting in a reference range of 0 to 3.3V, or the buffer can drive 2.5V onto the pin, resulting in a reference range of 0 to 2.5V. If switching between these two modes, the user must allow adequate time for the external capacitor to charge to the new voltage before using the ADC or buffered DAC. 0 - Internal 1.65V reference mode (3.3V reference range) 1 - Internal 2.5V reference mode (2.5V reference range)"]
    #[inline(always)]
    pub fn anaref2p5sel(&self) -> Anaref2p5selR {
        Anaref2p5selR::new(((self.bits >> 8) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ANAREFCTL")
            .field("anaref2p5sel", &self.anaref2p5sel())
            .field("anarefsel", &self.anarefsel())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Analog reference mode select. This bit selects whether the VREFHI pin uses internal reference mode (the device drives a voltage onto the VREFHI pin) or external reference mode (the system is expected to drive a voltage into the VREFHI pin). 0 - Internal reference mode 1 - External reference mode"]
    #[inline(always)]
    pub fn anarefsel(&mut self) -> AnarefselW<'_, AnarefctlSpec> {
        AnarefselW::new(self, 0)
    }
    #[doc = "Bit 8 - Analog reference 2.5V source select. In internal reference mode, this bit selects which voltage the internal reference buffer drives onto the VREFHI pin. The buffer can drive either 1.65V onto the pin, resulting in a reference range of 0 to 3.3V, or the buffer can drive 2.5V onto the pin, resulting in a reference range of 0 to 2.5V. If switching between these two modes, the user must allow adequate time for the external capacitor to charge to the new voltage before using the ADC or buffered DAC. 0 - Internal 1.65V reference mode (3.3V reference range) 1 - Internal 2.5V reference mode (2.5V reference range)"]
    #[inline(always)]
    pub fn anaref2p5sel(&mut self) -> Anaref2p5selW<'_, AnarefctlSpec> {
        Anaref2p5selW::new(self, 8)
    }
}
#[doc = "Analog Reference Select\n\nYou can [`read`](crate::Reg::read) this register and get [`anarefctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`anarefctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AnarefctlSpec;
impl crate::RegisterSpec for AnarefctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`anarefctl::R`](R) reader structure"]
impl crate::Readable for AnarefctlSpec {}
#[doc = "`write(|w| ..)` method takes [`anarefctl::W`](W) writer structure"]
impl crate::Writable for AnarefctlSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets ANAREFCTL to value 0x01"]
impl crate::Resettable for AnarefctlSpec {
    const RESET_VALUE: u32 = 0x01;
}
