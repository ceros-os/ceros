use crate::hardware::devices::{SmartPort, Device, SmartDevice, Encoder};


/// Enum of what faults a motor is experiencing
#[derive(Copy, Clone)]
pub enum MotorFaults {
    /// No faults
    None = 0x0,
    /// The motor is over temperature
    OverTemp = 0x01,
    /// The h-bridge is faulting
    HBridgeFault = 0x02,
    /// The motor is over current
    OverCurrent = 0x04,
    /// The h-bridge is over current
    HBridgeOverCurrent = 0x08,
}

/// Flags a motor can have
pub enum MotorFlags {
    /// No flags are set
    None = 0x0,
    /// Unable to communicate with the motor
    NoCommunication = 0x01,
    /// The motor is stopped
    Stopped = 0x02,
    /// The motor has a position of zero
    ZeroPosition = 0x04,
}

/// The gearbox a motor uses
pub enum MotorGearbox {
    Red,
    Green,
    Blue
}

/// The units to use in an encoder tick
#[derive(Default, Copy, Clone)]
pub enum MotorEncoderUnits {
    /// The units are in degrees
    #[default] Degrees,
    /// The units are in rotations
    Rotations,
    /// The units are in ticks
    Ticks,
}

/// The break mode of a motor
#[derive(Default, Copy, Clone)]
pub enum MotorBrakeMode {
    /// The motor will coast to a stop
    #[default] Coast,
    /// The motor will brake to a stop
    Brake,
    /// The motor will attempt to hold its current position
    /// reacting to outside forces
    Hold,
}

/// A basic smart motor
#[derive(Clone)]
pub struct SmartMotor {
    /// The smart port that this motor is connected to
    port: u32,
}

impl SmartMotor {
    

    /// Sets the voltage of the motor, clampung it to the range -127 to 127
    pub fn move_voltage(&mut self, voltage: i32) {

        // Lock the device
        let _mtx = self.lock();

        // Clamp the voltage to the range -127 to 127
        let voltage = voltage.min(127).max(-127);
        // Set the voltage
        unsafe {
            vexv5rt::vexDeviceMotorVoltageSet(self.get_vex_device(0), voltage);
        }
    }

    /// Moves the motor to a position at the given speed
    pub fn move_absolute(&mut self, position: f64, speed: i32) {

        // Lock the device
        let _mtx = self.lock();

        // Move the motor
        unsafe {
            vexv5rt::vexDeviceMotorAbsoluteTargetSet(self.get_vex_device(0), position, speed);
        }
    }

    /// Moves the motor to a position relative to its current position
    /// at the given speed
    pub fn move_relative(&mut self, position: f64, speed: i32) {

        // Lock the device
        let _mtx = self.lock();

        // Move the motor
        unsafe {
            vexv5rt::vexDeviceMotorRelativeTargetSet(self.get_vex_device(0), position, speed);
        }
    }

    /// Sets the velocity of the motor
    pub fn move_velocity(&mut self, velocity: i32) {

        // Lock the device
        let _mtx = self.lock();

        // Set the velocity
        unsafe {
            vexv5rt::vexDeviceMotorVelocitySet(self.get_vex_device(0), velocity);
        }
    }

    /// Stops the motor
    pub fn stop(&mut self) {

        // Lock the device
        let _mtx = self.lock();

        // Stop the motor
        unsafe {
            vexv5rt::vexDeviceMotorVelocitySet(self.get_vex_device(0), 0);
        }
    }


    /// Updates the target velocity for the function move_relative and move_absolute
    pub fn set_target_velocity(&mut self, velocity: i32) {
            
        // Lock the device
        let _mtx = self.lock();

        // Set the target velocity
        unsafe {
            vexv5rt::vexDeviceMotorVelocityUpdate(self.get_vex_device(0), velocity)
        }
    }

