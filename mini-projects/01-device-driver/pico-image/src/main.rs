//! Pico Game Controller with USB Interface

#![no_std]
#![no_main]

// The macro for our start-up function
use rp_pico::{entry, hal::{clocks::ClocksManager, fugit::MicrosDurationU32, gpio, timer::Alarm}, pac::{Interrupt, USBCTRL_DPRAM, USBCTRL_REGS}};

// Ensure we halt the program on panic (if we don't mention this crate it won't
// be linked)
use panic_halt as _;

// A shorter alias for the Peripheral Access Crate, which provides low-level
// register access
use rp_pico::hal::pac;
use rp_pico::hal::pac::interrupt;

// A shorter alias for the Hardware Abstraction Layer, which provides
// higher-level drivers.
use embedded_hal::digital::{InputPin, OutputPin, PinState, };
use rp_pico::hal;
use rp_pico::hal::gpio::Interrupt::EdgeLow;
use rp_pico::hal::Sio;
use rp_pico::Pins;

// USB Device support
use usb_device::{class_prelude::*, prelude::*};
use usbd_serial::SerialPort;

// Misc
use core::{borrow::BorrowMut, cell::Cell, fmt::Write, iter::Once, mem::transmute, ops::{Deref, DerefMut}};
use heapless::String;
use core::cell::RefCell;
use critical_section::Mutex;


// Enum for Device State Machine
#[repr(i32)]
#[derive(Clone, Copy, Debug)]
enum DeviceState {
    PendingInit = 0,
    PendingStart = 1,
    Running = 2,
    Complete = 3,
}

// Heartbeat LED Delay
const LED_TOGGLE_DELAY: u64 = 500_000;
const SERIAL_TX_PERIOD: u64 = 100_000;


// Type our button pins
type LeftButtonType = gpio::Pin<gpio::bank0::Gpio13, gpio::FunctionSioInput, gpio::PullDown>;
type TopButtonType = gpio::Pin<gpio::bank0::Gpio15, gpio::FunctionSioInput, gpio::PullDown>;
type BottomButtonType = gpio::Pin<gpio::bank0::Gpio14, gpio::FunctionSioInput, gpio::PullDown>;
type RightButtonType = gpio::Pin<gpio::bank0::Gpio12, gpio::FunctionSioInput, gpio::PullDown>;
type GreenLedType = gpio::Pin<gpio::bank0::Gpio16, gpio::FunctionSioOutput, gpio::PullDown>;
type YellowLedType = gpio::Pin<gpio::bank0::Gpio17, gpio::FunctionSioOutput, gpio::PullDown>;
type RedLedType = gpio::Pin<gpio::bank0::Gpio18, gpio::FunctionSioOutput, gpio::PullDown>;
struct ButtonPins {
    left: LeftButtonType,
    top: TopButtonType,
    bottom: BottomButtonType,
    right: RightButtonType,
    button_states: (PinState, PinState, PinState),
    led_green: GreenLedType,
    led_yellow: YellowLedType,
    led_red: RedLedType,
}
static GLOBAL_GPIO: Mutex<RefCell<Option<ButtonPins>>> = Mutex::new(RefCell::new(None));

// Global state for the position of the player
static GLOBAL_PLAYER_POSITION: Mutex<RefCell<(i32, i32)>> = Mutex::new(RefCell::new((0, 0)));

// Global state for the USB device
struct UsbSerialContainer<'a, B: usb_device::bus::UsbBus> {
    serial: SerialPort<'a, B>,
    usb_dev: UsbDevice<'a, B>,
}

static mut USB_BUS_ALLOCATOR: Option<UsbBusAllocator<hal::usb::UsbBus>> = None;
static GLOBAL_USB_DEVICE: Mutex<RefCell<Option<UsbSerialContainer<'_, hal::usb::UsbBus>>>> = Mutex::new(RefCell::new(None));
static GLOBAL_DEVICE_STATE: Mutex<RefCell<DeviceState>> = Mutex::new(RefCell::new(DeviceState::PendingInit));
static GLOBAL_TIMER: Mutex<RefCell<Option<hal::Timer>>> = Mutex::new(RefCell::new(None));


