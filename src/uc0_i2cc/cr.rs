#[doc = "Register `CR` reader"]
pub type R = crate::R<CrSpec>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CrSpec>;
#[doc = "Field `ENABLE` reader - Enable module. After this bit has been set, it should not be set again unless it has been cleared by writing a 0 or by a reset, otherwise transfer failures may occur."]
pub type EnableR = crate::BitReader;
#[doc = "Field `ENABLE` writer - Enable module. After this bit has been set, it should not be set again unless it has been cleared by writing a 0 or by a reset, otherwise transfer failures may occur."]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "MultiController mode. In MultiController mode the SCL high time counts once the SCL line has been detected high. If this is not enabled the high time counts as soon as the SCL line has been set high by the I2C controller.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mctl {
    #[doc = "0: Disable MultiController mode."]
    Disable = 0,
    #[doc = "1: Enable MultiController mode."]
    Enable = 1,
}
impl From<Mctl> for bool {
    #[inline(always)]
    fn from(variant: Mctl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MCTL` reader - MultiController mode. In MultiController mode the SCL high time counts once the SCL line has been detected high. If this is not enabled the high time counts as soon as the SCL line has been set high by the I2C controller."]
pub type MctlR = crate::BitReader<Mctl>;
impl MctlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mctl {
        match self.bits {
            false => Mctl::Disable,
            true => Mctl::Enable,
        }
    }
    #[doc = "Disable MultiController mode."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Mctl::Disable
    }
    #[doc = "Enable MultiController mode."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Mctl::Enable
    }
}
#[doc = "Field `MCTL` writer - MultiController mode. In MultiController mode the SCL high time counts once the SCL line has been detected high. If this is not enabled the high time counts as soon as the SCL line has been set high by the I2C controller."]
pub type MctlW<'a, REG> = crate::BitWriter<'a, REG, Mctl>;
impl<'a, REG> MctlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable MultiController mode."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Mctl::Disable)
    }
    #[doc = "Enable MultiController mode."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Mctl::Enable)
    }
}
#[doc = "Clock Stretching\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Clkstretch {
    #[doc = "0: Disables the clock stretching detection. This can be disabled if no Target on the bus does support clock stretching, so that the maximum speed on the bus can be reached."]
    Disable = 0,
    #[doc = "1: Enables the clock stretching detection. Enabling the clock stretching ensures compliance to the I2C standard but could limit the speed due the clock stretching."]
    Enable = 1,
}
impl From<Clkstretch> for bool {
    #[inline(always)]
    fn from(variant: Clkstretch) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLKSTRETCH` reader - Clock Stretching"]
pub type ClkstretchR = crate::BitReader<Clkstretch>;
impl ClkstretchR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Clkstretch {
        match self.bits {
            false => Clkstretch::Disable,
            true => Clkstretch::Enable,
        }
    }
    #[doc = "Disables the clock stretching detection. This can be disabled if no Target on the bus does support clock stretching, so that the maximum speed on the bus can be reached."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Clkstretch::Disable
    }
    #[doc = "Enables the clock stretching detection. Enabling the clock stretching ensures compliance to the I2C standard but could limit the speed due the clock stretching."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Clkstretch::Enable
    }
}
#[doc = "Field `CLKSTRETCH` writer - Clock Stretching"]
pub type ClkstretchW<'a, REG> = crate::BitWriter<'a, REG, Clkstretch>;
impl<'a, REG> ClkstretchW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disables the clock stretching detection. This can be disabled if no Target on the bus does support clock stretching, so that the maximum speed on the bus can be reached."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Clkstretch::Disable)
    }
    #[doc = "Enables the clock stretching detection. Enabling the clock stretching ensures compliance to the I2C standard but could limit the speed due the clock stretching."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Clkstretch::Enable)
    }
}
impl R {
    #[doc = "Bit 0 - Enable module. After this bit has been set, it should not be set again unless it has been cleared by writing a 0 or by a reset, otherwise transfer failures may occur."]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - MultiController mode. In MultiController mode the SCL high time counts once the SCL line has been detected high. If this is not enabled the high time counts as soon as the SCL line has been set high by the I2C controller."]
    #[inline(always)]
    pub fn mctl(&self) -> MctlR {
        MctlR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Clock Stretching"]
    #[inline(always)]
    pub fn clkstretch(&self) -> ClkstretchR {
        ClkstretchR::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR")
            .field("mctl", &self.mctl())
            .field("enable", &self.enable())
            .field("clkstretch", &self.clkstretch())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Enable module. After this bit has been set, it should not be set again unless it has been cleared by writing a 0 or by a reset, otherwise transfer failures may occur."]
    #[inline(always)]
    pub fn enable(&mut self) -> EnableW<'_, CrSpec> {
        EnableW::new(self, 0)
    }
    #[doc = "Bit 1 - MultiController mode. In MultiController mode the SCL high time counts once the SCL line has been detected high. If this is not enabled the high time counts as soon as the SCL line has been set high by the I2C controller."]
    #[inline(always)]
    pub fn mctl(&mut self) -> MctlW<'_, CrSpec> {
        MctlW::new(self, 1)
    }
    #[doc = "Bit 2 - Clock Stretching"]
    #[inline(always)]
    pub fn clkstretch(&mut self) -> ClkstretchW<'_, CrSpec> {
        ClkstretchW::new(self, 2)
    }
}
#[doc = "Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrSpec;
impl crate::RegisterSpec for CrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CrSpec {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CrSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CrSpec {}
