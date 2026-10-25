#[doc = "Register `RSIZE1` reader"]
pub type R = crate::R<Rsize1Spec>;
#[doc = "Register `RSIZE1` writer"]
pub type W = crate::W<Rsize1Spec>;
#[doc = "Current Size\n\nValue on reset: 3"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Size {
    #[doc = "1: Byte (8 bits)"]
    Size8Bit = 1,
    #[doc = "2: Half-word (16 bits)"]
    Size16Bit = 2,
    #[doc = "3: Word (32 bits)"]
    Size32Bit = 3,
}
impl From<Size> for u8 {
    #[inline(always)]
    fn from(variant: Size) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Size {
    type Ux = u8;
}
impl crate::IsEnum for Size {}
#[doc = "Field `SIZE` reader - Current Size"]
pub type SizeR = crate::FieldReader<Size>;
impl SizeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Size> {
        match self.bits {
            1 => Some(Size::Size8Bit),
            2 => Some(Size::Size16Bit),
            3 => Some(Size::Size32Bit),
            _ => None,
        }
    }
    #[doc = "Byte (8 bits)"]
    #[inline(always)]
    pub fn is_size_8_bit(&self) -> bool {
        *self == Size::Size8Bit
    }
    #[doc = "Half-word (16 bits)"]
    #[inline(always)]
    pub fn is_size_16_bit(&self) -> bool {
        *self == Size::Size16Bit
    }
    #[doc = "Word (32 bits)"]
    #[inline(always)]
    pub fn is_size_32_bit(&self) -> bool {
        *self == Size::Size32Bit
    }
}
#[doc = "Field `SIZE` writer - Current Size"]
pub type SizeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Size, crate::Safe>;
impl<'a, REG> SizeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Byte (8 bits)"]
    #[inline(always)]
    pub fn size_8_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Size::Size8Bit)
    }
    #[doc = "Half-word (16 bits)"]
    #[inline(always)]
    pub fn size_16_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Size::Size16Bit)
    }
    #[doc = "Word (32 bits)"]
    #[inline(always)]
    pub fn size_32_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Size::Size32Bit)
    }
}
impl R {
    #[doc = "Bits 0:1 - Current Size"]
    #[inline(always)]
    pub fn size(&self) -> SizeR {
        SizeR::new((self.bits & 3) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RSIZE1")
            .field("size", &self.size())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - Current Size"]
    #[inline(always)]
    pub fn size(&mut self) -> SizeW<'_, Rsize1Spec> {
        SizeW::new(self, 0)
    }
}
#[doc = "EPI Read Size 1\n\nYou can [`read`](crate::Reg::read) this register and get [`rsize1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rsize1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rsize1Spec;
impl crate::RegisterSpec for Rsize1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rsize1::R`](R) reader structure"]
impl crate::Readable for Rsize1Spec {}
#[doc = "`write(|w| ..)` method takes [`rsize1::W`](W) writer structure"]
impl crate::Writable for Rsize1Spec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets RSIZE1 to value 0x03"]
impl crate::Resettable for Rsize1Spec {
    const RESET_VALUE: u32 = 0x03;
}
