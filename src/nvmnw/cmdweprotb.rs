#[doc = "Register `CMDWEPROTB` reader"]
pub type R = crate::R<CmdweprotbSpec>;
#[doc = "Register `CMDWEPROTB` writer"]
pub type W = crate::W<CmdweprotbSpec>;
#[doc = "Field `VAL` reader - Each bit protects a group of 8 sectors. When a bit is 1, the associated 8 sectors in the flash will be protected from program and erase. A maximum of 256 sectors can be protected with this register."]
pub type ValR = crate::FieldReader<u16>;
#[doc = "Field `VAL` writer - Each bit protects a group of 8 sectors. When a bit is 1, the associated 8 sectors in the flash will be protected from program and erase. A maximum of 256 sectors can be protected with this register."]
pub type ValW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Each bit protects a group of 8 sectors. When a bit is 1, the associated 8 sectors in the flash will be protected from program and erase. A maximum of 256 sectors can be protected with this register."]
    #[inline(always)]
    pub fn val(&self) -> ValR {
        ValR::new((self.bits & 0x0fff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMDWEPROTB")
            .field("val", &self.val())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:11 - Each bit protects a group of 8 sectors. When a bit is 1, the associated 8 sectors in the flash will be protected from program and erase. A maximum of 256 sectors can be protected with this register."]
    #[inline(always)]
    pub fn val(&mut self) -> ValW<'_, CmdweprotbSpec> {
        ValW::new(self, 0)
    }
}
#[doc = "Command Write Erase Protect B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cmdweprotb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmdweprotb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CmdweprotbSpec;
impl crate::RegisterSpec for CmdweprotbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmdweprotb::R`](R) reader structure"]
impl crate::Readable for CmdweprotbSpec {}
#[doc = "`write(|w| ..)` method takes [`cmdweprotb::W`](W) writer structure"]
impl crate::Writable for CmdweprotbSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CMDWEPROTB to value 0"]
impl crate::Resettable for CmdweprotbSpec {}
