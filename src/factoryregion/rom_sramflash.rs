#[doc = "Register `ROM_SRAMFLASH` reader"]
pub type R = crate::R<RomSramflashSpec>;
#[doc = "Field `MAINFLASH_SZ` reader - The encoding of the field is that the value of the field is an integer to be interpreted as number of KBs. For eg: if the value of the field id 4, then it is 4KB, if the value is 32, then 32KB, and so on."]
pub type MainflashSzR = crate::FieldReader<u16>;
#[doc = "Field `MAINNUMBANKS` reader - Defines the number of main flash banks"]
pub type MainnumbanksR = crate::FieldReader;
#[doc = "Field `SRAM_SZ` reader - The encoding of the field is that the value of the field is an integer to be interpreted as number of KBs. For eg: if the value of the field id 4, then it is 4KB, if the value is 32, then 32KB, and so on."]
pub type SramSzR = crate::FieldReader<u16>;
#[doc = "Field `DATAFLASH_SZ` reader - The encoding of the field is that the value of the field is an integer to be interpreted as number of KBs. For eg: if the value of the field id 4, then it is 4KB, if the value is 32, then 32KB, and so on."]
pub type DataflashSzR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:11 - The encoding of the field is that the value of the field is an integer to be interpreted as number of KBs. For eg: if the value of the field id 4, then it is 4KB, if the value is 32, then 32KB, and so on."]
    #[inline(always)]
    pub fn mainflash_sz(&self) -> MainflashSzR {
        MainflashSzR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:13 - Defines the number of main flash banks"]
    #[inline(always)]
    pub fn mainnumbanks(&self) -> MainnumbanksR {
        MainnumbanksR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 16:25 - The encoding of the field is that the value of the field is an integer to be interpreted as number of KBs. For eg: if the value of the field id 4, then it is 4KB, if the value is 32, then 32KB, and so on."]
    #[inline(always)]
    pub fn sram_sz(&self) -> SramSzR {
        SramSzR::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    #[doc = "Bits 26:31 - The encoding of the field is that the value of the field is an integer to be interpreted as number of KBs. For eg: if the value of the field id 4, then it is 4KB, if the value is 32, then 32KB, and so on."]
    #[inline(always)]
    pub fn dataflash_sz(&self) -> DataflashSzR {
        DataflashSzR::new(((self.bits >> 26) & 0x3f) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ROM_SRAMFLASH")
            .field("mainflash_sz", &self.mainflash_sz())
            .field("mainnumbanks", &self.mainnumbanks())
            .field("sram_sz", &self.sram_sz())
            .field("dataflash_sz", &self.dataflash_sz())
            .finish()
    }
}
#[doc = "ROM_SRAMFLASH\n\nYou can [`read`](crate::Reg::read) this register and get [`rom_sramflash::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RomSramflashSpec;
impl crate::RegisterSpec for RomSramflashSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rom_sramflash::R`](R) reader structure"]
impl crate::Readable for RomSramflashSpec {}
#[doc = "`reset()` method sets ROM_SRAMFLASH to value 0"]
impl crate::Resettable for RomSramflashSpec {}
