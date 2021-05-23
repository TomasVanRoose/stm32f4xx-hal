use crate::pac::rcc::RegisterBlock as RccRB;

/// Enable/disable peripheral
pub trait Enable {
    fn enable(rcc: &RccRB);
    fn disable(rcc: &RccRB);
}

/// Low power enable/disable peripheral
pub trait LPEnable {
    fn low_power_enable(rcc: &RccRB);
    fn low_power_disable(rcc: &RccRB);
}

/// Reset peripheral
pub trait Reset {
    fn reset(rcc: &RccRB);
}

macro_rules! bus {
    ($($PER:ident => ($busenr:ident, $peren:ident, $buslpenr:ident, $perlpen:ident, $busrstr:ident, $perrst:ident),)+) => {
        $(
            impl Enable for crate::pac::$PER {
                #[inline(always)]
                fn enable(rcc: &RccRB) {
                    rcc.$busenr.modify(|_, w| w.$peren().set_bit());
                    // Stall the pipeline to work around erratum 2.1.13 (DM00037591)
                    cortex_m::asm::dsb();
                }
                #[inline(always)]
                fn disable(rcc: &RccRB) {
                    rcc.$busenr.modify(|_, w| w.$peren().clear_bit());
                }
            }

            impl LPEnable for crate::pac::$PER {
                #[inline(always)]
                fn low_power_enable(rcc: &RccRB) {
                    rcc.$buslpenr.modify(|_, w| w.$perlpen().set_bit());
                    // Stall the pipeline to work around erratum 2.1.13 (DM00037591)
                    cortex_m::asm::dsb();
                }
                #[inline(always)]
                fn low_power_disable(rcc: &RccRB) {
                    rcc.$buslpenr.modify(|_, w| w.$perlpen().clear_bit());
                }
            }
            impl Reset for crate::pac::$PER {
                #[inline(always)]
                fn reset(rcc: &RccRB) {
                    rcc.$busrstr.modify(|_, w| w.$perrst().set_bit());
                    rcc.$busrstr.modify(|_, w| w.$perrst().clear_bit());
                }
            }
        )+
    }
}

bus! {
    DMA1 => (ahb1enr, dma1en, ahb1lpenr, dma1lpen, ahb1rstr, dma1rst),
    DMA2 => (ahb1enr, dma2en, ahb1lpenr, dma2lpen, ahb1rstr, dma2rst),
}

bus! {
    GPIOA => (ahb1enr, gpioaen, ahb1lpenr, gpioalpen, ahb1rstr, gpioarst),
    GPIOB => (ahb1enr, gpioben, ahb1lpenr, gpioblpen, ahb1rstr, gpiobrst),
    GPIOC => (ahb1enr, gpiocen, ahb1lpenr, gpioclpen, ahb1rstr, gpiocrst),
    GPIOH => (ahb1enr, gpiohen, ahb1lpenr, gpiohlpen, ahb1rstr, gpiohrst),
}

#[cfg(any(feature = "gpiod", feature = "gpioe",))]
bus! {
    GPIOD => (ahb1enr, gpioden, ahb1lpenr, gpiodlpen, ahb1rstr, gpiodrst),
    GPIOE => (ahb1enr, gpioeen, ahb1lpenr, gpioelpen, ahb1rstr, gpioerst),
}
#[cfg(any(feature = "gpiof", feature = "gpiog",))]
bus! {
    GPIOF => (ahb1enr, gpiofen, ahb1lpenr, gpioflpen, ahb1rstr, gpiofrst),
    GPIOG => (ahb1enr, gpiogen, ahb1lpenr, gpioglpen, ahb1rstr, gpiogrst),
}

#[cfg(feature = "gpioi")]
bus! {
    GPIOI => (ahb1enr, gpioien, ahb1lpenr, gpioilpen, ahb1rstr, gpioirst),
}

#[cfg(any(feature = "gpioj", feature = "gpiok",))]
bus! {
    GPIOJ => (ahb1enr, gpiojen, ahb1lpenr, gpiojlpen, ahb1rstr, gpiojrst),
    GPIOK => (ahb1enr, gpioken, ahb1lpenr, gpioklpen, ahb1rstr, gpiokrst),
}

// TODO: need checks
#[cfg(feature = "otg-fs")]
bus! {
    OTG_FS_DEVICE => (ahb2enr, otgfsen, ahb2lpenr, otgfslpen, ahb2rstr, otgfsrst),
    OTG_FS_GLOBAL => (ahb2enr, otgfsen, ahb2lpenr, otgfslpen, ahb2rstr, otgfsrst),
    OTG_FS_HOST => (ahb2enr, otgfsen, ahb2lpenr, otgfslpen, ahb2rstr, otgfsrst),
    OTG_FS_PWRCLK => (ahb2enr, otgfsen, ahb2lpenr, otgfslpen, ahb2rstr, otgfsrst),
}

bus! {
    SPI1 => (apb2enr, spi1en, apb2lpenr, spi1lpen, apb2rstr, spi1rst),
    SPI2 => (apb1enr, spi2en, apb1lpenr, spi2lpen, apb1rstr, spi2rst),
}
#[cfg(feature = "spi3")]
bus! {
    SPI3 => (apb1enr, spi3en, apb1lpenr, spi3lpen, apb1rstr, spi3rst),
}

#[cfg(feature = "spi4")]
bus! {
    SPI4 => (apb2enr, spi4en, apb2lpenr, spi4lpen, apb2rstr, spi4rst),
}

#[cfg(feature = "spi5")]
bus! {
    SPI5 => (apb2enr, spi5en, apb2lpenr, spi5lpen, apb2rstr, spi5rst),
}

#[cfg(feature = "spi6")]
bus! {
    SPI6 => (apb2enr, spi6en, apb2lpenr, spi6lpen, apb2rstr, spi6rst),
}

bus! {
    I2C1 => (apb1enr, i2c1en, apb1lpenr, i2c1lpen, apb1rstr, i2c1rst),
    I2C2 => (apb1enr, i2c2en, apb1lpenr, i2c2lpen, apb1rstr, i2c2rst),
}
#[cfg(feature = "i2c3")]
bus! {
    I2C3 => (apb1enr, i2c3en, apb1lpenr, i2c3lpen, apb1rstr, i2c3rst),
}

// TODO: fix uart2rst
bus! {
    USART1 => (apb2enr, usart1en, apb2lpenr, usart1lpen, apb2rstr, usart1rst),
    USART2 => (apb1enr, usart2en, apb1lpenr, usart2lpen, apb1rstr, uart2rst),
    USART6 => (apb2enr, usart6en, apb2lpenr, usart6lpen, apb2rstr, usart6rst),
}
