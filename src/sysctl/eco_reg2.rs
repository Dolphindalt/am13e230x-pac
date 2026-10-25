#[doc = "Register `ECO_REG2` reader"]
pub type R = crate::R<EcoReg2Spec>;
#[doc = "Register `ECO_REG2` writer"]
pub type W = crate::W<EcoReg2Spec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Sysctl ECO Reg 2\n\nYou can [`read`](crate::Reg::read) this register and get [`eco_reg2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eco_reg2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EcoReg2Spec;
impl crate::RegisterSpec for EcoReg2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eco_reg2::R`](R) reader structure"]
impl crate::Readable for EcoReg2Spec {}
#[doc = "`write(|w| ..)` method takes [`eco_reg2::W`](W) writer structure"]
impl crate::Writable for EcoReg2Spec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets ECO_REG2 to value 0"]
impl crate::Resettable for EcoReg2Spec {}
