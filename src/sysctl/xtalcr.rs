#[doc = "Register `XTALCR` reader"]
pub type R = crate::R<XtalcrSpec>;
#[doc = "Register `XTALCR` writer"]
pub type W = crate::W<XtalcrSpec>;
#[doc = "Field `OSCOFF` reader - This bit if 1, powers-down the XTAL oscillator macro and hence doesnt let X2 to be driven by the XTAL oscillator. If a crystal is connected to X1/X2, user needs to first clear this bit, wait for the oscillator to power up (using X1CNT) and then only switch the clock source to X1/X2"]
pub type OscoffR = crate::BitReader;
#[doc = "Field `OSCOFF` writer - This bit if 1, powers-down the XTAL oscillator macro and hence doesnt let X2 to be driven by the XTAL oscillator. If a crystal is connected to X1/X2, user needs to first clear this bit, wait for the oscillator to power up (using X1CNT) and then only switch the clock source to X1/X2"]
pub type OscoffW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SE` reader - XTAL Oscilator in Single-Ended"]
pub type SeR = crate::BitReader;
#[doc = "Field `SE` writer - XTAL Oscilator in Single-Ended"]
pub type SeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - This bit if 1, powers-down the XTAL oscillator macro and hence doesnt let X2 to be driven by the XTAL oscillator. If a crystal is connected to X1/X2, user needs to first clear this bit, wait for the oscillator to power up (using X1CNT) and then only switch the clock source to X1/X2"]
    #[inline(always)]
    pub fn oscoff(&self) -> OscoffR {
        OscoffR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - XTAL Oscilator in Single-Ended"]
    #[inline(always)]
    pub fn se(&self) -> SeR {
        SeR::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("XTALCR")
            .field("se", &self.se())
            .field("oscoff", &self.oscoff())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - This bit if 1, powers-down the XTAL oscillator macro and hence doesnt let X2 to be driven by the XTAL oscillator. If a crystal is connected to X1/X2, user needs to first clear this bit, wait for the oscillator to power up (using X1CNT) and then only switch the clock source to X1/X2"]
    #[inline(always)]
    pub fn oscoff(&mut self) -> OscoffW<'_, XtalcrSpec> {
        OscoffW::new(self, 0)
    }
    #[doc = "Bit 1 - XTAL Oscilator in Single-Ended"]
    #[inline(always)]
    pub fn se(&mut self) -> SeW<'_, XtalcrSpec> {
        SeW::new(self, 1)
    }
}
#[doc = "XTAL Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`xtalcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xtalcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct XtalcrSpec;
impl crate::RegisterSpec for XtalcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`xtalcr::R`](R) reader structure"]
impl crate::Readable for XtalcrSpec {}
#[doc = "`write(|w| ..)` method takes [`xtalcr::W`](W) writer structure"]
impl crate::Writable for XtalcrSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets XTALCR to value 0x01"]
impl crate::Resettable for XtalcrSpec {
    const RESET_VALUE: u32 = 0x01;
}
