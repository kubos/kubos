
#define CSP_POSIX
#define YOTTA_CFG_HARDWARE_I2C
#define YOTTA_CFG_HARDWARE_I2C_COUNT 1
#define YOTTA_CFG_HARDWARE_I2C_I2C1
#define YOTTA_CFG_HARDWARE_I2C_I2C1_DEVICE /dev/i2c-0

#include "kubos-hal/i2c.h"
#include <stdbool.h>

#define YOTTA_CFG_HARDWARE_I2C_DEFAULTS_BUS K_I2C1
#define YOTTA_CFG_HARDWARE_I2C_DEFAULTS_ROLE K_MASTER
#define YOTTA_CFG_HARDWARE_I2C_DEFAULTS_CLOCKSPEED 100000
#define YOTTA_CFG_HARDWARE_I2C_DEFAULTS_ADDRESSINGMODE K_ADDRESSINGMODE_7BIT

#include "hal/kubos-hal/source/i2c.c"
#include "hal/kubos-hal/source/spi.c"
#include "hal/kubos-hal/source/gpio.c"
#include "hal/kubos-hal/source/sdio.c"
#include "hal/kubos-hal/source/uart.c"
#include "hal/kubos-hal-linux/source/i2c.c"
#include "hal/kubos-hal-linux/source/uart.c"
#include "hal/kubos-hal-linux/source/spi.c"
#include "apis/isis-trxvu-api/source/radio_core.c"
#include "apis/isis-trxvu-api/source/radio_tx.c"
#include "apis/isis-trxvu-api/source/radio_rx.c"
#include "libcsp/source/arch/posix/csp_semaphore.c"
