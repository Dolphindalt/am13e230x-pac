#[doc = "Register `BMON` reader"]
pub type R = crate::R<BmonSpec>;
#[doc = "Field `SCL` reader - I2C SCL Status"]
pub type SclR = crate::BitReader;
#[doc = "Field `SDA` reader - I2C SDA Status"]
pub type SdaR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - I2C SCL Status"]
    #[inline(always)]
    pub fn scl(&self) -> SclR {
        SclR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - I2C SDA Status"]
    #[inline(always)]
    pub fn sda(&self) -> SdaR {
        SdaR::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BMON")
            .field("scl", &self.scl())
            .field("sda", &self.sda())
            .finish()
    }
}
#[doc = "Bus Monitor\n\nYou can [`read`](crate::Reg::read) this register and get [`bmon::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BmonSpec;
impl crate::RegisterSpec for BmonSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bmon::R`](R) reader structure"]
impl crate::Readable for BmonSpec {}
#[doc = "`reset()` method sets BMON to value 0x03"]
impl crate::Resettable for BmonSpec {
    const RESET_VALUE: u32 = 0x03;
}
