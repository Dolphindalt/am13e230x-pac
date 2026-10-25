#[doc = "Register `CLKDIV` reader"]
pub type R = crate::R<ClkdivSpec>;
#[doc = "Register `CLKDIV` writer"]
pub type W = crate::W<ClkdivSpec>;
#[doc = "Selects divide ratio of module clock\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ratio {
    #[doc = "0: Do not divide clock source"]
    DivBy1 = 0,
    #[doc = "1: Divide clock source by 2"]
    DivBy2 = 1,
    #[doc = "2: Divide clock source by 3"]
    DivBy3 = 2,
    #[doc = "3: Divide clock source by 4"]
    DivBy4 = 3,
    #[doc = "4: Divide clock source by 5"]
    DivBy5 = 4,
    #[doc = "5: Divide clock source by 6"]
    DivBy6 = 5,
    #[doc = "6: Divide clock source by 7"]
    DivBy7 = 6,
    #[doc = "7: Divide clock source by 8"]
    DivBy8 = 7,
}
impl From<Ratio> for u8 {
    #[inline(always)]
    fn from(variant: Ratio) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ratio {
    type Ux = u8;
}
impl crate::IsEnum for Ratio {}
#[doc = "Field `RATIO` reader - Selects divide ratio of module clock"]
pub type RatioR = crate::FieldReader<Ratio>;
impl RatioR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ratio {
        match self.bits {
            0 => Ratio::DivBy1,
            1 => Ratio::DivBy2,
            2 => Ratio::DivBy3,
            3 => Ratio::DivBy4,
            4 => Ratio::DivBy5,
            5 => Ratio::DivBy6,
            6 => Ratio::DivBy7,
            7 => Ratio::DivBy8,
            _ => unreachable!(),
        }
    }
    #[doc = "Do not divide clock source"]
    #[inline(always)]
    pub fn is_div_by_1(&self) -> bool {
        *self == Ratio::DivBy1
    }
    #[doc = "Divide clock source by 2"]
    #[inline(always)]
    pub fn is_div_by_2(&self) -> bool {
        *self == Ratio::DivBy2
    }
    #[doc = "Divide clock source by 3"]
    #[inline(always)]
    pub fn is_div_by_3(&self) -> bool {
        *self == Ratio::DivBy3
    }
    #[doc = "Divide clock source by 4"]
    #[inline(always)]
    pub fn is_div_by_4(&self) -> bool {
        *self == Ratio::DivBy4
    }
    #[doc = "Divide clock source by 5"]
    #[inline(always)]
    pub fn is_div_by_5(&self) -> bool {
        *self == Ratio::DivBy5
    }
    #[doc = "Divide clock source by 6"]
    #[inline(always)]
    pub fn is_div_by_6(&self) -> bool {
        *self == Ratio::DivBy6
    }
    #[doc = "Divide clock source by 7"]
    #[inline(always)]
    pub fn is_div_by_7(&self) -> bool {
        *self == Ratio::DivBy7
    }
    #[doc = "Divide clock source by 8"]
    #[inline(always)]
    pub fn is_div_by_8(&self) -> bool {
        *self == Ratio::DivBy8
    }
}
#[doc = "Field `RATIO` writer - Selects divide ratio of module clock"]
pub type RatioW<'a, REG> = crate::FieldWriter<'a, REG, 3, Ratio, crate::Safe>;
impl<'a, REG> RatioW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Do not divide clock source"]
    #[inline(always)]
    pub fn div_by_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ratio::DivBy1)
    }
    #[doc = "Divide clock source by 2"]
    #[inline(always)]
    pub fn div_by_2(self) -> &'a mut crate::W<REG> {
        self.variant(Ratio::DivBy2)
    }
    #[doc = "Divide clock source by 3"]
    #[inline(always)]
    pub fn div_by_3(self) -> &'a mut crate::W<REG> {
        self.variant(Ratio::DivBy3)
    }
    #[doc = "Divide clock source by 4"]
    #[inline(always)]
    pub fn div_by_4(self) -> &'a mut crate::W<REG> {
        self.variant(Ratio::DivBy4)
    }
    #[doc = "Divide clock source by 5"]
    #[inline(always)]
    pub fn div_by_5(self) -> &'a mut crate::W<REG> {
        self.variant(Ratio::DivBy5)
    }
    #[doc = "Divide clock source by 6"]
    #[inline(always)]
    pub fn div_by_6(self) -> &'a mut crate::W<REG> {
        self.variant(Ratio::DivBy6)
    }
    #[doc = "Divide clock source by 7"]
    #[inline(always)]
    pub fn div_by_7(self) -> &'a mut crate::W<REG> {
        self.variant(Ratio::DivBy7)
    }
    #[doc = "Divide clock source by 8"]
    #[inline(always)]
    pub fn div_by_8(self) -> &'a mut crate::W<REG> {
        self.variant(Ratio::DivBy8)
    }
}
impl R {
    #[doc = "Bits 0:2 - Selects divide ratio of module clock"]
    #[inline(always)]
    pub fn ratio(&self) -> RatioR {
        RatioR::new((self.bits & 7) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLKDIV")
            .field("ratio", &self.ratio())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - Selects divide ratio of module clock"]
    #[inline(always)]
    pub fn ratio(&mut self) -> RatioW<'_, ClkdivSpec> {
        RatioW::new(self, 0)
    }
}
#[doc = "Clock Divider\n\nYou can [`read`](crate::Reg::read) this register and get [`clkdiv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkdiv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkdivSpec;
impl crate::RegisterSpec for ClkdivSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clkdiv::R`](R) reader structure"]
impl crate::Readable for ClkdivSpec {}
#[doc = "`write(|w| ..)` method takes [`clkdiv::W`](W) writer structure"]
impl crate::Writable for ClkdivSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CLKDIV to value 0"]
impl crate::Resettable for ClkdivSpec {}
