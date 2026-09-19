/* XIAO nRF54L15 (nrf54l15-app-ns) 用 memory.x */
MEMORY
{
  /* Non-Secure 領域の Flash と RAM の開始アドレスとサイズ */
  FLASH : ORIGIN = 0x00000000, LENGTH = 1524K
  RAM   : ORIGIN = 0x20000000, LENGTH = 256K
}

/* スタック割り当ての設定 */
_stack_start = ORIGIN(RAM) + LENGTH(RAM);