/// Entry point to our bare-metal application.
///
/// The `#[entry]` macro ensures the Cortex-M start-up code calls this function
/// as soon as all global variables are initialised.
#[entry]
fn main() -> ! {
    // Grab our singleton objects
    let mut pac = pac::Peripherals::take().expect("Peripherals should be accessible on entry");

    // Initialize LED Pins
    let sio = Sio::new(pac.SIO);
    let pins = Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    // LEDs
    let mut heartbeat_led = pins.led.into_push_pull_output();
    let mut red_led: RedLedType = pins.gpio18.into_push_pull_output();
    let mut yellow_led: YellowLedType = pins.gpio17.into_push_pull_output();
    let mut green_led: GreenLedType = pins.gpio16.into_push_pull_output();

    // Define buttons
    let mut left_button: LeftButtonType = pins.gpio13.into_pull_down_input();
    let mut top_button: TopButtonType = pins.gpio15.into_pull_down_input();
    let mut bottom_button: BottomButtonType = pins.gpio14.into_pull_down_input();
    let mut right_button: RightButtonType = pins.gpio12.into_pull_down_input();

    // Enable interrupts on the buttons
    left_button.set_interrupt_enabled(EdgeLow, true);
    top_button.set_interrupt_enabled(EdgeLow, true);
    bottom_button.set_interrupt_enabled(EdgeLow, true);
    right_button.set_interrupt_enabled(EdgeLow, true);

    // Give away ownership fo the buttons
    let button_pins = ButtonPins {
        left: left_button, 
        top: top_button, 
        bottom: bottom_button, 
        right: right_button, 
        button_states: (PinState::Low, PinState::Low, PinState::Low), 
        led_green: green_led, 
        led_yellow: yellow_led, 
        led_red: red_led 
    };
    critical_section::with(|cs| {
        GLOBAL_GPIO.borrow(cs).replace(Some(button_pins));
    });
    
    // Set up the watchdog driver - needed by the clock setup code
    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);


    // Configure the clocks
    //
    // The default is to generate a 125 MHz system clock
    let clocks = hal::clocks::init_clocks_and_plls(
        rp_pico::XOSC_CRYSTAL_FREQ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .expect("Clocks should never fail to initialize");

    let timer = hal::Timer::new(pac.TIMER, &mut pac.RESETS, &clocks);
    critical_section::with(|cs| {
        GLOBAL_TIMER.borrow(cs).replace(Some(timer));
    });

    // Set up the USB and serial driver
    let usb_bus: UsbBusAllocator<hal::usb::UsbBus> = UsbBusAllocator::new(hal::usb::UsbBus::new(
        pac.USBCTRL_REGS,
        pac.USBCTRL_DPRAM,
        clocks.usb_clock,
        true,
        &mut pac.RESETS,
    ));
    unsafe {
        USB_BUS_ALLOCATOR = Some(usb_bus);
    }
    let bus_ref = unsafe { USB_BUS_ALLOCATOR.as_ref().unwrap() };
    let serial = SerialPort::new(&bus_ref);
    let usb_dev = UsbDeviceBuilder::new(&bus_ref, UsbVidPid(0x16c0, 0x27dd))
    .strings(&[StringDescriptors::default()
        .manufacturer("Rustbox Studio")
        .product("Rusty Ports")
        .serial_number("RustboxController0")])
    .unwrap()
    .device_class(2) 
    .build();
    let usb_container = UsbSerialContainer { serial: serial, usb_dev: usb_dev };
    critical_section::with(|cs| {
        GLOBAL_USB_DEVICE.borrow(cs).replace(Some(usb_container));
    });

    // enable usb interrupt
    unsafe {
        pac::NVIC::unmask(hal::pac::Interrupt::USBCTRL_IRQ);
        pac::NVIC::unmask(hal::pac::Interrupt::IO_IRQ_BANK0);
    }
    loop {
        set_leds();
        cortex_m::asm::wfi();
    }
}


