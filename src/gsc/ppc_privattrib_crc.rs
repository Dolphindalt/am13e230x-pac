#[doc = "Register `PPC_PRIVATTRIB_CRC` reader"]
pub type R = crate::R<PpcPrivattribCrcSpec>;
#[doc = "Register `PPC_PRIVATTRIB_CRC` writer"]
pub type W = crate::W<PpcPrivattribCrcSpec>;
#[doc = "Field `PRIV_CRC` reader - Privilege access for CRC"]
pub type PrivCrcR = crate::BitReader;
#[doc = "Field `PRIV_CRC` writer - Privilege access for CRC"]
pub type PrivCrcW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Privilege access for CRC"]
    #[inline(always)]
    pub fn priv_crc(&self) -> PrivCrcR {
        PrivCrcR::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PPC_PRIVATTRIB_CRC")
            .field("priv_crc", &self.priv_crc())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Privilege access for CRC"]
    #[inline(always)]
    pub fn priv_crc(&mut self) -> PrivCrcW<'_, PpcPrivattribCrcSpec> {
        PrivCrcW::new(self, 0)
    }
}
#[doc = "Peripheral Privilege Access CRC\n\nYou can [`read`](crate::Reg::read) this register and get [`ppc_privattrib_crc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppc_privattrib_crc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PpcPrivattribCrcSpec;
impl crate::RegisterSpec for PpcPrivattribCrcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ppc_privattrib_crc::R`](R) reader structure"]
impl crate::Readable for PpcPrivattribCrcSpec {}
#[doc = "`write(|w| ..)` method takes [`ppc_privattrib_crc::W`](W) writer structure"]
impl crate::Writable for PpcPrivattribCrcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PPC_PRIVATTRIB_CRC to value 0x01"]
impl crate::Resettable for PpcPrivattribCrcSpec {
    const RESET_VALUE: u32 = 0x01;
}
