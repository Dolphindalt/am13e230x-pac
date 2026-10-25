#[doc = "Register `OAR` reader"]
pub type R = crate::R<OarSpec>;
#[doc = "Register `OAR` writer"]
pub type W = crate::W<OarSpec>;
#[doc = "Field `OAR` reader - Own Address: This field specifies bits A9 through A0 of the Target address. In 7-bit addressing mode as selected by I2CSOAR.MODE bit, the top 3 bits are don't care"]
pub type OarR = crate::FieldReader<u16>;
#[doc = "Field `OAR` writer - Own Address: This field specifies bits A9 through A0 of the Target address. In 7-bit addressing mode as selected by I2CSOAR.MODE bit, the top 3 bits are don't care"]
pub type OarW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16, crate::Safe>;
#[doc = "Own Address Enable\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Oaren {
    #[doc = "0: Disable OAR address"]
    Disable = 0,
    #[doc = "1: Enable OAR address"]
    Enable = 1,
}
impl From<Oaren> for bool {
    #[inline(always)]
    fn from(variant: Oaren) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OAREN` reader - Own Address Enable"]
pub type OarenR = crate::BitReader<Oaren>;
impl OarenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Oaren {
        match self.bits {
            false => Oaren::Disable,
            true => Oaren::Enable,
        }
    }
    #[doc = "Disable OAR address"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Oaren::Disable
    }
    #[doc = "Enable OAR address"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Oaren::Enable
    }
}
#[doc = "Field `OAREN` writer - Own Address Enable"]
pub type OarenW<'a, REG> = crate::BitWriter<'a, REG, Oaren>;
impl<'a, REG> OarenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable OAR address"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Oaren::Disable)
    }
    #[doc = "Enable OAR address"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Oaren::Enable)
    }
}
#[doc = "This bit selects the adressing mode to be used. When 0, 7-bit addressing is used. When 1, 10-bit addressing is used.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mode {
    #[doc = "0: Enable 7-bit addressing"]
    Mode7 = 0,
    #[doc = "1: Enable 10-bit addressing"]
    Mode10 = 1,
}
impl From<Mode> for bool {
    #[inline(always)]
    fn from(variant: Mode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MODE` reader - This bit selects the adressing mode to be used. When 0, 7-bit addressing is used. When 1, 10-bit addressing is used."]
pub type ModeR = crate::BitReader<Mode>;
impl ModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mode {
        match self.bits {
            false => Mode::Mode7,
            true => Mode::Mode10,
        }
    }
    #[doc = "Enable 7-bit addressing"]
    #[inline(always)]
    pub fn is_mode7(&self) -> bool {
        *self == Mode::Mode7
    }
    #[doc = "Enable 10-bit addressing"]
    #[inline(always)]
    pub fn is_mode10(&self) -> bool {
        *self == Mode::Mode10
    }
}
#[doc = "Field `MODE` writer - This bit selects the adressing mode to be used. When 0, 7-bit addressing is used. When 1, 10-bit addressing is used."]
pub type ModeW<'a, REG> = crate::BitWriter<'a, REG, Mode>;
impl<'a, REG> ModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable 7-bit addressing"]
    #[inline(always)]
    pub fn mode7(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Mode7)
    }
    #[doc = "Enable 10-bit addressing"]
    #[inline(always)]
    pub fn mode10(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Mode10)
    }
}
impl R {
    #[doc = "Bits 0:9 - Own Address: This field specifies bits A9 through A0 of the Target address. In 7-bit addressing mode as selected by I2CSOAR.MODE bit, the top 3 bits are don't care"]
    #[inline(always)]
    pub fn oar(&self) -> OarR {
        OarR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 14 - Own Address Enable"]
    #[inline(always)]
    pub fn oaren(&self) -> OarenR {
        OarenR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - This bit selects the adressing mode to be used. When 0, 7-bit addressing is used. When 1, 10-bit addressing is used."]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new(((self.bits >> 15) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OAR")
            .field("oar", &self.oar())
            .field("oaren", &self.oaren())
            .field("mode", &self.mode())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:9 - Own Address: This field specifies bits A9 through A0 of the Target address. In 7-bit addressing mode as selected by I2CSOAR.MODE bit, the top 3 bits are don't care"]
    #[inline(always)]
    pub fn oar(&mut self) -> OarW<'_, OarSpec> {
        OarW::new(self, 0)
    }
    #[doc = "Bit 14 - Own Address Enable"]
    #[inline(always)]
    pub fn oaren(&mut self) -> OarenW<'_, OarSpec> {
        OarenW::new(self, 14)
    }
    #[doc = "Bit 15 - This bit selects the adressing mode to be used. When 0, 7-bit addressing is used. When 1, 10-bit addressing is used."]
    #[inline(always)]
    pub fn mode(&mut self) -> ModeW<'_, OarSpec> {
        ModeW::new(self, 15)
    }
}
#[doc = "I2C Own Address\n\nYou can [`read`](crate::Reg::read) this register and get [`oar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OarSpec;
impl crate::RegisterSpec for OarSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`oar::R`](R) reader structure"]
impl crate::Readable for OarSpec {}
#[doc = "`write(|w| ..)` method takes [`oar::W`](W) writer structure"]
impl crate::Writable for OarSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets OAR to value 0x4000"]
impl crate::Resettable for OarSpec {
    const RESET_VALUE: u32 = 0x4000;
}
