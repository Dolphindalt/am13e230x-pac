#[doc = "Register `QEPSRCSEL` reader"]
pub type R = crate::R<QepsrcselSpec>;
#[doc = "Register `QEPSRCSEL` writer"]
pub type W = crate::W<QepsrcselSpec>;
#[doc = "Field `QEPASEL` reader - QEPA Source select"]
pub type QepaselR = crate::FieldReader;
#[doc = "Field `QEPASEL` writer - QEPA Source select"]
pub type QepaselW<'a, REG> = crate::FieldWriter<'a, REG, 4, u8, crate::Safe>;
#[doc = "Field `QEPBSEL` reader - QEPB Source select"]
pub type QepbselR = crate::FieldReader;
#[doc = "Field `QEPBSEL` writer - QEPB Source select"]
pub type QepbselW<'a, REG> = crate::FieldWriter<'a, REG, 4, u8, crate::Safe>;
#[doc = "Field `QEPISEL` reader - QEPI Source select"]
pub type QepiselR = crate::FieldReader;
#[doc = "Field `QEPISEL` writer - QEPI Source select"]
pub type QepiselW<'a, REG> = crate::FieldWriter<'a, REG, 4, u8, crate::Safe>;
#[doc = "Field `QEPSSEL` reader - QEPS Source select"]
pub type QepsselR = crate::FieldReader;
#[doc = "Field `QEPSSEL` writer - QEPS Source select"]
pub type QepsselW<'a, REG> = crate::FieldWriter<'a, REG, 4, u8, crate::Safe>;
impl R {
    #[doc = "Bits 0:3 - QEPA Source select"]
    #[inline(always)]
    pub fn qepasel(&self) -> QepaselR {
        QepaselR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - QEPB Source select"]
    #[inline(always)]
    pub fn qepbsel(&self) -> QepbselR {
        QepbselR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - QEPI Source select"]
    #[inline(always)]
    pub fn qepisel(&self) -> QepiselR {
        QepiselR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - QEPS Source select"]
    #[inline(always)]
    pub fn qepssel(&self) -> QepsselR {
        QepsselR::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("QEPSRCSEL")
            .field("qepasel", &self.qepasel())
            .field("qepbsel", &self.qepbsel())
            .field("qepisel", &self.qepisel())
            .field("qepssel", &self.qepssel())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - QEPA Source select"]
    #[inline(always)]
    pub fn qepasel(&mut self) -> QepaselW<'_, QepsrcselSpec> {
        QepaselW::new(self, 0)
    }
    #[doc = "Bits 4:7 - QEPB Source select"]
    #[inline(always)]
    pub fn qepbsel(&mut self) -> QepbselW<'_, QepsrcselSpec> {
        QepbselW::new(self, 4)
    }
    #[doc = "Bits 8:11 - QEPI Source select"]
    #[inline(always)]
    pub fn qepisel(&mut self) -> QepiselW<'_, QepsrcselSpec> {
        QepiselW::new(self, 8)
    }
    #[doc = "Bits 12:15 - QEPS Source select"]
    #[inline(always)]
    pub fn qepssel(&mut self) -> QepsselW<'_, QepsrcselSpec> {
        QepsselW::new(self, 12)
    }
}
#[doc = "QEP Source Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`qepsrcsel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qepsrcsel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QepsrcselSpec;
impl crate::RegisterSpec for QepsrcselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`qepsrcsel::R`](R) reader structure"]
impl crate::Readable for QepsrcselSpec {}
#[doc = "`write(|w| ..)` method takes [`qepsrcsel::W`](W) writer structure"]
impl crate::Writable for QepsrcselSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets QEPSRCSEL to value 0"]
impl crate::Resettable for QepsrcselSpec {}
