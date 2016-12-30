//! Interrupts
//!
//! **WARNING** Here be dragons. Interrupts are key for writing asynchronous
//! programs but they also open the door to data races. Tread with care and mind
//! your `unsafe`s.
//!
//! All the interrupts prefixed with an underscore (`_`) can be overridden by
//! the top crate.

use cortex_m::Handler;

/// List of all the interrupts as allocated in the vector table.
///
/// `None` indicates that the spot is RESERVED.
#[doc(hidden)]
#[export_name = "_INTERRUPTS"]
pub static INTERRUPTS: [Option<Handler>; 82] = [Some(_wwdg),
                                                Some(_pvd),
                                                Some(_tamp_stamp),
                                                Some(_rtc_wkup),
                                                Some(_flash),
                                                Some(_rcc),
                                                Some(_exti0),
                                                Some(_exti1),
                                                Some(_exti2),
                                                Some(_exti3),
                                                Some(_exti4),
                                                Some(_dma1_stream0),
                                                Some(_dma1_stream1),
                                                Some(_dma1_stream2),
                                                Some(_dma1_stream3),
                                                Some(_dma1_stream4),
                                                Some(_dma1_stream5),
                                                Some(_dma1_stream6),
                                                Some(_adc),
                                                Some(_can1_tx),
                                                Some(_can1_rx0),
                                                Some(_can1_rx1),
                                                Some(_can1_sce),
                                                Some(_exti9_5),
                                                Some(_tim1_brk_tim9),
                                                Some(_tim1_up_tim10),
                                                Some(_tim1_trg_com_tim11),
                                                Some(_tim1_cc),
                                                Some(_tim2),
                                                Some(_tim3),
                                                Some(_tim4),
                                                Some(_i2c1_ev),
                                                Some(_i2c1_er),
                                                Some(_i2c2_ev),
                                                Some(_i2c2_er),
                                                Some(_spi1),
                                                Some(_spi2),
                                                Some(_usart1),
                                                Some(_usart2),
                                                Some(_usart3),
                                                Some(_exti15_10),
                                                Some(_rtc_alarm),
                                                Some(_otg_fs_wkup),
                                                Some(_tim8_brk_tim12),
                                                Some(_tim8_up_tim13),
                                                Some(_tim8_trg_com_tim14),
                                                Some(_tim8_cc),
                                                Some(_dma1_stream7),
                                                Some(_fsmc),
                                                Some(_sdio),
                                                Some(_tim5),
                                                Some(_spi3),
                                                Some(_uart4),
                                                Some(_uart5),
                                                Some(_tim6_dac),
                                                Some(_tim7),
                                                Some(_dma2_stream0),
                                                Some(_dma2_stream1),
                                                Some(_dma2_stream2),
                                                Some(_dma2_stream3),
                                                Some(_dma2_stream4),
                                                Some(_eth),
                                                Some(_eth_wkup),
                                                Some(_can2_tx),
                                                Some(_can2_rx0),
                                                Some(_can2_rx1),
                                                Some(_can2_sce),
                                                Some(_otg_fs),
                                                Some(_dma2_stream5),
                                                Some(_dma2_stream6),
                                                Some(_dma2_stream7),
                                                Some(_usart6),
                                                Some(_i2c3_ev),
                                                Some(_i2c3_er),
                                                Some(_otg_hs_ep1_out),
                                                Some(_otg_hs_ep1_in),
                                                Some(_otg_hs_wkup),
                                                Some(_otg_hs),
                                                Some(_dcmi),
                                                Some(_cryp),
                                                Some(_hash_rng),
                                                Some(_fpu)];

