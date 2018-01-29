# ADC Demo with Kubos Linux

This is a simple application built on top of the [KubOS Linux Platform](https://github.com/kubostech/kubos-linux-build) 
demonstrating some basic data gathering from a thermistor connected to an ADC pin.

The project reads ADC values from the thermistor and then converts it to the temperature (in celsius) using the 
[B-parameter equation](https://en.wikipedia.org/wiki/Thermistor#B_or_.CE.B2_parameter_equation).

## Usage

```

    $ adc-thermistor
    usage: adc-thermistor [-c] [-d]
    
    optional arguments:
      -c        Read ADC pin once per second until program is exitted with Ctrl-C
      -d        Output debugging messages
      
```

## Configuration

This example is configured for an ADC pin with a 10-bit resolution connected to a 10 kOhm
thermistor with a 3.3V reference voltage and a voltage supply of 2.4V. These values might
need to be changed based on your test setup

