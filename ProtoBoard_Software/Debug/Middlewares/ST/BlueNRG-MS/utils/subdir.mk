################################################################################
# Automatically-generated file. Do not edit!
# Toolchain: xd
################################################################################

# Add inputs and outputs from these tool invocations to the build variables 
C_SRCS += \
../Middlewares/ST/BlueNRG-MS/utils/ble_list.c 

OBJS += \
./Middlewares/ST/BlueNRG-MS/utils/ble_list.o 

C_DEPS += \
./Middlewares/ST/BlueNRG-MS/utils/ble_list.d 


# Each subdirectory must supply rules for building sources it contributes
Middlewares/ST/BlueNRG-MS/utils/%.o Middlewares/ST/BlueNRG-MS/utils/%.su: ../Middlewares/ST/BlueNRG-MS/utils/%.c Middlewares/ST/BlueNRG-MS/utils/subdir.mk
	arm-none-eabi-gcc "$<" -mcpu=cortex-m4 -std=gnu11 -g3 -DDEBUG -DUSE_HAL_DRIVER -DSTM32L431xx -c -I../BlueNRG_MS/App -I../BlueNRG_MS/Target -I../Core/Inc -I../Drivers/BSP/custom -I../Drivers/STM32L4xx_HAL_Driver/Inc -I../Drivers/STM32L4xx_HAL_Driver/Inc/Legacy -I../Drivers/CMSIS/Device/ST/STM32L4xx/Include -I../Drivers/CMSIS/Include -I../Middlewares/ST/BlueNRG-MS/utils -I../Middlewares/ST/BlueNRG-MS/includes -I../Middlewares/ST/BlueNRG-MS/hci/hci_tl_patterns/Basic -O0 -ffunction-sections -fdata-sections -Wall -fstack-usage -MMD -MP -MF"$(@:%.o=%.d)" -MT"$@" --specs=nano.specs -mfpu=fpv4-sp-d16 -mfloat-abi=hard -mthumb -o "$@"

clean: clean-Middlewares-2f-ST-2f-BlueNRG-2d-MS-2f-utils

clean-Middlewares-2f-ST-2f-BlueNRG-2d-MS-2f-utils:
	-$(RM) ./Middlewares/ST/BlueNRG-MS/utils/ble_list.d ./Middlewares/ST/BlueNRG-MS/utils/ble_list.o ./Middlewares/ST/BlueNRG-MS/utils/ble_list.su

.PHONY: clean-Middlewares-2f-ST-2f-BlueNRG-2d-MS-2f-utils

