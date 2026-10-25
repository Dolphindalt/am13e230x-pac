#[doc = "Register `CCPD` reader"]
pub type R = crate::R<CcpdSpec>;
#[doc = "Register `CCPD` writer"]
pub type W = crate::W<CcpdSpec>;
#[doc = "CCP0 direction\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum C0ccp0 {
    #[doc = "0: Input"]
    Input = 0,
    #[doc = "1: Output"]
    Output = 1,
}
impl From<C0ccp0> for bool {
    #[inline(always)]
    fn from(variant: C0ccp0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `C0CCP0` reader - CCP0 direction"]
pub type C0ccp0R = crate::BitReader<C0ccp0>;
impl C0ccp0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> C0ccp0 {
        match self.bits {
            false => C0ccp0::Input,
            true => C0ccp0::Output,
        }
    }
    #[doc = "Input"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == C0ccp0::Input
    }
    #[doc = "Output"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == C0ccp0::Output
    }
}
#[doc = "Field `C0CCP0` writer - CCP0 direction"]
pub type C0ccp0W<'a, REG> = crate::BitWriter<'a, REG, C0ccp0>;
impl<'a, REG> C0ccp0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(C0ccp0::Input)
    }
    #[doc = "Output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(C0ccp0::Output)
    }
}
#[doc = "CCP1 direction\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum C0ccp1 {
    #[doc = "0: Input"]
    Input = 0,
    #[doc = "1: Output"]
    Output = 1,
}
impl From<C0ccp1> for bool {
    #[inline(always)]
    fn from(variant: C0ccp1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `C0CCP1` reader - CCP1 direction"]
pub type C0ccp1R = crate::BitReader<C0ccp1>;
impl C0ccp1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> C0ccp1 {
        match self.bits {
            false => C0ccp1::Input,
            true => C0ccp1::Output,
        }
    }
    #[doc = "Input"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == C0ccp1::Input
    }
    #[doc = "Output"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == C0ccp1::Output
    }
}
#[doc = "Field `C0CCP1` writer - CCP1 direction"]
pub type C0ccp1W<'a, REG> = crate::BitWriter<'a, REG, C0ccp1>;
impl<'a, REG> C0ccp1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(C0ccp1::Input)
    }
    #[doc = "Output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(C0ccp1::Output)
    }
}
#[doc = "CCP2 direction\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum C0ccp2 {
    #[doc = "0: input"]
    Input = 0,
    #[doc = "1: Output"]
    Output = 1,
}
impl From<C0ccp2> for bool {
    #[inline(always)]
    fn from(variant: C0ccp2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `C0CCP2` reader - CCP2 direction"]
pub type C0ccp2R = crate::BitReader<C0ccp2>;
impl C0ccp2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> C0ccp2 {
        match self.bits {
            false => C0ccp2::Input,
            true => C0ccp2::Output,
        }
    }
    #[doc = "input"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == C0ccp2::Input
    }
    #[doc = "Output"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == C0ccp2::Output
    }
}
#[doc = "Field `C0CCP2` writer - CCP2 direction"]
pub type C0ccp2W<'a, REG> = crate::BitWriter<'a, REG, C0ccp2>;
impl<'a, REG> C0ccp2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(C0ccp2::Input)
    }
    #[doc = "Output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(C0ccp2::Output)
    }
}
impl R {
    #[doc = "Bit 0 - CCP0 direction"]
    #[inline(always)]
    pub fn c0ccp0(&self) -> C0ccp0R {
        C0ccp0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CCP1 direction"]
    #[inline(always)]
    pub fn c0ccp1(&self) -> C0ccp1R {
        C0ccp1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CCP2 direction"]
    #[inline(always)]
    pub fn c0ccp2(&self) -> C0ccp2R {
        C0ccp2R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCPD")
            .field("c0ccp0", &self.c0ccp0())
            .field("c0ccp1", &self.c0ccp1())
            .field("c0ccp2", &self.c0ccp2())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - CCP0 direction"]
    #[inline(always)]
    pub fn c0ccp0(&mut self) -> C0ccp0W<'_, CcpdSpec> {
        C0ccp0W::new(self, 0)
    }
    #[doc = "Bit 1 - CCP1 direction"]
    #[inline(always)]
    pub fn c0ccp1(&mut self) -> C0ccp1W<'_, CcpdSpec> {
        C0ccp1W::new(self, 1)
    }
    #[doc = "Bit 2 - CCP2 direction"]
    #[inline(always)]
    pub fn c0ccp2(&mut self) -> C0ccp2W<'_, CcpdSpec> {
        C0ccp2W::new(self, 2)
    }
}
#[doc = "CCP Direction\n\nYou can [`read`](crate::Reg::read) this register and get [`ccpd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccpd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CcpdSpec;
impl crate::RegisterSpec for CcpdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccpd::R`](R) reader structure"]
impl crate::Readable for CcpdSpec {}
#[doc = "`write(|w| ..)` method takes [`ccpd::W`](W) writer structure"]
impl crate::Writable for CcpdSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CCPD to value 0"]
impl crate::Resettable for CcpdSpec {}