/// Sets the leds to the current state
fn set_leds () {
    critical_section::with(|cs| {
        let gpios = &mut *GLOBAL_GPIO.borrow(cs).borrow_mut();
        let gpios = gpios.as_mut().unwrap();
        let _ = gpios.led_red.set_state(gpios.button_states.0);
        let _ = gpios.led_yellow.set_state(gpios.button_states.1);
        let _ = gpios.led_green.set_state(gpios.button_states.2);
    });
}

fn debug_looper() {
    critical_section::with(|cs| {
        let usb_container = &mut *GLOBAL_USB_DEVICE.borrow(cs).borrow_mut();
        let state = &mut *GLOBAL_DEVICE_STATE.borrow(cs).borrow_mut();
        let player_position = &mut *GLOBAL_PLAYER_POSITION.borrow(cs).borrow_mut();
        let mut debug_str: String<100> = String::new();
        writeln!(debug_str, "State: {:?}, Position: {:?}\n", state, player_position).unwrap();
        let _ = usb_container.as_mut().unwrap().serial.write(debug_str.as_bytes());
        let _ = usb_container.as_mut().unwrap().serial.flush();
    }); 
}

/// Interrupt handler for button presses
#[allow(static_mut_refs)]
#[interrupt]
fn IO_IRQ_BANK0() {
    static mut DEBOUNCE_LEFT: u64 = 0;
    static mut DEBOUNCE_TOP: u64 = 0;
    static mut DEBOUNCE_RIGHT: u64 = 0;
    static mut DEBOUNCE_BOTTOM: u64 = 0;

    critical_section::with(|cs| {
        let player_position = &mut *GLOBAL_PLAYER_POSITION.borrow(cs).borrow_mut();
        let usb_container = &mut *GLOBAL_USB_DEVICE.borrow(cs).borrow_mut();
        let state = &mut *GLOBAL_DEVICE_STATE.borrow(cs).borrow_mut();
        let gpios = &mut *GLOBAL_GPIO.borrow(cs).borrow_mut();
        let gpios = gpios.as_mut().unwrap();
        let timer = &mut *GLOBAL_TIMER.borrow(cs).borrow_mut();
        let timer = timer.as_mut().unwrap();
        
        let current_time = timer.get_counter().ticks();
        let debounce_time = 50_000;

        // Calculate position difference
        let mut pos_diff = (0, 0);
        if gpios.left.interrupt_status(EdgeLow) {
            if current_time - *DEBOUNCE_LEFT > debounce_time {
                pos_diff.0 += 1;
                pos_diff.1 += 1;
                *DEBOUNCE_LEFT = current_time;
            }
            gpios.left.clear_interrupt(EdgeLow);
        }
        if gpios.top.interrupt_status(EdgeLow) {
            if current_time - *DEBOUNCE_TOP > debounce_time {
                pos_diff.0 -= 1;
                pos_diff.1 -= 1;
                *DEBOUNCE_TOP = current_time;
            }
            gpios.top.clear_interrupt(EdgeLow);
        }
        if gpios.right.interrupt_status(EdgeLow) {
            if current_time - *DEBOUNCE_RIGHT > debounce_time {
                pos_diff.0 += 1;
                pos_diff.1 -= 1;
                *DEBOUNCE_RIGHT = current_time;
            }
            gpios.right.clear_interrupt(EdgeLow);
        }
        if gpios.bottom.interrupt_status(EdgeLow) {
            if current_time - *DEBOUNCE_BOTTOM > debounce_time {
                pos_diff.0 -= 1;
                pos_diff.1 += 1;
                *DEBOUNCE_BOTTOM = current_time;
            }
            gpios.bottom.clear_interrupt(EdgeLow);
        }

        // Update and send positions if DeviceState::Running
        match state {   
            DeviceState::Running => {
                player_position.0 += pos_diff.0;
                player_position.1 += pos_diff.1;
                let mut message: String<20> = String::new();
                writeln!(message, "{:?},{:?}", player_position.0, player_position.1).unwrap();
                let _ = usb_container.as_mut().unwrap().serial.write(message.as_bytes());
                let _ = usb_container.as_mut().unwrap().serial.flush();
            }
            _ => {}
        }
    });

}


