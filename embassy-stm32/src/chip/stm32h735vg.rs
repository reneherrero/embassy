use embassy_extras::peripherals;
peripherals!(
    EXTI0, EXTI1, EXTI2, EXTI3, EXTI4, EXTI5, EXTI6, EXTI7, EXTI8, EXTI9, EXTI10, EXTI11, EXTI12,
    EXTI13, EXTI14, EXTI15, ADC1, ADC2, ADC3, BDMA, COMP1, COMP2, CORDIC, CRYP, DAC1, DCMI, DMA2D,
    DTS, ETH, FDCAN1, FDCAN2, FDCAN3, FMAC, PA0, PA1, PA2, PA3, PA4, PA5, PA6, PA7, PA8, PA9, PA10,
    PA11, PA12, PA13, PA14, PA15, PB0, PB1, PB2, PB3, PB4, PB5, PB6, PB7, PB8, PB9, PB10, PB11,
    PB12, PB13, PB14, PB15, PC0, PC1, PC2, PC3, PC4, PC5, PC6, PC7, PC8, PC9, PC10, PC11, PC12,
    PC13, PC14, PC15, PD0, PD1, PD2, PD3, PD4, PD5, PD6, PD7, PD8, PD9, PD10, PD11, PD12, PD13,
    PD14, PD15, PE0, PE1, PE2, PE3, PE4, PE5, PE6, PE7, PE8, PE9, PE10, PE11, PE12, PE13, PE14,
    PE15, PF0, PF1, PF2, PF3, PF4, PF5, PF6, PF7, PF8, PF9, PF10, PF11, PF12, PF13, PF14, PF15,
    PG0, PG1, PG2, PG3, PG4, PG5, PG6, PG7, PG8, PG9, PG10, PG11, PG12, PG13, PG14, PG15, PH0, PH1,
    PH2, PH3, PH4, PH5, PH6, PH7, PH8, PH9, PH10, PH11, PH12, PH13, PH14, PH15, PJ0, PJ1, PJ2, PJ3,
    PJ4, PJ5, PJ6, PJ7, PJ8, PJ9, PJ10, PJ11, PJ12, PJ13, PJ14, PJ15, PK0, PK1, PK2, PK3, PK4, PK5,
    PK6, PK7, PK8, PK9, PK10, PK11, PK12, PK13, PK14, PK15, HASH, I2C1, I2C2, I2C3, I2C4, I2C5,
    IWDG1, LPTIM1, LPTIM2, LPTIM3, LPTIM4, LPTIM5, LPUART1, LTDC, MDIOS, MDMA, OCTOSPI1, OCTOSPI2,
    OCTOSPIM, OPAMP1, OPAMP2, OTFDEC1, OTFDEC2, PSSI, PWR, RCC, RNG, RTC, SAI1, SAI4, SDMMC1,
    SDMMC2, SPI1, SPI2, SPI3, SPI4, SPI6, SWPMI1, SYSCFG, TIM1, TIM12, TIM13, TIM14, TIM15, TIM16,
    TIM17, TIM2, TIM23, TIM24, TIM3, TIM4, TIM5, TIM6, TIM7, TIM8, UART4, UART5, UART7, UART8,
    UART9, USART1, USART2, USART3, USART6, USB_OTG_HS, VREFBUF, WWDG1
);
pub const GPIO_BASE: usize = 0x58020000;
pub const GPIO_STRIDE: usize = 0x400;

pub mod interrupt {
    pub use cortex_m::interrupt::{CriticalSection, Mutex};
    pub use embassy::interrupt::{declare, take, Interrupt};
    pub use embassy_extras::interrupt::Priority4 as Priority;

