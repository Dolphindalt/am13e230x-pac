#[doc = "Register `USERID` reader"]
pub type R = crate::R<UseridSpec>;
#[doc = "Field `PART` reader - Bit pattern that uniquely identifying a part. This is used to identify the specific part based on the die identified in DEVICEID.device. This number shall be selected at random among the remaining numbers for DEVICEID.device such that the order of creation cannot be inferred by the number. This value does not encode the part number directly."]
pub type PartR = crate::FieldReader<u16>;
#[doc = "Field `VARIANT` reader - Bit pattern uniquely identifying a variant of a part. This is used to indicate memory or package variations of the same part number. This number shall be selected at random among the remaining numbers for the relevant combination of IDCODE.device and USERCODE.part such that the order of creation cannot be inferred by the number. The variant number does not encode specifics of the variant directly."]
pub type VariantR = crate::FieldReader;
#[doc = "Field `MINORREV` reader - Monotonic increasing value indicating a new revision of the SKU that preserves compatibility with lesser minorrev values. New capability may be introduced such that lesser minorrev numbers may not be compatible with greater if the new capability is used."]
pub type MinorrevR = crate::FieldReader;
#[doc = "Field `MAJORREV` reader - Monotonic increasing value indicating a new revision of the SKU significant enough that users of the device may have to revise PCB or or software design"]
pub type MajorrevR = crate::FieldReader;
#[doc = "Field `START` reader - START"]
pub type StartR = crate::BitReader;
impl R {
    #[doc = "Bits 0:15 - Bit pattern that uniquely identifying a part. This is used to identify the specific part based on the die identified in DEVICEID.device. This number shall be selected at random among the remaining numbers for DEVICEID.device such that the order of creation cannot be inferred by the number. This value does not encode the part number directly."]
    #[inline(always)]
    pub fn part(&self) -> PartR {
        PartR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - Bit pattern uniquely identifying a variant of a part. This is used to indicate memory or package variations of the same part number. This number shall be selected at random among the remaining numbers for the relevant combination of IDCODE.device and USERCODE.part such that the order of creation cannot be inferred by the number. The variant number does not encode specifics of the variant directly."]
    #[inline(always)]
    pub fn variant(&self) -> VariantR {
        VariantR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:27 - Monotonic increasing value indicating a new revision of the SKU that preserves compatibility with lesser minorrev values. New capability may be introduced such that lesser minorrev numbers may not be compatible with greater if the new capability is used."]
    #[inline(always)]
    pub fn minorrev(&self) -> MinorrevR {
        MinorrevR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:30 - Monotonic increasing value indicating a new revision of the SKU significant enough that users of the device may have to revise PCB or or software design"]
    #[inline(always)]
    pub fn majorrev(&self) -> MajorrevR {
        MajorrevR::new(((self.bits >> 28) & 7) as u8)
    }
    #[doc = "Bit 31 - START"]
    #[inline(always)]
    pub fn start(&self) -> StartR {
        StartR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USERID")
            .field("part", &self.part())
            .field("variant", &self.variant())
            .field("minorrev", &self.minorrev())
            .field("majorrev", &self.majorrev())
            .field("start", &self.start())
            .finish()
    }
}
#[doc = "Defined by TI, depending on device spin\n\nYou can [`read`](crate::Reg::read) this register and get [`userid::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UseridSpec;
impl crate::RegisterSpec for UseridSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`userid::R`](R) reader structure"]
impl crate::Readable for UseridSpec {}
#[doc = "`reset()` method sets USERID to value 0"]
impl crate::Resettable for UseridSpec {}