/// Interrupt handler for USB serial
#[interrupt]
fn USBCTRL_IRQ() {
    let mut buf = [0u8; 64];
    let message: Option<&str> = critical_section::with(|cs| {
        if let Some(usb_container) = GLOBAL_USB_DEVICE.borrow(cs).borrow_mut().as_mut() {
            if usb_container.usb_dev.poll(&mut [&mut usb_container.serial]) {
                return match usb_container.serial.read(&mut buf) {
                    Ok(0) => None,
                    Err(_) => None,
                    Ok(count) => {
                        return match core::str::from_utf8(&buf[..count]) {
                            Ok(s) => Some(s.trim()),
                            Err(_) => None
                        };
                    }
                }
            }
        }
        None
    });
    if let Some(message) = message {
        process_state_serial_message(message);
    }
}


fn process_state_serial_message(message: &str) {
    critical_section::with(|cs| {
        let player_position = &mut *GLOBAL_PLAYER_POSITION.borrow(cs).borrow_mut();
        let device_state = &mut *GLOBAL_DEVICE_STATE.borrow(cs).borrow_mut();
        let gpios: &mut Option<ButtonPins> = &mut *GLOBAL_GPIO.borrow(cs).borrow_mut();
        match device_state {
            DeviceState::PendingInit => {
                if message.contains("init controller") {
                    *device_state = DeviceState::PendingStart;
                }
                else if message.contains("reset") {
                    *device_state = DeviceState::PendingInit;
                    *player_position = (0, 0);
                    gpios.as_mut().unwrap().button_states = (PinState::Low, PinState::Low, PinState::Low);
                }
            }
            DeviceState::PendingStart => {
                if message.contains("set ready led") {
                    gpios.as_mut().unwrap().button_states = (PinState::High, PinState::Low, PinState::Low);
                } else if message.contains("set set led") {
                    gpios.as_mut().unwrap().button_states = (PinState::Low, PinState::High, PinState::Low);
                } else if message.contains("set go led") {
                    gpios.as_mut().unwrap().button_states = (PinState::Low, PinState::Low, PinState::High);
                } else if message.contains("set all leds") {
                    gpios.as_mut().unwrap().button_states = (PinState::High, PinState::High, PinState::High);
                } else if message.contains("clear all leds") {
                    gpios.as_mut().unwrap().button_states = (PinState::Low, PinState::Low, PinState::Low);
                }
                if message.contains("start controller") {
                    *device_state = DeviceState::Running;
                }
                else if message.contains("reset") {
                    *device_state = DeviceState::PendingInit;
                    *player_position = (0, 0);
                    gpios.as_mut().unwrap().button_states = (PinState::Low, PinState::Low, PinState::Low);
                }
            }
            DeviceState::Running => {
                if message.contains("stop controller") {
                    *device_state = DeviceState::Complete;
                }
                else if message.contains("reset") {
                    *device_state = DeviceState::PendingInit;
                    *player_position = (0, 0);
                    gpios.as_mut().unwrap().button_states = (PinState::Low, PinState::Low, PinState::Low);
                }
            }
            DeviceState::Complete => {
                if message.contains("reset") {
                    *player_position = (0, 0);
                    gpios.as_mut().unwrap().button_states = (PinState::Low, PinState::Low, PinState::Low);
                    *device_state = DeviceState::PendingInit;
                }
                else if message.contains("restart") {
                    *player_position = (0, 0);
                    gpios.as_mut().unwrap().button_states = (PinState::Low, PinState::Low, PinState::Low);
                    *device_state = DeviceState::PendingStart;
                }
                else if message.contains("start controller") {
                    *player_position = (0, 0);
                    gpios.as_mut().unwrap().button_states = (PinState::Low, PinState::Low, PinState::Low);
                    *device_state = DeviceState::Running;
                }
            }
        }
    });
}