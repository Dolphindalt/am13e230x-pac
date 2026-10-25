#[doc = "Register `QPOSCMP` reader"]
pub type R = crate::R<QposcmpSpec>;
#[doc = "Register `QPOSCMP` writer"]
pub type W = crate::W<QposcmpSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Position Compare\n\nYou can [`read`](crate::Reg::read) this register and get [`qposcmp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qposcmp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QposcmpSpec;
impl crate::RegisterSpec for QposcmpSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`qposcmp::R`](R) reader structure"]
impl crate::Readable for QposcmpSpec {}
#[doc = "`write(|w| ..)` method takes [`qposcmp::W`](W) writer structure"]
impl crate::Writable for QposcmpSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets QPOSCMP to value 0"]
impl crate::Resettable for QposcmpSpec {}
