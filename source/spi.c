
#if (defined YOTTA_CFG_HARDWARE_SPI) && (YOTTA_CFG_HARDWARE_SPI_COUNT > 0)
#include "kubos-hal/spi.h"
#include <string.h>

static KSPI k_spis[K_NUM_SPI];

void k_spi_init(KSPINum spi, KSPIConf * conf)
{
    KSPI * k_spi = kprv_spi_get(spi);
    memcpy(&k_spi->config, conf, sizeof(KSPIConf));

    k_spi->bus_num = spi;
    k_spi->spi_lock = xSemaphoreCreateMutex();
    kprv_spi_dev_init(spi);
}

void k_spi_terminate(KSPINum spi)
{
    kprv_spi_dev_terminate(spi);
}

KSPIConf k_spi_conf_defaults(void)
{
    return (KSPIConf) {
        .role = YOTTA_CFG_HARDWARE_SPI_DEFAULTS_ROLE,
        .direction = YOTTA_CFG_HARDWARE_SPI_DEFAULTS_DIRECTION,
        .data_size = YOTTA_CFG_HARDWARE_SPI_DEFAULTS_DATASIZE,
        .clock_phase = YOTTA_CFG_HARDWARE_SPI_DEFAULTS_CLOCKPHASE,
        .clock_polarity = YOTTA_CFG_HARDWARE_SPI_DEFAULTS_CLOCKPOLARITY,
        .first_bit = YOTTA_CFG_HARDWARE_SPI_DEFAULTS_FIRSTBIT,
        .speed = YOTTA_CFG_HARDWARE_SPI_DEFAULTS_SPEED
    };
}

void k_spi_default_init()
{
    KSPIConf conf = k_spi_conf_defaults();
    k_spi_init(DEFAULT_SPI, &conf);
}

KSPIStatus k_spi_write(KSPINum spi, uint8_t * buffer, uint32_t len)
{
    KSPI * kspi = kprv_spi_get(spi);
    KSPIStatus ret = SPI_ERROR;
    if (kspi->spi_lock != NULL)
    {
        // Today...block indefinitely
        if (xSemaphoreTake(kspi->spi_lock, (TickType_t)portMAX_DELAY) == pdTRUE)
        {
            ret = kprv_spi_write(spi, buffer, len);
            xSemaphoreGive(kspi->spi_lock);
        }
    }
    return ret;
}

KSPIStatus k_spi_read(KSPINum spi, uint8_t * buffer, uint32_t len)
{
    KSPI * kspi = kprv_spi_get(spi);
    KSPIStatus ret = SPI_ERROR;
    if (kspi->spi_lock != NULL)
    {
        // Today...block indefinitely
        if (xSemaphoreTake(kspi->spi_lock, (TickType_t)portMAX_DELAY) == pdTRUE)
        {
            ret = kprv_spi_read(spi, buffer, len);
            xSemaphoreGive(kspi->spi_lock);
        }
    }
    return ret;
}

KSPIStatus k_spi_write_read(KSPINum spi, uint8_t * txBuffer, uint8_t * rxBuffer, uint32_t len)
{
    KSPI * kspi = kprv_spi_get(spi);
    KSPIStatus ret = SPI_ERROR;
    if (kspi->spi_lock != NULL)
    {
        // Today...block indefinitely
        if (xSemaphoreTake(kspi->spi_lock, (TickType_t)portMAX_DELAY) == pdTRUE)
        {
            ret = kprv_spi_write_read(spi, txBuffer, rxBuffer, len);
            xSemaphoreGive(kspi->spi_lock);
        }
    }
    return ret;
}

KSPI * kprv_spi_get(KSPINum spi)
{
	if(spi > K_NUM_SPI-1)
	{
		return 0;
	}
    return &k_spis[spi];
}

#endif
