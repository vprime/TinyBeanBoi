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
    .global next_edge
next_edge:
    .long 0

    /* counter started when signal value changes.
     * Edge is debounced when the coutner reaches zero. */
    .global debounce_counter
debounce_counter:
    .long 0

    /* Value to which debounce_counter gets reset.
     * Set by the main program. */
    .global debounce_max_count
debounce_max_count:
    .long 0

    /* Total number of signal edges acquired */
    .global edge_count
edge_count:
    .long 0

    /* Number of edges to acquire before waking up the SoC.
     * Set by the main program */
    .global edge_count_to_wake_up
edge_count_to_wake_up:
    .long 0

    /* RTC IO number used to sample the input signal.
     * Set by the main program */
    .global io_number
io_number:
    .long 0

    /* Code goes into the .text section */
    .text
    .global entry
entry:
    /* Load the io_number */
    move r3, io_number
    ld r3, r3, 0

    /* lower 16 IOs and highter need to be handled separately
     * because r0-r3 registers are 16 bit wide.
     * check which IO this is.
     */
    move r0, r3
    jumpr read_io_high, 16, ge

    /* Read the value of lower 16 RTC IOs into R0 */
    READ_RTC_REG(RTC_GPIO_IN_REG, RTC_GPIO_IN_NEXT_S, 16)
    rsh r0, r0, r3
    jump read_done

    /* read the value of RTC IOs 16-17, into R0 */
read_io_high:
    READ_RTC_REG(RTC_GPIO_IN_REG, RTC_GPIO_IN_NEXT_S + 16, 2)
    sub r3, r3, 16
    rsh r0, r0, r3

read_done:
    and r0, r0, 1
    /* State of input changed? */
    move r3, next_edge
    ld r3, r3, 0
    add r3, r0, r3
    and r3, r3, 1
    jump changed, eq
    /* Not changed */
    /* Reset debounce counter to debounce max count */
    move r3, debounce_max_count
    move r2, debounce_counter
    ld r3, r3, 0
    st r3, r2, 0
    /* end program */
    halt

    .global changed
changed:
    /* Input state changed */
    /* Has the debounce counter reached zero? */
    move r3, debounce_counter
    ld r2, r3, 0
    add r2, r2, 0 /* Dummy add to use jump if ALU result is zero */
    jump edge_detected, eq
    /* Not yet, decrement the debounce counter */
    sub r2, r2, 1
    st r2, r3, 0
    /* end program */
    halt

    .global edge_detected
edge_detected:
    /* reset debounce counter to debounce max count */
    move r3, debounce_max_count
    move r2, debounce_counter
    ld r3, r3, 0
    st r3, r2, 0
    /* flip next edge */
    move r3, next_edge
    ld r2, r3, 0
    add r2, r2, 1
    and r2, r2, 1
    st r2, r3, 0
    /* increment edge count */
    move r3, edge_count
    ld r2, r3, 0
    add r2, r2, 1
    st r2, r3, 0
    /* compare edge count to edge count wake up */
    move r3, edge_count_to_wake_up
    ld r3, r3, 0
    sub r3, r3, r2
    jump wake_up, eq
    /* not yet ready to wake up, end program */
    halt

    .global wake_up
wake_up:
    /* Check if the system can be woken up */
    READ_RTC_FIELD(RTC_CNTL_LOW_POWER_ST_REG, RTC_CNTL_RDY_FOR_WAKEUP)
    /* If the system is in normal mode or if the system is in sleep mode with ready for wakeup set, we can signal the main CPU to wakeup */
    and r0, r0, 1
    jump wake_up, eq
    /* Wake up the SoC, end program */
    wake
    halt
