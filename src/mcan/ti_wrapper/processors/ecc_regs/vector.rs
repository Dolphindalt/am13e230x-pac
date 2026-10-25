#[doc = "Register `VECTOR` reader"]
pub type R = crate::R<VectorSpec>;
#[doc = "Register `VECTOR` writer"]
pub type W = crate::W<VectorSpec>;
#[doc = "Field `ECC_VECTOR` reader - ECC RAM ID"]
pub type EccVectorR = crate::FieldReader<u16>;
#[doc = "Field `ECC_VECTOR` writer - ECC RAM ID"]
pub type EccVectorW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16, crate::Safe>;
#[doc = "Field `RD_SVBUS` reader - Read Trigger"]
pub type RdSvbusR = crate::BitReader;
#[doc = "Field `RD_SVBUS` writer - Read Trigger"]
pub type RdSvbusW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RD_SVBUS_ADDRESS` reader - Read Address Offset"]
pub type RdSvbusAddressR = crate::FieldReader;
#[doc = "Field `RD_SVBUS_ADDRESS` writer - Read Address Offset"]
pub type RdSvbusAddressW<'a, REG> = crate::FieldWriter<'a, REG, 8, u8, crate::Safe>;
#[doc = "Field `RD_SVBUS_DONE` reader - Read Completion Flag"]
pub type RdSvbusDoneR = crate::BitReader;
impl R {
    #[doc = "Bits 0:10 - ECC RAM ID"]
    #[inline(always)]
    pub fn ecc_vector(&self) -> EccVectorR {
        EccVectorR::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bit 15 - Read Trigger"]
    #[inline(always)]
    pub fn rd_svbus(&self) -> RdSvbusR {
        RdSvbusR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:23 - Read Address Offset"]
    #[inline(always)]
    pub fn rd_svbus_address(&self) -> RdSvbusAddressR {
        RdSvbusAddressR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - Read Completion Flag"]
    #[inline(always)]
    pub fn rd_svbus_done(&self) -> RdSvbusDoneR {
        RdSvbusDoneR::new(((self.bits >> 24) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VECTOR")
            .field("ecc_vector", &self.ecc_vector())
            .field("rd_svbus", &self.rd_svbus())
            .field("rd_svbus_address", &self.rd_svbus_address())
            .field("rd_svbus_done", &self.rd_svbus_done())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:10 - ECC RAM ID"]
    #[inline(always)]
    pub fn ecc_vector(&mut self) -> EccVectorW<'_, VectorSpec> {
        EccVectorW::new(self, 0)
    }
    #[doc = "Bit 15 - Read Trigger"]
    #[inline(always)]
    pub fn rd_svbus(&mut self) -> RdSvbusW<'_, VectorSpec> {
        RdSvbusW::new(self, 15)
    }
    #[doc = "Bits 16:23 - Read Address Offset"]
    #[inline(always)]
    pub fn rd_svbus_address(&mut self) -> RdSvbusAddressW<'_, VectorSpec> {
        RdSvbusAddressW::new(self, 16)
    }
}
#[doc = "MCAN ECC Vector Register\n\nYou can [`read`](crate::Reg::read) this register and get [`vector::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vector::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VectorSpec;
impl crate::RegisterSpec for VectorSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vector::R`](R) reader structure"]
impl crate::Readable for VectorSpec {}
#[doc = "`write(|w| ..)` method takes [`vector::W`](W) writer structure"]
impl crate::Writable for VectorSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets VECTOR to value 0"]
impl crate::Resettable for VectorSpec {}
