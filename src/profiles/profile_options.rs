
#[derive(Debug, Clone)]
pub struct ProfileOptions {
    // Should chat be shown?
    pub show_chat: bool,
    // Should debugging be enabled?
    pub enable_debug: bool,
    // Should rotation require combat?
    pub require_combat: bool,
    // Rotation Designation?
    pub rotation_designation: bool,
    // Refresh rate?
    pub refresh_rate: f32,
    // Interrupt Delay?
    pub interrupt_delay: f32,
    // Enable interrupts?
    pub enable_interrupt: bool,
    //Interrupt-mode-change?
    pub interrupt_mode_change: bool,
    // Use smart key mode?
    pub smart_key_mode: bool,
    // Interrupt all spells?
    pub interrupt_all_spells: bool,
    // Change rotation?
    pub change_rotation: bool,
}

impl ProfileOptions {
    /// Default constructor for Profile Options
    pub fn default() -> Self {
        Self {
            show_chat: false,
            enable_debug: false,
            require_combat: false,
            rotation_designation: false,
            refresh_rate: 100.0,
            interrupt_delay: 0.0,
            enable_interrupt: false,
            interrupt_mode_change: false,
            smart_key_mode: false,
            interrupt_all_spells: false,
            change_rotation: false,
        }
    }
    /// Constructor for every value within struct
    pub fn new(
               show_chat: bool,
               enable_debug: bool,
               require_combat: bool,
               rotation_designation: bool,
               refresh_rate: f32,
               interrupt_delay: f32,
               enable_interrupt: bool,
               interrupt_mode_change: bool,
               smart_key_mode: bool,
               interrupt_all_spells: bool,
               change_rotation: bool
    ) -> Self {
        Self {
            show_chat,
            enable_debug,
            require_combat,
            rotation_designation,
            refresh_rate,
            interrupt_delay,
            enable_interrupt,
            interrupt_mode_change,
            smart_key_mode,
            interrupt_all_spells,
            change_rotation,
        }
    }
}