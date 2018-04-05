/*
 * Copyright (C) 2018 Kubos Corporation
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 *
 * ISIS iMTQ API - Core Functions and Configuration Commands
 */

#include <isis-imtq-api/imtq.h>
#include <kubos-hal/i2c.h>
#include <pthread.h>
#include <stdio.h>
#include <sys/syscall.h>
#include <time.h>
#include <unistd.h>

pthread_mutex_t imtq_mutex;

/**
 * I2C bus the iMTQ is connected to
 */
static KI2CNum i2c_bus = K_I2C1;

/**
 * iMTQ I2C address
 */
static uint16_t imqt_addr = 0x10;

/**
 * Watchdog timeout (in seconds)
 */
static int wd_timeout = 60;

KADCSStatus k_adcs_init(KI2CNum bus, uint16_t addr, int timeout)
{
    /*
     * All I2C configuration is done at the kernel level,
     * but we still need to pass a config structure to make
     * our I2C API happy.
     */
    KI2CConf conf = k_i2c_conf_defaults();
    i2c_bus = bus;
    imqt_addr = addr;
    wd_timeout = timeout;

    KI2CStatus status;
    status = k_i2c_init(i2c_bus, &conf);
    if (status != I2C_OK)
    {
        fprintf(stderr, "Failed to initialize iMTQ: %d\n", status);
        return ADCS_ERROR;
    }

    pthread_mutexattr_t mutex_attr;
    if (pthread_mutexattr_settype(&mutex_attr, PTHREAD_MUTEX_ERRORCHECK) != 0)
    {
        perror("Failed to set up MTQ mutex attr");
        k_adcs_terminate();
        return ADCS_ERROR_MUTEX;
    }
    if (pthread_mutex_init(&imtq_mutex, &mutex_attr) != 0)
    {
        perror("Failed to set up MTQ mutex");
        k_adcs_terminate();
        return ADCS_ERROR_MUTEX;
    }

    KADCSStatus imtq_status;

    /* Call noop to verify iMTQ is online */
    imtq_status = k_adcs_noop();
    if (imtq_status != ADCS_OK)
    {
        fprintf(stderr, "Failed to verify iMTQ is online: %d\n", imtq_status);
        k_adcs_terminate();
        return ADCS_ERROR;
    }

    return ADCS_OK;
}

void k_adcs_terminate(void)
{
    const struct timespec MUTEX_TIMEOUT = {.tv_sec = 1, .tv_nsec = 0 };

    /* Destroy the mutex */
    if (pthread_mutex_timedlock(&imtq_mutex, &MUTEX_TIMEOUT) != 0)
    {
        perror("Failed to take MTQ mutex");
        fprintf(stderr, "PID: %d TID: %ld", getpid(), syscall(SYS_gettid));
    }
    if (pthread_mutex_unlock(&imtq_mutex) != 0)
    {
        perror("Failed to unlock MTQ mutex");
        fprintf(stderr, "PID: %d TID: %ld", getpid(), syscall(SYS_gettid));
    }
    if (pthread_mutex_destroy(&imtq_mutex) != 0)
    {
        perror("Failed to destroy MTQ mutex");
        fprintf(stderr, "PID: %d TID: %ld", getpid(), syscall(SYS_gettid));
    }

    /* Close the I2C bus */
    k_i2c_terminate(i2c_bus);

    return;
}

/*
 * Pass a custom command packet directly through to the iMTQ
 */
KADCSStatus k_adcs_passthrough(const uint8_t * tx, int tx_len, uint8_t * rx,
                               int rx_len, const struct timespec * delay)
{
    return kprv_imtq_transfer(tx, tx_len, rx, rx_len, delay);
}

/*
 * This is a special transfer case. We have to wait way longer before trying
 * to get a response (since the system was rebooting)
 */

void * kprv_imtq_watchdog_thread(void * args)
{
    KADCSStatus status;

    while (1)
    {
        k_adcs_noop();

        sleep(wd_timeout / 3);
    }

    return NULL;
}

pthread_t handle_watchdog = { 0 };

