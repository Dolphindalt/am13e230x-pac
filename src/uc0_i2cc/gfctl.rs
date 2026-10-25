#[doc = "Register `GFCTL` reader"]
pub type R = crate::R<GfctlSpec>;
#[doc = "Register `GFCTL` writer"]
pub type W = crate::W<GfctlSpec>;
#[doc = "Glitch Suppression Pulse Width This field controls the pulse width select for glitch suppression on the SCL and SDA lines. The following values are the glitch suppression values in terms of functional clocks. (Core Domain only)\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dgfsel {
    #[doc = "0: Bypass"]
    Disabled = 0,
    #[doc = "1: 1 clock"]
    Clk1 = 1,
    #[doc = "2: 2 clocks"]
    Clk2 = 2,
    #[doc = "3: 3 clocks"]
    Clk3 = 3,
    #[doc = "4: 4 clocks"]
    Clk4 = 4,
    #[doc = "5: 8 clocks"]
    Clk8 = 5,
    #[doc = "6: 16 clocks"]
    Clk16 = 6,
    #[doc = "7: 31 clocks"]
    Clk31 = 7,
}
impl From<Dgfsel> for u8 {
    #[inline(always)]
    fn from(variant: Dgfsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dgfsel {
    type Ux = u8;
}
impl crate::IsEnum for Dgfsel {}
#[doc = "Field `DGFSEL` reader - Glitch Suppression Pulse Width This field controls the pulse width select for glitch suppression on the SCL and SDA lines. The following values are the glitch suppression values in terms of functional clocks. (Core Domain only)"]
pub type DgfselR = crate::FieldReader<Dgfsel>;
impl DgfselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dgfsel {
        match self.bits {
            0 => Dgfsel::Disabled,
            1 => Dgfsel::Clk1,
            2 => Dgfsel::Clk2,
            3 => Dgfsel::Clk3,
            4 => Dgfsel::Clk4,
            5 => Dgfsel::Clk8,
            6 => Dgfsel::Clk16,
            7 => Dgfsel::Clk31,
            _ => unreachable!(),
        }
    }
    #[doc = "Bypass"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Dgfsel::Disabled
    }
    #[doc = "1 clock"]
    #[inline(always)]
    pub fn is_clk_1(&self) -> bool {
        *self == Dgfsel::Clk1
    }
    #[doc = "2 clocks"]
    #[inline(always)]
    pub fn is_clk_2(&self) -> bool {
        *self == Dgfsel::Clk2
    }
    #[doc = "3 clocks"]
    #[inline(always)]
    pub fn is_clk_3(&self) -> bool {
        *self == Dgfsel::Clk3
    }
    #[doc = "4 clocks"]
    #[inline(always)]
    pub fn is_clk_4(&self) -> bool {
        *self == Dgfsel::Clk4
    }
    #[doc = "8 clocks"]
    #[inline(always)]
    pub fn is_clk_8(&self) -> bool {
        *self == Dgfsel::Clk8
    }
    #[doc = "16 clocks"]
    #[inline(always)]
    pub fn is_clk_16(&self) -> bool {
        *self == Dgfsel::Clk16
    }
    #[doc = "31 clocks"]
    #[inline(always)]
    pub fn is_clk_31(&self) -> bool {
        *self == Dgfsel::Clk31
    }
}
#[doc = "Field `DGFSEL` writer - Glitch Suppression Pulse Width This field controls the pulse width select for glitch suppression on the SCL and SDA lines. The following values are the glitch suppression values in terms of functional clocks. (Core Domain only)"]
pub type DgfselW<'a, REG> = crate::FieldWriter<'a, REG, 3, Dgfsel, crate::Safe>;
impl<'a, REG> DgfselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Bypass"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dgfsel::Disabled)
    }
    #[doc = "1 clock"]
    #[inline(always)]
    pub fn clk_1(self) -> &'a mut crate::W<REG> {
        self.variant(Dgfsel::Clk1)
    }
    #[doc = "2 clocks"]
    #[inline(always)]
    pub fn clk_2(self) -> &'a mut crate::W<REG> {
        self.variant(Dgfsel::Clk2)
    }
    #[doc = "3 clocks"]
    #[inline(always)]
    pub fn clk_3(self) -> &'a mut crate::W<REG> {
        self.variant(Dgfsel::Clk3)
    }
    #[doc = "4 clocks"]
    #[inline(always)]
    pub fn clk_4(self) -> &'a mut crate::W<REG> {
        self.variant(Dgfsel::Clk4)
    }
    #[doc = "8 clocks"]
    #[inline(always)]
    pub fn clk_8(self) -> &'a mut crate::W<REG> {
        self.variant(Dgfsel::Clk8)
    }
    #[doc = "16 clocks"]
    #[inline(always)]
    pub fn clk_16(self) -> &'a mut crate::W<REG> {
        self.variant(Dgfsel::Clk16)
    }
    #[doc = "31 clocks"]
    #[inline(always)]
    pub fn clk_31(self) -> &'a mut crate::W<REG> {
        self.variant(Dgfsel::Clk31)
    }
}
#[doc = "Analog Glitch Suppression Enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Agfen {
    #[doc = "0: Analog Glitch Filter disable"]
    Disable = 0,
    #[doc = "1: Analog Glitch Filter enable"]
    Enable = 1,
}
impl From<Agfen> for bool {
    #[inline(always)]
    fn from(variant: Agfen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AGFEN` reader - Analog Glitch Suppression Enable"]
pub type AgfenR = crate::BitReader<Agfen>;
impl AgfenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Agfen {
        match self.bits {
            false => Agfen::Disable,
            true => Agfen::Enable,
        }
    }
    #[doc = "Analog Glitch Filter disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Agfen::Disable
    }
    #[doc = "Analog Glitch Filter enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Agfen::Enable
    }
}
#[doc = "Field `AGFEN` writer - Analog Glitch Suppression Enable"]
pub type AgfenW<'a, REG> = crate::BitWriter<'a, REG, Agfen>;
impl<'a, REG> AgfenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Analog Glitch Filter disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Agfen::Disable)
    }
    #[doc = "Analog Glitch Filter enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Agfen::Enable)
    }
}
impl R {
    #[doc = "Bits 0:2 - Glitch Suppression Pulse Width This field controls the pulse width select for glitch suppression on the SCL and SDA lines. The following values are the glitch suppression values in terms of functional clocks. (Core Domain only)"]
    #[inline(always)]
    pub fn dgfsel(&self) -> DgfselR {
        DgfselR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 8 - Analog Glitch Suppression Enable"]
    #[inline(always)]
    pub fn agfen(&self) -> AgfenR {
        AgfenR::new(((self.bits >> 8) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GFCTL")
            .field("dgfsel", &self.dgfsel())
            .field("agfen", &self.agfen())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - Glitch Suppression Pulse Width This field controls the pulse width select for glitch suppression on the SCL and SDA lines. The following values are the glitch suppression values in terms of functional clocks. (Core Domain only)"]
    #[inline(always)]
    pub fn dgfsel(&mut self) -> DgfselW<'_, GfctlSpec> {
        DgfselW::new(self, 0)
    }
    #[doc = "Bit 8 - Analog Glitch Suppression Enable"]
    #[inline(always)]
    pub fn agfen(&mut self) -> AgfenW<'_, GfctlSpec> {
        AgfenW::new(self, 8)
    }
}
#[doc = "I2C Glitch Filter Control\n\nYou can [`read`](crate::Reg::read) this register and get [`gfctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gfctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GfctlSpec;
impl crate::RegisterSpec for GfctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gfctl::R`](R) reader structure"]
impl crate::Readable for GfctlSpec {}
#[doc = "`write(|w| ..)` method takes [`gfctl::W`](W) writer structure"]
impl crate::Writable for GfctlSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets GFCTL to value 0"]
impl crate::Resettable for GfctlSpec {}
