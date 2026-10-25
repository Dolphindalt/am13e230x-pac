#[doc = "Register `ROM_VERSION` reader"]
pub type R = crate::R<RomVersionSpec>;
#[doc = "Field `ROM_PATCH_VER` reader - ROM_PATCH_VER"]
pub type RomPatchVerR = crate::FieldReader;
#[doc = "Field `ROM_MINOR_VER` reader - ROM_MINOR_VER"]
pub type RomMinorVerR = crate::FieldReader;
#[doc = "Field `ROM_MAJOR_VER` reader - ROM_MAJOR_VER"]
pub type RomMajorVerR = crate::FieldReader;
#[doc = "Field `ROM_PG_VER` reader - ROM_PG_VER"]
pub type RomPgVerR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - ROM_PATCH_VER"]
    #[inline(always)]
    pub fn rom_patch_ver(&self) -> RomPatchVerR {
        RomPatchVerR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - ROM_MINOR_VER"]
    #[inline(always)]
    pub fn rom_minor_ver(&self) -> RomMinorVerR {
        RomMinorVerR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - ROM_MAJOR_VER"]
    #[inline(always)]
    pub fn rom_major_ver(&self) -> RomMajorVerR {
        RomMajorVerR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - ROM_PG_VER"]
    #[inline(always)]
    pub fn rom_pg_ver(&self) -> RomPgVerR {
        RomPgVerR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ROM_VERSION")
            .field("rom_patch_ver", &self.rom_patch_ver())
            .field("rom_minor_ver", &self.rom_minor_ver())
            .field("rom_major_ver", &self.rom_major_ver())
            .field("rom_pg_ver", &self.rom_pg_ver())
            .finish()
    }
}
#[doc = "ROM_VERSION\n\nYou can [`read`](crate::Reg::read) this register and get [`rom_version::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RomVersionSpec;
impl crate::RegisterSpec for RomVersionSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rom_version::R`](R) reader structure"]
impl crate::Readable for RomVersionSpec {}
#[doc = "`reset()` method sets ROM_VERSION to value 0x0101_0000"]
impl crate::Resettable for RomVersionSpec {
    const RESET_VALUE: u32 = 0x0101_0000;
}