    /// Gets the target velocity
    pub fn get_target_velocity(&self) -> i32 {

        // Lock the device
        let _mtx = self.lock();

        // Get the target velocity
        unsafe {
            vexv5rt::vexDeviceMotorVelocityGet(self.get_vex_device(0))
        }
    }

    /// Gets the target position
    pub fn get_target_position(&self) -> f64 {

        // Lock the device
        let _mtx = self.lock();

        // Get the target position
        unsafe {
            vexv5rt::vexDeviceMotorTargetGet(self.get_vex_device(0))
        }
    }
    

    /**************************************************************************
     * Telemetry functions                                                    *
     **************************************************************************/
    
    /// Gets the motor's position
    pub fn get_position(&self) -> f64 {

        // Lock the device
        let _mtx = self.lock();

        // Get the position
        unsafe {
            vexv5rt::vexDeviceMotorPositionGet(self.get_vex_device(0))
        }
    }

    /// Gets the motor's raw position at a given timestamp
    pub fn get_raw_position(&self, timestamp: *mut u32) -> i32 {

        // Lock the device
        let _mtx = self.lock();

        // Get the raw position
        unsafe {
            vexv5rt::vexDeviceMotorPositionRawGet(self.get_vex_device(0), timestamp)
        }
    }

    /// Get the velocity of the motor
    pub fn get_velocity(&self) -> i32 {

        // Lock the device
        let _mtx = self.lock();

        // Get the velocity
        unsafe {
            vexv5rt::vexDeviceMotorVelocityGet(self.get_vex_device(0))
        }
    }

    /// Get the torque generated by the motor in Newton meters
    pub fn get_torque(&self) -> f64 {

        // Lock the device
        let _mtx = self.lock();

        // Get the torque
        unsafe {
            vexv5rt::vexDeviceMotorTorqueGet(self.get_vex_device(0))
        }
    }

    /// Get the direction the motor is spinning in
    /// 1 for forward, -1 for reverse
    pub fn get_direction(&self) -> i32 {

        // Lock the device
        let _mtx = self.lock();

        // Get the direction
        unsafe {
            vexv5rt::vexDeviceMotorDirectionGet(self.get_vex_device(0))
        }
    }

    /// Get how much current the motor is drawing in mA
    pub fn get_current(&self) -> i32 {

        // Lock the device
        let _mtx = self.lock();

        // Get the current
        unsafe {
            vexv5rt::vexDeviceMotorCurrentGet(self.get_vex_device(0))
        }
    }

    /// Gets the power the motor is drawing in Watts
    pub fn get_power(&self) -> f64 {

        // Lock the device
        let _mtx = self.lock();

        // Get the power
        unsafe {
            vexv5rt::vexDeviceMotorPowerGet(self.get_vex_device(0))
        }
    }

    /// Get the voltage the motor is drawing in milli Volts
    pub fn get_voltage(&self) -> i32 {

        // Lock the device
        let _mtx = self.lock();

        // Get the voltage
        unsafe {
            vexv5rt::vexDeviceMotorVoltageGet(self.get_vex_device(0))
        }
    }
    
    /// Gets the efficiency of the motor in percent.
    /// 100% is the motor is moving but drawing no power, 0% is the motor is drawing
    /// power but not moving.
    pub fn get_efficiency(&self) -> f64 {

        // Lock the device
        let _mtx = self.lock();

        // Get the efficiency
        unsafe {
            vexv5rt::vexDeviceMotorEfficiencyGet(self.get_vex_device(0))
        }
    }

    /// Returns a bitmask of the faults that have occured on the motor
    pub fn get_faults(&self) -> u32 {

        // Lock the device
        let _mtx = self.lock();

        // Get the faults
        unsafe {
            vexv5rt::vexDeviceMotorFaultsGet(self.get_vex_device(0))
        }
    }

