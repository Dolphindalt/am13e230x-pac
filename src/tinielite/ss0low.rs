#[doc = "Register `SS0LOW` reader"]
pub type R = crate::R<Ss0lowSpec>;
#[doc = "Register `SS0LOW` writer"]
pub type W = crate::W<Ss0lowSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Scale and shift register 0, lower 32 bits.\n\nYou can [`read`](crate::Reg::read) this register and get [`ss0low::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ss0low::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ss0lowSpec;
impl crate::RegisterSpec for Ss0lowSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ss0low::R`](R) reader structure"]
impl crate::Readable for Ss0lowSpec {}
#[doc = "`write(|w| ..)` method takes [`ss0low::W`](W) writer structure"]
impl crate::Writable for Ss0lowSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SS0LOW to value 0"]
impl crate::Resettable for Ss0lowSpec {}
