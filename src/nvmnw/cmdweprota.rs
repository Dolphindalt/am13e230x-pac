#[doc = "Register `CMDWEPROTA` reader"]
pub type R = crate::R<CmdweprotaSpec>;
#[doc = "Register `CMDWEPROTA` writer"]
pub type W = crate::W<CmdweprotaSpec>;
#[doc = "Field `VAL` reader - Each bit protects 1 sector. bit \\[0\\]: When 1, sector 0 of the flash memory will be protected from program and erase. bit \\[1\\]: When 1, sector 1 of the flash memory will be protected from program and erase. : : bit \\[31\\]: When 1, sector 31 of the flash memory will be protected from program and erase."]
pub type ValR = crate::FieldReader<u32>;
#[doc = "Field `VAL` writer - Each bit protects 1 sector. bit \\[0\\]: When 1, sector 0 of the flash memory will be protected from program and erase. bit \\[1\\]: When 1, sector 1 of the flash memory will be protected from program and erase. : : bit \\[31\\]: When 1, sector 31 of the flash memory will be protected from program and erase."]
pub type ValW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Each bit protects 1 sector. bit \\[0\\]: When 1, sector 0 of the flash memory will be protected from program and erase. bit \\[1\\]: When 1, sector 1 of the flash memory will be protected from program and erase. : : bit \\[31\\]: When 1, sector 31 of the flash memory will be protected from program and erase."]
    #[inline(always)]
    pub fn val(&self) -> ValR {
        ValR::new(self.bits)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMDWEPROTA")
            .field("val", &self.val())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Each bit protects 1 sector. bit \\[0\\]: When 1, sector 0 of the flash memory will be protected from program and erase. bit \\[1\\]: When 1, sector 1 of the flash memory will be protected from program and erase. : : bit \\[31\\]: When 1, sector 31 of the flash memory will be protected from program and erase."]
    #[inline(always)]
    pub fn val(&mut self) -> ValW<'_, CmdweprotaSpec> {
        ValW::new(self, 0)
    }
}
#[doc = "Command Write Erase Protect A Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cmdweprota::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmdweprota::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CmdweprotaSpec;
impl crate::RegisterSpec for CmdweprotaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmdweprota::R`](R) reader structure"]
impl crate::Readable for CmdweprotaSpec {}
#[doc = "`write(|w| ..)` method takes [`cmdweprota::W`](W) writer structure"]
impl crate::Writable for CmdweprotaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CMDWEPROTA to value 0"]
impl crate::Resettable for CmdweprotaSpec {}
