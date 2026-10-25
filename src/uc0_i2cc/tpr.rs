#[doc = "Register `TPR` reader"]
pub type R = crate::R<TprSpec>;
#[doc = "Register `TPR` writer"]
pub type W = crate::W<TprSpec>;
#[doc = "Field `TPR` reader - Timer Period This field is used in the equation to configure SCL_PERIOD : SCL_PERIOD = (1 + TPR ) * (SCL_LP + SCL_HP ) * INT_CLK_PRD where: SCL_PRD is the SCL line period (I2C clock). TPR is the Timer Period register value (range of 1 to 127). SCL_LP is the SCL Low period (fixed at 6). SCL_HP is the SCL High period (fixed at 4). CLK_PRD is the functional clock period in ns."]
pub type TprR = crate::FieldReader;
#[doc = "Field `TPR` writer - Timer Period This field is used in the equation to configure SCL_PERIOD : SCL_PERIOD = (1 + TPR ) * (SCL_LP + SCL_HP ) * INT_CLK_PRD where: SCL_PRD is the SCL line period (I2C clock). TPR is the Timer Period register value (range of 1 to 127). SCL_LP is the SCL Low period (fixed at 6). SCL_HP is the SCL High period (fixed at 4). CLK_PRD is the functional clock period in ns."]
pub type TprW<'a, REG> = crate::FieldWriter<'a, REG, 7, u8, crate::Safe>;
impl R {
    #[doc = "Bits 0:6 - Timer Period This field is used in the equation to configure SCL_PERIOD : SCL_PERIOD = (1 + TPR ) * (SCL_LP + SCL_HP ) * INT_CLK_PRD where: SCL_PRD is the SCL line period (I2C clock). TPR is the Timer Period register value (range of 1 to 127). SCL_LP is the SCL Low period (fixed at 6). SCL_HP is the SCL High period (fixed at 4). CLK_PRD is the functional clock period in ns."]
    #[inline(always)]
    pub fn tpr(&self) -> TprR {
        TprR::new((self.bits & 0x7f) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TPR").field("tpr", &self.tpr()).finish()
    }
}
impl W {
    #[doc = "Bits 0:6 - Timer Period This field is used in the equation to configure SCL_PERIOD : SCL_PERIOD = (1 + TPR ) * (SCL_LP + SCL_HP ) * INT_CLK_PRD where: SCL_PRD is the SCL line period (I2C clock). TPR is the Timer Period register value (range of 1 to 127). SCL_LP is the SCL Low period (fixed at 6). SCL_HP is the SCL High period (fixed at 4). CLK_PRD is the functional clock period in ns."]
    #[inline(always)]
    pub fn tpr(&mut self) -> TprW<'_, TprSpec> {
        TprW::new(self, 0)
    }
}
#[doc = "Timer Period\n\nYou can [`read`](crate::Reg::read) this register and get [`tpr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tpr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TprSpec;
impl crate::RegisterSpec for TprSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tpr::R`](R) reader structure"]
impl crate::Readable for TprSpec {}
#[doc = "`write(|w| ..)` method takes [`tpr::W`](W) writer structure"]
impl crate::Writable for TprSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets TPR to value 0x01"]
impl crate::Resettable for TprSpec {
    const RESET_VALUE: u32 = 0x01;
}
