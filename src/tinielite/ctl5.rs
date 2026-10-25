#[doc = "Register `CTL5` reader"]
pub type R = crate::R<Ctl5Spec>;
#[doc = "Register `CTL5` writer"]
pub type W = crate::W<Ctl5Spec>;
#[doc = "Field `RUN` reader - Start the accelerator. Writing a 1 to this bit starts the accelerator. When the STOP instruction is executed this bit is cleared by hardware and DONE interrupt flag is set."]
pub type RunR = crate::BitReader;
#[doc = "Field `RUN` writer - Start the accelerator. Writing a 1 to this bit starts the accelerator. When the STOP instruction is executed this bit is cleared by hardware and DONE interrupt flag is set."]
pub type RunW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable circular addressing on input feature map read address.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enifmapcirc {
    #[doc = "0: `0`"]
    Disable = 0,
    #[doc = "1: Enable circular buffering on the input feature map"]
    Enable = 1,
}
impl From<Enifmapcirc> for bool {
    #[inline(always)]
    fn from(variant: Enifmapcirc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENIFMAPCIRC` reader - Enable circular addressing on input feature map read address."]
pub type EnifmapcircR = crate::BitReader<Enifmapcirc>;
impl EnifmapcircR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enifmapcirc {
        match self.bits {
            false => Enifmapcirc::Disable,
            true => Enifmapcirc::Enable,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Enifmapcirc::Disable
    }
    #[doc = "Enable circular buffering on the input feature map"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Enifmapcirc::Enable
    }
}
#[doc = "Field `ENIFMAPCIRC` writer - Enable circular addressing on input feature map read address."]
pub type EnifmapcircW<'a, REG> = crate::BitWriter<'a, REG, Enifmapcirc>;
impl<'a, REG> EnifmapcircW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Enifmapcirc::Disable)
    }
    #[doc = "Enable circular buffering on the input feature map"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Enifmapcirc::Enable)
    }
}
#[doc = "Enable circular addressing on output feature map read address.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enofmapcirc {
    #[doc = "0: Disable circular buffering on the output feature map"]
    Disable = 0,
    #[doc = "1: Enable circular buffering on the output feature map"]
    Enable = 1,
}
impl From<Enofmapcirc> for bool {
    #[inline(always)]
    fn from(variant: Enofmapcirc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENOFMAPCIRC` reader - Enable circular addressing on output feature map read address."]
pub type EnofmapcircR = crate::BitReader<Enofmapcirc>;
impl EnofmapcircR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enofmapcirc {
        match self.bits {
            false => Enofmapcirc::Disable,
            true => Enofmapcirc::Enable,
        }
    }
    #[doc = "Disable circular buffering on the output feature map"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Enofmapcirc::Disable
    }
    #[doc = "Enable circular buffering on the output feature map"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Enofmapcirc::Enable
    }
}
#[doc = "Field `ENOFMAPCIRC` writer - Enable circular addressing on output feature map read address."]
pub type EnofmapcircW<'a, REG> = crate::BitWriter<'a, REG, Enofmapcirc>;
impl<'a, REG> EnofmapcircW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable circular buffering on the output feature map"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Enofmapcirc::Disable)
    }
    #[doc = "Enable circular buffering on the output feature map"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Enofmapcirc::Enable)
    }
}
impl R {
    #[doc = "Bit 0 - Start the accelerator. Writing a 1 to this bit starts the accelerator. When the STOP instruction is executed this bit is cleared by hardware and DONE interrupt flag is set."]
    #[inline(always)]
    pub fn run(&self) -> RunR {
        RunR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - Enable circular addressing on input feature map read address."]
    #[inline(always)]
    pub fn enifmapcirc(&self) -> EnifmapcircR {
        EnifmapcircR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable circular addressing on output feature map read address."]
    #[inline(always)]
    pub fn enofmapcirc(&self) -> EnofmapcircR {
        EnofmapcircR::new(((self.bits >> 9) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTL5")
            .field("run", &self.run())
            .field("enifmapcirc", &self.enifmapcirc())
            .field("enofmapcirc", &self.enofmapcirc())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Start the accelerator. Writing a 1 to this bit starts the accelerator. When the STOP instruction is executed this bit is cleared by hardware and DONE interrupt flag is set."]
    #[inline(always)]
    pub fn run(&mut self) -> RunW<'_, Ctl5Spec> {
        RunW::new(self, 0)
    }
    #[doc = "Bit 8 - Enable circular addressing on input feature map read address."]
    #[inline(always)]
    pub fn enifmapcirc(&mut self) -> EnifmapcircW<'_, Ctl5Spec> {
        EnifmapcircW::new(self, 8)
    }
    #[doc = "Bit 9 - Enable circular addressing on output feature map read address."]
    #[inline(always)]
    pub fn enofmapcirc(&mut self) -> EnofmapcircW<'_, Ctl5Spec> {
        EnofmapcircW::new(self, 9)
    }
}
#[doc = "CTL5 register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ctl5Spec;
impl crate::RegisterSpec for Ctl5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl5::R`](R) reader structure"]
impl crate::Readable for Ctl5Spec {}
#[doc = "`write(|w| ..)` method takes [`ctl5::W`](W) writer structure"]
impl crate::Writable for Ctl5Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTL5 to value 0"]
impl crate::Resettable for Ctl5Spec {}
