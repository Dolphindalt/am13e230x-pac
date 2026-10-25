#[doc = "Register `BSLPIN_I2C` reader"]
pub type R = crate::R<BslpinI2cSpec>;
#[doc = "Field `I2C_SDA_PAD` reader - I2C SDA Pin used by BSL"]
pub type I2cSdaPadR = crate::FieldReader;
#[doc = "Field `I2C_SDA_PF` reader - I2C SDA Pin Function Selection Value"]
pub type I2cSdaPfR = crate::FieldReader;
#[doc = "Field `I2C_SCL_PAD` reader - I2C SCL Pin used by BSL"]
pub type I2cSclPadR = crate::FieldReader;
#[doc = "Field `I2C_SCL_PF` reader - I2C SCL Pin Function Selection Value"]
pub type I2cSclPfR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - I2C SDA Pin used by BSL"]
    #[inline(always)]
    pub fn i2c_sda_pad(&self) -> I2cSdaPadR {
        I2cSdaPadR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - I2C SDA Pin Function Selection Value"]
    #[inline(always)]
    pub fn i2c_sda_pf(&self) -> I2cSdaPfR {
        I2cSdaPfR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - I2C SCL Pin used by BSL"]
    #[inline(always)]
    pub fn i2c_scl_pad(&self) -> I2cSclPadR {
        I2cSclPadR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - I2C SCL Pin Function Selection Value"]
    #[inline(always)]
    pub fn i2c_scl_pf(&self) -> I2cSclPfR {
        I2cSclPfR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BSLPIN_I2C")
            .field("i2c_sda_pad", &self.i2c_sda_pad())
            .field("i2c_sda_pf", &self.i2c_sda_pf())
            .field("i2c_scl_pad", &self.i2c_scl_pad())
            .field("i2c_scl_pf", &self.i2c_scl_pf())
            .finish()
    }
}
#[doc = "BSL I2C Pin Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`bslpin_i2c::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BslpinI2cSpec;
impl crate::RegisterSpec for BslpinI2cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bslpin_i2c::R`](R) reader structure"]
impl crate::Readable for BslpinI2cSpec {}
#[doc = "`reset()` method sets BSLPIN_I2C to value 0"]
impl crate::Resettable for BslpinI2cSpec {}
