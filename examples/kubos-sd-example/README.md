# Kubos SPI SD Example App

This is a simple application built on top of the [KubOS RT Platform](https://github.com/kubostech/KubOS-rt) demonstrating SD over SPI using Kubos Core's FatFS library.

This application runs a series of commands against an SD card connected to SPI bus 1 using the FatFS library.

The application covers how to:

Mount/unmount a SD card
  - Open a file for writing (file will be created if it doesn’t exist)
  - Open a file for reading
  - Close a file
  - Write a string to a file
  - Read a specified length from a file
  - Sync the file system
  - Get the stats (size, timestamp, attributes) of a file

The easiest way to get started building this is with the [Kubos SDK](http://docs.kubos.co/latest/md_docs_kubos-sdk.html).