    #[derive(Copy, Clone, Debug, PartialEq, Eq)]
    #[allow(non_camel_case_types)]
    enum InterruptEnum {
        ADC = 18,
        ADC3 = 127,
        BDMA_Channel0 = 129,
        BDMA_Channel1 = 130,
        BDMA_Channel2 = 131,
        BDMA_Channel3 = 132,
        BDMA_Channel4 = 133,
        BDMA_Channel5 = 134,
        BDMA_Channel6 = 135,
        BDMA_Channel7 = 136,
        CEC = 94,
        CORDIC = 154,
        CRS = 144,
        CRYP = 79,
        DCMI_PSSI = 78,
        DMA1_Stream0 = 11,
        DMA1_Stream1 = 12,
        DMA1_Stream2 = 13,
        DMA1_Stream3 = 14,
        DMA1_Stream4 = 15,
        DMA1_Stream5 = 16,
        DMA1_Stream6 = 17,
        DMA1_Stream7 = 47,
        DMA2D = 90,
        DMA2_Stream0 = 56,
        DMA2_Stream1 = 57,
        DMA2_Stream2 = 58,
        DMA2_Stream3 = 59,
        DMA2_Stream4 = 60,
        DMA2_Stream5 = 68,
        DMA2_Stream6 = 69,
        DMA2_Stream7 = 70,
        DTS = 147,
        ECC = 145,
        ETH = 61,
        ETH_WKUP = 62,
        EXTI0 = 6,
        EXTI1 = 7,
        EXTI15_10 = 40,
        EXTI2 = 8,
        EXTI3 = 9,
        EXTI4 = 10,
        EXTI9_5 = 23,
        FDCAN1_IT0 = 19,
        FDCAN1_IT1 = 21,
        FDCAN2_IT0 = 20,
        FDCAN2_IT1 = 22,
        FDCAN3_IT0 = 159,
        FDCAN3_IT1 = 160,
        FDCAN_CAL = 63,
        FLASH = 4,
        FMAC = 153,
        FMC = 48,
        FPU = 81,
        HASH_RNG = 80,
        HSEM1 = 125,
        I2C1_ER = 32,
        I2C1_EV = 31,
        I2C2_ER = 34,
        I2C2_EV = 33,
        I2C3_ER = 73,
        I2C3_EV = 72,
        I2C4_ER = 96,
        I2C4_EV = 95,
        I2C5_ER = 158,
        I2C5_EV = 157,
        LPTIM1 = 93,
        LPTIM2 = 138,
        LPTIM3 = 139,
        LPTIM4 = 140,
        LPTIM5 = 141,
        LPUART1 = 142,
        LTDC = 88,
        LTDC_ER = 89,
        MDIOS = 120,
        MDIOS_WKUP = 119,
        MDMA = 122,
        OCTOSPI1 = 92,
        OCTOSPI2 = 150,
        OTFDEC1 = 151,
        OTFDEC2 = 152,
        OTG_HS = 77,
        OTG_HS_EP1_IN = 75,
        OTG_HS_EP1_OUT = 74,
        OTG_HS_WKUP = 76,
        PVD_AVD = 1,
        RCC = 5,
        RTC_Alarm = 41,
        RTC_WKUP = 3,
        SAI1 = 87,
        SAI4 = 146,
        SDMMC1 = 49,
        SDMMC2 = 124,
        SPDIF_RX = 97,
        SPI1 = 35,
        SPI2 = 36,
        SPI3 = 51,
        SPI4 = 84,
        SPI5 = 85,
        SPI6 = 86,
        SWPMI1 = 115,
        TAMP_STAMP = 2,
        TIM15 = 116,
        TIM16 = 117,
        TIM17 = 118,
        TIM1_BRK = 24,
        TIM1_CC = 27,
        TIM1_TRG_COM = 26,
        TIM1_UP = 25,
        TIM2 = 28,
        TIM23 = 161,
        TIM24 = 162,
        TIM3 = 29,
        TIM4 = 30,
        TIM5 = 50,
        TIM6_DAC = 54,
        TIM7 = 55,
        TIM8_BRK_TIM12 = 43,
        TIM8_CC = 46,
        TIM8_TRG_COM_TIM14 = 45,
        TIM8_UP_TIM13 = 44,
        UART4 = 52,
        UART5 = 53,
        UART7 = 82,
        UART8 = 83,
        UART9 = 155,
        USART1 = 37,
        USART10 = 156,
        USART2 = 38,
        USART3 = 39,
        USART6 = 71,
        WAKEUP_PIN = 149,
        WWDG = 0,
    }
    unsafe impl cortex_m::interrupt::InterruptNumber for InterruptEnum {
        #[inline(always)]
        fn number(self) -> u16 {
            self as u16
        }
    }

