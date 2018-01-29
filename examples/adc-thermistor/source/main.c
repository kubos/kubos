/*
 * Kubos Linux
 * Copyright (C) 2017 Kubos Corporation
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
 * ADC demo code using a thermistor
 *
 * Options:
 *  -c: Take an ADC reading once a second until the program is exited with
 *      Ctrl+C
 *  -d: Print the debug messages
 */

#include <errno.h>
#include <fcntl.h>
#include <math.h>
#include <signal.h>
#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>

#define RESOLUTION 1023     /* 2^(# resolution bits) - 1 */
#define R_REF 10000         /* Fixed resistor value */
#define T_NOMINAL 25        /* Nominal temperature */
#define R_NOMINAL 10000     /* Resistance at nominal temperature */
#define BCOEFFICIENT 3950   /* Thermistor's beta coefficient */
#define V_CC 2.4            /* Supplied voltage */
#define V_REF 3.3           /* Internal reference voltage */

#define dbg(args...) \
    if (debug) fprintf(stderr, args)

int running;
int debug;

int therm_read_temperature(float * temp)
{
    int fd = open("/sys/bus/iio/devices/iio:device0/in_voltage1_raw", O_RDONLY);

    if (fd < 0)
    {
        printf("Error opening ADC raw file\n");
        return -1;
    }

    char val[4];

    /* Get the current ADC reading */

    int ret = read(fd, val, sizeof(val));

    if (ret != sizeof(val))
    {
        perror("Failed to read raw ADC value");
        close(fd);
        return -2;
    }

    close(fd);

    float raw = (float) atoi(val);

    /* Convert the raw ADC value into the thermistors current resistance value */

    float res = R_REF * (((RESOLUTION * raw) * (V_REF / V_CC)) - 1);

    dbg("Resistance: %f\n", res);

    /* Calculate the temperature using the B-parameter equation */

    *temp = res / R_NOMINAL;
    *temp = log(*temp);
    *temp /= BCOEFFICIENT;
    *temp += 1.0 / (T_NOMINAL + 273.15);
    *temp = 1.0 / *temp;
    *temp -= 273.15;

    printf("Temperature: %f\n", *temp);

    return 0;
}

void sigint_handler(int sig)
{
    running = 0;
    signal(SIGINT, SIG_DFL);
}

int main(int argc, char * argv[])
{
    int opt;

    running = 0;
    debug   = 0;

    while ((opt = getopt(argc, argv, "cd")) != -1)
    {
        switch (opt)
        {
            /* Option -c: get continuous readings */
            case 'c':
                running = 1;
                break;
            /* Enable debug lines */
            case 'd':
                debug = 1;
                break;
            default:
                printf("Unknown option: %c", opt);
        }
    }

    signal(SIGINT, sigint_handler);

    float temp;

    do
    {
        if (therm_read_temperature(&temp) != 0)
        {
            return -1;
        }

        sleep(1);
    } while (running);

    return 0;
}
