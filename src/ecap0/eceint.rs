#[doc = "Register `ECEINT` reader"]
pub type R = crate::R<EceintSpec>;
#[doc = "Register `ECEINT` writer"]
pub type W = crate::W<EceintSpec>;
#[doc = "Capture Event 1 Interrupt Enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cevt1 {
    #[doc = "0: Disable Capture Event 1 as an Interrupt source"]
    Disab = 0,
    #[doc = "1: Enable Capture Event 1 as an Interrupt source"]
    Enab = 1,
}
impl From<Cevt1> for bool {
    #[inline(always)]
    fn from(variant: Cevt1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CEVT1` reader - Capture Event 1 Interrupt Enable"]
pub type Cevt1R = crate::BitReader<Cevt1>;
impl Cevt1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cevt1 {
        match self.bits {
            false => Cevt1::Disab,
            true => Cevt1::Enab,
        }
    }
    #[doc = "Disable Capture Event 1 as an Interrupt source"]
    #[inline(always)]
    pub fn is_disab(&self) -> bool {
        *self == Cevt1::Disab
    }
    #[doc = "Enable Capture Event 1 as an Interrupt source"]
    #[inline(always)]
    pub fn is_enab(&self) -> bool {
        *self == Cevt1::Enab
    }
}
#[doc = "Field `CEVT1` writer - Capture Event 1 Interrupt Enable"]
pub type Cevt1W<'a, REG> = crate::BitWriter<'a, REG, Cevt1>;
impl<'a, REG> Cevt1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Capture Event 1 as an Interrupt source"]
    #[inline(always)]
    pub fn disab(self) -> &'a mut crate::W<REG> {
        self.variant(Cevt1::Disab)
    }
    #[doc = "Enable Capture Event 1 as an Interrupt source"]
    #[inline(always)]
    pub fn enab(self) -> &'a mut crate::W<REG> {
        self.variant(Cevt1::Enab)
    }
}
#[doc = "Capture Event 2 Interrupt Enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cevt2 {
    #[doc = "0: Disable Capture Event 2 as an Interrupt source"]
    Disab = 0,
    #[doc = "1: Enable Capture Event 2 as an Interrupt source"]
    Enab = 1,
}
impl From<Cevt2> for bool {
    #[inline(always)]
    fn from(variant: Cevt2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CEVT2` reader - Capture Event 2 Interrupt Enable"]
pub type Cevt2R = crate::BitReader<Cevt2>;
impl Cevt2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cevt2 {
        match self.bits {
            false => Cevt2::Disab,
            true => Cevt2::Enab,
        }
    }
    #[doc = "Disable Capture Event 2 as an Interrupt source"]
    #[inline(always)]
    pub fn is_disab(&self) -> bool {
        *self == Cevt2::Disab
    }
    #[doc = "Enable Capture Event 2 as an Interrupt source"]
    #[inline(always)]
    pub fn is_enab(&self) -> bool {
        *self == Cevt2::Enab
    }
}
#[doc = "Field `CEVT2` writer - Capture Event 2 Interrupt Enable"]
pub type Cevt2W<'a, REG> = crate::BitWriter<'a, REG, Cevt2>;
impl<'a, REG> Cevt2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Capture Event 2 as an Interrupt source"]
    #[inline(always)]
    pub fn disab(self) -> &'a mut crate::W<REG> {
        self.variant(Cevt2::Disab)
    }
    #[doc = "Enable Capture Event 2 as an Interrupt source"]
    #[inline(always)]
    pub fn enab(self) -> &'a mut crate::W<REG> {
        self.variant(Cevt2::Enab)
    }
}
#[doc = "Capture Event 3 Interrupt Enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cevt3 {
    #[doc = "0: Disable Capture Event 3 as an Interrupt source"]
    Disab = 0,
    #[doc = "1: Enable Capture Event 3 as an Interrupt source"]
    Enab = 1,
}
impl From<Cevt3> for bool {
    #[inline(always)]
    fn from(variant: Cevt3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CEVT3` reader - Capture Event 3 Interrupt Enable"]
pub type Cevt3R = crate::BitReader<Cevt3>;
impl Cevt3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cevt3 {
        match self.bits {
            false => Cevt3::Disab,
            true => Cevt3::Enab,
        }
    }
    #[doc = "Disable Capture Event 3 as an Interrupt source"]
    #[inline(always)]
    pub fn is_disab(&self) -> bool {
        *self == Cevt3::Disab
    }
    #[doc = "Enable Capture Event 3 as an Interrupt source"]
    #[inline(always)]
    pub fn is_enab(&self) -> bool {
        *self == Cevt3::Enab
    }
}
#[doc = "Field `CEVT3` writer - Capture Event 3 Interrupt Enable"]
pub type Cevt3W<'a, REG> = crate::BitWriter<'a, REG, Cevt3>;
impl<'a, REG> Cevt3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Capture Event 3 as an Interrupt source"]
    #[inline(always)]
    pub fn disab(self) -> &'a mut crate::W<REG> {
        self.variant(Cevt3::Disab)
    }
    #[doc = "Enable Capture Event 3 as an Interrupt source"]
    #[inline(always)]
    pub fn enab(self) -> &'a mut crate::W<REG> {
        self.variant(Cevt3::Enab)
    }
}
#[doc = "Capture Event 4 Interrupt Enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cevt4 {
    #[doc = "0: Disable Capture Event 4 as an Interrupt source"]
    Disab = 0,
    #[doc = "1: Capture Event 4 Interrupt Enable"]
    Enab = 1,
}
impl From<Cevt4> for bool {
    #[inline(always)]
    fn from(variant: Cevt4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CEVT4` reader - Capture Event 4 Interrupt Enable"]
pub type Cevt4R = crate::BitReader<Cevt4>;
impl Cevt4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cevt4 {
        match self.bits {
            false => Cevt4::Disab,
            true => Cevt4::Enab,
        }
    }
    #[doc = "Disable Capture Event 4 as an Interrupt source"]
    #[inline(always)]
    pub fn is_disab(&self) -> bool {
        *self == Cevt4::Disab
    }
    #[doc = "Capture Event 4 Interrupt Enable"]
    #[inline(always)]
    pub fn is_enab(&self) -> bool {
        *self == Cevt4::Enab
    }
}
#[doc = "Field `CEVT4` writer - Capture Event 4 Interrupt Enable"]
pub type Cevt4W<'a, REG> = crate::BitWriter<'a, REG, Cevt4>;
impl<'a, REG> Cevt4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Capture Event 4 as an Interrupt source"]
    #[inline(always)]
    pub fn disab(self) -> &'a mut crate::W<REG> {
        self.variant(Cevt4::Disab)
    }
    #[doc = "Capture Event 4 Interrupt Enable"]
    #[inline(always)]
    pub fn enab(self) -> &'a mut crate::W<REG> {
        self.variant(Cevt4::Enab)
    }
}
#[doc = "Counter Overflow Interrupt Enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctrovf {
    #[doc = "0: Disabled counter Overflow as an Interrupt source"]
    Disab = 0,
    #[doc = "1: Enable counter Overflow as an Interrupt source"]
    Enab = 1,
}
impl From<Ctrovf> for bool {
    #[inline(always)]
    fn from(variant: Ctrovf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTROVF` reader - Counter Overflow Interrupt Enable"]
pub type CtrovfR = crate::BitReader<Ctrovf>;
impl CtrovfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctrovf {
        match self.bits {
            false => Ctrovf::Disab,
            true => Ctrovf::Enab,
        }
    }
    #[doc = "Disabled counter Overflow as an Interrupt source"]
    #[inline(always)]
    pub fn is_disab(&self) -> bool {
        *self == Ctrovf::Disab
    }
    #[doc = "Enable counter Overflow as an Interrupt source"]
    #[inline(always)]
    pub fn is_enab(&self) -> bool {
        *self == Ctrovf::Enab
    }
}
#[doc = "Field `CTROVF` writer - Counter Overflow Interrupt Enable"]
pub type CtrovfW<'a, REG> = crate::BitWriter<'a, REG, Ctrovf>;
impl<'a, REG> CtrovfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled counter Overflow as an Interrupt source"]
    #[inline(always)]
    pub fn disab(self) -> &'a mut crate::W<REG> {
        self.variant(Ctrovf::Disab)
    }
    #[doc = "Enable counter Overflow as an Interrupt source"]
    #[inline(always)]
    pub fn enab(self) -> &'a mut crate::W<REG> {
        self.variant(Ctrovf::Enab)
    }
}
#[doc = "Period Equal Interrupt Enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CtrEqPrd {
    #[doc = "0: Disable Period Equal as an Interrupt source"]
    Disab = 0,
    #[doc = "1: Enable Period Equal as an Interrupt source"]
    Enab = 1,
}
impl From<CtrEqPrd> for bool {
    #[inline(always)]
    fn from(variant: CtrEqPrd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTR_EQ_PRD` reader - Period Equal Interrupt Enable"]
pub type CtrEqPrdR = crate::BitReader<CtrEqPrd>;
impl CtrEqPrdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CtrEqPrd {
        match self.bits {
            false => CtrEqPrd::Disab,
            true => CtrEqPrd::Enab,
        }
    }
    #[doc = "Disable Period Equal as an Interrupt source"]
    #[inline(always)]
    pub fn is_disab(&self) -> bool {
        *self == CtrEqPrd::Disab
    }
    #[doc = "Enable Period Equal as an Interrupt source"]
    #[inline(always)]
    pub fn is_enab(&self) -> bool {
        *self == CtrEqPrd::Enab
    }
}
#[doc = "Field `CTR_EQ_PRD` writer - Period Equal Interrupt Enable"]
pub type CtrEqPrdW<'a, REG> = crate::BitWriter<'a, REG, CtrEqPrd>;
impl<'a, REG> CtrEqPrdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Period Equal as an Interrupt source"]
    #[inline(always)]
    pub fn disab(self) -> &'a mut crate::W<REG> {
        self.variant(CtrEqPrd::Disab)
    }
    #[doc = "Enable Period Equal as an Interrupt source"]
    #[inline(always)]
    pub fn enab(self) -> &'a mut crate::W<REG> {
        self.variant(CtrEqPrd::Enab)
    }
}
#[doc = "Compare Equal Interrupt Enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CtrEqCmp {
    #[doc = "0: Disable Compare Equal as an Interrupt source"]
    Disab = 0,
    #[doc = "1: Enable Compare Equal as an Interrupt source"]
    Enab = 1,
}
impl From<CtrEqCmp> for bool {
    #[inline(always)]
    fn from(variant: CtrEqCmp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTR_EQ_CMP` reader - Compare Equal Interrupt Enable"]
pub type CtrEqCmpR = crate::BitReader<CtrEqCmp>;
impl CtrEqCmpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CtrEqCmp {
        match self.bits {
            false => CtrEqCmp::Disab,
            true => CtrEqCmp::Enab,
        }
    }
    #[doc = "Disable Compare Equal as an Interrupt source"]
    #[inline(always)]
    pub fn is_disab(&self) -> bool {
        *self == CtrEqCmp::Disab
    }
    #[doc = "Enable Compare Equal as an Interrupt source"]
    #[inline(always)]
    pub fn is_enab(&self) -> bool {
        *self == CtrEqCmp::Enab
    }
}
#[doc = "Field `CTR_EQ_CMP` writer - Compare Equal Interrupt Enable"]
pub type CtrEqCmpW<'a, REG> = crate::BitWriter<'a, REG, CtrEqCmp>;
impl<'a, REG> CtrEqCmpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Compare Equal as an Interrupt source"]
    #[inline(always)]
    pub fn disab(self) -> &'a mut crate::W<REG> {
        self.variant(CtrEqCmp::Disab)
    }
    #[doc = "Enable Compare Equal as an Interrupt source"]
    #[inline(always)]
    pub fn enab(self) -> &'a mut crate::W<REG> {
        self.variant(CtrEqCmp::Enab)
    }
}
impl R {
    #[doc = "Bit 1 - Capture Event 1 Interrupt Enable"]
    #[inline(always)]
    pub fn cevt1(&self) -> Cevt1R {
        Cevt1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Capture Event 2 Interrupt Enable"]
    #[inline(always)]
    pub fn cevt2(&self) -> Cevt2R {
        Cevt2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Capture Event 3 Interrupt Enable"]
    #[inline(always)]
    pub fn cevt3(&self) -> Cevt3R {
        Cevt3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Capture Event 4 Interrupt Enable"]
    #[inline(always)]
    pub fn cevt4(&self) -> Cevt4R {
        Cevt4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Counter Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn ctrovf(&self) -> CtrovfR {
        CtrovfR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Period Equal Interrupt Enable"]
    #[inline(always)]
    pub fn ctr_eq_prd(&self) -> CtrEqPrdR {
        CtrEqPrdR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Compare Equal Interrupt Enable"]
    #[inline(always)]
    pub fn ctr_eq_cmp(&self) -> CtrEqCmpR {
        CtrEqCmpR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ECEINT")
            .field("cevt1", &self.cevt1())
            .field("cevt2", &self.cevt2())
            .field("cevt3", &self.cevt3())
            .field("cevt4", &self.cevt4())
            .field("ctrovf", &self.ctrovf())
            .field("ctr_eq_prd", &self.ctr_eq_prd())
            .field("ctr_eq_cmp", &self.ctr_eq_cmp())
            .finish()
    }
}
impl W {
    #[doc = "Bit 1 - Capture Event 1 Interrupt Enable"]
    #[inline(always)]
    pub fn cevt1(&mut self) -> Cevt1W<'_, EceintSpec> {
        Cevt1W::new(self, 1)
    }
    #[doc = "Bit 2 - Capture Event 2 Interrupt Enable"]
    #[inline(always)]
    pub fn cevt2(&mut self) -> Cevt2W<'_, EceintSpec> {
        Cevt2W::new(self, 2)
    }
    #[doc = "Bit 3 - Capture Event 3 Interrupt Enable"]
    #[inline(always)]
    pub fn cevt3(&mut self) -> Cevt3W<'_, EceintSpec> {
        Cevt3W::new(self, 3)
    }
    #[doc = "Bit 4 - Capture Event 4 Interrupt Enable"]
    #[inline(always)]
    pub fn cevt4(&mut self) -> Cevt4W<'_, EceintSpec> {
        Cevt4W::new(self, 4)
    }
    #[doc = "Bit 5 - Counter Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn ctrovf(&mut self) -> CtrovfW<'_, EceintSpec> {
        CtrovfW::new(self, 5)
    }
    #[doc = "Bit 6 - Period Equal Interrupt Enable"]
    #[inline(always)]
    pub fn ctr_eq_prd(&mut self) -> CtrEqPrdW<'_, EceintSpec> {
        CtrEqPrdW::new(self, 6)
    }
    #[doc = "Bit 7 - Compare Equal Interrupt Enable"]
    #[inline(always)]
    pub fn ctr_eq_cmp(&mut self) -> CtrEqCmpW<'_, EceintSpec> {
        CtrEqCmpW::new(self, 7)
    }
}
#[doc = "Capture Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`eceint::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eceint::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EceintSpec;
impl crate::RegisterSpec for EceintSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`eceint::R`](R) reader structure"]
impl crate::Readable for EceintSpec {}
#[doc = "`write(|w| ..)` method takes [`eceint::W`](W) writer structure"]
impl crate::Writable for EceintSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets ECEINT to value 0"]
impl crate::Resettable for EceintSpec {}