    declare!(ADC);
    declare!(ADC3);
    declare!(BDMA_Channel0);
    declare!(BDMA_Channel1);
    declare!(BDMA_Channel2);
    declare!(BDMA_Channel3);
    declare!(BDMA_Channel4);
    declare!(BDMA_Channel5);
    declare!(BDMA_Channel6);
    declare!(BDMA_Channel7);
    declare!(CEC);
    declare!(CORDIC);
    declare!(CRS);
    declare!(CRYP);
    declare!(DCMI_PSSI);
    declare!(DMA1_Stream0);
    declare!(DMA1_Stream1);
    declare!(DMA1_Stream2);
    declare!(DMA1_Stream3);
    declare!(DMA1_Stream4);
    declare!(DMA1_Stream5);
    declare!(DMA1_Stream6);
    declare!(DMA1_Stream7);
    declare!(DMA2D);
    declare!(DMA2_Stream0);
    declare!(DMA2_Stream1);
    declare!(DMA2_Stream2);
    declare!(DMA2_Stream3);
    declare!(DMA2_Stream4);
    declare!(DMA2_Stream5);
    declare!(DMA2_Stream6);
    declare!(DMA2_Stream7);
    declare!(DTS);
    declare!(ECC);
    declare!(ETH);
    declare!(ETH_WKUP);
    declare!(EXTI0);
    declare!(EXTI1);
    declare!(EXTI15_10);
    declare!(EXTI2);
    declare!(EXTI3);
    declare!(EXTI4);
    declare!(EXTI9_5);
    declare!(FDCAN1_IT0);
    declare!(FDCAN1_IT1);
    declare!(FDCAN2_IT0);
    declare!(FDCAN2_IT1);
    declare!(FDCAN3_IT0);
    declare!(FDCAN3_IT1);
    declare!(FDCAN_CAL);
    declare!(FLASH);
    declare!(FMAC);
    declare!(FMC);
    declare!(FPU);
    declare!(HASH_RNG);
    declare!(HSEM1);
    declare!(I2C1_ER);
    declare!(I2C1_EV);
    declare!(I2C2_ER);
    declare!(I2C2_EV);
    declare!(I2C3_ER);
    declare!(I2C3_EV);
    declare!(I2C4_ER);
    declare!(I2C4_EV);
    declare!(I2C5_ER);
    declare!(I2C5_EV);
    declare!(LPTIM1);
    declare!(LPTIM2);
    declare!(LPTIM3);
    declare!(LPTIM4);
    declare!(LPTIM5);
    declare!(LPUART1);
    declare!(LTDC);
    declare!(LTDC_ER);
    declare!(MDIOS);
    declare!(MDIOS_WKUP);
    declare!(MDMA);
    declare!(OCTOSPI1);
    declare!(OCTOSPI2);
    declare!(OTFDEC1);
    declare!(OTFDEC2);
    declare!(OTG_HS);
    declare!(OTG_HS_EP1_IN);
    declare!(OTG_HS_EP1_OUT);
    declare!(OTG_HS_WKUP);
    declare!(PVD_AVD);
    declare!(RCC);
    declare!(RTC_Alarm);
    declare!(RTC_WKUP);
    declare!(SAI1);
    declare!(SAI4);
    declare!(SDMMC1);
    declare!(SDMMC2);
    declare!(SPDIF_RX);
    declare!(SPI1);
    declare!(SPI2);
    declare!(SPI3);
    declare!(SPI4);
    declare!(SPI5);
    declare!(SPI6);
    declare!(SWPMI1);
    declare!(TAMP_STAMP);
    declare!(TIM15);
    declare!(TIM16);
    declare!(TIM17);
    declare!(TIM1_BRK);
    declare!(TIM1_CC);
    declare!(TIM1_TRG_COM);
    declare!(TIM1_UP);
    declare!(TIM2);
    declare!(TIM23);
    declare!(TIM24);
    declare!(TIM3);
    declare!(TIM4);
    declare!(TIM5);
    declare!(TIM6_DAC);
    declare!(TIM7);
    declare!(TIM8_BRK_TIM12);
    declare!(TIM8_CC);
    declare!(TIM8_TRG_COM_TIM14);
    declare!(TIM8_UP_TIM13);
    declare!(UART4);
    declare!(UART5);
    declare!(UART7);
    declare!(UART8);
    declare!(UART9);
    declare!(USART1);
    declare!(USART10);
    declare!(USART2);
    declare!(USART3);
    declare!(USART6);
    declare!(WAKEUP_PIN);
    declare!(WWDG);
}
mod interrupt_vector {
    extern "C" {
        fn ADC();
        fn ADC3();
        fn BDMA_Channel0();
        fn BDMA_Channel1();
        fn BDMA_Channel2();
        fn BDMA_Channel3();
        fn BDMA_Channel4();
        fn BDMA_Channel5();
        fn BDMA_Channel6();
        fn BDMA_Channel7();
        fn CEC();
        fn CORDIC();
        fn CRS();
        fn CRYP();
        fn DCMI_PSSI();
        fn DMA1_Stream0();
        fn DMA1_Stream1();
        fn DMA1_Stream2();
        fn DMA1_Stream3();
        fn DMA1_Stream4();
        fn DMA1_Stream5();
        fn DMA1_Stream6();
        fn DMA1_Stream7();
        fn DMA2D();
        fn DMA2_Stream0();
        fn DMA2_Stream1();
        fn DMA2_Stream2();
        fn DMA2_Stream3();
        fn DMA2_Stream4();
        fn DMA2_Stream5();
        fn DMA2_Stream6();
        fn DMA2_Stream7();
        fn DTS();
        fn ECC();
        fn ETH();
        fn ETH_WKUP();
        fn EXTI0();
        fn EXTI1();
        fn EXTI15_10();
        fn EXTI2();
        fn EXTI3();
        fn EXTI4();
        fn EXTI9_5();
        fn FDCAN1_IT0();
        fn FDCAN1_IT1();
        fn FDCAN2_IT0();
        fn FDCAN2_IT1();
        fn FDCAN3_IT0();
        fn FDCAN3_IT1();
        fn FDCAN_CAL();
        fn FLASH();
        fn FMAC();
        fn FMC();
        fn FPU();
        fn HASH_RNG();
        fn HSEM1();
        fn I2C1_ER();
        fn I2C1_EV();
        fn I2C2_ER();
        fn I2C2_EV();
        fn I2C3_ER();
        fn I2C3_EV();
        fn I2C4_ER();
        fn I2C4_EV();
        fn I2C5_ER();
        fn I2C5_EV();
        fn LPTIM1();
        fn LPTIM2();
        fn LPTIM3();
        fn LPTIM4();
        fn LPTIM5();
        fn LPUART1();
        fn LTDC();
        fn LTDC_ER();
        fn MDIOS();
        fn MDIOS_WKUP();
        fn MDMA();
        fn OCTOSPI1();
        fn OCTOSPI2();
        fn OTFDEC1();
        fn OTFDEC2();
        fn OTG_HS();
        fn OTG_HS_EP1_IN();
        fn OTG_HS_EP1_OUT();
        fn OTG_HS_WKUP();
        fn PVD_AVD();
        fn RCC();
        fn RTC_Alarm();
        fn RTC_WKUP();
        fn SAI1();
        fn SAI4();
        fn SDMMC1();
        fn SDMMC2();
        fn SPDIF_RX();
        fn SPI1();
        fn SPI2();
        fn SPI3();
        fn SPI4();
        fn SPI5();
        fn SPI6();
        fn SWPMI1();
        fn TAMP_STAMP();
        fn TIM15();
        fn TIM16();
        fn TIM17();
        fn TIM1_BRK();
        fn TIM1_CC();
        fn TIM1_TRG_COM();
        fn TIM1_UP();
        fn TIM2();
        fn TIM23();
        fn TIM24();
        fn TIM3();
        fn TIM4();
        fn TIM5();
        fn TIM6_DAC();
        fn TIM7();
        fn TIM8_BRK_TIM12();
        fn TIM8_CC();
        fn TIM8_TRG_COM_TIM14();
        fn TIM8_UP_TIM13();
        fn UART4();
        fn UART5();
        fn UART7();
        fn UART8();
        fn UART9();
        fn USART1();
        fn USART10();
        fn USART2();
        fn USART3();
        fn USART6();
        fn WAKEUP_PIN();
        fn WWDG();
    }
    pub union Vector {
        _handler: unsafe extern "C" fn(),
        _reserved: u32,
    }
    #[link_section = ".vector_table.interrupts"]
    #[no_mangle]
    pub static __INTERRUPTS: [Vector; 163] = [
        Vector { _handler: WWDG },
        Vector { _handler: PVD_AVD },
        Vector {
            _handler: TAMP_STAMP,
        },
        Vector { _handler: RTC_WKUP },
        Vector { _handler: FLASH },
        Vector { _handler: RCC },
        Vector { _handler: EXTI0 },
        Vector { _handler: EXTI1 },
        Vector { _handler: EXTI2 },
        Vector { _handler: EXTI3 },
        Vector { _handler: EXTI4 },
        Vector {
            _handler: DMA1_Stream0,
        },
        Vector {
            _handler: DMA1_Stream1,
        },
        Vector {
            _handler: DMA1_Stream2,
        },
        Vector {
            _handler: DMA1_Stream3,
        },
        Vector {
            _handler: DMA1_Stream4,
        },
        Vector {
            _handler: DMA1_Stream5,
        },
        Vector {
            _handler: DMA1_Stream6,
        },
        Vector { _handler: ADC },
        Vector {
            _handler: FDCAN1_IT0,
        },
        Vector {
            _handler: FDCAN2_IT0,
        },
        Vector {
            _handler: FDCAN1_IT1,
        },
        Vector {
            _handler: FDCAN2_IT1,
        },
        Vector { _handler: EXTI9_5 },
        Vector { _handler: TIM1_BRK },
        Vector { _handler: TIM1_UP },
        Vector {
            _handler: TIM1_TRG_COM,
        },
        Vector { _handler: TIM1_CC },
        Vector { _handler: TIM2 },
        Vector { _handler: TIM3 },
        Vector { _handler: TIM4 },
        Vector { _handler: I2C1_EV },
        Vector { _handler: I2C1_ER },
        Vector { _handler: I2C2_EV },
        Vector { _handler: I2C2_ER },
        Vector { _handler: SPI1 },
        Vector { _handler: SPI2 },
        Vector { _handler: USART1 },
        Vector { _handler: USART2 },
        Vector { _handler: USART3 },
        Vector {
            _handler: EXTI15_10,
        },
        Vector {
            _handler: RTC_Alarm,
        },
        Vector { _reserved: 0 },
        Vector {
            _handler: TIM8_BRK_TIM12,
        },
        Vector {
            _handler: TIM8_UP_TIM13,
        },
        Vector {
            _handler: TIM8_TRG_COM_TIM14,
        },
        Vector { _handler: TIM8_CC },
        Vector {
            _handler: DMA1_Stream7,
        },
        Vector { _handler: FMC },
        Vector { _handler: SDMMC1 },
        Vector { _handler: TIM5 },
        Vector { _handler: SPI3 },
        Vector { _handler: UART4 },
        Vector { _handler: UART5 },
        Vector { _handler: TIM6_DAC },
        Vector { _handler: TIM7 },
        Vector {
            _handler: DMA2_Stream0,
        },
        Vector {
            _handler: DMA2_Stream1,
        },
        Vector {
            _handler: DMA2_Stream2,
        },
        Vector {
            _handler: DMA2_Stream3,
        },
        Vector {
            _handler: DMA2_Stream4,
        },
        Vector { _handler: ETH },
        Vector { _handler: ETH_WKUP },
        Vector {
            _handler: FDCAN_CAL,
        },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector {
            _handler: DMA2_Stream5,
        },
        Vector {
            _handler: DMA2_Stream6,
        },
        Vector {
            _handler: DMA2_Stream7,
        },
        Vector { _handler: USART6 },
        Vector { _handler: I2C3_EV },
        Vector { _handler: I2C3_ER },
        Vector {
            _handler: OTG_HS_EP1_OUT,
        },
        Vector {
            _handler: OTG_HS_EP1_IN,
        },
        Vector {
            _handler: OTG_HS_WKUP,
        },
        Vector { _handler: OTG_HS },
        Vector {
            _handler: DCMI_PSSI,
        },
        Vector { _handler: CRYP },
        Vector { _handler: HASH_RNG },
        Vector { _handler: FPU },
        Vector { _handler: UART7 },
        Vector { _handler: UART8 },
        Vector { _handler: SPI4 },
        Vector { _handler: SPI5 },
        Vector { _handler: SPI6 },
        Vector { _handler: SAI1 },
        Vector { _handler: LTDC },
        Vector { _handler: LTDC_ER },
        Vector { _handler: DMA2D },
        Vector { _reserved: 0 },
        Vector { _handler: OCTOSPI1 },
        Vector { _handler: LPTIM1 },
        Vector { _handler: CEC },
        Vector { _handler: I2C4_EV },
        Vector { _handler: I2C4_ER },
        Vector { _handler: SPDIF_RX },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _handler: SWPMI1 },
        Vector { _handler: TIM15 },
        Vector { _handler: TIM16 },
        Vector { _handler: TIM17 },
        Vector {
            _handler: MDIOS_WKUP,
        },
        Vector { _handler: MDIOS },
        Vector { _reserved: 0 },
        Vector { _handler: MDMA },
        Vector { _reserved: 0 },
        Vector { _handler: SDMMC2 },
        Vector { _handler: HSEM1 },
        Vector { _reserved: 0 },
        Vector { _handler: ADC3 },
        Vector { _reserved: 0 },
        Vector {
            _handler: BDMA_Channel0,
        },
        Vector {
            _handler: BDMA_Channel1,
        },
        Vector {
            _handler: BDMA_Channel2,
        },
        Vector {
            _handler: BDMA_Channel3,
        },
        Vector {
            _handler: BDMA_Channel4,
        },
        Vector {
            _handler: BDMA_Channel5,
        },
        Vector {
            _handler: BDMA_Channel6,
        },
        Vector {
            _handler: BDMA_Channel7,
        },
        Vector { _reserved: 0 },
        Vector { _handler: LPTIM2 },
        Vector { _handler: LPTIM3 },
        Vector { _handler: LPTIM4 },
        Vector { _handler: LPTIM5 },
        Vector { _handler: LPUART1 },
        Vector { _reserved: 0 },
        Vector { _handler: CRS },
        Vector { _handler: ECC },
        Vector { _handler: SAI4 },
        Vector { _handler: DTS },
        Vector { _reserved: 0 },
        Vector {
            _handler: WAKEUP_PIN,
        },
        Vector { _handler: OCTOSPI2 },
        Vector { _handler: OTFDEC1 },
        Vector { _handler: OTFDEC2 },
        Vector { _handler: FMAC },
        Vector { _handler: CORDIC },
        Vector { _handler: UART9 },
        Vector { _handler: USART10 },
        Vector { _handler: I2C5_EV },
        Vector { _handler: I2C5_ER },
        Vector {
            _handler: FDCAN3_IT0,
        },
        Vector {
            _handler: FDCAN3_IT1,
        },
        Vector { _handler: TIM23 },
        Vector { _handler: TIM24 },
    ];
}
impl_gpio_pin!(PA0, 0, 0, EXTI0);
impl_gpio_pin!(PA1, 0, 1, EXTI1);
impl_gpio_pin!(PA2, 0, 2, EXTI2);
impl_gpio_pin!(PA3, 0, 3, EXTI3);
impl_gpio_pin!(PA4, 0, 4, EXTI4);
impl_gpio_pin!(PA5, 0, 5, EXTI5);
impl_gpio_pin!(PA6, 0, 6, EXTI6);
impl_gpio_pin!(PA7, 0, 7, EXTI7);
impl_gpio_pin!(PA8, 0, 8, EXTI8);
impl_gpio_pin!(PA9, 0, 9, EXTI9);
impl_gpio_pin!(PA10, 0, 10, EXTI10);
impl_gpio_pin!(PA11, 0, 11, EXTI11);
impl_gpio_pin!(PA12, 0, 12, EXTI12);
impl_gpio_pin!(PA13, 0, 13, EXTI13);
impl_gpio_pin!(PA14, 0, 14, EXTI14);
impl_gpio_pin!(PA15, 0, 15, EXTI15);
impl_gpio_pin!(PB0, 1, 0, EXTI0);
impl_gpio_pin!(PB1, 1, 1, EXTI1);
impl_gpio_pin!(PB2, 1, 2, EXTI2);
impl_gpio_pin!(PB3, 1, 3, EXTI3);
impl_gpio_pin!(PB4, 1, 4, EXTI4);
impl_gpio_pin!(PB5, 1, 5, EXTI5);
impl_gpio_pin!(PB6, 1, 6, EXTI6);
impl_gpio_pin!(PB7, 1, 7, EXTI7);
impl_gpio_pin!(PB8, 1, 8, EXTI8);
impl_gpio_pin!(PB9, 1, 9, EXTI9);
impl_gpio_pin!(PB10, 1, 10, EXTI10);
impl_gpio_pin!(PB11, 1, 11, EXTI11);
impl_gpio_pin!(PB12, 1, 12, EXTI12);
impl_gpio_pin!(PB13, 1, 13, EXTI13);
impl_gpio_pin!(PB14, 1, 14, EXTI14);
impl_gpio_pin!(PB15, 1, 15, EXTI15);
impl_gpio_pin!(PC0, 2, 0, EXTI0);
impl_gpio_pin!(PC1, 2, 1, EXTI1);
impl_gpio_pin!(PC2, 2, 2, EXTI2);
impl_gpio_pin!(PC3, 2, 3, EXTI3);
impl_gpio_pin!(PC4, 2, 4, EXTI4);
impl_gpio_pin!(PC5, 2, 5, EXTI5);
impl_gpio_pin!(PC6, 2, 6, EXTI6);
impl_gpio_pin!(PC7, 2, 7, EXTI7);
impl_gpio_pin!(PC8, 2, 8, EXTI8);
impl_gpio_pin!(PC9, 2, 9, EXTI9);
impl_gpio_pin!(PC10, 2, 10, EXTI10);
impl_gpio_pin!(PC11, 2, 11, EXTI11);
impl_gpio_pin!(PC12, 2, 12, EXTI12);
impl_gpio_pin!(PC13, 2, 13, EXTI13);
impl_gpio_pin!(PC14, 2, 14, EXTI14);
impl_gpio_pin!(PC15, 2, 15, EXTI15);
impl_gpio_pin!(PD0, 3, 0, EXTI0);
impl_gpio_pin!(PD1, 3, 1, EXTI1);
impl_gpio_pin!(PD2, 3, 2, EXTI2);
impl_gpio_pin!(PD3, 3, 3, EXTI3);
impl_gpio_pin!(PD4, 3, 4, EXTI4);
impl_gpio_pin!(PD5, 3, 5, EXTI5);
impl_gpio_pin!(PD6, 3, 6, EXTI6);
impl_gpio_pin!(PD7, 3, 7, EXTI7);
impl_gpio_pin!(PD8, 3, 8, EXTI8);
impl_gpio_pin!(PD9, 3, 9, EXTI9);
impl_gpio_pin!(PD10, 3, 10, EXTI10);
impl_gpio_pin!(PD11, 3, 11, EXTI11);
impl_gpio_pin!(PD12, 3, 12, EXTI12);
impl_gpio_pin!(PD13, 3, 13, EXTI13);
impl_gpio_pin!(PD14, 3, 14, EXTI14);
impl_gpio_pin!(PD15, 3, 15, EXTI15);
impl_gpio_pin!(PE0, 4, 0, EXTI0);
impl_gpio_pin!(PE1, 4, 1, EXTI1);
impl_gpio_pin!(PE2, 4, 2, EXTI2);
impl_gpio_pin!(PE3, 4, 3, EXTI3);
impl_gpio_pin!(PE4, 4, 4, EXTI4);
impl_gpio_pin!(PE5, 4, 5, EXTI5);
impl_gpio_pin!(PE6, 4, 6, EXTI6);
impl_gpio_pin!(PE7, 4, 7, EXTI7);
impl_gpio_pin!(PE8, 4, 8, EXTI8);
impl_gpio_pin!(PE9, 4, 9, EXTI9);
impl_gpio_pin!(PE10, 4, 10, EXTI10);
impl_gpio_pin!(PE11, 4, 11, EXTI11);
impl_gpio_pin!(PE12, 4, 12, EXTI12);
impl_gpio_pin!(PE13, 4, 13, EXTI13);
impl_gpio_pin!(PE14, 4, 14, EXTI14);
impl_gpio_pin!(PE15, 4, 15, EXTI15);
impl_gpio_pin!(PF0, 5, 0, EXTI0);
impl_gpio_pin!(PF1, 5, 1, EXTI1);
impl_gpio_pin!(PF2, 5, 2, EXTI2);
impl_gpio_pin!(PF3, 5, 3, EXTI3);
impl_gpio_pin!(PF4, 5, 4, EXTI4);
impl_gpio_pin!(PF5, 5, 5, EXTI5);
impl_gpio_pin!(PF6, 5, 6, EXTI6);
impl_gpio_pin!(PF7, 5, 7, EXTI7);
impl_gpio_pin!(PF8, 5, 8, EXTI8);
impl_gpio_pin!(PF9, 5, 9, EXTI9);
impl_gpio_pin!(PF10, 5, 10, EXTI10);
impl_gpio_pin!(PF11, 5, 11, EXTI11);
impl_gpio_pin!(PF12, 5, 12, EXTI12);
impl_gpio_pin!(PF13, 5, 13, EXTI13);
impl_gpio_pin!(PF14, 5, 14, EXTI14);
impl_gpio_pin!(PF15, 5, 15, EXTI15);
impl_gpio_pin!(PG0, 6, 0, EXTI0);
impl_gpio_pin!(PG1, 6, 1, EXTI1);
impl_gpio_pin!(PG2, 6, 2, EXTI2);
impl_gpio_pin!(PG3, 6, 3, EXTI3);
impl_gpio_pin!(PG4, 6, 4, EXTI4);
impl_gpio_pin!(PG5, 6, 5, EXTI5);
impl_gpio_pin!(PG6, 6, 6, EXTI6);
impl_gpio_pin!(PG7, 6, 7, EXTI7);
impl_gpio_pin!(PG8, 6, 8, EXTI8);
impl_gpio_pin!(PG9, 6, 9, EXTI9);
impl_gpio_pin!(PG10, 6, 10, EXTI10);
impl_gpio_pin!(PG11, 6, 11, EXTI11);
impl_gpio_pin!(PG12, 6, 12, EXTI12);
impl_gpio_pin!(PG13, 6, 13, EXTI13);
impl_gpio_pin!(PG14, 6, 14, EXTI14);
impl_gpio_pin!(PG15, 6, 15, EXTI15);
impl_gpio_pin!(PH0, 7, 0, EXTI0);
impl_gpio_pin!(PH1, 7, 1, EXTI1);
impl_gpio_pin!(PH2, 7, 2, EXTI2);
impl_gpio_pin!(PH3, 7, 3, EXTI3);
impl_gpio_pin!(PH4, 7, 4, EXTI4);
impl_gpio_pin!(PH5, 7, 5, EXTI5);
impl_gpio_pin!(PH6, 7, 6, EXTI6);
impl_gpio_pin!(PH7, 7, 7, EXTI7);
impl_gpio_pin!(PH8, 7, 8, EXTI8);
impl_gpio_pin!(PH9, 7, 9, EXTI9);
impl_gpio_pin!(PH10, 7, 10, EXTI10);
impl_gpio_pin!(PH11, 7, 11, EXTI11);
impl_gpio_pin!(PH12, 7, 12, EXTI12);
impl_gpio_pin!(PH13, 7, 13, EXTI13);
impl_gpio_pin!(PH14, 7, 14, EXTI14);
impl_gpio_pin!(PH15, 7, 15, EXTI15);
impl_gpio_pin!(PJ0, 9, 0, EXTI0);
impl_gpio_pin!(PJ1, 9, 1, EXTI1);
impl_gpio_pin!(PJ2, 9, 2, EXTI2);
impl_gpio_pin!(PJ3, 9, 3, EXTI3);
impl_gpio_pin!(PJ4, 9, 4, EXTI4);
impl_gpio_pin!(PJ5, 9, 5, EXTI5);
impl_gpio_pin!(PJ6, 9, 6, EXTI6);
impl_gpio_pin!(PJ7, 9, 7, EXTI7);
impl_gpio_pin!(PJ8, 9, 8, EXTI8);
impl_gpio_pin!(PJ9, 9, 9, EXTI9);
impl_gpio_pin!(PJ10, 9, 10, EXTI10);
impl_gpio_pin!(PJ11, 9, 11, EXTI11);
impl_gpio_pin!(PJ12, 9, 12, EXTI12);
impl_gpio_pin!(PJ13, 9, 13, EXTI13);
impl_gpio_pin!(PJ14, 9, 14, EXTI14);
impl_gpio_pin!(PJ15, 9, 15, EXTI15);
impl_gpio_pin!(PK0, 10, 0, EXTI0);
impl_gpio_pin!(PK1, 10, 1, EXTI1);
impl_gpio_pin!(PK2, 10, 2, EXTI2);
impl_gpio_pin!(PK3, 10, 3, EXTI3);
impl_gpio_pin!(PK4, 10, 4, EXTI4);
impl_gpio_pin!(PK5, 10, 5, EXTI5);
impl_gpio_pin!(PK6, 10, 6, EXTI6);
impl_gpio_pin!(PK7, 10, 7, EXTI7);
impl_gpio_pin!(PK8, 10, 8, EXTI8);
impl_gpio_pin!(PK9, 10, 9, EXTI9);
impl_gpio_pin!(PK10, 10, 10, EXTI10);
impl_gpio_pin!(PK11, 10, 11, EXTI11);
impl_gpio_pin!(PK12, 10, 12, EXTI12);
impl_gpio_pin!(PK13, 10, 13, EXTI13);
impl_gpio_pin!(PK14, 10, 14, EXTI14);
impl_gpio_pin!(PK15, 10, 15, EXTI15);
impl_rng!(0x48021800);
impl_sdmmc!(SDMMC1, 0x52007000);
impl_sdmmc_pin!(SDMMC1, D0Pin, PB13, 12);
impl_sdmmc_pin!(SDMMC1, D4Pin, PB8, 12);
impl_sdmmc_pin!(SDMMC1, D5Pin, PB9, 12);
impl_sdmmc_pin!(SDMMC1, D2Pin, PC10, 12);
impl_sdmmc_pin!(SDMMC1, D3Pin, PC11, 12);
impl_sdmmc_pin!(SDMMC1, CkPin, PC12, 12);
impl_sdmmc_pin!(SDMMC1, D6Pin, PC6, 12);
impl_sdmmc_pin!(SDMMC1, D7Pin, PC7, 12);
impl_sdmmc_pin!(SDMMC1, D0Pin, PC8, 12);
impl_sdmmc_pin!(SDMMC1, D1Pin, PC9, 12);
impl_sdmmc_pin!(SDMMC1, CmdPin, PD2, 12);
impl_sdmmc!(SDMMC2, 0x48022400);
impl_sdmmc_pin!(SDMMC2, CmdPin, PA0, 9);
impl_sdmmc_pin!(SDMMC2, D0Pin, PB14, 9);
impl_sdmmc_pin!(SDMMC2, D1Pin, PB15, 9);
impl_sdmmc_pin!(SDMMC2, D2Pin, PB3, 9);
impl_sdmmc_pin!(SDMMC2, D3Pin, PB4, 9);
impl_sdmmc_pin!(SDMMC2, D4Pin, PB8, 10);
impl_sdmmc_pin!(SDMMC2, D5Pin, PB9, 10);
impl_sdmmc_pin!(SDMMC2, CkPin, PC1, 9);
impl_sdmmc_pin!(SDMMC2, D6Pin, PC6, 10);
impl_sdmmc_pin!(SDMMC2, D7Pin, PC7, 10);
impl_sdmmc_pin!(SDMMC2, CkPin, PD6, 11);
impl_sdmmc_pin!(SDMMC2, CmdPin, PD7, 11);
impl_sdmmc_pin!(SDMMC2, D1Pin, PG10, 11);
impl_sdmmc_pin!(SDMMC2, D2Pin, PG11, 10);
impl_sdmmc_pin!(SDMMC2, D3Pin, PG12, 10);
impl_sdmmc_pin!(SDMMC2, D6Pin, PG13, 10);
impl_sdmmc_pin!(SDMMC2, D7Pin, PG14, 10);
impl_sdmmc_pin!(SDMMC2, D0Pin, PG9, 11);
