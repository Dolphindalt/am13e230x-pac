#[doc = "Register `ROM_INM_VERSION` reader"]
pub type R = crate::R<RomInmVersionSpec>;
#[doc = "Field `ROM_INM_PATCH_VER` reader - ROM_INM_PATCH_VER"]
pub type RomInmPatchVerR = crate::FieldReader;
#[doc = "Field `ROM_INM_MINOR_VER` reader - ROM_INM_MINOR_VER"]
pub type RomInmMinorVerR = crate::FieldReader;
#[doc = "Field `ROM_INM_MAJOR_VER` reader - ROM_INM_MAJOR_VER"]
pub type RomInmMajorVerR = crate::FieldReader;
#[doc = "Field `ROM_INM_PG_VER` reader - ROM_INM_PG_VER"]
pub type RomInmPgVerR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - ROM_INM_PATCH_VER"]
    #[inline(always)]
    pub fn rom_inm_patch_ver(&self) -> RomInmPatchVerR {
        RomInmPatchVerR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - ROM_INM_MINOR_VER"]
    #[inline(always)]
    pub fn rom_inm_minor_ver(&self) -> RomInmMinorVerR {
        RomInmMinorVerR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - ROM_INM_MAJOR_VER"]
    #[inline(always)]
    pub fn rom_inm_major_ver(&self) -> RomInmMajorVerR {
        RomInmMajorVerR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - ROM_INM_PG_VER"]
    #[inline(always)]
    pub fn rom_inm_pg_ver(&self) -> RomInmPgVerR {
        RomInmPgVerR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ROM_INM_VERSION")
            .field("rom_inm_patch_ver", &self.rom_inm_patch_ver())
            .field("rom_inm_minor_ver", &self.rom_inm_minor_ver())
            .field("rom_inm_major_ver", &self.rom_inm_major_ver())
            .field("rom_inm_pg_ver", &self.rom_inm_pg_ver())
            .finish()
    }
}
#[doc = "ROM_INM_VERSION\n\nYou can [`read`](crate::Reg::read) this register and get [`rom_inm_version::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RomInmVersionSpec;
impl crate::RegisterSpec for RomInmVersionSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rom_inm_version::R`](R) reader structure"]
impl crate::Readable for RomInmVersionSpec {}
#[doc = "`reset()` method sets ROM_INM_VERSION to value 0x0101_0000"]
impl crate::Resettable for RomInmVersionSpec {
    const RESET_VALUE: u32 = 0x0101_0000;
}
