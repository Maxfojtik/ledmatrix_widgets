const ON_FULL: u8 = 255;
const ON_DIM: u8 = 64;
const OFF: u8 = 0;

pub struct Shape {
    pub x: usize,
    pub y: usize,
}

/// A standard set of instructions for widgets that can be updated from the system
pub trait UpdatableWidget {
    fn get_matrix(&mut self) -> Vec<u8>;
    fn get_shape(&mut self) -> Shape;
}

// ================ Frames ================
/// Battery frame with empty interior (9x4 shape)
const BAT_FRAME: &'static [u8] = [
    ON_FULL, ON_FULL, ON_FULL, ON_FULL, ON_FULL, ON_FULL, ON_FULL, ON_FULL, OFF, ON_FULL, OFF, OFF, OFF,
    OFF, OFF, OFF, ON_FULL, ON_FULL, ON_FULL, OFF, OFF, OFF, OFF, OFF, OFF, ON_FULL, ON_FULL, ON_FULL,
    ON_FULL, ON_FULL, ON_FULL, ON_FULL, ON_FULL, ON_FULL, ON_FULL, OFF,
]
.as_slice();

// ================ Widgets ================
/// -------- Battery Widget --------
/// Create a widget that displays the battery remaining in the laptop
pub struct BatteryWidget {
    bat_level_pct: f32
}

impl BatteryWidget {
    pub fn new() -> BatteryWidget {
        BatteryWidget {
            bat_level_pct: 0.0,
        }
    }
}

impl UpdatableWidget for BatteryWidget {

    fn get_matrix(&mut self) -> Vec<u8> {
        // Update the battery percentage
        self.bat_level_pct = battery::Manager::new()
            .unwrap()
            .batteries()
            .unwrap()
            .enumerate()
            .next()
            .unwrap()
            .1
            .unwrap()
            .state_of_charge()
            .get::<battery::units::ratio::percent>();

        // Create the matrix
        let mut out: Vec<u8> = Vec::new();
        out.extend_from_slice(BAT_FRAME);

        let num_illum = (self.bat_level_pct * 6.0 / 100.0).round();

        for i in 1..7 {
            if i <= num_illum as usize {
                out[(self.get_shape().x) + i]= ON_DIM;
                out[(self.get_shape().x * 2) + i]= ON_DIM;
            }
        }
        
        out
    }

    fn get_shape(&mut self) -> Shape {
        return Shape { x: 9, y: 4 };
    }
}

// -------- All Cores CPU Usage Widget --------
/// Create a widget that displays the usage of all CPU cores, one per row.
pub struct AllCPUsWidget {
    cpu_usages: Vec<u8>,
    sys: sysinfo::System
}

impl AllCPUsWidget {
    pub fn new() -> AllCPUsWidget {
        let mut newsys = sysinfo::System::new();
        newsys.refresh_cpu();

        AllCPUsWidget {
            cpu_usages: vec![0; newsys.cpus().len()],
            sys: newsys
        }
    }
}

impl UpdatableWidget for AllCPUsWidget {

    /// Refresh the CPU usage and redraw the matrix
    fn get_matrix(&mut self) -> Vec<u8> {
        // Refresh the cpu usage
        self.sys.refresh_cpu();

        for i in 0..self.sys.cpus().len() {
            self.cpu_usages[i] = self.sys.cpus()[i].cpu_usage().round() as u8;
        }

        // Create the matrix
        let width = self.get_shape().x;
        let mut out = vec![0;self.cpu_usages.len() * width];

        for y in 0..self.cpu_usages.len() {
            for x in 0..self.get_shape().x {
                if x <= (self.cpu_usages[y] as f32 * width as f32 / 100.0) as usize {
                    out[x + (y * width)] = ON_FULL;
                }
            }         
        }

        out
    }

    fn get_shape(&mut self) -> Shape {
        return Shape {x: 9, y:self.cpu_usages.len()};
    }
}