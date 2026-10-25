#[doc = "Register `KEYLOCK` reader"]
pub type R = crate::R<KeylockSpec>;
#[doc = "Register `KEYLOCK` writer"]
pub type W = crate::W<KeylockSpec>;
#[doc = "Field `LOCKBIT` reader - Bitvector of lock bits to lock certain slots from being used by application."]
pub type LockbitR = crate::FieldReader;
#[doc = "Field `LOCKBIT` writer - Bitvector of lock bits to lock certain slots from being used by application."]
pub type LockbitW<'a, REG> = crate::FieldWriter<'a, REG, 8, u8, crate::Safe>;
impl R {
    #[doc = "Bits 0:7 - Bitvector of lock bits to lock certain slots from being used by application."]
    #[inline(always)]
    pub fn lockbit(&self) -> LockbitR {
        LockbitR::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("KEYLOCK")
            .field("lockbit", &self.lockbit())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - Bitvector of lock bits to lock certain slots from being used by application."]
    #[inline(always)]
    pub fn lockbit(&mut self) -> LockbitW<'_, KeylockSpec> {
        LockbitW::new(self, 0)
    }
}
#[doc = "Keylock\n\nYou can [`read`](crate::Reg::read) this register and get [`keylock::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`keylock::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KeylockSpec;
impl crate::RegisterSpec for KeylockSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`keylock::R`](R) reader structure"]
impl crate::Readable for KeylockSpec {}
#[doc = "`write(|w| ..)` method takes [`keylock::W`](W) writer structure"]
impl crate::Writable for KeylockSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets KEYLOCK to value 0"]
impl crate::Resettable for KeylockSpec {}
