# ADC Multiplexing test code
The STM32CubeIDE project here can be ran on the MCU to verify that the ADC Multiplexing is working correctly.

Once the program is ran, we may start feeding the MCU a signal over the ADCs. After 10240 samples have been sent we can use the debug mode of the IDE to dump the two buffers into a text file.

Afterwards extractexported.py can be used to prune and format the data, and extract.py to plot it.