KADCSStatus k_imtq_watchdog_start(void)
{
    if (handle_watchdog != 0)
    {
        fprintf(stderr, "ADCS watchdog thread already started\n");
        return ADCS_OK;
    }

    if (wd_timeout == 0)
    {
        fprintf(
            stderr,
            "ADCS watchdog has been disabled. No thread will be startd\n");
        return ADCS_OK;
    }

    if (pthread_create(&handle_watchdog, NULL, kprv_imtq_watchdog_thread, NULL)
        != 0)
    {
        perror("Failed to create ADCS watchdog thread");
        handle_watchdog = 0;
        return ADCS_ERROR;
    }

    return ADCS_OK;
}

KADCSStatus k_imtq_watchdog_stop(void)
{
    /* Send the cancel request */
    if (pthread_cancel(handle_watchdog) != 0)
    {
        perror("Failed to cancel ADCS watchdog thread");
        return ADCS_ERROR;
    }

    /* Wait for the cancellation to complete */
    if (pthread_join(handle_watchdog, NULL) != 0)
    {
        perror("Failed to rejoin ADCS watchdog thread");
        return ADCS_ERROR;
    }

    handle_watchdog = 0;

    return ADCS_OK;
}

KADCSStatus k_imtq_reset(void)
{
    return k_adcs_reset(SOFT_RESET);
}

KADCSStatus kprv_imtq_transfer(const uint8_t * tx, int tx_len, uint8_t * rx,
                               int rx_len, const struct timespec * delay)
{
    KI2CStatus status;

    const struct timespec MUTEX_TIMEOUT = {.tv_sec = 1, .tv_nsec = 0 };

    if (tx == NULL || tx_len < 1 || rx == NULL
        || rx_len < (int) sizeof(imtq_resp_header))
    {
        return ADCS_ERROR_CONFIG;
    }

    if (pthread_mutex_timedlock(&imtq_mutex, &MUTEX_TIMEOUT) != 0)
    {
        perror("Failed to take MTQ mutex");
        fprintf(stderr, "PID: %d TID: %ld", getpid(), syscall(SYS_gettid));
        return ADCS_ERROR_MUTEX;
    }

    status = k_i2c_write(i2c_bus, imqt_addr, (uint8_t *) tx, tx_len);
    if (status != I2C_OK)
    {
        fprintf(stderr, "Failed to send MTQ command: %d\n", status);
        if (pthread_mutex_unlock(&imtq_mutex) != 0)
        {
            perror("Failed to unlock MTQ mutex");
            fprintf(stderr, "PID: %d TID: %ld", getpid(), syscall(SYS_gettid));
        }
        return ADCS_ERROR;
    }

    if (delay == NULL)
    {
        /* There must be at least a 1ms delay in-between each I2C transfer */
        const struct timespec TRANSFER_DELAY
            = {.tv_sec = 0, .tv_nsec = 1000001 };

        nanosleep(&TRANSFER_DELAY, NULL);
    }
    else
    {
        /* Wait the requested amount of time before fetching the response */
        nanosleep(delay, NULL);
    }

    status = k_i2c_read(i2c_bus, imqt_addr, rx, rx_len);

    if (pthread_mutex_unlock(&imtq_mutex) != 0)
    {
        perror("Failed to unlock MTQ mutex");
        fprintf(stderr, "PID: %d TID: %ld", getpid(), syscall(SYS_gettid));
    }

    if (status != I2C_OK)
    {
        fprintf(stderr, "Failed to read MTQ response (%x): %d\n", tx[0],
                status);
        return ADCS_ERROR;
    }

    imtq_resp_header response = {.cmd = rx[0], .status = rx[1] };

    if (response.cmd == 0xFF)
    {
        /*
         * This isn't always an error, so we'll let the caller decide whether
         * or not to print an error message
         */
        return ADCS_ERROR_NO_RESPONSE;
    }
    else if (response.cmd != tx[0])
    {
        /* Echoed command should match command requested */
        fprintf(stderr, "Command mismatch - Sent: %x Received: %x\n", tx[0],
                response.cmd);
        return ADCS_ERROR;
    }

    /* Check the iMTQ's return code */
    KIMTQStatus imtq_status = kprv_imtq_check_error(response.status);
    if (imtq_status != IMTQ_OK)
    {
        fprintf(stderr, "iMTQ returned an error (%x): %d\n", tx[0],
                imtq_status);
        return ADCS_ERROR_INTERNAL;
    }

    return ADCS_OK;
}
