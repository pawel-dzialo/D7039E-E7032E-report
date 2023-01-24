#![no_main]
#![no_std]

use panic_rtt_target as _;

#[rtic::app(device = stm32f4::stm32f411, dispatchers = [EXTI0])]
mod app {
    use embedded_hal::spi::{MODE_0, MODE_1};
    use rtt_target::{rprintln, rtt_init_print};
    use stm32f4xx_hal::otg_fs::{UsbBus, USB};
    use stm32f4xx_hal::prelude::*;
    use stm32f4xx_hal::spi::Spi;
    use stm32f4xx_hal::spi::TransferModeNormal;
    /*use stm32f4xx_hal::gpio::Pin;
    use stm32f4xx_hal::gpio::Alternate;
    use stm32f4xx_hal::gpio::PushPull;
    use stm32f4xx_hal::gpio::Output;
    use stm32f4xx_hal::gpio::Speed;
    use stm32f4xx_hal::gpio::Edge;
    use stm32f4xx_hal::gpio::Input;*/
    use stm32f4xx_hal::gpio::{Pin,Alternate,PushPull,Output,Speed,Edge,Input,Floating, PullDown, PullUp};
    use stm32f4xx_hal::timer::Delay;
    //use stm32f4xx_hal::gpio::{Pin,Alternate,PushPull,Output,Speed};
    use usb_device::{bus::UsbBusAllocator, prelude::*};
    use usbd_serial::SerialPort;
    use embedded_hal::{
        blocking::{
            delay::DelayUs,
            spi::{Transfer, Write},
        },
        digital::v2::OutputPin,
        spi::MODE_3,
    };
    use stm32f4::stm32f411::{SPI1, TIM5, TIM4,gpiob};
    use cortex_m::{asm, peripheral::DWT};


    type SCK = Pin<Alternate<PushPull, 5_u8>, 'B', 3_u8>;
    type MOSI = Pin<Alternate<PushPull, 5_u8>, 'A', 7_u8>;
    type MISO = Pin<Alternate<PushPull, 5_u8>, 'A', 6_u8>;
    type CS = Pin<Output<PushPull>, 'A', 4_u8>;

    type SPI = Spi<SPI1, (SCK, MISO, MOSI), TransferModeNormal>;
    type DELAY = Delay<TIM5, 1000000_u32>;
    type DELAY2 = Delay<TIM4, 1000000_u32>;


    #[shared]
    struct Shared {
        idx:u8,
        usb_buffer:[[u8;2];8],
        slaveselect:Pin<Output<PushPull>, 'A' , 4>,
        spi: SPI,
        ready: bool,
    }

