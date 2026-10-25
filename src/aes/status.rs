#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Key write status. 0 - user write to KEY register is allowed. 1 - user write to KEY register is ignored. In order to allow user write, perform a module reset.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Keywr {
    #[doc = "0: User write to KEY MMR is allowed"]
    Enabled = 0,
    #[doc = "1: User write to KEY MMR is disabled. Writing has no effect."]
    Disabled = 1,
}
impl From<Keywr> for bool {
    #[inline(always)]
    fn from(variant: Keywr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `KEYWR` reader - Key write status. 0 - user write to KEY register is allowed. 1 - user write to KEY register is ignored. In order to allow user write, perform a module reset."]
pub type KeywrR = crate::BitReader<Keywr>;
impl KeywrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Keywr {
        match self.bits {
            false => Keywr::Enabled,
            true => Keywr::Disabled,
        }
    }
    #[doc = "User write to KEY MMR is allowed"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Keywr::Enabled
    }
    #[doc = "User write to KEY MMR is disabled. Writing has no effect."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Keywr::Disabled
    }
}
impl R {
    #[doc = "Bit 0 - Key write status. 0 - user write to KEY register is allowed. 1 - user write to KEY register is ignored. In order to allow user write, perform a module reset."]
    #[inline(always)]
    pub fn keywr(&self) -> KeywrR {
        KeywrR::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STATUS")
            .field("keywr", &self.keywr())
            .finish()
    }
}
#[doc = "Status\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusSpec;
impl crate::RegisterSpec for StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for StatusSpec {}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for StatusSpec {}
