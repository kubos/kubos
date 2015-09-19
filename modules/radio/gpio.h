/**
 * Title:   gpio.h
 *
 * Author:  Andrew Montag
 *          ajmontag@gmail.com
 *          sites.google.com/site/andrewmontag
 *
 * Licence: Boost Software Licence - Verison 1.0
 *          http://www.boost.org/users/license.html
 *
 * Purpose:
 * Helpers for configuring and accessing GPIO pins.
 */

#ifndef _GPIO_H_
#define _GPIO_H_

/**
 * @ return 0 on success
 *
 * @param gpio the gpio pin number.
 *  If the pin is GPIOm_n, then the pin number is
 *  m * 32 + n. Example: GPIO3_21 = 3*32+21 = 117
 */

/**
 * gpio_export
 * export a gpio pin for use in the user space.
 * must be called before the pin can be used.
 */
int gpio_export(unsigned int gpio);


/**
 * gpio_unexport
 * undo the export action.
 */
int gpio_unexport(unsigned int gpio);

#define GPIO_DIR_INPUT  (0)
#define GPIO_DIR_OUTPUT (1)

/**
 * gpio_set_dir
 * @param out_flag true=output, false=input
 */
int gpio_set_dir(unsigned int gpio, unsigned int out_flag);


/**
 * gpio_set_value
 * writes the boolean value to the pin.
 */
int gpio_set_value(unsigned int gpio, unsigned int value);


/**
 * gpio_get_value
 * reads the state of the pin.
 * @param as return, 0 or 1
 */
int gpio_get_value(unsigned int gpio, unsigned int *value);

/** @param fd an fd opened using gpio_fd_open */
int gpio_get_value_fd(int fd, unsigned int *value);


//static const char* kPollEdge_rising = "rising";
//static const char* kPollEdge_falling = "falling";
//static const char* kPollEdge_both = "both";

/**
 * gpio_set_edge
 * @param edge should be "rising", "falling", or "both"
 */
int gpio_set_edge(unsigned int gpio, const char *edge);


/**
 * gpio_fd_open
 * @return an open an fd for later use.
 * useful when using poll().
 */
int gpio_fd_open(unsigned int gpio);


/**
 * gpio_fd_close
 * close an open fd.
 */
int gpio_fd_close(int fd);

#endif