    #[local]
    struct Local {
        usb_dev: UsbDevice<'static, UsbBus<USB>>,
        serial: SerialPort<'static, UsbBus<USB>>,
        received:u32,
        errors:u32,
        muxarr1:Pin<Input<PullDown>, 'C' , 5>,
        muxarr2:Pin<Input<PullDown>, 'C' , 6>,
        muxarr3:Pin<Input<PullDown>, 'C' , 8>,
        muxarr4:Pin<Input<PullDown>, 'C' , 9>,
        arr: [u16;16],
        delay: DELAY,
        delay2: DELAY2,
        usb_index: usize,
        total: u8,
    }
/*
#[shared]
struct Shared {
}

#[local]
struct Local {
    usb_dev: UsbDevice<'static, UsbBus<USB>>,
    serial: SerialPort<'static, UsbBus<USB>>,
    received:u32,
    errors:u32,
}*/
    #[init(local = [EP_MEMORY: [u32; 1024] = [0; 1024], bus: Option<UsbBusAllocator<UsbBus<USB>>> = None])]
    fn init(mut cx: init::Context) -> (Shared, Local, init::Monotonics) {
        rtt_init_print!();
        cx.core.DCB.enable_trace();
        cx.core.DWT.enable_cycle_counter();
       // rprintln!("init");
        let mut dp = cx.device;
        let mut syscfg = dp.SYSCFG.constrain();
        let rcc = dp.RCC.constrain();

        let clocks = rcc.cfgr.sysclk(48.MHz()).require_pll48clk().freeze();

        let gpioa = dp.GPIOA.split();
        let gpiob = dp.GPIOB.split();
        let gpioc = dp.GPIOC.split();

        //set up SPI
        let mut delay: DELAY = dp.TIM5.delay_us(&clocks);
        let mut delay2: DELAY2 = dp.TIM4.delay_us(&clocks);
        let sck: SCK = gpiob.pb3.into_alternate().set_speed(Speed::VeryHigh);
        let miso: MISO = gpioa.pa6.into_alternate().set_speed(Speed::VeryHigh);
        let mosi: MOSI = gpioa.pa7.into_alternate().set_speed(Speed::VeryHigh);
        let mut spi: SPI = Spi::new(dp.SPI1,(sck,miso,mosi), MODE_1,1.MHz(),&clocks);
        let mut total: u8 = 0;
        
        //set up MUX pins
        let mut muxarr1 = gpioc.pc5.into_pull_down_input();
        let mut muxarr2 = gpioc.pc6.into_pull_down_input();
        let mut muxarr3 = gpioc.pc8.into_pull_down_input();
        let mut muxarr4 = gpioc.pc9.into_pull_down_input();

        //set up slave select pin
        let mut slaveselect = gpioa.pa4.into_push_pull_output().set_speed(Speed::VeryHigh);
        slaveselect.set_low();

        //testarr
        let arr:[u16;16] = [0xffff,0x0002,0x0004,0x0006,0x0008,0x000A,0x000C,0x000F,0x1000,0x1000,0x3000,0x5000,0x7000,0x9000,0xB000,0xD000];

        //USB receiver buffer setup
        let usb_index = 0;
        let mut usb_buffer = [[0u8;2];8];
        let mut ready = false;

   /*     //set up MUX interrupts
        muxarr1.make_interrupt_source(&mut syscfg);
        muxarr1.enable_interrupt(&mut dp.EXTI);
        muxarr1.trigger_on_edge(&mut dp.EXTI, Edge::RisingFalling);

        muxarr2.make_interrupt_source(&mut syscfg);
        muxarr2.enable_interrupt(&mut dp.EXTI);
        muxarr2.trigger_on_edge(&mut dp.EXTI, Edge::RisingFalling);

        muxarr3.make_interrupt_source(&mut syscfg);
        muxarr3.enable_interrupt(&mut dp.EXTI);
        muxarr3.trigger_on_edge(&mut dp.EXTI, Edge::RisingFalling);

        muxarr4.make_interrupt_source(&mut syscfg);
        muxarr4.enable_interrupt(&mut dp.EXTI);
        muxarr4.trigger_on_edge(&mut dp.EXTI, Edge::RisingFalling);
     */   
 
        //mux index
        let mut idx = 0;


         
        //cs.set_low();
        //spi.write(&byte);
        spi.enable(true);
        /*
        loop{
        let byte:[u8;1] = [0b10101010];
        spi.write(&byte).unwrap();
        delay.delay_us(70u32);
        let byte:[u8;1] = [0b0000000];
        spi.write(&byte).unwrap();
        delay.delay_us(70u32);
        let byte:[u8;1] = [0b11111111];
        spi.write(&byte).unwrap();
        delay.delay_us(70u32);
        
        rprintln!("{:?}",&byte);
        }
        cs.set_high();
        */

        let usb = USB {
            usb_global: dp.OTG_FS_GLOBAL,
            usb_device: dp.OTG_FS_DEVICE,
            usb_pwrclk: dp.OTG_FS_PWRCLK,
            pin_dm: gpioa.pa11.into_alternate(),
            pin_dp: gpioa.pa12.into_alternate(),
            hclk: clocks.hclk(),
        };

        cx.local.bus.replace(UsbBus::new(usb, cx.local.EP_MEMORY));

        let serial = usbd_serial::SerialPort::new(cx.local.bus.as_ref().unwrap());
        let received = 0;
        let errors = 0;
        let usb_dev =
            UsbDeviceBuilder::new(cx.local.bus.as_ref().unwrap(), UsbVidPid(0x16c0, 0x27dd))
                .manufacturer("e7020e")
                .product("Serial port")
                .serial_number("1337")
                .device_class(usbd_serial::USB_CLASS_CDC)
                .build();

        (Shared {idx,usb_buffer,slaveselect,spi,ready}, Local { total,delay2,usb_dev, serial, received, errors, muxarr1, muxarr2, muxarr3, muxarr4, arr, delay,usb_index }, init::Monotonics())
        //(Shared {}, Local { usb_dev, serial, received, errors}, init::Monotonics())
    }

