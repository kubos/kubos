/**
 * \file libkiss.h
 * \brief Implementation of the KISS amatuer radio TNC protocol
 *
 * FROM adsllc/libkiss
 */

#ifndef LIBKISS_H
#define LIBKISS_H

/* Allow C++ to use our C-based library */
#ifdef __cplusplus
extern "C" {
#endif

/** \brief Frame End (frame delimiter) */
#define LIBKISS_FEND 0xC0
/** \brief Frame Escape (escape character) */
#define LIBKISS_FESC 0xDB
/** \brief Transposed Frame End */
#define LIBKISS_TFEND 0xDC
/** \brief Transposed Frame Escape */
#define LIBKISS_TFESC 0xDD

/** \brief Context handle (should not be accessed directly by clients) */
struct libkiss_struct {
	/** \brief Size of the data buffer */
	unsigned int bufsize;
	/** \brief The data buffer */
	unsigned char *buf;
	/** \brief Number of bytes being used */
	unsigned int nbytes;
};

/** \brief Keyboard saver */
typedef struct libkiss_struct *LIBKISS;

/** \brief Function codes */
enum libkiss_func_enum {
	LIBKISS_FUNC_DATA,			/**< \brief 0x00 Data packet */
	LIBKISS_FUNC_TXDELAY,		/**< \brief 0x01 Delay before transmit (10 ms units) */
	LIBKISS_FUNC_PERSISTENCE,	/**< \brief 0x02 Persistence = p * 256 - 1 */
	LIBKISS_FUNC_SLOTTIME,		/**< \brief 0x03 Slot interval (10ms units) */
	LIBKISS_FUNC_TXTAIL,		/**< \brief 0x04 Transmit tail delay (10ms units) \deprecated Obsolete per the spec */
	LIBKISS_FUNC_DUPLEX,		/**< \brief 0x05 Transmit/receive duplex \sa libkiss_duplex_enum*/
	LIBKISS_FUNC_HARDWARE,		/**< \brief 0x06 Hardware-specific data */
	LIBKISS_FUNC_RETURN=0xF,	/**< \brief 0x0F Exit KISS mode */
	LIBKISS_FUNC_INVALID		/**< \brief Values >= this are invalid */
};

/** \brief Duplex definitions */
enum libkiss_duplex_enum {
	LIBKISS_DUPLEX_HALF,		/**< \brief Half Duplex */
	LIBKISS_DUPLEX_FULL,		/**< \brief Full Duplex */
	LIBKISS_DUPLEX_INVALID		/**< \brief Values >= this are invalid */
};

/** \brief Open a new libkiss context
 * \param handle The handle for the new context
 * \param bufsize The size of the buffer (32k suggested)
 * \return 0=Sucess, other=Failure
 */
int libkiss_open(LIBKISS handle, unsigned int bufsize);

/** \brief Close/free a libkiss context
 * \param handle The context
 * \return 0=Sucess, other=Failure
 */
int libkiss_close(LIBKISS handle);

/** Build a generic KISS packet by adding a start code, function/port byte,
 * end codes, and escaping bytes as necessary.
 *  \brief Build a generic KISS packet
 * \param function Function code
 * \param port Port to use (Use 0 for single-port devices)
 * \param data Data to put in the packet (NULL if N/A)
 * \param datalen Data length (0 if N/A)
 * \param packet Resulting packet
 * \param packetmaxlen Maximum size available for the resulting packet
 * \param packetlen Actual size of the resulting packet (undefined on error)
 * \return 0=Sucess, other=Failure
 */
int libkiss_buildpacket(
	const enum libkiss_func_enum function, unsigned int port,
	const void *data, unsigned int datalen,
	const void *packet, unsigned int packetmaxlen, unsigned int *packetlen
	);

/** \brief Add data to a decoding buffer
 * \param handle LIBKISS context to use
 * \param data Data buffer
 * \param nbytes Size of data buffer
 * \return 0=Sucess, other=Failure
 */
int libkiss_adddata(LIBKISS handle, const void *data, unsigned int nbytes);

/** \brief Get data from a decoding buffer
 * \param handle LIBKISS context to use
 * \param func KISS function code
 * \param port Port number (0 for single-port devices)
 * \param data Data buffer
 * \param maxdatalen Maximum size of the data buffer
 * \param nbytes Number of bytes used in the data buffer
 * \return 0=Sucess, other=Failure
 */
int libkiss_getdata(LIBKISS handle, enum libkiss_func_enum func, int port, void *data, unsigned int maxdatalen, unsigned int *nbytes);

/* Allow C++ to use our C-based library */
#ifdef __cplusplus
}
#endif

#endif /* LIBKISS_H */