#[doc = "Register `SECURITY_ERR_ADDR` reader"]
pub type R = crate::R<SecurityErrAddrSpec>;
#[doc = "Register `SECURITY_ERR_ADDR` writer"]
pub type W = crate::W<SecurityErrAddrSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Security error address\n\nYou can [`read`](crate::Reg::read) this register and get [`security_err_addr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`security_err_addr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SecurityErrAddrSpec;
impl crate::RegisterSpec for SecurityErrAddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`security_err_addr::R`](R) reader structure"]
impl crate::Readable for SecurityErrAddrSpec {}
#[doc = "`write(|w| ..)` method takes [`security_err_addr::W`](W) writer structure"]
impl crate::Writable for SecurityErrAddrSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets SECURITY_ERR_ADDR to value 0"]
impl crate::Resettable for SecurityErrAddrSpec {}
