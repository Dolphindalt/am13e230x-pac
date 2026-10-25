/* AM13E23019 (LP-AM13E230), per the AM13E230x TRM (SPRUJF2B) section 3.5 and
 * table 13-3. The AM13E23018 and AM13E23017 have the same map with 128K and
 * 64K flash banks; bank 1 always starts at 0x40000.
 */
MEMORY
{
  FLASH       : ORIGIN = 0x00000000, LENGTH = 256K

  /* On the AM13E23019 this bank is contiguous with bank 0, and FLASH may span both (512K). */
  FLASH_BANK1 : ORIGIN = 0x00040000, LENGTH = 256K

  /* SRAM banks 0-2, the only SRAM reachable by DMA and the NPU (TRM table 3-20). */
  RAM         : ORIGIN = 0x20000000, LENGTH = 96K

  /* SRAM bank 3, CPU only. Also at 0x20018000: link to one range, never both. */
  SRAM3       : ORIGIN = 0x00C18000, LENGTH = 32K
}

/* Address space that must not be a linker region:
 *
 *   0x01000000-0x0100BFFF  ROM, disabled once the application starts.
 *   0x60100000-0x60101FFF  NONMAIN boot configuration. An invalid erase or
 *                          write locks the device for good.
 *   0x60110000             FACTORY, read-only TI data. TI's FACTORYREGION
 *                          registers start at 0x60111000.
 *   0x60200000             ECC codes of MAIN; NONMAIN at 0x60260000, FACTORY
 *                          at 0x60270000.
 *   0x80000000-0xDFFFFFFF  External memory through the EPI. Add a region if the
 *                          board has memory there.
 */
