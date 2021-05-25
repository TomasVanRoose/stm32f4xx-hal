//! Digital Camera Interface
//!
//! ## Main features (*to implement*)
//! * [ ]  8-, 10-, 12- or 14-bit parallel interface
//! * [ ] Embedded/external line and frame synchronization
//! * [ ] Continuous or snapshot mode
//! * [ ] Crop feature
//! * [ ] Supports the following data formats:
//!     + [ ] 8/10/12/14- bit progressive video: either monochrome or raw bayer
//!     + [ ] YCbCr 4:2:2 progressive video
//!     + [ ] RGB 565 progressive video
//!     + [ ] Compressed data: JPEG
//!
//! ## DCMI pins
//! | Name | Signal type |
//! |------|-------------|
//! | D\[0:13\] | Data inputs |
//! | HSYNC | Horizontal synchronization input |
//! | VSYNC | Vertical synchronization input |
//! | PIXCLK | Pixel clock input |
//!
//! ## DMA interface
//! The DMA interface is active when the CAPTURE bit in the DCMI_CR register is set.
//! A DMA request is generated each time the camera interface receives a complete 32-bit data block in its register.
//!
//! ## Notes
//! For compressed data, the DCMI supports only the hardware synchronization mode.
//! In this case, VSYNC is used as a start/end of the image, and HSYNC is used as a Data Valid signal.
pub mod traits;

use crate::dma;
use config::DcmiConfig;
use traits::{
    sealed::Bits, DataPins, DcmiDataPins, DcmiSignalPins, Instance, RccEnable, SignalPins,
};

/// DCMI abstraction.
pub struct Dcmi<DataPins, SignalPins, S, C, I, BUF>
where
    S: dma::traits::Stream,
    C: dma::traits::Channel,
    I: dma::traits::PeriAddress<MemSize = u32>,
{
    dma_transfer: dma::Transfer<S, C, I, dma::PeripheralToMemory, BUF>,
    config: DcmiConfig,
    data_pins: DataPins,
    signal_pins: SignalPins,
}

impl<DPins, SPins, S, C, I, BUF> Dcmi<DPins, SPins, S, C, I, BUF>
where
    DPins: DcmiDataPins,
    SPins: DcmiSignalPins,
    S: dma::traits::Stream,
    C: dma::traits::Channel,
    I: Instance
        + dma::traits::PeriAddress<MemSize = u32>
        + dma::traits::DMASet<S, C, dma::PeripheralToMemory>,
{
    pub fn init(
        data_pins: DPins,
        signal_pins: SPins,
        dma_transfer: dma::Transfer<S, C, I, dma::PeripheralToMemory, BUF>,
        config: DcmiConfig,
    ) -> Self {
        // Make sure the dma_transfer is not yet enabled
        //assert!(unsafe { dma_transfer.get_stream().is_enabled() });

        let dcmi = unsafe { &(*I::ptr()) };

        dcmi.cr.reset();
        // Write to the DCMI config register
        dcmi.cr.modify(|_, w| unsafe {
            w.edm()
                .bits(config.ext_data_mode.into())
                .fcrc()
                .bits(config.frame_capt_ctrl.into())
                .vspol()
                .bit(config.vertical_sync_polarity.into())
                .hspol()
                .bit(config.horizontal_sync_polarity.into())
                .pckpol()
                .bit(config.pixel_clock_polarity.into())
                .ess()
                .bit(config.embedded_sync)
                .jpeg()
                .bit(config.jpeg)
                .crop()
                .bit(config.crop)
        });
        // Enable interrupts
        dcmi.ier
            .modify(|_, w| unsafe { w.bits(config.interrupt_raw.into()) });
        // Enable DCMI timer
        crate::pac::DCMI::rcc_enable();
        Self {
            data_pins,
            signal_pins,
            dma_transfer,
            config,
        }
    }

    pub fn start(&mut self) {
        let capture_mode: bool = self.config.capture_mode.into();
        self.dma_transfer.start(|dcmi| {
            dcmi.cr.modify(|_, w| w.capture().bit(capture_mode));
            dcmi.cr.modify(|_, w| w.enable().set_bit());
        });
    }

    pub fn clear_interrupt(&mut self, interrupt: DcmiInterrupt) {
        let dcmi = unsafe { &(*I::ptr()) };
        dcmi.icr
            .write(|w| unsafe { w.bits(interrupt.bits().into()) });
    }
}

/// DCMI Interrupts.
#[derive(Copy, Clone)]
pub enum DcmiInterrupt {
    /// Indicates the end of line.
    Line,
    /// Indicates the end of frame capture.
    Frame,
    /// indicates the overrun of data reception.
    Overrun,
    /// Indicates the synchronization frame.
    VSync,
    /// Indicates the detection of an error in the embedded synchronization frame detection.
    Error,
    /// All interrupts.
    All,
}

impl Bits<u8> for DcmiInterrupt {
    fn bits(self) -> u8 {
        match self {
            DcmiInterrupt::Line => 0b0001_0000,
            DcmiInterrupt::VSync => 0b1000,
            DcmiInterrupt::Error => 0b0100,
            DcmiInterrupt::Overrun => 0b0010,
            DcmiInterrupt::Frame => 0b0001,
            DcmiInterrupt::All => 0b0001_1111,
        }
    }
}

