#[doc = "Register `WFIFOCNT` reader"]
pub type R = crate::R<WfifocntSpec>;
#[doc = "Register `WFIFOCNT` writer"]
pub type W = crate::W<WfifocntSpec>;
#[doc = "Field `WTAV` reader - Available Write Transactions."]
pub type WtavR = crate::FieldReader;
#[doc = "Field `WTAV` writer - Available Write Transactions."]
pub type WtavW<'a, REG> = crate::FieldWriter<'a, REG, 3, u8, crate::Safe>;
impl R {
    #[doc = "Bits 0:2 - Available Write Transactions."]
    #[inline(always)]
    pub fn wtav(&self) -> WtavR {
        WtavR::new((self.bits & 7) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WFIFOCNT")
            .field("wtav", &self.wtav())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - Available Write Transactions."]
    #[inline(always)]
    pub fn wtav(&mut self) -> WtavW<'_, WfifocntSpec> {
        WtavW::new(self, 0)
    }
}
#[doc = "EPI Write FIFO Count\n\nYou can [`read`](crate::Reg::read) this register and get [`wfifocnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wfifocnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WfifocntSpec;
impl crate::RegisterSpec for WfifocntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wfifocnt::R`](R) reader structure"]
impl crate::Readable for WfifocntSpec {}
#[doc = "`write(|w| ..)` method takes [`wfifocnt::W`](W) writer structure"]
impl crate::Writable for WfifocntSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets WFIFOCNT to value 0x04"]
impl crate::Resettable for WfifocntSpec {
    const RESET_VALUE: u32 = 0x04;
}
