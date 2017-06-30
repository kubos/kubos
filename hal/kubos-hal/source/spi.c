
#if (defined YOTTA_CFG_HARDWARE_SPI) && (YOTTA_CFG_HARDWARE_SPI_COUNT > 0)
#include "kubos-hal/spi.h"
#include <string.h>

static KSPI k_spis[K_NUM_SPI];

void k_spi_init(KSPINum spi, KSPIConf * conf)
{
    KSPI * k_spi = kprv_spi_get(spi);
    if (k_spi->bus_num == K_SPI_NO_BUS)
    {
        memcpy(&k_spi->config, conf, sizeof(KSPIConf));

        k_spi->bus_num = spi;
        csp_mutex_create(&(k_spi->spi_lock));
        kprv_spi_dev_init(spi);
    }
}

void k_spi_terminate(KSPINum spi)
{
    KSPI * k_spi = kprv_spi_get(spi);
    kprv_spi_dev_terminate(spi);
    csp_mutex_remove(&(k_spi->spi_lock));
    k_spi->bus_num = K_SPI_NO_BUS;
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
    if ((kspi->bus_num != K_SPI_NO_BUS) && (buffer != NULL))
    {
        if (csp_mutex_lock(&(kspi->spi_lock), CSP_MAX_DELAY) == CSP_SEMAPHORE_OK)
        {
            ret = kprv_spi_write(spi, buffer, len);
            csp_mutex_unlock(&(kspi->spi_lock));
        }
    }
    return ret;
}

KSPIStatus k_spi_read(KSPINum spi, uint8_t * buffer, uint32_t len)
{
    KSPI * kspi = kprv_spi_get(spi);
    KSPIStatus ret = SPI_ERROR;
    if ((kspi->bus_num != K_SPI_NO_BUS) && (buffer != NULL))
    {
        if (csp_mutex_lock(&(kspi->spi_lock), CSP_MAX_DELAY) == CSP_SEMAPHORE_OK)
        {
            ret = kprv_spi_read(spi, buffer, len);
            csp_mutex_unlock(&(kspi->spi_lock));
        }
    }
    return ret;
}

KSPIStatus k_spi_write_read(KSPINum spi, uint8_t * txBuffer, uint8_t * rxBuffer, uint32_t len)
{
    KSPI * kspi = kprv_spi_get(spi);
    KSPIStatus ret = SPI_ERROR;
    if (kspi->bus_num != K_SPI_NO_BUS)
    {
        if (csp_mutex_lock(&(kspi->spi_lock), CSP_MAX_DELAY) == CSP_SEMAPHORE_OK)
        {
            ret = kprv_spi_write_read(spi, txBuffer, rxBuffer, len);
            csp_mutex_unlock(&(kspi->spi_lock));
        }
    }
    return ret;
}

KSPI * kprv_spi_get(KSPINum spi)
{
	if(spi > K_NUM_SPI)
	{
		return NULL;
	}
    return &k_spis[spi - 1];
}

#endif
