#[doc = "Register `CRCPOLY` reader"]
pub type R = crate::R<CrcpolySpec>;
#[doc = "Register `CRCPOLY` writer"]
pub type W = crate::W<CrcpolySpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "CRC Polynomial configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`crcpoly::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crcpoly::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrcpolySpec;
impl crate::RegisterSpec for CrcpolySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crcpoly::R`](R) reader structure"]
impl crate::Readable for CrcpolySpec {}
#[doc = "`write(|w| ..)` method takes [`crcpoly::W`](W) writer structure"]
impl crate::Writable for CrcpolySpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CRCPOLY to value 0"]
impl crate::Resettable for CrcpolySpec {}