    /// Returns a bitmask of the flags that are set on the motor
    pub fn get_flags(&self) -> u32 {

        // Lock the device
        let _mtx = self.lock();

        // Get the flags
        unsafe {
            vexv5rt::vexDeviceMotorFlagsGet(self.get_vex_device(0))
        }
    }

    /// Gets the motor's temperature in degrees Celsius at a resolution of 5 degrees
    pub fn get_temperature(&self) -> f64 {

        // Lock the device
        let _mtx = self.lock();

        // Get the temperature
        unsafe {
            vexv5rt::vexDeviceMotorTemperatureGet(self.get_vex_device(0))
        }
    }
    
    /// Returns true if the motor is at absolute position zero
    pub fn is_at_zero(&self) -> bool {

        // Lock the device
        let _mtx = self.lock();

        // Get the motor flags
        let flags = self.get_flags();

        // Check if the position zero flag is set
        (flags & MotorFlags::ZeroPosition as u32) != 0
    }

    /// Returns true if the motor is stopped
    pub fn is_stopped(&self) -> bool {

        // Lock the device
        let _mtx = self.lock();

        // Get the motor flags
        let flags = self.get_flags();

        // Check if the motor is stopped
        (flags & MotorFlags::Stopped as u32) != 0
    }

    /// Returns true if the motor is over temperature
    pub fn is_over_temp(&self) -> bool {

        // Lock the device
        let _mtx = self.lock();

        // Get the motor faults
        let flags = self.get_faults();

        // Check if the motor is over temperature
        (flags & MotorFaults::OverTemp as u32) != 0
    }

    // Returns true if the motor is over current
    pub fn is_over_current(&self) -> bool {

        // Lock the device
        let _mtx = self.lock();

        // Get the motor faults
        let flags = self.get_faults();

        // Check if the motor is over current
        (flags & MotorFaults::OverCurrent as u32) != 0
    }

    /************************************
     * Configuration
     ************************************/
    
    /// Sets the motor's encoder units
    pub fn set_encoder_units(&mut self, units: MotorEncoderUnits) {

        // Lock the device
        let _mtx = self.lock();

        unsafe {
            vexv5rt::vexDeviceMotorEncoderUnitsSet(self.get_vex_device(0), units as u32);
        }
    }

    /// Gets the motor's encoder units
    pub fn get_encoder_units(&self) -> MotorEncoderUnits {

        // Lock the device
        let _mtx = self.lock();

        unsafe {
            match vexv5rt::vexDeviceMotorEncoderUnitsGet(self.get_vex_device(0)) {
                0 => MotorEncoderUnits::Degrees,
                1 => MotorEncoderUnits::Rotations,
                _ => MotorEncoderUnits::Ticks,
            }
        }
    }

    /// Sets the motor's brake mode
    pub fn set_brake_mode(&mut self, mode: MotorBrakeMode) {

        // Lock the device
        let _mtx = self.lock();


        unsafe {
            vexv5rt::vexDeviceMotorBrakeModeSet(self.get_vex_device(0), mode as u32);
        }
    }

    /// Gets the motor's brake mode
    pub fn get_brake_mode(&self) -> MotorBrakeMode {

        // Lock the device
        let _mtx = self.lock();

        unsafe {
            match vexv5rt::vexDeviceMotorBrakeModeGet(self.get_vex_device(0)) {
                0 => MotorBrakeMode::Coast,
                1 => MotorBrakeMode::Brake,
                _ => MotorBrakeMode::Hold,
            }
        }
    }

    /// Gets the motor's current limit in mA
    pub fn get_current_limit(&self) -> i32 {

        // Lock the device
        let _mtx = self.lock();

        // Get the current limit
        unsafe {
            vexv5rt::vexDeviceMotorCurrentLimitGet(self.get_vex_device(0))
        }
    }

    /// Sets the motor's current limit in mA
    pub fn set_current_limit(&mut self, limit: i32) {

        // Lock the device
        let _mtx = self.lock();

        unsafe {
            vexv5rt::vexDeviceMotorCurrentLimitSet(self.get_vex_device(0), limit);
        }
    }
    
