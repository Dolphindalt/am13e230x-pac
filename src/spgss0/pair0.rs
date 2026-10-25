#[doc = "Register `PAIR0` reader"]
pub type R = crate::R<Pair0Spec>;
#[doc = "Register `PAIR0` writer"]
pub type W = crate::W<Pair0Spec>;
#[doc = "Enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum En {
    #[doc = "0: Does not select this clock as a source"]
    Disable = 0,
    #[doc = "1: Select this clock as a source"]
    Enable = 1,
}
impl From<En> for bool {
    #[inline(always)]
    fn from(variant: En) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EN` reader - Enable"]
pub type EnR = crate::BitReader<En>;
impl EnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> En {
        match self.bits {
            false => En::Disable,
            true => En::Enable,
        }
    }
    #[doc = "Does not select this clock as a source"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == En::Disable
    }
    #[doc = "Select this clock as a source"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == En::Enable
    }
}
#[doc = "Field `EN` writer - Enable"]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG, En>;
impl<'a, REG> EnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Does not select this clock as a source"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(En::Disable)
    }
    #[doc = "Select this clock as a source"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(En::Enable)
    }
}
#[doc = "UNICOMM instance for loopback. This instance will be treated as the controller.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ctl {
    #[doc = "0: SPG commander select for loopback."]
    Spg1 = 0,
    #[doc = "1: SPG commander select for loopback."]
    Spg2 = 1,
    #[doc = "2: SPG commander select for loopback."]
    Spg3 = 2,
    #[doc = "3: SPG commander select for loopback."]
    Spg4 = 3,
    #[doc = "4: SPG commander select for loopback."]
    Spg5 = 4,
    #[doc = "5: SPG commander select for loopback."]
    Spg6 = 5,
    #[doc = "6: SPG commander select for loopback."]
    Spg7 = 6,
    #[doc = "7: SPG commander select for loopback."]
    Spg8 = 7,
    #[doc = "8: SPG commander select for loopback."]
    Spg9 = 8,
    #[doc = "9: SPG commander select for loopback."]
    Spg10 = 9,
    #[doc = "10: SPG commander select for loopback."]
    Spg11 = 10,
    #[doc = "11: SPG commander select for loopback."]
    Spg12 = 11,
    #[doc = "12: SPG commander select for loopback."]
    Spg13 = 12,
    #[doc = "13: SPG commander select for loopback."]
    Spg14 = 13,
    #[doc = "14: SPG commander select for loopback."]
    Spg15 = 14,
    #[doc = "15: SPG commander select for loopback."]
    Spg16 = 15,
}
impl From<Ctl> for u8 {
    #[inline(always)]
    fn from(variant: Ctl) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ctl {
    type Ux = u8;
}
impl crate::IsEnum for Ctl {}
#[doc = "Field `CTL` reader - UNICOMM instance for loopback. This instance will be treated as the controller."]
pub type CtlR = crate::FieldReader<Ctl>;
impl CtlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctl {
        match self.bits {
            0 => Ctl::Spg1,
            1 => Ctl::Spg2,
            2 => Ctl::Spg3,
            3 => Ctl::Spg4,
            4 => Ctl::Spg5,
            5 => Ctl::Spg6,
            6 => Ctl::Spg7,
            7 => Ctl::Spg8,
            8 => Ctl::Spg9,
            9 => Ctl::Spg10,
            10 => Ctl::Spg11,
            11 => Ctl::Spg12,
            12 => Ctl::Spg13,
            13 => Ctl::Spg14,
            14 => Ctl::Spg15,
            15 => Ctl::Spg16,
            _ => unreachable!(),
        }
    }
    #[doc = "SPG commander select for loopback."]
    #[inline(always)]
    pub fn is_spg1(&self) -> bool {
        *self == Ctl::Spg1
    }
    #[doc = "SPG commander select for loopback."]
    #[inline(always)]
    pub fn is_spg2(&self) -> bool {
        *self == Ctl::Spg2
    }
    #[doc = "SPG commander select for loopback."]
    #[inline(always)]
    pub fn is_spg3(&self) -> bool {
        *self == Ctl::Spg3
    }
    #[doc = "SPG commander select for loopback."]
    #[inline(always)]
    pub fn is_spg4(&self) -> bool {
        *self == Ctl::Spg4
    }
    #[doc = "SPG commander select for loopback."]
    #[inline(always)]
    pub fn is_spg5(&self) -> bool {
        *self == Ctl::Spg5
    }
    #[doc = "SPG commander select for loopback."]
    #[inline(always)]
    pub fn is_spg6(&self) -> bool {
        *self == Ctl::Spg6
    }
    #[doc = "SPG commander select for loopback."]
    #[inline(always)]
    pub fn is_spg7(&self) -> bool {
        *self == Ctl::Spg7
    }
    #[doc = "SPG commander select for loopback."]
    #[inline(always)]
    pub fn is_spg8(&self) -> bool {
        *self == Ctl::Spg8
    }
    #[doc = "SPG commander select for loopback."]
    #[inline(always)]
    pub fn is_spg9(&self) -> bool {
        *self == Ctl::Spg9
    }
    #[doc = "SPG commander select for loopback."]
    #[inline(always)]
    pub fn is_spg10(&self) -> bool {
        *self == Ctl::Spg10
    }
    #[doc = "SPG commander select for loopback."]
    #[inline(always)]
    pub fn is_spg11(&self) -> bool {
        *self == Ctl::Spg11
    }
    #[doc = "SPG commander select for loopback."]
    #[inline(always)]
    pub fn is_spg12(&self) -> bool {
        *self == Ctl::Spg12
    }
    #[doc = "SPG commander select for loopback."]
    #[inline(always)]
    pub fn is_spg13(&self) -> bool {
        *self == Ctl::Spg13
    }
    #[doc = "SPG commander select for loopback."]
    #[inline(always)]
    pub fn is_spg14(&self) -> bool {
        *self == Ctl::Spg14
    }
    #[doc = "SPG commander select for loopback."]
    #[inline(always)]
    pub fn is_spg15(&self) -> bool {
        *self == Ctl::Spg15
    }
    #[doc = "SPG commander select for loopback."]
    #[inline(always)]
    pub fn is_spg16(&self) -> bool {
        *self == Ctl::Spg16
    }
}
#[doc = "Field `CTL` writer - UNICOMM instance for loopback. This instance will be treated as the controller."]
pub type CtlW<'a, REG> = crate::FieldWriter<'a, REG, 4, Ctl, crate::Safe>;
impl<'a, REG> CtlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "SPG commander select for loopback."]
    #[inline(always)]
    pub fn spg1(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl::Spg1)
    }
    #[doc = "SPG commander select for loopback."]
    #[inline(always)]
    pub fn spg2(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl::Spg2)
    }
    #[doc = "SPG commander select for loopback."]
    #[inline(always)]
    pub fn spg3(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl::Spg3)
    }
    #[doc = "SPG commander select for loopback."]
    #[inline(always)]
    pub fn spg4(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl::Spg4)
    }
    #[doc = "SPG commander select for loopback."]
    #[inline(always)]
    pub fn spg5(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl::Spg5)
    }
    #[doc = "SPG commander select for loopback."]
    #[inline(always)]
    pub fn spg6(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl::Spg6)
    }
    #[doc = "SPG commander select for loopback."]
    #[inline(always)]
    pub fn spg7(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl::Spg7)
    }
    #[doc = "SPG commander select for loopback."]
    #[inline(always)]
    pub fn spg8(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl::Spg8)
    }
    #[doc = "SPG commander select for loopback."]
    #[inline(always)]
    pub fn spg9(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl::Spg9)
    }
    #[doc = "SPG commander select for loopback."]
    #[inline(always)]
    pub fn spg10(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl::Spg10)
    }
    #[doc = "SPG commander select for loopback."]
    #[inline(always)]
    pub fn spg11(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl::Spg11)
    }
    #[doc = "SPG commander select for loopback."]
    #[inline(always)]
    pub fn spg12(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl::Spg12)
    }
    #[doc = "SPG commander select for loopback."]
    #[inline(always)]
    pub fn spg13(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl::Spg13)
    }
    #[doc = "SPG commander select for loopback."]
    #[inline(always)]
    pub fn spg14(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl::Spg14)
    }
    #[doc = "SPG commander select for loopback."]
    #[inline(always)]
    pub fn spg15(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl::Spg15)
    }
    #[doc = "SPG commander select for loopback."]
    #[inline(always)]
    pub fn spg16(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl::Spg16)
    }
}
#[doc = "UNICOMM instance for loopback. This instance will be treated as the target\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Target {
    #[doc = "0: SPG responder select for loopback."]
    Spg1 = 0,
    #[doc = "1: SPG responder select for loopback."]
    Spg2 = 1,
    #[doc = "2: SPG responder select for loopback."]
    Spg3 = 2,
    #[doc = "3: SPG responder select for loopback."]
    Spg4 = 3,
    #[doc = "4: SPG responder select for loopback."]
    Spg5 = 4,
    #[doc = "5: SPG responder select for loopback."]
    Spg6 = 5,
    #[doc = "6: SPG responder select for loopback."]
    Spg7 = 6,
    #[doc = "7: SPG responder select for loopback."]
    Spg8 = 7,
    #[doc = "8: SPG responder select for loopback."]
    Spg9 = 8,
    #[doc = "9: SPG responder select for loopback."]
    Spg10 = 9,
    #[doc = "10: SPG responder select for loopback."]
    Spg11 = 10,
    #[doc = "11: SPG responder select for loopback."]
    Spg12 = 11,
    #[doc = "12: SPG responder select for loopback."]
    Spg13 = 12,
    #[doc = "13: SPG responder select for loopback."]
    Spg14 = 13,
    #[doc = "14: SPG responder select for loopback."]
    Spg15 = 14,
    #[doc = "15: SPG responder select for loopback."]
    Spg16 = 15,
}
impl From<Target> for u8 {
    #[inline(always)]
    fn from(variant: Target) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Target {
    type Ux = u8;
}
impl crate::IsEnum for Target {}
#[doc = "Field `TARGET` reader - UNICOMM instance for loopback. This instance will be treated as the target"]
pub type TargetR = crate::FieldReader<Target>;
impl TargetR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Target {
        match self.bits {
            0 => Target::Spg1,
            1 => Target::Spg2,
            2 => Target::Spg3,
            3 => Target::Spg4,
            4 => Target::Spg5,
            5 => Target::Spg6,
            6 => Target::Spg7,
            7 => Target::Spg8,
            8 => Target::Spg9,
            9 => Target::Spg10,
            10 => Target::Spg11,
            11 => Target::Spg12,
            12 => Target::Spg13,
            13 => Target::Spg14,
            14 => Target::Spg15,
            15 => Target::Spg16,
            _ => unreachable!(),
        }
    }
    #[doc = "SPG responder select for loopback."]
    #[inline(always)]
    pub fn is_spg1(&self) -> bool {
        *self == Target::Spg1
    }
    #[doc = "SPG responder select for loopback."]
    #[inline(always)]
    pub fn is_spg2(&self) -> bool {
        *self == Target::Spg2
    }
    #[doc = "SPG responder select for loopback."]
    #[inline(always)]
    pub fn is_spg3(&self) -> bool {
        *self == Target::Spg3
    }
    #[doc = "SPG responder select for loopback."]
    #[inline(always)]
    pub fn is_spg4(&self) -> bool {
        *self == Target::Spg4
    }
    #[doc = "SPG responder select for loopback."]
    #[inline(always)]
    pub fn is_spg5(&self) -> bool {
        *self == Target::Spg5
    }
    #[doc = "SPG responder select for loopback."]
    #[inline(always)]
    pub fn is_spg6(&self) -> bool {
        *self == Target::Spg6
    }
    #[doc = "SPG responder select for loopback."]
    #[inline(always)]
    pub fn is_spg7(&self) -> bool {
        *self == Target::Spg7
    }
    #[doc = "SPG responder select for loopback."]
    #[inline(always)]
    pub fn is_spg8(&self) -> bool {
        *self == Target::Spg8
    }
    #[doc = "SPG responder select for loopback."]
    #[inline(always)]
    pub fn is_spg9(&self) -> bool {
        *self == Target::Spg9
    }
    #[doc = "SPG responder select for loopback."]
    #[inline(always)]
    pub fn is_spg10(&self) -> bool {
        *self == Target::Spg10
    }
    #[doc = "SPG responder select for loopback."]
    #[inline(always)]
    pub fn is_spg11(&self) -> bool {
        *self == Target::Spg11
    }
    #[doc = "SPG responder select for loopback."]
    #[inline(always)]
    pub fn is_spg12(&self) -> bool {
        *self == Target::Spg12
    }
    #[doc = "SPG responder select for loopback."]
    #[inline(always)]
    pub fn is_spg13(&self) -> bool {
        *self == Target::Spg13
    }
    #[doc = "SPG responder select for loopback."]
    #[inline(always)]
    pub fn is_spg14(&self) -> bool {
        *self == Target::Spg14
    }
    #[doc = "SPG responder select for loopback."]
    #[inline(always)]
    pub fn is_spg15(&self) -> bool {
        *self == Target::Spg15
    }
    #[doc = "SPG responder select for loopback."]
    #[inline(always)]
    pub fn is_spg16(&self) -> bool {
        *self == Target::Spg16
    }
}
#[doc = "Field `TARGET` writer - UNICOMM instance for loopback. This instance will be treated as the target"]
pub type TargetW<'a, REG> = crate::FieldWriter<'a, REG, 4, Target, crate::Safe>;
impl<'a, REG> TargetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "SPG responder select for loopback."]
    #[inline(always)]
    pub fn spg1(self) -> &'a mut crate::W<REG> {
        self.variant(Target::Spg1)
    }
    #[doc = "SPG responder select for loopback."]
    #[inline(always)]
    pub fn spg2(self) -> &'a mut crate::W<REG> {
        self.variant(Target::Spg2)
    }
    #[doc = "SPG responder select for loopback."]
    #[inline(always)]
    pub fn spg3(self) -> &'a mut crate::W<REG> {
        self.variant(Target::Spg3)
    }
    #[doc = "SPG responder select for loopback."]
    #[inline(always)]
    pub fn spg4(self) -> &'a mut crate::W<REG> {
        self.variant(Target::Spg4)
    }
    #[doc = "SPG responder select for loopback."]
    #[inline(always)]
    pub fn spg5(self) -> &'a mut crate::W<REG> {
        self.variant(Target::Spg5)
    }
    #[doc = "SPG responder select for loopback."]
    #[inline(always)]
    pub fn spg6(self) -> &'a mut crate::W<REG> {
        self.variant(Target::Spg6)
    }
    #[doc = "SPG responder select for loopback."]
    #[inline(always)]
    pub fn spg7(self) -> &'a mut crate::W<REG> {
        self.variant(Target::Spg7)
    }
    #[doc = "SPG responder select for loopback."]
    #[inline(always)]
    pub fn spg8(self) -> &'a mut crate::W<REG> {
        self.variant(Target::Spg8)
    }
    #[doc = "SPG responder select for loopback."]
    #[inline(always)]
    pub fn spg9(self) -> &'a mut crate::W<REG> {
        self.variant(Target::Spg9)
    }
    #[doc = "SPG responder select for loopback."]
    #[inline(always)]
    pub fn spg10(self) -> &'a mut crate::W<REG> {
        self.variant(Target::Spg10)
    }
    #[doc = "SPG responder select for loopback."]
    #[inline(always)]
    pub fn spg11(self) -> &'a mut crate::W<REG> {
        self.variant(Target::Spg11)
    }
    #[doc = "SPG responder select for loopback."]
    #[inline(always)]
    pub fn spg12(self) -> &'a mut crate::W<REG> {
        self.variant(Target::Spg12)
    }
    #[doc = "SPG responder select for loopback."]
    #[inline(always)]
    pub fn spg13(self) -> &'a mut crate::W<REG> {
        self.variant(Target::Spg13)
    }
    #[doc = "SPG responder select for loopback."]
    #[inline(always)]
    pub fn spg14(self) -> &'a mut crate::W<REG> {
        self.variant(Target::Spg14)
    }
    #[doc = "SPG responder select for loopback."]
    #[inline(always)]
    pub fn spg15(self) -> &'a mut crate::W<REG> {
        self.variant(Target::Spg15)
    }
    #[doc = "SPG responder select for loopback."]
    #[inline(always)]
    pub fn spg16(self) -> &'a mut crate::W<REG> {
        self.variant(Target::Spg16)
    }
}
impl R {
    #[doc = "Bit 0 - Enable"]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:11 - UNICOMM instance for loopback. This instance will be treated as the controller."]
    #[inline(always)]
    pub fn ctl(&self) -> CtlR {
        CtlR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - UNICOMM instance for loopback. This instance will be treated as the target"]
    #[inline(always)]
    pub fn target(&self) -> TargetR {
        TargetR::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PAIR0")
            .field("ctl", &self.ctl())
            .field("target", &self.target())
            .field("en", &self.en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Enable"]
    #[inline(always)]
    pub fn en(&mut self) -> EnW<'_, Pair0Spec> {
        EnW::new(self, 0)
    }
    #[doc = "Bits 8:11 - UNICOMM instance for loopback. This instance will be treated as the controller."]
    #[inline(always)]
    pub fn ctl(&mut self) -> CtlW<'_, Pair0Spec> {
        CtlW::new(self, 8)
    }
    #[doc = "Bits 12:15 - UNICOMM instance for loopback. This instance will be treated as the target"]
    #[inline(always)]
    pub fn target(&mut self) -> TargetW<'_, Pair0Spec> {
        TargetW::new(self, 12)
    }
}
#[doc = "Pairing of I2C for MULTI-MASTER/SMBUS applications\n\nYou can [`read`](crate::Reg::read) this register and get [`pair0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pair0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pair0Spec;
impl crate::RegisterSpec for Pair0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pair0::R`](R) reader structure"]
impl crate::Readable for Pair0Spec {}
#[doc = "`write(|w| ..)` method takes [`pair0::W`](W) writer structure"]
impl crate::Writable for Pair0Spec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets PAIR0 to value 0"]
impl crate::Resettable for Pair0Spec {}
