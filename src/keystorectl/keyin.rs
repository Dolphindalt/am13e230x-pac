#[doc = "Register `KEYIN` writer"]
pub type W = crate::W<KeyinSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<KeyinSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
#[doc = "Input key\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`keyin::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KeyinSpec;
impl crate::RegisterSpec for KeyinSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`keyin::W`](W) writer structure"]
impl crate::Writable for KeyinSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets KEYIN to value 0"]
impl crate::Resettable for KeyinSpec {}
