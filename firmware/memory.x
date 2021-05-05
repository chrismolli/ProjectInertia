/*
  These definitions are necessary for the linker to place
  the sections in the right place of the MSP430FR6972
*/

MEMORY{
  RAM : ORIGIN = 0x1C00, LENGTH = 0x0800
  ROM : ORIGIN = 0x4400, LENGTH = 0xBB80
  VECTORS : ORIGIN = 0xFF90, LENGTH = 0x70
}
