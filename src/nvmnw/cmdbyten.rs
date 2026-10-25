#[doc = "Register `CMDBYTEN` reader"]
pub type R = crate::R<CmdbytenSpec>;
#[doc = "Register `CMDBYTEN` writer"]
pub type W = crate::W<CmdbytenSpec>;
#[doc = "Field `VAL` reader - Command Byte Enable value. A 1-bit per flash word byte value is placed in this register."]
pub type ValR = crate::FieldReader<u32>;
#[doc = "Field `VAL` writer - Command Byte Enable value. A 1-bit per flash word byte value is placed in this register."]
pub type ValW<'a, REG> = crate::FieldWriter<'a, REG, 18, u32>;
impl R {
    #[doc = "Bits 0:17 - Command Byte Enable value. A 1-bit per flash word byte value is placed in this register."]
    #[inline(always)]
    pub fn val(&self) -> ValR {
        ValR::new(self.bits & 0x0003_ffff)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMDBYTEN")
            .field("val", &self.val())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:17 - Command Byte Enable value. A 1-bit per flash word byte value is placed in this register."]
    #[inline(always)]
    pub fn val(&mut self) -> ValW<'_, CmdbytenSpec> {
        ValW::new(self, 0)
    }
}
#[doc = "Command Program Byte Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cmdbyten::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmdbyten::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CmdbytenSpec;
impl crate::RegisterSpec for CmdbytenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmdbyten::R`](R) reader structure"]
impl crate::Readable for CmdbytenSpec {}
#[doc = "`write(|w| ..)` method takes [`cmdbyten::W`](W) writer structure"]
impl crate::Writable for CmdbytenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CMDBYTEN to value 0"]
impl crate::Resettable for CmdbytenSpec {}
