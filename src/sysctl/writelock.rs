#[doc = "Register `WRITELOCK` reader"]
pub type R = crate::R<WritelockSpec>;
#[doc = "Register `WRITELOCK` writer"]
pub type W = crate::W<WritelockSpec>;
#[doc = "ACTIVE controls whether critical SYSCTL registers are write protected or not.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Active {
    #[doc = "0: Allow writes to lockable registers"]
    Disable = 0,
    #[doc = "1: Disallow writes to lockable registers"]
    Enable = 1,
}
impl From<Active> for bool {
    #[inline(always)]
    fn from(variant: Active) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACTIVE` reader - ACTIVE controls whether critical SYSCTL registers are write protected or not."]
pub type ActiveR = crate::BitReader<Active>;
impl ActiveR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Active {
        match self.bits {
            false => Active::Disable,
            true => Active::Enable,
        }
    }
    #[doc = "Allow writes to lockable registers"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Active::Disable
    }
    #[doc = "Disallow writes to lockable registers"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Active::Enable
    }
}
#[doc = "Field `ACTIVE` writer - ACTIVE controls whether critical SYSCTL registers are write protected or not."]
pub type ActiveW<'a, REG> = crate::BitWriter<'a, REG, Active>;
impl<'a, REG> ActiveW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Allow writes to lockable registers"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Active::Disable)
    }
    #[doc = "Disallow writes to lockable registers"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Active::Enable)
    }
}
impl R {
    #[doc = "Bit 0 - ACTIVE controls whether critical SYSCTL registers are write protected or not."]
    #[inline(always)]
    pub fn active(&self) -> ActiveR {
        ActiveR::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WRITELOCK")
            .field("active", &self.active())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - ACTIVE controls whether critical SYSCTL registers are write protected or not."]
    #[inline(always)]
    pub fn active(&mut self) -> ActiveW<'_, WritelockSpec> {
        ActiveW::new(self, 0)
    }
}
#[doc = "SYSCTL register write lockout\n\nYou can [`read`](crate::Reg::read) this register and get [`writelock::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`writelock::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WritelockSpec;
impl crate::RegisterSpec for WritelockSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`writelock::R`](R) reader structure"]
impl crate::Readable for WritelockSpec {}
#[doc = "`write(|w| ..)` method takes [`writelock::W`](W) writer structure"]
impl crate::Writable for WritelockSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets WRITELOCK to value 0"]
impl crate::Resettable for WritelockSpec {}
