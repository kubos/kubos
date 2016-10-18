#!/usr/bin/env python
# poor man's "lsusb -d" for Mac OS X
import argparse
import plistlib
import subprocess
import sys

devices = []
def add_device(item):
    if '_items' in item:
        for i in item['_items']:
            add_device(i)

    product_id = item.get('product_id', 'unknown')
    vendor_id = item.get('vendor_id', 'unknown')
    if '(' in vendor_id:
        vendor_id = vendor_id[:vendor_id.index('(')]

    vendor_id = vendor_id.replace('0x', '').strip()
    product_id = product_id.replace('0x', '').strip()
    devices.append('%s:%s' % (vendor_id, product_id))

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('-d', '--device')
    args = parser.parse_args()

    data = subprocess.check_output(['system_profiler', '-xml', 'SPUSBDataType'])
    usb_data = plistlib.readPlistFromString(data)

    for _item in usb_data[0]['_items']:
        for item in _item['_items']:
            add_device(item)

    if args.device:
        matches = filter(lambda d: d.startswith(args.device), devices)
        if len(matches) == 0:
            sys.exit(1)

        for match in matches:
            print match

        sys.exit(0)

    for device in devices:
        print device

if __name__ == '__main__':
    main()
