#[doc = "Register `DEVICEID` reader"]
pub type R = crate::R<DeviceidSpec>;
#[doc = "Field `ALWAYS_1` reader - This is always 1"]
pub type Always1R = crate::BitReader;
#[doc = "Field `MANUFACTURER` reader - TI's JEDEC bank and company code, which is: 00000010111b"]
pub type ManufacturerR = crate::FieldReader<u16>;
#[doc = "Field `PARTNUM` reader - Part number of the device."]
pub type PartnumR = crate::FieldReader<u16>;
#[doc = "Field `VERSION` reader - Revision of the device. This field should change each time that the logic or mask set of the device is revised."]
pub type VersionR = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - This is always 1"]
    #[inline(always)]
    pub fn always_1(&self) -> Always1R {
        Always1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:11 - TI's JEDEC bank and company code, which is: 00000010111b"]
    #[inline(always)]
    pub fn manufacturer(&self) -> ManufacturerR {
        ManufacturerR::new(((self.bits >> 1) & 0x07ff) as u16)
    }
    #[doc = "Bits 12:27 - Part number of the device."]
    #[inline(always)]
    pub fn partnum(&self) -> PartnumR {
        PartnumR::new(((self.bits >> 12) & 0xffff) as u16)
    }
    #[doc = "Bits 28:31 - Revision of the device. This field should change each time that the logic or mask set of the device is revised."]
    #[inline(always)]
    pub fn version(&self) -> VersionR {
        VersionR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DEVICEID")
            .field("always_1", &self.always_1())
            .field("manufacturer", &self.manufacturer())
            .field("partnum", &self.partnum())
            .field("version", &self.version())
            .finish()
    }
}
#[doc = "DEVICEID\n\nYou can [`read`](crate::Reg::read) this register and get [`deviceid::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DeviceidSpec;
impl crate::RegisterSpec for DeviceidSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`deviceid::R`](R) reader structure"]
impl crate::Readable for DeviceidSpec {}
#[doc = "`reset()` method sets DEVICEID to value 0x1bbb_702f"]
impl crate::Resettable for DeviceidSpec {
    const RESET_VALUE: u32 = 0x1bbb_702f;
}