    #[task(local = [muxarr1,muxarr2,muxarr3,muxarr4,arr,delay,total], shared = [idx,slaveselect,spi,usb_buffer,ready])]
    fn get_idx(mut cx: get_idx::Context){
        //needs delay
       // let start = DWT::cycle_count();
        let mut r_l = false;
        cx.shared.ready.lock(|ready|{r_l = *ready;});
        if(r_l){
        // *cx.local.total +=1;
        let mut idx_l = 0;
       // cx.local.delay.delay_us(50u32);
        cx.shared.idx.lock(|idx| {
            *idx =u8::from(cx.local.muxarr1.is_high())|
            u8::from(cx.local.muxarr2.is_high()) << 1|
            u8::from(cx.local.muxarr3.is_high()) << 2 ;
            idx_l = *idx;
        }); 
        cx.shared.slaveselect.lock(|slaveselect|{slaveselect.set_high()});
     //   cx.local.delay.delay_us(50u32);
        cx.shared.spi.lock(|spi|{
            cx.shared.usb_buffer.lock(|usb_buffer|{
              //  rprintln!("{:?}",idx_l);
              //  rprintln!("{:?}",&usb_buffer[idx_l as usize]);
                spi.write(&usb_buffer[idx_l as usize]).unwrap();
            });
        });
        cx.shared.slaveselect.lock(|slaveselect|{slaveselect.set_low()});    
        if(*cx.local.total==6){
            cx.shared.ready.lock(|ready|{*ready = false});
            *cx.local.total = 0;
        }
        else{
            *cx.local.total +=1;
            get_idx::spawn().ok();
        }
        }
    }

    #[task(binds=OTG_FS, local = [usb_dev, serial, received, errors,usb_index,delay2],shared=[usb_buffer,spi,slaveselect,ready])]
    fn usb_fs(mut cx: usb_fs::Context) {
        let serial = cx.local.serial;
        let usb_dev = cx.local.usb_dev;
        let mut r_l = true;
        cx.shared.ready.lock(|ready|{r_l = *ready;});
        if(!r_l){
       // rprintln!("called");
        if usb_dev.poll(&mut [serial]) {
            let mut buf = [0u8; 64];
            match serial.read(&mut buf){
                Ok(count) if count > 0 => {
                   // rprintln!("RX");
                    let mut i = 0;
                    let mut j = 0;
                    for c in buf[0..count].iter_mut() {
                        cx.shared.usb_buffer.lock(|usb_buffer|{
                            usb_buffer[j][i] = *c;
                        });
                        if(i==0){
                            i+=1;
                        }
                        else{
                            i=0;
                            j+=1;
                            if(j==8){
                                break;
                            }
                        }
                        
                    }
                    /*  cx.shared.usb_buffer.lock(|usb_buffer|{
                        rprintln!("{:?}",usb_buffer);
                    });*/
                       // rprintln!("sending");
                        *cx.local.usb_index = 0;
                        //cx.local.delay2.delay_ms(1u32);  
                        //rprintln!("true");
                        cx.shared.slaveselect.lock(|slaveselect|{slaveselect.set_high()});
                        //  rprintln!("{:?}",cx.local.slaveselect.is_set_high());
                //        cx.local.delay2.delay_us(50u32);
                        cx.shared.spi.lock(|spi|{
                            cx.shared.usb_buffer.lock(|usb_buffer|{
                                spi.write(&usb_buffer[0]).unwrap();
                            });              
                        });
                       // cx.local.delay2.delay_us(70u32);
                        cx.shared.slaveselect.lock(|slaveselect|{slaveselect.set_low()});  
                        cx.shared.ready.lock(|ready|{*ready=true;});  
                        get_idx::spawn().ok();
                    }
                
                _ => {}
            }
        }
    }
}
}