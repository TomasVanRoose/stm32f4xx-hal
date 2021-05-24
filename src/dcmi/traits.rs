use crate::gpio::{
    gpioa::*, gpiob::*, gpioc::*, gpiod::*, gpioe::*, gpiof::*, gpiog::*, gpioh::*, gpioi::*,
    Alternate, AF13,
};
use crate::pac::dcmi::RegisterBlock as DCMIRegisterBlock;

pub trait Instance: Deref<Target = DCMIRegisterBlock> {}

pub trait DcmiD0 {}
pub trait DcmiD1 {}
pub trait DcmiD2 {}
pub trait DcmiD3 {}
pub trait DcmiD4 {}
pub trait DcmiD5 {}
pub trait DcmiD6 {}
pub trait DcmiD7 {}
pub trait DcmiD8 {}
pub trait DcmiD9 {}
pub trait DcmiD10 {}
pub trait DcmiD11 {}
pub trait DcmiD12 {}
pub trait DcmiD13 {}
pub trait DcmiHSYNC {}
pub trait DcmiVSYNC {}
pub trait DcmiPIXCLK {}

impl DcmiHSYNC for PA4<Alternate<AF13>> {}
impl DcmiPIXCLK for PA6<Alternate<AF13>> {}
impl DcmiHSYNC for PH8<Alternate<AF13>> {}
impl DcmiVSYNC for PB7<Alternate<AF13>> {}
impl DcmiVSYNC for PI5<Alternate<AF13>> {}

impl DcmiD0 for PA9<Alternate<AF13>> {}
impl DcmiD0 for PC6<Alternate<AF13>> {}
impl DcmiD0 for PH9<Alternate<AF13>> {}

impl DcmiD1 for PA10<Alternate<AF13>> {}
impl DcmiD1 for PC7<Alternate<AF13>> {}
impl DcmiD1 for PH10<Alternate<AF13>> {}

impl DcmiD2 for PC8<Alternate<AF13>> {}
impl DcmiD2 for PE0<Alternate<AF13>> {}
impl DcmiD2 for PH11<Alternate<AF13>> {}

impl DcmiD3 for PC9<Alternate<AF13>> {}
impl DcmiD3 for PE1<Alternate<AF13>> {}
impl DcmiD3 for PH12<Alternate<AF13>> {}

impl DcmiD4 for PC11<Alternate<AF13>> {}
impl DcmiD4 for PE4<Alternate<AF13>> {}
impl DcmiD4 for PH14<Alternate<AF13>> {}

impl DcmiD5 for PB6<Alternate<AF13>> {}
impl DcmiD5 for PI4<Alternate<AF13>> {}

impl DcmiD6 for PB8<Alternate<AF13>> {}
impl DcmiD6 for PE5<Alternate<AF13>> {}
impl DcmiD6 for PI6<Alternate<AF13>> {}

impl DcmiD7 for PB9<Alternate<AF13>> {}
impl DcmiD7 for PE6<Alternate<AF13>> {}
impl DcmiD7 for PI7<Alternate<AF13>> {}

impl DcmiD8 for PC10<Alternate<AF13>> {}
impl DcmiD8 for PI1<Alternate<AF13>> {}

impl DcmiD9 for PC12<Alternate<AF13>> {}
impl DcmiD9 for PI2<Alternate<AF13>> {}

impl DcmiD10 for PB5<Alternate<AF13>> {}
impl DcmiD10 for PI3<Alternate<AF13>> {}

impl DcmiD11 for PD2<Alternate<AF13>> {}
impl DcmiD11 for PH15<Alternate<AF13>> {}

impl DcmiD12 for PF11<Alternate<AF13>> {}

impl DcmiD13 for PG15<Alternate<AF13>> {}
impl DcmiD13 for PI0<Alternate<AF13>> {}

pub trait DcmiSignalPins {}

pub struct SignalPins<H, V, PXL> {
    _hsync: H,
    _vsync: V,
    _pxl: PXL,
}

impl<H, V, PXL> DcmiSignalPins for SignalPins<H, V, PXL>
where
    H: DcmiHSYNC,
    V: DcmiVSYNC,
    PXL: DcmiPIXCLK,
{
}
impl<H, V, PXL> SignalPins<H, V, PXL>
where
    H: DcmiHSYNC,
    V: DcmiVSYNC,
    PXL: DcmiPIXCLK,
{
    pub fn new(hsync_pin: H, vsync_pin: V, pixel_clock_pin: PXL) -> Self {
        Self {
            _hsync: hsync_pin,
            _vsync: vsync_pin,
            _pxl: pixel_clock_pin,
        }
    }
}

pub trait DcmiDataPins {}

pub struct DataPins<D0, D1, D2, D3, D4, D5, D6, D7> {
    _data_pin0: D0,
    _data_pin1: D1,
    _data_pin2: D2,
    _data_pin3: D3,
    _data_pin4: D4,
    _data_pin5: D5,
    _data_pin6: D6,
    _data_pin7: D7,
}
impl<D0, D1, D2, D3, D4, D5, D6, D7> DcmiDataPins for DataPins<D0, D1, D2, D3, D4, D5, D6, D7>
where
    D0: DcmiD0,
    D1: DcmiD1,
    D2: DcmiD2,
    D3: DcmiD3,
    D4: DcmiD4,
    D5: DcmiD5,
    D6: DcmiD6,
    D7: DcmiD7,
{
}

impl<D0, D1, D2, D3, D4, D5, D6, D7> DataPins<D0, D1, D2, D3, D4, D5, D6, D7>
where
    D0: DcmiD0,
    D1: DcmiD1,
    D2: DcmiD2,
    D3: DcmiD3,
    D4: DcmiD4,
    D5: DcmiD5,
    D6: DcmiD6,
    D7: DcmiD7,
{
    pub fn new_for_8_bits(
        data_pin0: D0,
        data_pin1: D1,
        data_pin2: D2,
        data_pin3: D3,
        data_pin4: D4,
        data_pin5: D5,
        data_pin6: D6,
        data_pin7: D7,
    ) -> Self {
        Self {
            _data_pin0: data_pin0,
            _data_pin1: data_pin1,
            _data_pin2: data_pin2,
            _data_pin3: data_pin3,
            _data_pin4: data_pin4,
            _data_pin5: data_pin5,
            _data_pin6: data_pin6,
            _data_pin7: data_pin7,
        }
    }
}
