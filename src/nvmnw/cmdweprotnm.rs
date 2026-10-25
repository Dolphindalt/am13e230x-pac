#[doc = "Register `CMDWEPROTNM` reader"]
pub type R = crate::R<CmdweprotnmSpec>;
#[doc = "Register `CMDWEPROTNM` writer"]
pub type W = crate::W<CmdweprotnmSpec>;
#[doc = "Field `VAL` reader - Each bit protects 1 sector. bit \\[0\\]: When 1, sector 0 of the non-main region will be protected from program and erase. bit \\[1\\]: When 1, sector 1 of the non-main region will be protected from program and erase. : : bit \\[31\\]: When 1, sector 31 of the non-main will be protected from program and erase."]
pub type ValR = crate::FieldReader;
#[doc = "Field `VAL` writer - Each bit protects 1 sector. bit \\[0\\]: When 1, sector 0 of the non-main region will be protected from program and erase. bit \\[1\\]: When 1, sector 1 of the non-main region will be protected from program and erase. : : bit \\[31\\]: When 1, sector 31 of the non-main will be protected from program and erase."]
pub type ValW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - Each bit protects 1 sector. bit \\[0\\]: When 1, sector 0 of the non-main region will be protected from program and erase. bit \\[1\\]: When 1, sector 1 of the non-main region will be protected from program and erase. : : bit \\[31\\]: When 1, sector 31 of the non-main will be protected from program and erase."]
    #[inline(always)]
    pub fn val(&self) -> ValR {
        ValR::new((self.bits & 3) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMDWEPROTNM")
            .field("val", &self.val())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - Each bit protects 1 sector. bit \\[0\\]: When 1, sector 0 of the non-main region will be protected from program and erase. bit \\[1\\]: When 1, sector 1 of the non-main region will be protected from program and erase. : : bit \\[31\\]: When 1, sector 31 of the non-main will be protected from program and erase."]
    #[inline(always)]
    pub fn val(&mut self) -> ValW<'_, CmdweprotnmSpec> {
        ValW::new(self, 0)
    }
}
#[doc = "Command Write Erase Protect Non-Main Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cmdweprotnm::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmdweprotnm::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CmdweprotnmSpec;
impl crate::RegisterSpec for CmdweprotnmSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmdweprotnm::R`](R) reader structure"]
impl crate::Readable for CmdweprotnmSpec {}
#[doc = "`write(|w| ..)` method takes [`cmdweprotnm::W`](W) writer structure"]
impl crate::Writable for CmdweprotnmSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CMDWEPROTNM to value 0"]
impl crate::Resettable for CmdweprotnmSpec {}
