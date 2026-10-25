#[doc = "Register `TMUROM_PAR_ADDR` reader"]
pub type R = crate::R<TmuromParAddrSpec>;
#[doc = "Register `TMUROM_PAR_ADDR` writer"]
pub type W = crate::W<TmuromParAddrSpec>;
#[doc = "Field `ADDRESS` reader - Address"]
pub type AddressR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:14 - Address"]
    #[inline(always)]
    pub fn address(&self) -> AddressR {
        AddressR::new((self.bits & 0x7fff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TMUROM_PAR_ADDR")
            .field("address", &self.address())
            .finish()
    }
}
impl W {}
#[doc = "SYSMEM DED address\n\nYou can [`read`](crate::Reg::read) this register and get [`tmurom_par_addr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmurom_par_addr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TmuromParAddrSpec;
impl crate::RegisterSpec for TmuromParAddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tmurom_par_addr::R`](R) reader structure"]
impl crate::Readable for TmuromParAddrSpec {}
#[doc = "`write(|w| ..)` method takes [`tmurom_par_addr::W`](W) writer structure"]
impl crate::Writable for TmuromParAddrSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets TMUROM_PAR_ADDR to value 0"]
impl crate::Resettable for TmuromParAddrSpec {}