/// Configuration
pub mod config {
    use super::traits::sealed::Bits;
    use super::DcmiInterrupt;

    /// DCMI configuration builder struct.
    pub struct DcmiConfig {
        pub(crate) ext_data_mode: ExtendedDataMode,
        pub(crate) frame_capt_ctrl: FrameRateCaptureControl,
        pub(crate) vertical_sync_polarity: Polarity,
        pub(crate) horizontal_sync_polarity: Polarity,
        pub(crate) pixel_clock_polarity: PixelClockPolarity,
        pub(crate) embedded_sync: bool,
        pub(crate) jpeg: bool,
        pub(crate) crop: bool,
        pub(crate) capture_mode: CaptureMode,
        // stores the logic OR of all interrupts
        pub(crate) interrupt_raw: u8,
    }

    impl DcmiConfig {
        pub fn ext_data_mode(mut self, ext_data_mode: ExtendedDataMode) -> Self {
            self.ext_data_mode = ext_data_mode;
            self
        }
        pub fn frame_capt_ctrl(mut self, frame_capt_ctrl: FrameRateCaptureControl) -> Self {
            self.frame_capt_ctrl = frame_capt_ctrl;
            self
        }
        pub fn vertical_sync_polarity(mut self, vsync_polarity: Polarity) -> Self {
            self.vertical_sync_polarity = vsync_polarity;
            self
        }
        pub fn horizontal_sync_polarity(mut self, hsync_polarity: Polarity) -> Self {
            self.horizontal_sync_polarity = hsync_polarity;
            self
        }
        pub fn pixel_clock_polarity(mut self, pxl_clk_polarity: PixelClockPolarity) -> Self {
            self.pixel_clock_polarity = pxl_clk_polarity;
            self
        }
        pub fn embedded_sync(mut self, embedded_sync: bool) -> Self {
            self.embedded_sync = embedded_sync;
            self
        }
        pub fn jpeg(mut self, jpeg: bool) -> Self {
            self.jpeg = jpeg;
            self
        }
        pub fn crop(mut self, crop: bool) -> Self {
            self.crop = crop;
            self
        }
        pub fn capture_mode(mut self, capture_mode: CaptureMode) -> Self {
            self.capture_mode = capture_mode;
            self
        }
        pub fn attach_interrupt(mut self, int: DcmiInterrupt) -> Self {
            self.interrupt_raw |= int.bits() as u8;
            self
        }
    }

    /// ## Default configuration
    /// | Config | Value |
    /// |------|-----|
    /// | Extended data mode | 8 bits |
    /// | Frame capture control | All frames|
    /// | Vertical sync polarity | Active low |
    /// | Horizontal sync polarity | Active low |
    /// | Pixel clock polarity | Falling edge |
    /// | Embedded sync | Disabled |
    /// | jpeg | Disabled |
    /// | Crop | Disabled |
    /// | Capture mode | Continuous capture mode|
    /// | Enabled interrupts | None |
    impl Default for DcmiConfig {
        fn default() -> Self {
            Self {
                ext_data_mode: ExtendedDataMode::Bit8,
                frame_capt_ctrl: FrameRateCaptureControl::All,
                vertical_sync_polarity: Polarity::ActiveLow,
                horizontal_sync_polarity: Polarity::ActiveLow,
                pixel_clock_polarity: PixelClockPolarity::FallingEdge,
                embedded_sync: false,
                jpeg: false,
                crop: false,
                capture_mode: CaptureMode::Continuous,
                interrupt_raw: 0,
            }
        }
    }

    #[derive(Copy, Clone)]
    pub enum ExtendedDataMode {
        Bit8,
        Bit10,
        Bit12,
        Bit14,
    }
    impl Into<u8> for ExtendedDataMode {
        fn into(self) -> u8 {
            match self {
                Self::Bit8 => 0b00,
                Self::Bit10 => 0b01,
                Self::Bit12 => 0b10,
                Self::Bit14 => 0b11,
            }
        }
    }
    #[derive(Copy, Clone)]
    pub enum FrameRateCaptureControl {
        All,
        EverySecond,
        EveryFourth,
    }
    impl Into<u8> for FrameRateCaptureControl {
        fn into(self) -> u8 {
            match self {
                Self::All => 0b00,
                Self::EverySecond => 0b01,
                Self::EveryFourth => 0b10,
            }
        }
    }
    #[derive(Copy, Clone)]
    pub enum Polarity {
        ActiveHigh,
        ActiveLow,
    }
    impl Into<bool> for Polarity {
        fn into(self) -> bool {
            match self {
                Self::ActiveHigh => true,
                Self::ActiveLow => false,
            }
        }
    }
    #[derive(Copy, Clone)]
    pub enum PixelClockPolarity {
        FallingEdge,
        RisingEdge,
    }
    impl Into<bool> for PixelClockPolarity {
        fn into(self) -> bool {
            match self {
                Self::RisingEdge => true,
                Self::FallingEdge => false,
            }
        }
    }
    #[derive(Copy, Clone)]
    pub enum CaptureMode {
        Continuous,
        Snapshot,
    }
    impl Into<bool> for CaptureMode {
        fn into(self) -> bool {
            match self {
                Self::Snapshot => true,
                Self::Continuous => false,
            }
        }
    }
}
