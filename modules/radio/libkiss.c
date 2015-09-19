/**
 * \mainpage
 *
 * LIBKISS is a library for processing KISS packets used by amatuer radio TNCs.
 *
 * The original KISS protocol spec can be found here:
 * http://www.ka9q.net/papers/kiss.html
 * or here:
 * http://www.ax25.net/kiss.aspx
 *
 * ******** FROM adsllc/libkiss -- CHECK LICENSES!! ********
 *
 */

#ifdef HAVE_CONFIG_H
#include "am_config.h"
#endif

//#ifdef HAVE_STDLIB_H
# include <stdlib.h>
//#endif
//#ifdef HAVE_STRING_H
# include <string.h>
//#endif

#include "libkiss.h"
#include "kernel.h" //new

int libkiss_open(LIBKISS handle, unsigned int bufsize) {
	memset(handle,0,sizeof(*handle));
	handle->bufsize=bufsize;
	handle->buf=calloc(1,bufsize);
	if (!handle->buf)
		return 1;

	return 0;
}

int libkiss_close(LIBKISS handle) {
	if (handle->bufsize>0)
		free(handle->buf);

	memset(handle,0,sizeof(*handle));
	return 0;
}

int libkiss_adddata(LIBKISS handle, const void *data, unsigned int nbytes) {
	if (handle->nbytes+nbytes > handle->bufsize) {
		/* Not enough room to add the data */
		return -1;
	}

	memcpy(&(handle->buf[handle->nbytes]),data,nbytes);

	return 0;
}

int libkiss_getdata(LIBKISS handle, enum libkiss_func_enum func, int port, void *data, unsigned int maxdatalen, unsigned int *nbytes) {
	unsigned int start,end,i;
	int error=0;

	if (handle->nbytes<3) {
		/* Not enough data to be a valid packet */
		return -1;
	}

	if (handle->buf[0]!=LIBKISS_FEND) {
		/* Skip the garbage at the front of the buffer to find the start of the packet */
		for (start=0;start<handle->nbytes;start++) {
			if (handle->buf[start]==LIBKISS_FEND) {
				break;
			}
		}

		if (start>=handle->nbytes) {
			/* No valid packet start found, empty the buffer */
			handle->nbytes=0;
			return -1;
		}

		/* Move the packet to the start of the buffer */
		memmove(handle->buf,&(handle->buf[start]),handle->nbytes-start);
		handle->nbytes-=start;
		start=0;
	}

	/* At this point we have a FEND byte at the beginning of the buffer */

	/* Find the end of the packet */
	for (end=2;end<handle->nbytes;end++) {
		if (handle->buf[end]==LIBKISS_FEND) {
			break;
		}
	}

	if (end>=handle->nbytes) {
		/* No valid packet end found */
		return -2;
	}

	/* At this point we have a complete packet */

	func=handle->buf[1] >> 8;
	port=handle->buf[1] & 0x0F;

	printf("func is %d, port is %d", func, port);

	for (i=2;i<=end;i++) {
		((unsigned char*)data)[(*nbytes)++]=handle->buf[i];
		if (*nbytes>=maxdatalen && i<end) {
			/* The output buffer is full, but there's more data waiting */
			error=-3;
			break;
		}

		if (handle->buf[i]==LIBKISS_FESC) {
			i++;
			if (handle->buf[i]==LIBKISS_TFEND) {
				((unsigned char*)data)[(*nbytes)]=LIBKISS_FEND;
			} else if (handle->buf[i]==LIBKISS_TFESC) {
				((unsigned char*)data)[(*nbytes)]=LIBKISS_FESC;
			} else {
				/* Invalid escape code */
				error=-4;
				break;
			}
		}
	}

	/* Clean the packet from the buffer */
	if (end==handle->nbytes) {
		/* There's no more data */
		handle->nbytes=0;
	} else {
		/* Move the rest of the data forward */
		memmove(handle->buf,&(handle->buf[end+1]),handle->nbytes-end-1);
		handle->nbytes-=end;
	}

	return error;
}

int libkiss_buildpacket(
	const enum libkiss_func_enum function, unsigned int port,
	const void *data, unsigned int datalen,
	const void *packet, unsigned int packetmaxlen, unsigned int *packetlen
	) {

	unsigned int datai;
	unsigned int escs=0;

	/* Sanity check the function */
	if (function>=LIBKISS_FUNC_INVALID)
		return -1;

	/* Hijack the port value if it's a return command */
	if (function==LIBKISS_FUNC_RETURN)
		port=0x0F;

	/* Sanity check the port (must be 4 bits or less) */
	if (port & 0xF0)
		return -2;

	/* Check for a valid pointer if we're supposed to have data */
	if (datalen && !data)
		return -3;

	/* We need room for the Start, Function/Code, Data, and End */
	if (datalen+3>packetmaxlen)
		return -4;

	*packetlen=0;
	/* Add packet start byte */
	((unsigned char *)packet)[(*packetlen)++]=LIBKISS_FEND;
	/* Add function/port byte */
	((unsigned char *)packet)[(*packetlen)++]=((port & 0x0F)<<4) | (function & 0x0F);

	for (datai=0; datai<datalen; datai++) {
		if (((unsigned char*)data)[datai]==LIBKISS_FEND || ((unsigned char*)data)[datai]==LIBKISS_FESC) {
			/* We need to add an escape byte */

			/* Check for room */
			escs++;
			if (datalen+3+escs>packetmaxlen)
				return -5;

			/* Add the escape */
			((unsigned char *)packet)[(*packetlen)++]=LIBKISS_FESC;

			/* Add the transposed value */
			if (((unsigned char*)data)[datai]==LIBKISS_FEND)
				((unsigned char *)packet)[(*packetlen)++]=LIBKISS_TFEND;
			else
				((unsigned char *)packet)[(*packetlen)++]=LIBKISS_TFESC;

		} else {
			/* Add the byte directly */
			((unsigned char *)packet)[(*packetlen)++]=((char*)data)[datai];
		}
	}

	/* Add packet end byte */
	((unsigned char *)packet)[(*packetlen)++]=LIBKISS_FEND;

	return 0;
}