#[doc = "Register `CRCCTRL` reader"]
pub type R = crate::R<CrcctrlSpec>;
#[doc = "Register `CRCCTRL` writer"]
pub type W = crate::W<CrcctrlSpec>;
#[doc = "This bit indicates which CRC calculation is performed by the generator.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Polysize {
    #[doc = "0: CRC-32 ISO-3309 calulation is performed"]
    Crc32 = 0,
    #[doc = "1: CRC-16 CCITT is performed"]
    Crc16 = 1,
}
impl From<Polysize> for bool {
    #[inline(always)]
    fn from(variant: Polysize) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POLYSIZE` reader - This bit indicates which CRC calculation is performed by the generator."]
pub type PolysizeR = crate::BitReader<Polysize>;
impl PolysizeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Polysize {
        match self.bits {
            false => Polysize::Crc32,
            true => Polysize::Crc16,
        }
    }
    #[doc = "CRC-32 ISO-3309 calulation is performed"]
    #[inline(always)]
    pub fn is_crc32(&self) -> bool {
        *self == Polysize::Crc32
    }
    #[doc = "CRC-16 CCITT is performed"]
    #[inline(always)]
    pub fn is_crc16(&self) -> bool {
        *self == Polysize::Crc16
    }
}
#[doc = "Field `POLYSIZE` writer - This bit indicates which CRC calculation is performed by the generator."]
pub type PolysizeW<'a, REG> = crate::BitWriter<'a, REG, Polysize>;
impl<'a, REG> PolysizeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CRC-32 ISO-3309 calulation is performed"]
    #[inline(always)]
    pub fn crc32(self) -> &'a mut crate::W<REG> {
        self.variant(Polysize::Crc32)
    }
    #[doc = "CRC-16 CCITT is performed"]
    #[inline(always)]
    pub fn crc16(self) -> &'a mut crate::W<REG> {
        self.variant(Polysize::Crc16)
    }
}
#[doc = "CRC Bit Input and output Reverse. This bit indictes that the bit order of each input byte used for the CRC calculation is reversed before it is passed to the generator, and that the bit order of the calculated CRC is be reversed when read from CRC_RESULT.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bitreverse {
    #[doc = "0: Bit order is not reversed."]
    NotReversed = 0,
    #[doc = "1: Bit order is reversed."]
    Reversed = 1,
}
impl From<Bitreverse> for bool {
    #[inline(always)]
    fn from(variant: Bitreverse) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BITREVERSE` reader - CRC Bit Input and output Reverse. This bit indictes that the bit order of each input byte used for the CRC calculation is reversed before it is passed to the generator, and that the bit order of the calculated CRC is be reversed when read from CRC_RESULT."]
pub type BitreverseR = crate::BitReader<Bitreverse>;
impl BitreverseR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bitreverse {
        match self.bits {
            false => Bitreverse::NotReversed,
            true => Bitreverse::Reversed,
        }
    }
    #[doc = "Bit order is not reversed."]
    #[inline(always)]
    pub fn is_not_reversed(&self) -> bool {
        *self == Bitreverse::NotReversed
    }
    #[doc = "Bit order is reversed."]
    #[inline(always)]
    pub fn is_reversed(&self) -> bool {
        *self == Bitreverse::Reversed
    }
}
#[doc = "Field `BITREVERSE` writer - CRC Bit Input and output Reverse. This bit indictes that the bit order of each input byte used for the CRC calculation is reversed before it is passed to the generator, and that the bit order of the calculated CRC is be reversed when read from CRC_RESULT."]
pub type BitreverseW<'a, REG> = crate::BitWriter<'a, REG, Bitreverse>;
impl<'a, REG> BitreverseW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Bit order is not reversed."]
    #[inline(always)]
    pub fn not_reversed(self) -> &'a mut crate::W<REG> {
        self.variant(Bitreverse::NotReversed)
    }
    #[doc = "Bit order is reversed."]
    #[inline(always)]
    pub fn reversed(self) -> &'a mut crate::W<REG> {
        self.variant(Bitreverse::Reversed)
    }
}
#[doc = "CRC Endian. This bit indicates the byte order within a word or half word of input data.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InputEndianness {
    #[doc = "0: LSB is lowest memory address and first to be processed."]
    Little = 0,
    #[doc = "1: LSB is highest memory address and last to be processed."]
    Big = 1,
}
impl From<InputEndianness> for bool {
    #[inline(always)]
    fn from(variant: InputEndianness) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INPUT_ENDIANNESS` reader - CRC Endian. This bit indicates the byte order within a word or half word of input data."]
pub type InputEndiannessR = crate::BitReader<InputEndianness>;
impl InputEndiannessR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> InputEndianness {
        match self.bits {
            false => InputEndianness::Little,
            true => InputEndianness::Big,
        }
    }
    #[doc = "LSB is lowest memory address and first to be processed."]
    #[inline(always)]
    pub fn is_little(&self) -> bool {
        *self == InputEndianness::Little
    }
    #[doc = "LSB is highest memory address and last to be processed."]
    #[inline(always)]
    pub fn is_big(&self) -> bool {
        *self == InputEndianness::Big
    }
}
#[doc = "Field `INPUT_ENDIANNESS` writer - CRC Endian. This bit indicates the byte order within a word or half word of input data."]
pub type InputEndiannessW<'a, REG> = crate::BitWriter<'a, REG, InputEndianness>;
impl<'a, REG> InputEndiannessW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LSB is lowest memory address and first to be processed."]
    #[inline(always)]
    pub fn little(self) -> &'a mut crate::W<REG> {
        self.variant(InputEndianness::Little)
    }
    #[doc = "LSB is highest memory address and last to be processed."]
    #[inline(always)]
    pub fn big(self) -> &'a mut crate::W<REG> {
        self.variant(InputEndianness::Big)
    }
}
#[doc = "CRC Output Byteswap Enable. This bit controls whether the output is byte-swapped upon a read of the CRCOUT register. If CRCOUT is accessed as a half-word, and the OUTPUT_BYTESWAP is set to to 1, then the two bytes in the 16-bit access are swapped and returned. B1 is returned as B0 B0 is returned as B1 If CRCOUT is accessed as a word, and the OUTPUT_BYTESWAP is set to 1, then the four bytes in the 32-bit read are swapped. B3 is returned as B0 B2 is returned as B1 B1 is returned as B2 B0 is returned as B3 Note that if the CRC POLYSIZE is 16-bit and a 32-bit read of CRCOUT is performed with OUTPUT_BYTESWAP enabled, then the output is: MSB LSB 0x0 0x0 B0 B1 If the CRC POLYSIZE is 16-bit and a 32-bit read of CRCOUT is performed with OUTPUT_BYTESWAP disabled, then the output is: MSB LSB 0x0 0x0 B1 B0\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OutputByteswap {
    #[doc = "0: Output byteswapping is disabled"]
    Disable = 0,
    #[doc = "1: Output byteswapping is enabled."]
    Enable = 1,
}
impl From<OutputByteswap> for bool {
    #[inline(always)]
    fn from(variant: OutputByteswap) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OUTPUT_BYTESWAP` reader - CRC Output Byteswap Enable. This bit controls whether the output is byte-swapped upon a read of the CRCOUT register. If CRCOUT is accessed as a half-word, and the OUTPUT_BYTESWAP is set to to 1, then the two bytes in the 16-bit access are swapped and returned. B1 is returned as B0 B0 is returned as B1 If CRCOUT is accessed as a word, and the OUTPUT_BYTESWAP is set to 1, then the four bytes in the 32-bit read are swapped. B3 is returned as B0 B2 is returned as B1 B1 is returned as B2 B0 is returned as B3 Note that if the CRC POLYSIZE is 16-bit and a 32-bit read of CRCOUT is performed with OUTPUT_BYTESWAP enabled, then the output is: MSB LSB 0x0 0x0 B0 B1 If the CRC POLYSIZE is 16-bit and a 32-bit read of CRCOUT is performed with OUTPUT_BYTESWAP disabled, then the output is: MSB LSB 0x0 0x0 B1 B0"]
pub type OutputByteswapR = crate::BitReader<OutputByteswap>;
impl OutputByteswapR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OutputByteswap {
        match self.bits {
            false => OutputByteswap::Disable,
            true => OutputByteswap::Enable,
        }
    }
    #[doc = "Output byteswapping is disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == OutputByteswap::Disable
    }
    #[doc = "Output byteswapping is enabled."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == OutputByteswap::Enable
    }
}
#[doc = "Field `OUTPUT_BYTESWAP` writer - CRC Output Byteswap Enable. This bit controls whether the output is byte-swapped upon a read of the CRCOUT register. If CRCOUT is accessed as a half-word, and the OUTPUT_BYTESWAP is set to to 1, then the two bytes in the 16-bit access are swapped and returned. B1 is returned as B0 B0 is returned as B1 If CRCOUT is accessed as a word, and the OUTPUT_BYTESWAP is set to 1, then the four bytes in the 32-bit read are swapped. B3 is returned as B0 B2 is returned as B1 B1 is returned as B2 B0 is returned as B3 Note that if the CRC POLYSIZE is 16-bit and a 32-bit read of CRCOUT is performed with OUTPUT_BYTESWAP enabled, then the output is: MSB LSB 0x0 0x0 B0 B1 If the CRC POLYSIZE is 16-bit and a 32-bit read of CRCOUT is performed with OUTPUT_BYTESWAP disabled, then the output is: MSB LSB 0x0 0x0 B1 B0"]
pub type OutputByteswapW<'a, REG> = crate::BitWriter<'a, REG, OutputByteswap>;
impl<'a, REG> OutputByteswapW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output byteswapping is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(OutputByteswap::Disable)
    }
    #[doc = "Output byteswapping is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(OutputByteswap::Enable)
    }
}
impl R {
    #[doc = "Bit 0 - This bit indicates which CRC calculation is performed by the generator."]
    #[inline(always)]
    pub fn polysize(&self) -> PolysizeR {
        PolysizeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CRC Bit Input and output Reverse. This bit indictes that the bit order of each input byte used for the CRC calculation is reversed before it is passed to the generator, and that the bit order of the calculated CRC is be reversed when read from CRC_RESULT."]
    #[inline(always)]
    pub fn bitreverse(&self) -> BitreverseR {
        BitreverseR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CRC Endian. This bit indicates the byte order within a word or half word of input data."]
    #[inline(always)]
    pub fn input_endianness(&self) -> InputEndiannessR {
        InputEndiannessR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - CRC Output Byteswap Enable. This bit controls whether the output is byte-swapped upon a read of the CRCOUT register. If CRCOUT is accessed as a half-word, and the OUTPUT_BYTESWAP is set to to 1, then the two bytes in the 16-bit access are swapped and returned. B1 is returned as B0 B0 is returned as B1 If CRCOUT is accessed as a word, and the OUTPUT_BYTESWAP is set to 1, then the four bytes in the 32-bit read are swapped. B3 is returned as B0 B2 is returned as B1 B1 is returned as B2 B0 is returned as B3 Note that if the CRC POLYSIZE is 16-bit and a 32-bit read of CRCOUT is performed with OUTPUT_BYTESWAP enabled, then the output is: MSB LSB 0x0 0x0 B0 B1 If the CRC POLYSIZE is 16-bit and a 32-bit read of CRCOUT is performed with OUTPUT_BYTESWAP disabled, then the output is: MSB LSB 0x0 0x0 B1 B0"]
    #[inline(always)]
    pub fn output_byteswap(&self) -> OutputByteswapR {
        OutputByteswapR::new(((self.bits >> 4) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CRCCTRL")
            .field("polysize", &self.polysize())
            .field("bitreverse", &self.bitreverse())
            .field("input_endianness", &self.input_endianness())
            .field("output_byteswap", &self.output_byteswap())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - This bit indicates which CRC calculation is performed by the generator."]
    #[inline(always)]
    pub fn polysize(&mut self) -> PolysizeW<'_, CrcctrlSpec> {
        PolysizeW::new(self, 0)
    }
    #[doc = "Bit 1 - CRC Bit Input and output Reverse. This bit indictes that the bit order of each input byte used for the CRC calculation is reversed before it is passed to the generator, and that the bit order of the calculated CRC is be reversed when read from CRC_RESULT."]
    #[inline(always)]
    pub fn bitreverse(&mut self) -> BitreverseW<'_, CrcctrlSpec> {
        BitreverseW::new(self, 1)
    }
    #[doc = "Bit 2 - CRC Endian. This bit indicates the byte order within a word or half word of input data."]
    #[inline(always)]
    pub fn input_endianness(&mut self) -> InputEndiannessW<'_, CrcctrlSpec> {
        InputEndiannessW::new(self, 2)
    }
    #[doc = "Bit 4 - CRC Output Byteswap Enable. This bit controls whether the output is byte-swapped upon a read of the CRCOUT register. If CRCOUT is accessed as a half-word, and the OUTPUT_BYTESWAP is set to to 1, then the two bytes in the 16-bit access are swapped and returned. B1 is returned as B0 B0 is returned as B1 If CRCOUT is accessed as a word, and the OUTPUT_BYTESWAP is set to 1, then the four bytes in the 32-bit read are swapped. B3 is returned as B0 B2 is returned as B1 B1 is returned as B2 B0 is returned as B3 Note that if the CRC POLYSIZE is 16-bit and a 32-bit read of CRCOUT is performed with OUTPUT_BYTESWAP enabled, then the output is: MSB LSB 0x0 0x0 B0 B1 If the CRC POLYSIZE is 16-bit and a 32-bit read of CRCOUT is performed with OUTPUT_BYTESWAP disabled, then the output is: MSB LSB 0x0 0x0 B1 B0"]
    #[inline(always)]
    pub fn output_byteswap(&mut self) -> OutputByteswapW<'_, CrcctrlSpec> {
        OutputByteswapW::new(self, 4)
    }
}
#[doc = "CRC Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`crcctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crcctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrcctrlSpec;
impl crate::RegisterSpec for CrcctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crcctrl::R`](R) reader structure"]
impl crate::Readable for CrcctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`crcctrl::W`](W) writer structure"]
impl crate::Writable for CrcctrlSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CRCCTRL to value 0"]
impl crate::Resettable for CrcctrlSpec {}