    /// Sets the motor's voltage limit in V
    pub fn set_voltage_limit(&mut self, limit: i32) {

        // Lock the device
        let _mtx = self.lock();

        unsafe {
            vexv5rt::vexDeviceMotorVoltageLimitSet(self.get_vex_device(0), limit);
        }
    }

    /// Gets the motor's voltage limit in V
    pub fn get_voltage_limit(&self) -> i32 {

        // Lock the device
        let _mtx = self.lock();

        unsafe {
            vexv5rt::vexDeviceMotorVoltageLimitGet(self.get_vex_device(0))
        }
    }

    /// Gets the motor's gearbox
    pub fn get_gearbox(&self) -> MotorGearbox {

        // Lock the device
        let _mtx = self.lock();

        unsafe {
            match vexv5rt::vexDeviceMotorGearingGet(self.get_vex_device(0)) {
                0 => MotorGearbox::Red,
                1 => MotorGearbox::Green,
                _ => MotorGearbox::Blue,
            }
        }
    }

    /// Sets the motor's gearbox
    pub fn set_gearbox(&mut self, gearbox: MotorGearbox) {

        // Lock the device
        let _mtx = self.lock();

        unsafe {
            vexv5rt::vexDeviceMotorGearingSet(self.get_vex_device(0), gearbox as u32);
        }
    }

    /// Returns true if the motor is reversed
    pub fn is_reversed(&self) -> bool {

        // Lock the device
        let _mtx = self.lock();

        unsafe {
            vexv5rt::vexDeviceMotorReverseFlagGet(self.get_vex_device(0))
        }
    }

    /// Sets the motor's reversed flag
    pub fn set_reversed(&mut self, reversed: bool) {

        // Lock the device
        let _mtx = self.lock();

        unsafe {
            vexv5rt::vexDeviceMotorReverseFlagSet(self.get_vex_device(0), reversed);
        }
    }

}


impl Device for SmartMotor {
    fn init(&mut self) {
        // Set the encoder ticks to default
        self.set_encoder_units(MotorEncoderUnits::default());

        // Set the break mode to default
        self.set_brake_mode(MotorBrakeMode::default());

    }

    fn calibrate(&mut self) {
        // Reset the encoder
        self.reset_encoder();
    }

    fn get_smart_ports(&self) -> alloc::vec::Vec<(u32, SmartPort)> {
        vec![(self.port, SmartPort::Motor)]
    }

    fn get_any(&self) -> &dyn core::any::Any {
        self
    }
}

impl SmartDevice for SmartMotor {
    fn new_smart(port: u32) -> Self {
        Self {
            port,
        }
    }

    fn get_smart_port(&self) -> u32 {
        self.port
    }

    fn get_smart_port_type(&self) -> SmartPort {
        SmartPort::Motor
    }
}

impl Encoder for SmartMotor {
    fn get_ticks(&self) -> f64 {

        // Lock the device
        let _mtx = self.lock();

        unsafe {
            vexv5rt::vexDeviceMotorPositionGet(self.get_vex_device(0))
        }
    }

    fn get_rate(&self) -> f64 {

        // Lock the device
        let _mtx = self.lock();

        unsafe {
            <f64>::from(vexv5rt::vexDeviceMotorVelocityGet(self.get_vex_device(0))) * 6.0f64 // Converting from rpm to degrees/sec
        } 
    }

    fn reset_encoder(&mut self) {

        // Lock the device
        let _mtx = self.lock();

        unsafe {
            vexv5rt::vexDeviceMotorPositionReset(self.get_vex_device(0));
        }
    }

    fn set_zero_position(&mut self, position: f64) {
        
        // Lock the device
        let _mtx = self.lock();

        unsafe {
            vexv5rt::vexDeviceMotorPositionSet(self.get_vex_device(0), position);
        }
    }
}