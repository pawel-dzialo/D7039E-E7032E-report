# Simulator-end software

The software here may be used to simulate the behavior of the amplification/multiplexing circuits.

A file from the CHB-MIT must first be converted to correctly scaled and pruned values using ./genfile/genfile.py

Afterwards, the example coms_huge in simulator_devkit must be flashed to a NUCLEO-F411RE . The proper USB connections must be made between the PC and the devkit.

Lastly, the devkit and the prototype board are connected as mentioned in the code, and the example ex4 from com can be ran on the PC to feed the converted data to our prototype board.