extern "C" {
    /// Window Watchdog interrupt
    pub fn _wwdg();
    /// PVD through EXTI line detection interrupt
    pub fn _pvd();
    /// Tamper and TimeStamp interrupts through the EXTI line
    pub fn _tamp_stamp();
    /// RTC Wakeup interrupt through the EXTI line
    pub fn _rtc_wkup();
    /// Flash global interrupt
    pub fn _flash();
    /// RCC global interrupt
    pub fn _rcc();
    /// EXTI Line0 interrupt
    pub fn _exti0();
    /// EXTI Line1 interrupt
    pub fn _exti1();
    /// EXTI Line2 interrupt
    pub fn _exti2();
    /// EXTI Line3 interrupt
    pub fn _exti3();
    /// EXTI Line4 interrupt
    pub fn _exti4();
    /// DMA1 Stream0 global interrupt
    pub fn _dma1_stream0();
    /// DMA1 Stream1 global interrupt
    pub fn _dma1_stream1();
    /// DMA1 Stream2 global interrupt
    pub fn _dma1_stream2();
    /// DMA1 Stream3 global interrupt
    pub fn _dma1_stream3();
    /// DMA1 Stream4 global interrupt
    pub fn _dma1_stream4();
    /// DMA1 Stream5 global interrupt
    pub fn _dma1_stream5();
    /// DMA1 Stream6 global interrupt
    pub fn _dma1_stream6();
    /// ADC1, ADC2 and ADC3 global interrupts
    pub fn _adc();
    /// CAN1 TX interrupt
    pub fn _can1_tx();
    /// CAN1 RX0 interrupt
    pub fn _can1_rx0();
    /// CAN1 RX1 interrupt
    pub fn _can1_rx1();
    /// CAN1 SCE interrupt
    pub fn _can1_sce();
    /// EXTI Line[9:5] interrupts
    pub fn _exti9_5();
    /// TIM1 Break interrupt and TIM9 global interrupt
    pub fn _tim1_brk_tim9();
    /// TIM1 Update interrupt and TIM10 global interrupt
    pub fn _tim1_up_tim10();
    /// TIM1 Trigger and Commutation interrupts and TIM11 global interrupt
    pub fn _tim1_trg_com_tim11();
    /// TIM1 Capture Compare interrupt
    pub fn _tim1_cc();
    /// TIM2 global interrupt
    pub fn _tim2();
    /// TIM3 global interrupt
    pub fn _tim3();
    /// TIM4 global interrupt
    pub fn _tim4();
    /// I2C1 event interrupt
    pub fn _i2c1_ev();
    /// I2C1 error interrupt
    pub fn _i2c1_er();
    /// I2C2 event interrupt
    pub fn _i2c2_ev();
    /// I2C2 error interrupt
    pub fn _i2c2_er();
    /// SPI1 global interrupt
    pub fn _spi1();
    /// SPI2 global interrupt
    pub fn _spi2();
    /// USART1 globlal interrupt
    pub fn _usart1();
    /// USART2 global interrupt
    pub fn _usart2();
    /// USART3 global interrupt
    pub fn _usart3();
    /// EXTI Line[15:10] interrupts
    pub fn _exti15_10();
    /// RTC Alarms (A and B) through EXTI line interrupt
    pub fn _rtc_alarm();
    /// USB On-The-Go FS Wakeup through EXTI line interrupt
    pub fn _otg_fs_wkup();
    /// TIM8 Break interrupt and TIM12 global interrupt
    pub fn _tim8_brk_tim12();
    /// TIM8 Update interrupt and TIM13 global interrupt
    pub fn _tim8_up_tim13();
    /// TIM8 Trigger and Communtation interrupts and TIM14 global interrupt
    pub fn _tim8_trg_com_tim14();
    /// TIM8 Capture Compare interrupt
    pub fn _tim8_cc();
    /// DMA1 Stream7 global interrupt
    pub fn _dma1_stream7();
    /// FSMC global interrupt
    pub fn _fsmc();
    /// SDIO global interrupt
    pub fn _sdio();
    /// TIM5 global interrupt
    pub fn _tim5();
    /// SPI3 global interrupt
    pub fn _spi3();
    /// UART4 global interrupt
    pub fn _uart4();
    /// UART5 global interrupt
    pub fn _uart5();
    /// TIM6 global interrupt, DAC1 and DAC2 underrun error interrupts
    pub fn _tim6_dac();
    /// TIM7 global interrupt
    pub fn _tim7();
    /// DMA2 Stream0 global interrupt
    pub fn _dma2_stream0();
    /// DMA2 Stream1 global interrupt
    pub fn _dma2_stream1();
    /// DMA2 Stream2 global interrupt
    pub fn _dma2_stream2();
    /// DMA2 Stream3 global interrupt
    pub fn _dma2_stream3();
    /// DMA2 Stream4 global interrupt
    pub fn _dma2_stream4();
    /// Ethernet global interrupt
    pub fn _eth();
    /// Ethernet Wakeup through EXTI line interrupt
    pub fn _eth_wkup();
    /// CAN2 TX interrupt
    pub fn _can2_tx();
    /// CAN2 RX0 interrupt
    pub fn _can2_rx0();
    /// CAN2 RX1 interrupt
    pub fn _can2_rx1();
    /// CAN2 SCN interrupt
    pub fn _can2_sce();
    /// USB On-The-Go FS global interrupt
    pub fn _otg_fs();
    /// DMA2 Stream5 global interrupt
    pub fn _dma2_stream5();
    /// DMA2 Stream6 global interrupt
    pub fn _dma2_stream6();
    /// DMA2 Stream7 global interrupt
    pub fn _dma2_stream7();
    /// USART6 global interrupt
    pub fn _usart6();
    /// I2C3 event interrupt
    pub fn _i2c3_ev();
    /// I2C3 error interrupt
    pub fn _i2c3_er();
    /// USB On-The-Go HS End Point 1 Out global interrupt
    pub fn _otg_hs_ep1_out();
    /// USB On-The-Go HS End Point 1 In global interrupt
    pub fn _otg_hs_ep1_in();
    /// USB On-The-Go HS Wakeup through EXTI interrupt
    pub fn _otg_hs_wkup();
    /// USB On-The-Go HS global interrupt
    pub fn _otg_hs();
    /// DCMI global interrupt
    pub fn _dcmi();
    /// CRYP crypto global interrupt
    pub fn _cryp();
    /// Hask and RNG global interrupt
    pub fn _hash_rng();
    /// FPU global interrupt
    pub fn _fpu();
}
