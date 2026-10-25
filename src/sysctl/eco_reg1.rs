#[doc = "Register `ECO_REG1` reader"]
pub type R = crate::R<EcoReg1Spec>;
#[doc = "Register `ECO_REG1` writer"]
pub type W = crate::W<EcoReg1Spec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Sysctl ECO Reg 1\n\nYou can [`read`](crate::Reg::read) this register and get [`eco_reg1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eco_reg1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EcoReg1Spec;
impl crate::RegisterSpec for EcoReg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eco_reg1::R`](R) reader structure"]
impl crate::Readable for EcoReg1Spec {}
#[doc = "`write(|w| ..)` method takes [`eco_reg1::W`](W) writer structure"]
impl crate::Writable for EcoReg1Spec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets ECO_REG1 to value 0"]
impl crate::Resettable for EcoReg1Spec {}
