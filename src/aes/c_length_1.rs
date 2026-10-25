#[doc = "Register `C_LENGTH_1` writer"]
pub type W = crate::W<CLength1Spec>;
#[doc = "Field `DATA` writer - Bits \\[60:0\\] of the crypto length registers (LSW and MSW) store the cryptographic data length in bytes for all modes. Once processing with this context is started, this length decrements to zero. Data lengths up to (261-1) bytes are allowed. For GCM, any value up to 236-32 bytes can be used. This is because a 32-bit counter mode is used; the maximum number of 128-bit blocks is 232-2, resulting in a maximum number of bytes of 236-32. A write to this register triggers the engine to start using this context. This is valid for all modes except GCM and CCM. Note that for the combined modes, this length does not include the authentication only data; the authentication length is specified in the AES_AAD_LENGTH register below. All modes must have a length > 0. For the combined modes, it is allowed to have one of the lengths equal to zero. For the basic encryption modes (ECB/CBC/CTR/ICM/CFB/OFB) it is allowed to program zero to the length field; in that case the length is assumed infinite. All data must be byte (8-bit) aligned for stream cipher modes; bit aligned data streams are not supported. For block cipher modes, the data length must be programmed in multiples of the block cipher size, 16 bytes."]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 29, u32, crate::Safe>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<CLength1Spec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:28 - Bits \\[60:0\\] of the crypto length registers (LSW and MSW) store the cryptographic data length in bytes for all modes. Once processing with this context is started, this length decrements to zero. Data lengths up to (261-1) bytes are allowed. For GCM, any value up to 236-32 bytes can be used. This is because a 32-bit counter mode is used; the maximum number of 128-bit blocks is 232-2, resulting in a maximum number of bytes of 236-32. A write to this register triggers the engine to start using this context. This is valid for all modes except GCM and CCM. Note that for the combined modes, this length does not include the authentication only data; the authentication length is specified in the AES_AAD_LENGTH register below. All modes must have a length > 0. For the combined modes, it is allowed to have one of the lengths equal to zero. For the basic encryption modes (ECB/CBC/CTR/ICM/CFB/OFB) it is allowed to program zero to the length field; in that case the length is assumed infinite. All data must be byte (8-bit) aligned for stream cipher modes; bit aligned data streams are not supported. For block cipher modes, the data length must be programmed in multiples of the block cipher size, 16 bytes."]
    #[inline(always)]
    pub fn data(&mut self) -> DataW<'_, CLength1Spec> {
        DataW::new(self, 0)
    }
}
#[doc = "Crypto data length (MSW)\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c_length_1::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLength1Spec;
impl crate::RegisterSpec for CLength1Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`c_length_1::W`](W) writer structure"]
impl crate::Writable for CLength1Spec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets C_LENGTH_1 to value 0"]
impl crate::Resettable for CLength1Spec {}
