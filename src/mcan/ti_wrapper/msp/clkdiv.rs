#[doc = "Register `CLKDIV` reader"]
pub type R = crate::R<ClkdivSpec>;
#[doc = "Register `CLKDIV` writer"]
pub type W = crate::W<ClkdivSpec>;
#[doc = "MCAN functional clock divide settings\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ratio {
    #[doc = "0: Divides input clock by 1"]
    DivBy1_ = 0,
    #[doc = "1: Divides input clock by 2"]
    DivBy2_ = 1,
    #[doc = "2: Divides input clock by 4"]
    DivBy4_ = 2,
    #[doc = "3: Divides input clock by 1"]
    DivBy1_1 = 3,
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
#[doc = "Field `RATIO` reader - MCAN functional clock divide settings"]
pub type RatioR = crate::FieldReader<Ratio>;
impl RatioR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ratio {
        match self.bits {
            0 => Ratio::DivBy1_,
            1 => Ratio::DivBy2_,
            2 => Ratio::DivBy4_,
            3 => Ratio::DivBy1_1,
            _ => unreachable!(),
        }
    }
    #[doc = "Divides input clock by 1"]
    #[inline(always)]
    pub fn is_div_by_1_(&self) -> bool {
        *self == Ratio::DivBy1_
    }
    #[doc = "Divides input clock by 2"]
    #[inline(always)]
    pub fn is_div_by_2_(&self) -> bool {
        *self == Ratio::DivBy2_
    }
    #[doc = "Divides input clock by 4"]
    #[inline(always)]
    pub fn is_div_by_4_(&self) -> bool {
        *self == Ratio::DivBy4_
    }
    #[doc = "Divides input clock by 1"]
    #[inline(always)]
    pub fn is_div_by_1_1(&self) -> bool {
        *self == Ratio::DivBy1_1
    }
}
#[doc = "Field `RATIO` writer - MCAN functional clock divide settings"]
pub type RatioW<'a, REG> = crate::FieldWriter<'a, REG, 2, Ratio, crate::Safe>;
impl<'a, REG> RatioW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Divides input clock by 1"]
    #[inline(always)]
    pub fn div_by_1_(self) -> &'a mut crate::W<REG> {
        self.variant(Ratio::DivBy1_)
    }
    #[doc = "Divides input clock by 2"]
    #[inline(always)]
    pub fn div_by_2_(self) -> &'a mut crate::W<REG> {
        self.variant(Ratio::DivBy2_)
    }
    #[doc = "Divides input clock by 4"]
    #[inline(always)]
    pub fn div_by_4_(self) -> &'a mut crate::W<REG> {
        self.variant(Ratio::DivBy4_)
    }
    #[doc = "Divides input clock by 1"]
    #[inline(always)]
    pub fn div_by_1_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ratio::DivBy1_1)
    }
}
impl R {
    #[doc = "Bits 0:1 - MCAN functional clock divide settings"]
    #[inline(always)]
    pub fn ratio(&self) -> RatioR {
        RatioR::new((self.bits & 3) as u8)
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
    #[doc = "Bits 0:1 - MCAN functional clock divide settings"]
    #[inline(always)]
    pub fn ratio(&mut self) -> RatioW<'_, ClkdivSpec> {
        RatioW::new(self, 0)
    }
}
#[doc = "Clock divider\n\nYou can [`read`](crate::Reg::read) this register and get [`clkdiv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkdiv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
