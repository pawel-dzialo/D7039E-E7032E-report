################################################################################
# Automatically-generated file. Do not edit!
# Toolchain: xd
################################################################################

# Add inputs and outputs from these tool invocations to the build variables 
C_SRCS += \
../Drivers/BSP/custom/custom.c 

OBJS += \
./Drivers/BSP/custom/custom.o 

C_DEPS += \
./Drivers/BSP/custom/custom.d 


# Each subdirectory must supply rules for building sources it contributes
Drivers/BSP/custom/%.o Drivers/BSP/custom/%.su: ../Drivers/BSP/custom/%.c Drivers/BSP/custom/subdir.mk
	arm-none-eabi-gcc "$<" -mcpu=cortex-m4 -std=gnu11 -g3 -DDEBUG -DUSE_HAL_DRIVER -DSTM32L431xx -c -I../BlueNRG_MS/App -I../BlueNRG_MS/Target -I../Core/Inc -I../Drivers/BSP/custom -I../Drivers/STM32L4xx_HAL_Driver/Inc -I../Drivers/STM32L4xx_HAL_Driver/Inc/Legacy -I../Drivers/CMSIS/Device/ST/STM32L4xx/Include -I../Drivers/CMSIS/Include -I../Middlewares/ST/BlueNRG-MS/utils -I../Middlewares/ST/BlueNRG-MS/includes -I../Middlewares/ST/BlueNRG-MS/hci/hci_tl_patterns/Basic -O0 -ffunction-sections -fdata-sections -Wall -fstack-usage -MMD -MP -MF"$(@:%.o=%.d)" -MT"$@" --specs=nano.specs -mfpu=fpv4-sp-d16 -mfloat-abi=hard -mthumb -o "$@"

clean: clean-Drivers-2f-BSP-2f-custom

clean-Drivers-2f-BSP-2f-custom:
	-$(RM) ./Drivers/BSP/custom/custom.d ./Drivers/BSP/custom/custom.o ./Drivers/BSP/custom/custom.su

.PHONY: clean-Drivers-2f-BSP-2f-custom

