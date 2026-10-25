#[doc = "Register `GENCLKEN` reader"]
pub type R = crate::R<GenclkenSpec>;
#[doc = "Register `GENCLKEN` writer"]
pub type W = crate::W<GenclkenSpec>;
#[doc = "EXCLKEN enables the CLK_OUT external clock output block.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Exclken {
    #[doc = "0: CLK_OUT block is disabled"]
    Disable = 0,
    #[doc = "1: CLK_OUT block is enabled"]
    Enable = 1,
}
impl From<Exclken> for bool {
    #[inline(always)]
    fn from(variant: Exclken) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EXCLKEN` reader - EXCLKEN enables the CLK_OUT external clock output block."]
pub type ExclkenR = crate::BitReader<Exclken>;
impl ExclkenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Exclken {
        match self.bits {
            false => Exclken::Disable,
            true => Exclken::Enable,
        }
    }
    #[doc = "CLK_OUT block is disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Exclken::Disable
    }
    #[doc = "CLK_OUT block is enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Exclken::Enable
    }
}
#[doc = "Field `EXCLKEN` writer - EXCLKEN enables the CLK_OUT external clock output block."]
pub type ExclkenW<'a, REG> = crate::BitWriter<'a, REG, Exclken>;
impl<'a, REG> ExclkenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLK_OUT block is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Exclken::Disable)
    }
    #[doc = "CLK_OUT block is enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Exclken::Enable)
    }
}
#[doc = "EXTDIVMCLK selects the divider value for the divider for the PLL Source MCLK.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Extdivmclk {
    #[doc = "0: CLK_OUT source is divided by 2"]
    Div2 = 0,
    #[doc = "1: CLK_OUT source is divided by 4"]
    Div4 = 1,
    #[doc = "2: CLK_OUT source is divided by 6"]
    Div6 = 2,
    #[doc = "3: CLK_OUT source is divided by 8"]
    Div8 = 3,
    #[doc = "4: CLK_OUT source is divided by 10"]
    Div10 = 4,
    #[doc = "5: CLK_OUT source is divided by 12"]
    Div12 = 5,
    #[doc = "6: CLK_OUT source is divided by 14"]
    Div14 = 6,
    #[doc = "7: CLK_OUT source is divided by 16"]
    Div16 = 7,
}
impl From<Extdivmclk> for u8 {
    #[inline(always)]
    fn from(variant: Extdivmclk) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Extdivmclk {
    type Ux = u8;
}
impl crate::IsEnum for Extdivmclk {}
#[doc = "Field `EXTDIVMCLK` reader - EXTDIVMCLK selects the divider value for the divider for the PLL Source MCLK."]
pub type ExtdivmclkR = crate::FieldReader<Extdivmclk>;
impl ExtdivmclkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Extdivmclk {
        match self.bits {
            0 => Extdivmclk::Div2,
            1 => Extdivmclk::Div4,
            2 => Extdivmclk::Div6,
            3 => Extdivmclk::Div8,
            4 => Extdivmclk::Div10,
            5 => Extdivmclk::Div12,
            6 => Extdivmclk::Div14,
            7 => Extdivmclk::Div16,
            _ => unreachable!(),
        }
    }
    #[doc = "CLK_OUT source is divided by 2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == Extdivmclk::Div2
    }
    #[doc = "CLK_OUT source is divided by 4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == Extdivmclk::Div4
    }
    #[doc = "CLK_OUT source is divided by 6"]
    #[inline(always)]
    pub fn is_div6(&self) -> bool {
        *self == Extdivmclk::Div6
    }
    #[doc = "CLK_OUT source is divided by 8"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == Extdivmclk::Div8
    }
    #[doc = "CLK_OUT source is divided by 10"]
    #[inline(always)]
    pub fn is_div10(&self) -> bool {
        *self == Extdivmclk::Div10
    }
    #[doc = "CLK_OUT source is divided by 12"]
    #[inline(always)]
    pub fn is_div12(&self) -> bool {
        *self == Extdivmclk::Div12
    }
    #[doc = "CLK_OUT source is divided by 14"]
    #[inline(always)]
    pub fn is_div14(&self) -> bool {
        *self == Extdivmclk::Div14
    }
    #[doc = "CLK_OUT source is divided by 16"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == Extdivmclk::Div16
    }
}
#[doc = "Field `EXTDIVMCLK` writer - EXTDIVMCLK selects the divider value for the divider for the PLL Source MCLK."]
pub type ExtdivmclkW<'a, REG> = crate::FieldWriter<'a, REG, 3, Extdivmclk, crate::Safe>;
impl<'a, REG> ExtdivmclkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CLK_OUT source is divided by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(Extdivmclk::Div2)
    }
    #[doc = "CLK_OUT source is divided by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(Extdivmclk::Div4)
    }
    #[doc = "CLK_OUT source is divided by 6"]
    #[inline(always)]
    pub fn div6(self) -> &'a mut crate::W<REG> {
        self.variant(Extdivmclk::Div6)
    }
    #[doc = "CLK_OUT source is divided by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(Extdivmclk::Div8)
    }
    #[doc = "CLK_OUT source is divided by 10"]
    #[inline(always)]
    pub fn div10(self) -> &'a mut crate::W<REG> {
        self.variant(Extdivmclk::Div10)
    }
    #[doc = "CLK_OUT source is divided by 12"]
    #[inline(always)]
    pub fn div12(self) -> &'a mut crate::W<REG> {
        self.variant(Extdivmclk::Div12)
    }
    #[doc = "CLK_OUT source is divided by 14"]
    #[inline(always)]
    pub fn div14(self) -> &'a mut crate::W<REG> {
        self.variant(Extdivmclk::Div14)
    }
    #[doc = "CLK_OUT source is divided by 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(Extdivmclk::Div16)
    }
}
#[doc = "MCLKEXTDIVEN enables or disables the divider function of the PLL Source to MCLK.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mclkextdiven {
    #[doc = "0: CLock divider is disabled (passthrough, EXTDIVMCLK is not applied)"]
    Passthru = 0,
    #[doc = "1: Clock divider is enabled (EXTDIVMCLK is applied)"]
    Enable = 1,
}
impl From<Mclkextdiven> for bool {
    #[inline(always)]
    fn from(variant: Mclkextdiven) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MCLKEXTDIVEN` reader - MCLKEXTDIVEN enables or disables the divider function of the PLL Source to MCLK."]
pub type MclkextdivenR = crate::BitReader<Mclkextdiven>;
impl MclkextdivenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mclkextdiven {
        match self.bits {
            false => Mclkextdiven::Passthru,
            true => Mclkextdiven::Enable,
        }
    }
    #[doc = "CLock divider is disabled (passthrough, EXTDIVMCLK is not applied)"]
    #[inline(always)]
    pub fn is_passthru(&self) -> bool {
        *self == Mclkextdiven::Passthru
    }
    #[doc = "Clock divider is enabled (EXTDIVMCLK is applied)"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Mclkextdiven::Enable
    }
}
#[doc = "Field `MCLKEXTDIVEN` writer - MCLKEXTDIVEN enables or disables the divider function of the PLL Source to MCLK."]
pub type MclkextdivenW<'a, REG> = crate::BitWriter<'a, REG, Mclkextdiven>;
impl<'a, REG> MclkextdivenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLock divider is disabled (passthrough, EXTDIVMCLK is not applied)"]
    #[inline(always)]
    pub fn passthru(self) -> &'a mut crate::W<REG> {
        self.variant(Mclkextdiven::Passthru)
    }
    #[doc = "Clock divider is enabled (EXTDIVMCLK is applied)"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Mclkextdiven::Enable)
    }
}
#[doc = "EXTDIVCAN selects the divider value for the divider for the PLL Source to CAN.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Extdivcan {
    #[doc = "0: CLK_OUT source is divided by 2"]
    Div2 = 0,
    #[doc = "1: CLK_OUT source is divided by 4"]
    Div4 = 1,
    #[doc = "2: CLK_OUT source is divided by 6"]
    Div6 = 2,
    #[doc = "3: CLK_OUT source is divided by 8"]
    Div8 = 3,
    #[doc = "4: CLK_OUT source is divided by 10"]
    Div10 = 4,
    #[doc = "5: CLK_OUT source is divided by 12"]
    Div12 = 5,
    #[doc = "6: CLK_OUT source is divided by 14"]
    Div14 = 6,
    #[doc = "7: CLK_OUT source is divided by 16"]
    Div16 = 7,
}
impl From<Extdivcan> for u8 {
    #[inline(always)]
    fn from(variant: Extdivcan) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Extdivcan {
    type Ux = u8;
}
impl crate::IsEnum for Extdivcan {}
#[doc = "Field `EXTDIVCAN` reader - EXTDIVCAN selects the divider value for the divider for the PLL Source to CAN."]
pub type ExtdivcanR = crate::FieldReader<Extdivcan>;
impl ExtdivcanR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Extdivcan {
        match self.bits {
            0 => Extdivcan::Div2,
            1 => Extdivcan::Div4,
            2 => Extdivcan::Div6,
            3 => Extdivcan::Div8,
            4 => Extdivcan::Div10,
            5 => Extdivcan::Div12,
            6 => Extdivcan::Div14,
            7 => Extdivcan::Div16,
            _ => unreachable!(),
        }
    }
    #[doc = "CLK_OUT source is divided by 2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == Extdivcan::Div2
    }
    #[doc = "CLK_OUT source is divided by 4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == Extdivcan::Div4
    }
    #[doc = "CLK_OUT source is divided by 6"]
    #[inline(always)]
    pub fn is_div6(&self) -> bool {
        *self == Extdivcan::Div6
    }
    #[doc = "CLK_OUT source is divided by 8"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == Extdivcan::Div8
    }
    #[doc = "CLK_OUT source is divided by 10"]
    #[inline(always)]
    pub fn is_div10(&self) -> bool {
        *self == Extdivcan::Div10
    }
    #[doc = "CLK_OUT source is divided by 12"]
    #[inline(always)]
    pub fn is_div12(&self) -> bool {
        *self == Extdivcan::Div12
    }
    #[doc = "CLK_OUT source is divided by 14"]
    #[inline(always)]
    pub fn is_div14(&self) -> bool {
        *self == Extdivcan::Div14
    }
    #[doc = "CLK_OUT source is divided by 16"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == Extdivcan::Div16
    }
}
#[doc = "Field `EXTDIVCAN` writer - EXTDIVCAN selects the divider value for the divider for the PLL Source to CAN."]
pub type ExtdivcanW<'a, REG> = crate::FieldWriter<'a, REG, 3, Extdivcan, crate::Safe>;
impl<'a, REG> ExtdivcanW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CLK_OUT source is divided by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(Extdivcan::Div2)
    }
    #[doc = "CLK_OUT source is divided by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(Extdivcan::Div4)
    }
    #[doc = "CLK_OUT source is divided by 6"]
    #[inline(always)]
    pub fn div6(self) -> &'a mut crate::W<REG> {
        self.variant(Extdivcan::Div6)
    }
    #[doc = "CLK_OUT source is divided by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(Extdivcan::Div8)
    }
    #[doc = "CLK_OUT source is divided by 10"]
    #[inline(always)]
    pub fn div10(self) -> &'a mut crate::W<REG> {
        self.variant(Extdivcan::Div10)
    }
    #[doc = "CLK_OUT source is divided by 12"]
    #[inline(always)]
    pub fn div12(self) -> &'a mut crate::W<REG> {
        self.variant(Extdivcan::Div12)
    }
    #[doc = "CLK_OUT source is divided by 14"]
    #[inline(always)]
    pub fn div14(self) -> &'a mut crate::W<REG> {
        self.variant(Extdivcan::Div14)
    }
    #[doc = "CLK_OUT source is divided by 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(Extdivcan::Div16)
    }
}
#[doc = "CANEXTDIVEN enables or disables the divider function of the PLL Source to CAN.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Canextdiven {
    #[doc = "0: CLock divider is disabled (passthrough, EXTDIVCAN is not applied)"]
    Passthru = 0,
    #[doc = "1: Clock divider is enabled (EXTDIVCAN is applied)"]
    Enable = 1,
}
impl From<Canextdiven> for bool {
    #[inline(always)]
    fn from(variant: Canextdiven) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CANEXTDIVEN` reader - CANEXTDIVEN enables or disables the divider function of the PLL Source to CAN."]
pub type CanextdivenR = crate::BitReader<Canextdiven>;
impl CanextdivenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Canextdiven {
        match self.bits {
            false => Canextdiven::Passthru,
            true => Canextdiven::Enable,
        }
    }
    #[doc = "CLock divider is disabled (passthrough, EXTDIVCAN is not applied)"]
    #[inline(always)]
    pub fn is_passthru(&self) -> bool {
        *self == Canextdiven::Passthru
    }
    #[doc = "Clock divider is enabled (EXTDIVCAN is applied)"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Canextdiven::Enable
    }
}
#[doc = "Field `CANEXTDIVEN` writer - CANEXTDIVEN enables or disables the divider function of the PLL Source to CAN."]
pub type CanextdivenW<'a, REG> = crate::BitWriter<'a, REG, Canextdiven>;
impl<'a, REG> CanextdivenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLock divider is disabled (passthrough, EXTDIVCAN is not applied)"]
    #[inline(always)]
    pub fn passthru(self) -> &'a mut crate::W<REG> {
        self.variant(Canextdiven::Passthru)
    }
    #[doc = "Clock divider is enabled (EXTDIVCAN is applied)"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Canextdiven::Enable)
    }
}
impl R {
    #[doc = "Bit 0 - EXCLKEN enables the CLK_OUT external clock output block."]
    #[inline(always)]
    pub fn exclken(&self) -> ExclkenR {
        ExclkenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:10 - EXTDIVMCLK selects the divider value for the divider for the PLL Source MCLK."]
    #[inline(always)]
    pub fn extdivmclk(&self) -> ExtdivmclkR {
        ExtdivmclkR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - MCLKEXTDIVEN enables or disables the divider function of the PLL Source to MCLK."]
    #[inline(always)]
    pub fn mclkextdiven(&self) -> MclkextdivenR {
        MclkextdivenR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - EXTDIVCAN selects the divider value for the divider for the PLL Source to CAN."]
    #[inline(always)]
    pub fn extdivcan(&self) -> ExtdivcanR {
        ExtdivcanR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - CANEXTDIVEN enables or disables the divider function of the PLL Source to CAN."]
    #[inline(always)]
    pub fn canextdiven(&self) -> CanextdivenR {
        CanextdivenR::new(((self.bits >> 15) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GENCLKEN")
            .field("extdivmclk", &self.extdivmclk())
            .field("exclken", &self.exclken())
            .field("mclkextdiven", &self.mclkextdiven())
            .field("extdivcan", &self.extdivcan())
            .field("canextdiven", &self.canextdiven())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - EXCLKEN enables the CLK_OUT external clock output block."]
    #[inline(always)]
    pub fn exclken(&mut self) -> ExclkenW<'_, GenclkenSpec> {
        ExclkenW::new(self, 0)
    }
    #[doc = "Bits 8:10 - EXTDIVMCLK selects the divider value for the divider for the PLL Source MCLK."]
    #[inline(always)]
    pub fn extdivmclk(&mut self) -> ExtdivmclkW<'_, GenclkenSpec> {
        ExtdivmclkW::new(self, 8)
    }
    #[doc = "Bit 11 - MCLKEXTDIVEN enables or disables the divider function of the PLL Source to MCLK."]
    #[inline(always)]
    pub fn mclkextdiven(&mut self) -> MclkextdivenW<'_, GenclkenSpec> {
        MclkextdivenW::new(self, 11)
    }
    #[doc = "Bits 12:14 - EXTDIVCAN selects the divider value for the divider for the PLL Source to CAN."]
    #[inline(always)]
    pub fn extdivcan(&mut self) -> ExtdivcanW<'_, GenclkenSpec> {
        ExtdivcanW::new(self, 12)
    }
    #[doc = "Bit 15 - CANEXTDIVEN enables or disables the divider function of the PLL Source to CAN."]
    #[inline(always)]
    pub fn canextdiven(&mut self) -> CanextdivenW<'_, GenclkenSpec> {
        CanextdivenW::new(self, 15)
    }
}
#[doc = "General clock enable control\n\nYou can [`read`](crate::Reg::read) this register and get [`genclken::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`genclken::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GenclkenSpec;
impl crate::RegisterSpec for GenclkenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`genclken::R`](R) reader structure"]
impl crate::Readable for GenclkenSpec {}
#[doc = "`write(|w| ..)` method takes [`genclken::W`](W) writer structure"]
impl crate::Writable for GenclkenSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets GENCLKEN to value 0"]
impl crate::Resettable for GenclkenSpec {}
