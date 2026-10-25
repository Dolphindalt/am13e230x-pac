#[doc = "Register `SYNCSELECT` reader"]
pub type R = crate::R<SyncselectSpec>;
#[doc = "Register `SYNCSELECT` writer"]
pub type W = crate::W<SyncselectSpec>;
#[doc = "Field `SYNCOUT` reader - Select EXT Syncout Source"]
pub type SyncoutR = crate::FieldReader;
#[doc = "Field `SYNCOUT` writer - Select EXT Syncout Source"]
pub type SyncoutW<'a, REG> = crate::FieldWriter<'a, REG, 5, u8, crate::Safe>;
impl R {
    #[doc = "Bits 24:28 - Select EXT Syncout Source"]
    #[inline(always)]
    pub fn syncout(&self) -> SyncoutR {
        SyncoutR::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SYNCSELECT")
            .field("syncout", &self.syncout())
            .finish()
    }
}
impl W {
    #[doc = "Bits 24:28 - Select EXT Syncout Source"]
    #[inline(always)]
    pub fn syncout(&mut self) -> SyncoutW<'_, SyncselectSpec> {
        SyncoutW::new(self, 24)
    }
}
#[doc = "Sync Input and Output Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`syncselect::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syncselect::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SyncselectSpec;
impl crate::RegisterSpec for SyncselectSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syncselect::R`](R) reader structure"]
impl crate::Readable for SyncselectSpec {}
#[doc = "`write(|w| ..)` method takes [`syncselect::W`](W) writer structure"]
impl crate::Writable for SyncselectSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets SYNCSELECT to value 0"]
impl crate::Resettable for SyncselectSpec {}
