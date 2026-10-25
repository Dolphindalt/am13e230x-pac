#[doc = "Register `QUPRD` reader"]
pub type R = crate::R<QuprdSpec>;
#[doc = "Register `QUPRD` writer"]
pub type W = crate::W<QuprdSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "QEP Unit Period\n\nYou can [`read`](crate::Reg::read) this register and get [`quprd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`quprd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QuprdSpec;
impl crate::RegisterSpec for QuprdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`quprd::R`](R) reader structure"]
impl crate::Readable for QuprdSpec {}
#[doc = "`write(|w| ..)` method takes [`quprd::W`](W) writer structure"]
impl crate::Writable for QuprdSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets QUPRD to value 0"]
impl crate::Resettable for QuprdSpec {}
