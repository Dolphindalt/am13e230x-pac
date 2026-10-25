#[doc = "Register `IFCTL_23[%s]` reader"]
pub type R = crate::R<Ifctl23Spec>;
#[doc = "Register `IFCTL_23[%s]` writer"]
pub type W = crate::W<Ifctl23Spec>;
#[doc = "Input Select (CCP0) This field selects the input source to the filter input. 4h-7h = Reserved\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Isel {
    #[doc = "0: CCP of the corresponding capture compare unit"]
    CcpxInput = 0,
    #[doc = "1: Input pair CCPX of the capture compare unit. For CCP0 input pair is CCP1 and for CCP1 input pair is CCP0."]
    CcpxInputPair = 1,
    #[doc = "2: CCP0 of the counter"]
    Ccp0Input = 2,
    #[doc = "3: Trigger"]
    TrigInput = 3,
    #[doc = "4: XOR of CCP inputs as input source (Used in Hall input mode)."]
    CcpXor = 4,
    #[doc = "5: subscriber 0 event as input source."]
    Fsub0 = 5,
    #[doc = "6: subscriber 1 event as input source."]
    Fsub1 = 6,
    #[doc = "7: Comparator 0 output."]
    Comp0 = 7,
    #[doc = "8: Comparator 1 output."]
    Comp1 = 8,
    #[doc = "9: Comparator 2 output."]
    Comp2 = 9,
}
impl From<Isel> for u8 {
    #[inline(always)]
    fn from(variant: Isel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Isel {
    type Ux = u8;
}
impl crate::IsEnum for Isel {}
#[doc = "Field `ISEL` reader - Input Select (CCP0) This field selects the input source to the filter input. 4h-7h = Reserved"]
pub type IselR = crate::FieldReader<Isel>;
impl IselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Isel> {
        match self.bits {
            0 => Some(Isel::CcpxInput),
            1 => Some(Isel::CcpxInputPair),
            2 => Some(Isel::Ccp0Input),
            3 => Some(Isel::TrigInput),
            4 => Some(Isel::CcpXor),
            5 => Some(Isel::Fsub0),
            6 => Some(Isel::Fsub1),
            7 => Some(Isel::Comp0),
            8 => Some(Isel::Comp1),
            9 => Some(Isel::Comp2),
            _ => None,
        }
    }
    #[doc = "CCP of the corresponding capture compare unit"]
    #[inline(always)]
    pub fn is_ccpx_input(&self) -> bool {
        *self == Isel::CcpxInput
    }
    #[doc = "Input pair CCPX of the capture compare unit. For CCP0 input pair is CCP1 and for CCP1 input pair is CCP0."]
    #[inline(always)]
    pub fn is_ccpx_input_pair(&self) -> bool {
        *self == Isel::CcpxInputPair
    }
    #[doc = "CCP0 of the counter"]
    #[inline(always)]
    pub fn is_ccp0_input(&self) -> bool {
        *self == Isel::Ccp0Input
    }
    #[doc = "Trigger"]
    #[inline(always)]
    pub fn is_trig_input(&self) -> bool {
        *self == Isel::TrigInput
    }
    #[doc = "XOR of CCP inputs as input source (Used in Hall input mode)."]
    #[inline(always)]
    pub fn is_ccp_xor(&self) -> bool {
        *self == Isel::CcpXor
    }
    #[doc = "subscriber 0 event as input source."]
    #[inline(always)]
    pub fn is_fsub0(&self) -> bool {
        *self == Isel::Fsub0
    }
    #[doc = "subscriber 1 event as input source."]
    #[inline(always)]
    pub fn is_fsub1(&self) -> bool {
        *self == Isel::Fsub1
    }
    #[doc = "Comparator 0 output."]
    #[inline(always)]
    pub fn is_comp0(&self) -> bool {
        *self == Isel::Comp0
    }
    #[doc = "Comparator 1 output."]
    #[inline(always)]
    pub fn is_comp1(&self) -> bool {
        *self == Isel::Comp1
    }
    #[doc = "Comparator 2 output."]
    #[inline(always)]
    pub fn is_comp2(&self) -> bool {
        *self == Isel::Comp2
    }
}
#[doc = "Field `ISEL` writer - Input Select (CCP0) This field selects the input source to the filter input. 4h-7h = Reserved"]
pub type IselW<'a, REG> = crate::FieldWriter<'a, REG, 4, Isel, crate::Safe>;
impl<'a, REG> IselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CCP of the corresponding capture compare unit"]
    #[inline(always)]
    pub fn ccpx_input(self) -> &'a mut crate::W<REG> {
        self.variant(Isel::CcpxInput)
    }
    #[doc = "Input pair CCPX of the capture compare unit. For CCP0 input pair is CCP1 and for CCP1 input pair is CCP0."]
    #[inline(always)]
    pub fn ccpx_input_pair(self) -> &'a mut crate::W<REG> {
        self.variant(Isel::CcpxInputPair)
    }
    #[doc = "CCP0 of the counter"]
    #[inline(always)]
    pub fn ccp0_input(self) -> &'a mut crate::W<REG> {
        self.variant(Isel::Ccp0Input)
    }
    #[doc = "Trigger"]
    #[inline(always)]
    pub fn trig_input(self) -> &'a mut crate::W<REG> {
        self.variant(Isel::TrigInput)
    }
    #[doc = "XOR of CCP inputs as input source (Used in Hall input mode)."]
    #[inline(always)]
    pub fn ccp_xor(self) -> &'a mut crate::W<REG> {
        self.variant(Isel::CcpXor)
    }
    #[doc = "subscriber 0 event as input source."]
    #[inline(always)]
    pub fn fsub0(self) -> &'a mut crate::W<REG> {
        self.variant(Isel::Fsub0)
    }
    #[doc = "subscriber 1 event as input source."]
    #[inline(always)]
    pub fn fsub1(self) -> &'a mut crate::W<REG> {
        self.variant(Isel::Fsub1)
    }
    #[doc = "Comparator 0 output."]
    #[inline(always)]
    pub fn comp0(self) -> &'a mut crate::W<REG> {
        self.variant(Isel::Comp0)
    }
    #[doc = "Comparator 1 output."]
    #[inline(always)]
    pub fn comp1(self) -> &'a mut crate::W<REG> {
        self.variant(Isel::Comp1)
    }
    #[doc = "Comparator 2 output."]
    #[inline(always)]
    pub fn comp2(self) -> &'a mut crate::W<REG> {
        self.variant(Isel::Comp2)
    }
}
#[doc = "Input Inversion This bit controls whether the selected input is inverted.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Inv {
    #[doc = "0: Noninverted"]
    Noinvert = 0,
    #[doc = "1: Inverted"]
    Invert = 1,
}
impl From<Inv> for bool {
    #[inline(always)]
    fn from(variant: Inv) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INV` reader - Input Inversion This bit controls whether the selected input is inverted."]
pub type InvR = crate::BitReader<Inv>;
impl InvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Inv {
        match self.bits {
            false => Inv::Noinvert,
            true => Inv::Invert,
        }
    }
    #[doc = "Noninverted"]
    #[inline(always)]
    pub fn is_noinvert(&self) -> bool {
        *self == Inv::Noinvert
    }
    #[doc = "Inverted"]
    #[inline(always)]
    pub fn is_invert(&self) -> bool {
        *self == Inv::Invert
    }
}
#[doc = "Field `INV` writer - Input Inversion This bit controls whether the selected input is inverted."]
pub type InvW<'a, REG> = crate::BitWriter<'a, REG, Inv>;
impl<'a, REG> InvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Noninverted"]
    #[inline(always)]
    pub fn noinvert(self) -> &'a mut crate::W<REG> {
        self.variant(Inv::Noinvert)
    }
    #[doc = "Inverted"]
    #[inline(always)]
    pub fn invert(self) -> &'a mut crate::W<REG> {
        self.variant(Inv::Invert)
    }
}
#[doc = "Filter Period. This field specifies the sample period for the input filter. I.e. The input is sampled for FP timer clocks during filtering.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fp {
    #[doc = "0: The division factor is 3"]
    Div3 = 0,
    #[doc = "1: The division factor is 5"]
    Div5 = 1,
    #[doc = "2: The division factor is 8"]
    Div8 = 2,
}
impl From<Fp> for u8 {
    #[inline(always)]
    fn from(variant: Fp) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fp {
    type Ux = u8;
}
impl crate::IsEnum for Fp {}
#[doc = "Field `FP` reader - Filter Period. This field specifies the sample period for the input filter. I.e. The input is sampled for FP timer clocks during filtering."]
pub type FpR = crate::FieldReader<Fp>;
impl FpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Fp> {
        match self.bits {
            0 => Some(Fp::Div3),
            1 => Some(Fp::Div5),
            2 => Some(Fp::Div8),
            _ => None,
        }
    }
    #[doc = "The division factor is 3"]
    #[inline(always)]
    pub fn is_div_3(&self) -> bool {
        *self == Fp::Div3
    }
    #[doc = "The division factor is 5"]
    #[inline(always)]
    pub fn is_div_5(&self) -> bool {
        *self == Fp::Div5
    }
    #[doc = "The division factor is 8"]
    #[inline(always)]
    pub fn is_div_8(&self) -> bool {
        *self == Fp::Div8
    }
}
#[doc = "Field `FP` writer - Filter Period. This field specifies the sample period for the input filter. I.e. The input is sampled for FP timer clocks during filtering."]
pub type FpW<'a, REG> = crate::FieldWriter<'a, REG, 2, Fp, crate::Safe>;
impl<'a, REG> FpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The division factor is 3"]
    #[inline(always)]
    pub fn div_3(self) -> &'a mut crate::W<REG> {
        self.variant(Fp::Div3)
    }
    #[doc = "The division factor is 5"]
    #[inline(always)]
    pub fn div_5(self) -> &'a mut crate::W<REG> {
        self.variant(Fp::Div5)
    }
    #[doc = "The division factor is 8"]
    #[inline(always)]
    pub fn div_8(self) -> &'a mut crate::W<REG> {
        self.variant(Fp::Div8)
    }
}
#[doc = "Consecutive Period/Voting Select This bit controls whether the input filter uses a stricter consecutive period count or majority voting.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cpv {
    #[doc = "0: Consecutive Periods The input must be at a specific logic level for the period defined by FP before it is passed to the filter output."]
    Consecutive = 0,
    #[doc = "1: Voting The filter ignores one clock of opposite logic over the filter period. I.e. Over FP samples of the input, up to 1 sample may be of an opposite logic value (glitch) without affecting the output."]
    Voting = 1,
}
impl From<Cpv> for bool {
    #[inline(always)]
    fn from(variant: Cpv) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPV` reader - Consecutive Period/Voting Select This bit controls whether the input filter uses a stricter consecutive period count or majority voting."]
pub type CpvR = crate::BitReader<Cpv>;
impl CpvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cpv {
        match self.bits {
            false => Cpv::Consecutive,
            true => Cpv::Voting,
        }
    }
    #[doc = "Consecutive Periods The input must be at a specific logic level for the period defined by FP before it is passed to the filter output."]
    #[inline(always)]
    pub fn is_consecutive(&self) -> bool {
        *self == Cpv::Consecutive
    }
    #[doc = "Voting The filter ignores one clock of opposite logic over the filter period. I.e. Over FP samples of the input, up to 1 sample may be of an opposite logic value (glitch) without affecting the output."]
    #[inline(always)]
    pub fn is_voting(&self) -> bool {
        *self == Cpv::Voting
    }
}
#[doc = "Field `CPV` writer - Consecutive Period/Voting Select This bit controls whether the input filter uses a stricter consecutive period count or majority voting."]
pub type CpvW<'a, REG> = crate::BitWriter<'a, REG, Cpv>;
impl<'a, REG> CpvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Consecutive Periods The input must be at a specific logic level for the period defined by FP before it is passed to the filter output."]
    #[inline(always)]
    pub fn consecutive(self) -> &'a mut crate::W<REG> {
        self.variant(Cpv::Consecutive)
    }
    #[doc = "Voting The filter ignores one clock of opposite logic over the filter period. I.e. Over FP samples of the input, up to 1 sample may be of an opposite logic value (glitch) without affecting the output."]
    #[inline(always)]
    pub fn voting(self) -> &'a mut crate::W<REG> {
        self.variant(Cpv::Voting)
    }
}
#[doc = "Filter Enable This bit controls whether the input is filtered by the input filter or bypasses to the edge detect.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fe {
    #[doc = "0: Bypass."]
    Disabled = 0,
    #[doc = "1: Filtered."]
    Enabled = 1,
}
impl From<Fe> for bool {
    #[inline(always)]
    fn from(variant: Fe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FE` reader - Filter Enable This bit controls whether the input is filtered by the input filter or bypasses to the edge detect."]
pub type FeR = crate::BitReader<Fe>;
impl FeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fe {
        match self.bits {
            false => Fe::Disabled,
            true => Fe::Enabled,
        }
    }
    #[doc = "Bypass."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Fe::Disabled
    }
    #[doc = "Filtered."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Fe::Enabled
    }
}
#[doc = "Field `FE` writer - Filter Enable This bit controls whether the input is filtered by the input filter or bypasses to the edge detect."]
pub type FeW<'a, REG> = crate::BitWriter<'a, REG, Fe>;
impl<'a, REG> FeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Bypass."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Fe::Disabled)
    }
    #[doc = "Filtered."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Fe::Enabled)
    }
}
impl R {
    #[doc = "Bits 0:3 - Input Select (CCP0) This field selects the input source to the filter input. 4h-7h = Reserved"]
    #[inline(always)]
    pub fn isel(&self) -> IselR {
        IselR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 7 - Input Inversion This bit controls whether the selected input is inverted."]
    #[inline(always)]
    pub fn inv(&self) -> InvR {
        InvR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Filter Period. This field specifies the sample period for the input filter. I.e. The input is sampled for FP timer clocks during filtering."]
    #[inline(always)]
    pub fn fp(&self) -> FpR {
        FpR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 11 - Consecutive Period/Voting Select This bit controls whether the input filter uses a stricter consecutive period count or majority voting."]
    #[inline(always)]
    pub fn cpv(&self) -> CpvR {
        CpvR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Filter Enable This bit controls whether the input is filtered by the input filter or bypasses to the edge detect."]
    #[inline(always)]
    pub fn fe(&self) -> FeR {
        FeR::new(((self.bits >> 12) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IFCTL_23")
            .field("isel", &self.isel())
            .field("inv", &self.inv())
            .field("fp", &self.fp())
            .field("cpv", &self.cpv())
            .field("fe", &self.fe())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - Input Select (CCP0) This field selects the input source to the filter input. 4h-7h = Reserved"]
    #[inline(always)]
    pub fn isel(&mut self) -> IselW<'_, Ifctl23Spec> {
        IselW::new(self, 0)
    }
    #[doc = "Bit 7 - Input Inversion This bit controls whether the selected input is inverted."]
    #[inline(always)]
    pub fn inv(&mut self) -> InvW<'_, Ifctl23Spec> {
        InvW::new(self, 7)
    }
    #[doc = "Bits 8:9 - Filter Period. This field specifies the sample period for the input filter. I.e. The input is sampled for FP timer clocks during filtering."]
    #[inline(always)]
    pub fn fp(&mut self) -> FpW<'_, Ifctl23Spec> {
        FpW::new(self, 8)
    }
    #[doc = "Bit 11 - Consecutive Period/Voting Select This bit controls whether the input filter uses a stricter consecutive period count or majority voting."]
    #[inline(always)]
    pub fn cpv(&mut self) -> CpvW<'_, Ifctl23Spec> {
        CpvW::new(self, 11)
    }
    #[doc = "Bit 12 - Filter Enable This bit controls whether the input is filtered by the input filter or bypasses to the edge detect."]
    #[inline(always)]
    pub fn fe(&mut self) -> FeW<'_, Ifctl23Spec> {
        FeW::new(self, 12)
    }
}
#[doc = "Input Filter Control Register 2/3\n\nYou can [`read`](crate::Reg::read) this register and get [`ifctl_23::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ifctl_23::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ifctl23Spec;
impl crate::RegisterSpec for Ifctl23Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ifctl_23::R`](R) reader structure"]
impl crate::Readable for Ifctl23Spec {}
#[doc = "`write(|w| ..)` method takes [`ifctl_23::W`](W) writer structure"]
impl crate::Writable for Ifctl23Spec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets IFCTL_23[%s] to value 0"]
impl crate::Resettable for Ifctl23Spec {}
