#!/usr/bin/env python

import sys
import argparse

def on_boot():
    
    print "OnBoot logic"
    
def on_command():
    
    print "OnCommand logic"

def main():
    parser = argparse.ArgumentParser()
    
    parser.add_argument('--run', '-r', nargs=1, default='OnCommand')
    
    args = parser.parse_args()
    
    if args.run[0] == 'OnBoot':
        on_boot()
    else:
        on_command()
    
if __name__ == "__main__":
    main()