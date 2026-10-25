#[doc = "Register `ERR_CTRL2` reader"]
pub type R = crate::R<ErrCtrl2Spec>;
#[doc = "Register `ERR_CTRL2` writer"]
pub type W = crate::W<ErrCtrl2Spec>;
#[doc = "Field `ECC_BIT1` reader - Force Error Bit1 Column Index"]
pub type EccBit1R = crate::FieldReader<u16>;
#[doc = "Field `ECC_BIT1` writer - Force Error Bit1 Column Index"]
pub type EccBit1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16, crate::Safe>;
#[doc = "Field `ECC_BIT2` reader - Force Error Bit2 Column Index"]
pub type EccBit2R = crate::FieldReader<u16>;
#[doc = "Field `ECC_BIT2` writer - Force Error Bit2 Column Index"]
pub type EccBit2W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16, crate::Safe>;
impl R {
    #[doc = "Bits 0:15 - Force Error Bit1 Column Index"]
    #[inline(always)]
    pub fn ecc_bit1(&self) -> EccBit1R {
        EccBit1R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Force Error Bit2 Column Index"]
    #[inline(always)]
    pub fn ecc_bit2(&self) -> EccBit2R {
        EccBit2R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ERR_CTRL2")
            .field("ecc_bit1", &self.ecc_bit1())
            .field("ecc_bit2", &self.ecc_bit2())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - Force Error Bit1 Column Index"]
    #[inline(always)]
    pub fn ecc_bit1(&mut self) -> EccBit1W<'_, ErrCtrl2Spec> {
        EccBit1W::new(self, 0)
    }
    #[doc = "Bits 16:31 - Force Error Bit2 Column Index"]
    #[inline(always)]
    pub fn ecc_bit2(&mut self) -> EccBit2W<'_, ErrCtrl2Spec> {
        EccBit2W::new(self, 16)
    }
}
#[doc = "MCAN ECC Error Control 2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`err_ctrl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`err_ctrl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ErrCtrl2Spec;
impl crate::RegisterSpec for ErrCtrl2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`err_ctrl2::R`](R) reader structure"]
impl crate::Readable for ErrCtrl2Spec {}
#[doc = "`write(|w| ..)` method takes [`err_ctrl2::W`](W) writer structure"]
impl crate::Writable for ErrCtrl2Spec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets ERR_CTRL2 to value 0"]
impl crate::Resettable for ErrCtrl2Spec {}
