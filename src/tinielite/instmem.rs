#[doc = "Register `INSTMEM[%s]` reader"]
pub type R = crate::R<InstmemSpec>;
#[doc = "Register `INSTMEM[%s]` writer"]
pub type W = crate::W<InstmemSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Instruction memory word.\n\nYou can [`read`](crate::Reg::read) this register and get [`instmem::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`instmem::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InstmemSpec;
impl crate::RegisterSpec for InstmemSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`instmem::R`](R) reader structure"]
impl crate::Readable for InstmemSpec {}
#[doc = "`write(|w| ..)` method takes [`instmem::W`](W) writer structure"]
impl crate::Writable for InstmemSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INSTMEM[%s] to value 0"]
impl crate::Resettable for InstmemSpec {}
