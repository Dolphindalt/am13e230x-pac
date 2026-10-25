#[doc = "Register `HB8TIME3` reader"]
pub type R = crate::R<Hb8time3Spec>;
#[doc = "Register `HB8TIME3` writer"]
pub type W = crate::W<Hb8time3Spec>;
#[doc = "CS2n Read Wait State Minus One\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rdwsm {
    #[doc = "0: No change in the number of wait state clock cycles programmed in the RDWS field of EPIHB8CFG."]
    Unchanged = 0,
    #[doc = "1: Wait state value is now (RDWS - 1) where the RDWS field is programmed in EPIHB8CFG."]
    MinusOne = 1,
}
impl From<Rdwsm> for bool {
    #[inline(always)]
    fn from(variant: Rdwsm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RDWSM` reader - CS2n Read Wait State Minus One"]
pub type RdwsmR = crate::BitReader<Rdwsm>;
impl RdwsmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rdwsm {
        match self.bits {
            false => Rdwsm::Unchanged,
            true => Rdwsm::MinusOne,
        }
    }
    #[doc = "No change in the number of wait state clock cycles programmed in the RDWS field of EPIHB8CFG."]
    #[inline(always)]
    pub fn is_unchanged(&self) -> bool {
        *self == Rdwsm::Unchanged
    }
    #[doc = "Wait state value is now (RDWS - 1) where the RDWS field is programmed in EPIHB8CFG."]
    #[inline(always)]
    pub fn is_minus_one(&self) -> bool {
        *self == Rdwsm::MinusOne
    }
}
#[doc = "Field `RDWSM` writer - CS2n Read Wait State Minus One"]
pub type RdwsmW<'a, REG> = crate::BitWriter<'a, REG, Rdwsm>;
impl<'a, REG> RdwsmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No change in the number of wait state clock cycles programmed in the RDWS field of EPIHB8CFG."]
    #[inline(always)]
    pub fn unchanged(self) -> &'a mut crate::W<REG> {
        self.variant(Rdwsm::Unchanged)
    }
    #[doc = "Wait state value is now (RDWS - 1) where the RDWS field is programmed in EPIHB8CFG."]
    #[inline(always)]
    pub fn minus_one(self) -> &'a mut crate::W<REG> {
        self.variant(Rdwsm::MinusOne)
    }
}
#[doc = "CS2n Write Wait State Minus One\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wrwsm {
    #[doc = "0: No change in the number of wait state clock cycles programmed in the in WRWS field in EPIHB8CFG register."]
    Unchanged = 0,
    #[doc = "1: Wait state value is now (WRWS - 1) where the WRWS field is programmed in EPIHB8CFG."]
    MinusOne = 1,
}
impl From<Wrwsm> for bool {
    #[inline(always)]
    fn from(variant: Wrwsm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WRWSM` reader - CS2n Write Wait State Minus One"]
pub type WrwsmR = crate::BitReader<Wrwsm>;
impl WrwsmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wrwsm {
        match self.bits {
            false => Wrwsm::Unchanged,
            true => Wrwsm::MinusOne,
        }
    }
    #[doc = "No change in the number of wait state clock cycles programmed in the in WRWS field in EPIHB8CFG register."]
    #[inline(always)]
    pub fn is_unchanged(&self) -> bool {
        *self == Wrwsm::Unchanged
    }
    #[doc = "Wait state value is now (WRWS - 1) where the WRWS field is programmed in EPIHB8CFG."]
    #[inline(always)]
    pub fn is_minus_one(&self) -> bool {
        *self == Wrwsm::MinusOne
    }
}
#[doc = "Field `WRWSM` writer - CS2n Write Wait State Minus One"]
pub type WrwsmW<'a, REG> = crate::BitWriter<'a, REG, Wrwsm>;
impl<'a, REG> WrwsmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No change in the number of wait state clock cycles programmed in the in WRWS field in EPIHB8CFG register."]
    #[inline(always)]
    pub fn unchanged(self) -> &'a mut crate::W<REG> {
        self.variant(Wrwsm::Unchanged)
    }
    #[doc = "Wait state value is now (WRWS - 1) where the WRWS field is programmed in EPIHB8CFG."]
    #[inline(always)]
    pub fn minus_one(self) -> &'a mut crate::W<REG> {
        self.variant(Wrwsm::MinusOne)
    }
}
#[doc = "CS2n Inter-transfer Capture Width\n\nValue on reset: 2"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Capwidth {
    #[doc = "1: 1 EPI clock."]
    Clk1 = 1,
    #[doc = "2: 2 EPI clock."]
    Clk2 = 2,
}
impl From<Capwidth> for u8 {
    #[inline(always)]
    fn from(variant: Capwidth) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Capwidth {
    type Ux = u8;
}
impl crate::IsEnum for Capwidth {}
#[doc = "Field `CAPWIDTH` reader - CS2n Inter-transfer Capture Width"]
pub type CapwidthR = crate::FieldReader<Capwidth>;
impl CapwidthR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Capwidth> {
        match self.bits {
            1 => Some(Capwidth::Clk1),
            2 => Some(Capwidth::Clk2),
            _ => None,
        }
    }
    #[doc = "1 EPI clock."]
    #[inline(always)]
    pub fn is_clk_1(&self) -> bool {
        *self == Capwidth::Clk1
    }
    #[doc = "2 EPI clock."]
    #[inline(always)]
    pub fn is_clk_2(&self) -> bool {
        *self == Capwidth::Clk2
    }
}
#[doc = "Field `CAPWIDTH` writer - CS2n Inter-transfer Capture Width"]
pub type CapwidthW<'a, REG> = crate::FieldWriter<'a, REG, 2, Capwidth, crate::Safe>;
impl<'a, REG> CapwidthW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1 EPI clock."]
    #[inline(always)]
    pub fn clk_1(self) -> &'a mut crate::W<REG> {
        self.variant(Capwidth::Clk1)
    }
    #[doc = "2 EPI clock."]
    #[inline(always)]
    pub fn clk_2(self) -> &'a mut crate::W<REG> {
        self.variant(Capwidth::Clk2)
    }
}
#[doc = "CS2n Input Ready Delay\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Irdydly {
    #[doc = "1: Stall begins one EPI clocks past iRDY low being sampled on the rising edge of EPIO clock."]
    Clk1 = 1,
    #[doc = "2: Stall begins two EPI clocks past iRDY low being sampled on the rising edge of EPIO clock."]
    Clk2 = 2,
    #[doc = "3: Stall begins three EPI clocks past iRDY low being sampled on the rising edge of EPIO clock."]
    Clk3 = 3,
}
impl From<Irdydly> for u8 {
    #[inline(always)]
    fn from(variant: Irdydly) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Irdydly {
    type Ux = u8;
}
impl crate::IsEnum for Irdydly {}
#[doc = "Field `IRDYDLY` reader - CS2n Input Ready Delay"]
pub type IrdydlyR = crate::FieldReader<Irdydly>;
impl IrdydlyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Irdydly> {
        match self.bits {
            1 => Some(Irdydly::Clk1),
            2 => Some(Irdydly::Clk2),
            3 => Some(Irdydly::Clk3),
            _ => None,
        }
    }
    #[doc = "Stall begins one EPI clocks past iRDY low being sampled on the rising edge of EPIO clock."]
    #[inline(always)]
    pub fn is_clk_1(&self) -> bool {
        *self == Irdydly::Clk1
    }
    #[doc = "Stall begins two EPI clocks past iRDY low being sampled on the rising edge of EPIO clock."]
    #[inline(always)]
    pub fn is_clk_2(&self) -> bool {
        *self == Irdydly::Clk2
    }
    #[doc = "Stall begins three EPI clocks past iRDY low being sampled on the rising edge of EPIO clock."]
    #[inline(always)]
    pub fn is_clk_3(&self) -> bool {
        *self == Irdydly::Clk3
    }
}
#[doc = "Field `IRDYDLY` writer - CS2n Input Ready Delay"]
pub type IrdydlyW<'a, REG> = crate::FieldWriter<'a, REG, 2, Irdydly, crate::Safe>;
impl<'a, REG> IrdydlyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Stall begins one EPI clocks past iRDY low being sampled on the rising edge of EPIO clock."]
    #[inline(always)]
    pub fn clk_1(self) -> &'a mut crate::W<REG> {
        self.variant(Irdydly::Clk1)
    }
    #[doc = "Stall begins two EPI clocks past iRDY low being sampled on the rising edge of EPIO clock."]
    #[inline(always)]
    pub fn clk_2(self) -> &'a mut crate::W<REG> {
        self.variant(Irdydly::Clk2)
    }
    #[doc = "Stall begins three EPI clocks past iRDY low being sampled on the rising edge of EPIO clock."]
    #[inline(always)]
    pub fn clk_3(self) -> &'a mut crate::W<REG> {
        self.variant(Irdydly::Clk3)
    }
}
impl R {
    #[doc = "Bit 0 - CS2n Read Wait State Minus One"]
    #[inline(always)]
    pub fn rdwsm(&self) -> RdwsmR {
        RdwsmR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - CS2n Write Wait State Minus One"]
    #[inline(always)]
    pub fn wrwsm(&self) -> WrwsmR {
        WrwsmR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 12:13 - CS2n Inter-transfer Capture Width"]
    #[inline(always)]
    pub fn capwidth(&self) -> CapwidthR {
        CapwidthR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 24:25 - CS2n Input Ready Delay"]
    #[inline(always)]
    pub fn irdydly(&self) -> IrdydlyR {
        IrdydlyR::new(((self.bits >> 24) & 3) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HB8TIME3")
            .field("rdwsm", &self.rdwsm())
            .field("wrwsm", &self.wrwsm())
            .field("capwidth", &self.capwidth())
            .field("irdydly", &self.irdydly())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - CS2n Read Wait State Minus One"]
    #[inline(always)]
    pub fn rdwsm(&mut self) -> RdwsmW<'_, Hb8time3Spec> {
        RdwsmW::new(self, 0)
    }
    #[doc = "Bit 4 - CS2n Write Wait State Minus One"]
    #[inline(always)]
    pub fn wrwsm(&mut self) -> WrwsmW<'_, Hb8time3Spec> {
        WrwsmW::new(self, 4)
    }
    #[doc = "Bits 12:13 - CS2n Inter-transfer Capture Width"]
    #[inline(always)]
    pub fn capwidth(&mut self) -> CapwidthW<'_, Hb8time3Spec> {
        CapwidthW::new(self, 12)
    }
    #[doc = "Bits 24:25 - CS2n Input Ready Delay"]
    #[inline(always)]
    pub fn irdydly(&mut self) -> IrdydlyW<'_, Hb8time3Spec> {
        IrdydlyW::new(self, 24)
    }
}
#[doc = "EPI Host-Bus 8 Timing Extension\n\nYou can [`read`](crate::Reg::read) this register and get [`hb8time3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hb8time3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hb8time3Spec;
impl crate::RegisterSpec for Hb8time3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hb8time3::R`](R) reader structure"]
impl crate::Readable for Hb8time3Spec {}
#[doc = "`write(|w| ..)` method takes [`hb8time3::W`](W) writer structure"]
impl crate::Writable for Hb8time3Spec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets HB8TIME3 to value 0x0002_2000"]
impl crate::Resettable for Hb8time3Spec {
    const RESET_VALUE: u32 = 0x0002_2000;
}
