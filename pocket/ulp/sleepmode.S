/*
 Goals:
Wake up on double tap of left button
Record number of taps on the right button

Main code will reboot on wakeup, it should look for a wakeup reason and load the number of taps from RTC_SLOW_MEM.

ULP Instruction Set
 https://docs.espressif.com/projects/esp-idf/en/latest/esp32/api-reference/system/ulp_instruction_set.html

ULP ADC Example https://github.com/espressif/esp-idf/tree/9106c43accd9f5e75379f62f12597677213f5023/examples/system/ulp/ulp_fsm/ulp_adc
ULP Example https://github.com/espressif/esp-idf/tree/9106c43accd9f5e75379f62f12597677213f5023/examples/system/ulp/ulp_fsm/ulp



*/
#include "sdkconfig.h"
#include "soc/rtc_cntl_reg.h"
#include "soc/rtc_io_reg.h"
#include "soc/soc_ulp.h"
#include "soc/sens_reg.h"

/* Define variables for bss section */
    .bss

    /* Next input signal edge expected: 0 (neg) or 1 (positive) */
    .global time
time:
    .long 0

    .global wakeup
wakeup:
    .long 0

    /* Code goes into the .text section */
    .text
    .global entry
entry:
    /* Trigger the register to update */
    WRITE_RTC_REG(RTC_CNTL_TIME_UPDATE_REG, RTC_CNTL_TIME_UPDATE_S, 1, 1)
    jump check_time_valid

check_time_valid:
    /* Check if RTC_CNTL_TIME_VALID bit is 1, otherwise repeat */
    READ_RTC_REG(RTC_CNTL_TIME_UPDATE_REG, RTC_CNTL_TIME_VALID_S, 1)
    AND R0, R0, 1
    JUMP check_time_valid, EQ

    /*REG_RD 0x3FF48010, 31, 16*/
    READ_RTC_REG(RTC_CNTL_TIME0_REG, 15, 16)
    MOVE R2, time
    ST R0, R2, 0
    MOVE R3, wakeup
    LD R1, R3, 0
    SUB R1, R0, R1
    JUMP wake_up, eq
    HALT

    .global exit
exit:
    halt

    .global wake_up
wake_up:
    READ_RTC_FIELD(RTC_CNTL_LOW_POWER_ST_REG, RTC_CNTL_RDY_FOR_WAKEUP)
    /* If the system can be woken, then wake up */
    and r0, r0, 1
    jump exit, eq

    /* Wake up the SoC, end program */
    wake
    halt
