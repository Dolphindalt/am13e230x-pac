#[doc = "Register `ERR_CTRL1` reader"]
pub type R = crate::R<ErrCtrl1Spec>;
#[doc = "Register `ERR_CTRL1` writer"]
pub type W = crate::W<ErrCtrl1Spec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "MCAN ECC Error Control 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`err_ctrl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`err_ctrl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ErrCtrl1Spec;
impl crate::RegisterSpec for ErrCtrl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`err_ctrl1::R`](R) reader structure"]
impl crate::Readable for ErrCtrl1Spec {}
#[doc = "`write(|w| ..)` method takes [`err_ctrl1::W`](W) writer structure"]
impl crate::Writable for ErrCtrl1Spec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets ERR_CTRL1 to value 0"]
impl crate::Resettable for ErrCtrl1Spec {}
