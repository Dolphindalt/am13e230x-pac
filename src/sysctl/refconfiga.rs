#[doc = "Register `REFCONFIGA` reader"]
pub type R = crate::R<RefconfigaSpec>;
#[doc = "Register `REFCONFIGA` writer"]
pub type W = crate::W<RefconfigaSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Reference Configuration Regsiter\n\nYou can [`read`](crate::Reg::read) this register and get [`refconfiga::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`refconfiga::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RefconfigaSpec;
impl crate::RegisterSpec for RefconfigaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`refconfiga::R`](R) reader structure"]
impl crate::Readable for RefconfigaSpec {}
#[doc = "`write(|w| ..)` method takes [`refconfiga::W`](W) writer structure"]
impl crate::Writable for RefconfigaSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets REFCONFIGA to value 0"]
impl crate::Resettable for RefconfigaSpec {}
