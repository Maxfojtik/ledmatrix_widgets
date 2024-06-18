use chrono::{Local, Timelike};

const ON_FULL: u8 = 120;
const ON_DIM: u8 = 68;
const OFF: u8 = 0;

pub struct Shape {
    pub x: usize,
    pub y: usize,
}

/// A standard set of instructions for widgets that can be updated from the system
pub trait UpdatableWidget {
    fn update(&mut self);
    fn get_matrix(&self) -> Vec<u8>;
    fn get_shape(&self) -> Shape;
}

// ================ Frames ================
/// Battery frame with empty interior (9x4 shape)
const BAT_FRAME: &'static [u8] = [
    ON_FULL, ON_FULL, ON_FULL, ON_FULL, ON_FULL, ON_FULL, ON_FULL, ON_FULL, OFF, ON_FULL, OFF, OFF,
    OFF, OFF, OFF, OFF, ON_FULL, ON_FULL, ON_FULL, OFF, OFF, OFF, OFF, OFF, OFF, ON_FULL, ON_FULL,
    ON_FULL, ON_FULL, ON_FULL, ON_FULL, ON_FULL, ON_FULL, ON_FULL, ON_FULL, OFF,
]
.as_slice();

const DIGIT_0: &'static [u8] = [
    OFF, ON_FULL, OFF, ON_FULL, OFF, ON_FULL, ON_FULL, OFF, ON_FULL, ON_FULL, OFF, ON_FULL, OFF,
    ON_FULL, OFF,
]
.as_slice();

const DIGIT_1: &'static [u8] = [
    OFF, OFF, ON_FULL, OFF, ON_DIM, ON_FULL, OFF, OFF, ON_FULL, OFF, OFF, ON_FULL, OFF, OFF,
    ON_FULL,
]
.as_slice();

const DIGIT_2: &'static [u8] = [
    ON_FULL, ON_FULL, ON_FULL, OFF, OFF, ON_FULL, ON_FULL, ON_FULL, ON_FULL, ON_FULL, OFF, OFF,
    ON_FULL, ON_FULL, ON_FULL,
]
.as_slice();

const DIGIT_3: &'static [u8] = [
    ON_FULL, ON_FULL, ON_FULL, OFF, OFF, ON_FULL, ON_FULL, ON_FULL, OFF, OFF, OFF, ON_FULL,
    ON_FULL, ON_FULL, ON_FULL,
]
.as_slice();

const DIGIT_4: &'static [u8] = [
    ON_FULL, OFF, ON_FULL, ON_FULL, OFF, ON_FULL, ON_FULL, ON_FULL, ON_FULL, OFF, OFF, ON_FULL,
    OFF, OFF, ON_FULL,
]
.as_slice();

const DIGIT_5: &'static [u8] = [
    ON_FULL, ON_FULL, ON_FULL, ON_FULL, OFF, OFF, ON_FULL, ON_FULL, ON_FULL, OFF, OFF, ON_FULL,
    ON_FULL, ON_FULL, ON_FULL,
]
.as_slice();

const DIGIT_6: &'static [u8] = [
    OFF, ON_FULL, ON_DIM, ON_FULL, OFF, OFF, ON_FULL, ON_FULL, ON_FULL, ON_FULL, OFF, ON_FULL,
    ON_FULL, ON_FULL, ON_FULL,
]
.as_slice();

const DIGIT_7: &'static [u8] = [
    ON_FULL, ON_FULL, ON_FULL, ON_DIM, OFF, ON_FULL, OFF, OFF, ON_FULL, OFF, ON_FULL, OFF, OFF,
    ON_FULL, OFF,
]
.as_slice();

const DIGIT_8: &'static [u8] = [
    ON_FULL, ON_FULL, ON_FULL, ON_FULL, OFF, ON_FULL, ON_FULL, ON_FULL, ON_FULL, ON_FULL, OFF,
    ON_FULL, ON_FULL, ON_FULL, ON_FULL,
]
.as_slice();

const DIGIT_9: &'static [u8] = [
    ON_FULL, ON_FULL, ON_FULL, ON_FULL, OFF, ON_FULL, ON_FULL, ON_FULL, ON_FULL, OFF, OFF, ON_FULL,
    ON_DIM, ON_FULL, OFF,
]
.as_slice();



// ================ Widgets ================

/// -------- RAM Widget --------
/// Create a widget that displays the battery remaining in the laptop
pub struct RAMWidget {
    ram_perc: f32,
    sys: sysinfo::System,
}

impl RAMWidget {
    pub fn new() -> RAMWidget {
        println!("Initializing RAMWidget");
        let newsys = sysinfo::System::new();
        RAMWidget { ram_perc: 0.0, sys: newsys }
    }
}

impl UpdatableWidget for RAMWidget {
    fn update(&mut self) {
        // Refresh the cpu usage
        self.sys.refresh_memory();
        self.ram_perc = self.sys.used_memory() as f32 /self.sys.total_memory() as f32;
        // println!("{0} {1}",self.sys.used_memory(), self.sys.total_memory());
    }

    fn get_matrix(&self) -> Vec<u8> {
        // Create the matrix
        let mut out: Vec<u8> = vec![OFF; self.get_shape().x * self.get_shape().y];

        let width = self.get_shape().x;

        let bar_width_in_pixels = self.ram_perc * width as f32;
        for x in 0..width {
            let percent_on = bar_width_in_pixels - x as f32;// this is a float telling how much the pixel should be on
            if percent_on > 1.0 {//if we are more than 100% on
                out[x] = ON_FULL;
            }
            else if percent_on > 0.0//if we are fractionally on - the end of the bar
            {
                out[x] = (ON_FULL as f32 * percent_on) as u8;
            }
            out[x + width] = out[x];
        }

        out
    }

    fn get_shape(&self) -> Shape {
        return Shape { x: 9, y: 2 };
    }
}


/// -------- Battery Widget --------
/// Create a widget that displays the battery remaining in the laptop
pub struct BatteryWidget {
    bat_level_pct: f32,
}

impl BatteryWidget {
    pub fn new() -> BatteryWidget {
        println!("Initializing BatteryWidget");
        BatteryWidget { bat_level_pct: 0.0}
    }
}

impl UpdatableWidget for BatteryWidget {
    fn update(&mut self) {
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
    }

    fn get_matrix(&self) -> Vec<u8> {
        // Create the matrix
        let mut out: Vec<u8> = Vec::new();
        out.extend_from_slice(BAT_FRAME);

        let num_illum = (self.bat_level_pct * 6.0 / 100.0).round();

        for i in 1..7 {
            if i <= num_illum as usize {
                out[(self.get_shape().x) + i] = ON_DIM;
                out[(self.get_shape().x * 2) + i] = ON_DIM;
            }
        }

        out
    }

    fn get_shape(&self) -> Shape {
        return Shape { x: 9, y: 4 };
    }
}

const HISTORY_LENGTH: u8 = 120;
const HIGHLIGHT_WIDTH: f32 = 1.5;

/// -------- Battery Widget Ugly --------
/// Create a widget that displays the battery remaining in the laptop
pub struct BatteryWidgetUgly {
    bat_level_pct: f32,
    bat_level_pct_hist: Vec<f32>,
    state: battery::State,
    highlight_pos: f32,
    bar_width_in_pixels: f32,

}

impl BatteryWidgetUgly {
    pub fn new() -> BatteryWidgetUgly {
        println!("Initializing BatteryWidgetUgly");
        let bat_man = battery::Manager::new();
        let bat_level_pct = bat_man.as_ref()
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
        BatteryWidgetUgly { bat_level_pct, state: battery::State::Discharging, highlight_pos: 0.0, bar_width_in_pixels: 0.0, bat_level_pct_hist: vec![bat_level_pct] }
    }
}

impl UpdatableWidget for BatteryWidgetUgly {
    fn update(&mut self) {
        // Update the battery percentage
        let bat_man = battery::Manager::new();
        self.bat_level_pct = bat_man.as_ref()
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
        
        let mut highlight_speed = 0.0;
        self.bat_level_pct_hist.insert(0, self.bat_level_pct);
        let popping = self.bat_level_pct_hist.len() > HISTORY_LENGTH.into();
        if popping//we above full length, pop the last one to shorten
        {
            let last_val = self.bat_level_pct_hist.pop().unwrap();
            highlight_speed = (self.bat_level_pct - last_val) * 5.0;
        }

        let state = bat_man.as_ref()
            .unwrap()
            .batteries()
            .unwrap()
            .enumerate()
            .next()
            .unwrap()
            .1
            .unwrap()
            .state();
        self.state = state;
        // self.looper = self.looper.wrapping_add(1);
        let width = self.get_shape().x;
        self.bar_width_in_pixels = self.bat_level_pct / 100.0 * width as f32;


        self.highlight_pos += highlight_speed;
        // println!("{highlight_speed}");


        if popping
        {
            if self.highlight_pos > self.bar_width_in_pixels + HIGHLIGHT_WIDTH
            {
                self.highlight_pos = -HIGHLIGHT_WIDTH;
            }
            if self.highlight_pos < -HIGHLIGHT_WIDTH
            {
                self.highlight_pos = self.bar_width_in_pixels + HIGHLIGHT_WIDTH;
            }
        }
        else
        {
            self.highlight_pos = -100.0;
        }
    }

    fn get_matrix(&self) -> Vec<u8> {
        // Create the matrix
        let mut out: Vec<u8> = vec![OFF; self.get_shape().x * self.get_shape().y];

        let width = self.get_shape().x;

        // let bar_width_in_pixels;
        for x in 0..width {
            if self.state == battery::State::Full
            {
                out[x] = ON_DIM;
            }
            else
            {
                let percent_on = self.bar_width_in_pixels - x as f32;// this is a float telling how much the pixel should be on
                if percent_on > 1.0 {//if we are more than 100% on
                    out[x] = ON_DIM;
                }
                else if percent_on > 0.0//if we are fractionally on - the end of the bar
                {
                    out[x] = (ON_DIM as f32 * percent_on) as u8;
                }
                let highlight = (HIGHLIGHT_WIDTH-(self.highlight_pos - x as f32).abs()) / HIGHLIGHT_WIDTH;//0-1
                let highlight255 = (highlight * 255 as f32) as u8;
                if highlight255 > out[x] && out[x] > 5
                {
                    out[x] = highlight255;
                }
                
                // if (x as f32) < bar_width_in_pixels
                // {
                //     if self.state == battery::State::Charging
                //     {
                //         if (self.looper.wrapping_sub(x as u32)) % (bar_width_in_pixels as u32 + 1) < 2
                //         {
                //             out[x] = 255;
                //         }
                //     }
                //     else
                //     {
                //         if (self.looper.wrapping_add(x as u32)) % (bar_width_in_pixels as u32 + 1) < 2
                //         {
                //             out[x] = 255;
                //         }
                //     }
                // }
            }
            out[x + width] = out[x];
        }
        out
    }

    fn get_shape(&self) -> Shape {
        return Shape { x: 9, y: 2 };
    }
}

// -------- All Cores CPU Usage Widget --------
/// Create a widget that displays the usage of all CPU cores, one per row.
pub struct AllCPUsWidget {
    cpu_usages: Vec<f32>,
    merge_threads: bool,
    sys: sysinfo::System,
}

impl AllCPUsWidget {
    pub fn new(merge_threads: bool) -> AllCPUsWidget {
        let mut newsys = sysinfo::System::new();
        newsys.refresh_cpu();

        println!("Initializing AllCPUsWidget");

        AllCPUsWidget {
            cpu_usages: vec![0.0; newsys.cpus().len()],
            merge_threads,
            sys: newsys,
        }
    }
}

impl UpdatableWidget for AllCPUsWidget {
    fn update(&mut self) {
        // Refresh the cpu usage
        self.sys.refresh_cpu();

        for (idx, usage) in self.sys.cpus().iter().enumerate() {
            self.cpu_usages[idx] = usage.cpu_usage();
        }
    }

    /// Refresh the CPU usage and redraw the matrix
    fn get_matrix(&self) -> Vec<u8> {
        // Create the matrix
        let width = self.get_shape().x;
		let height = self.get_shape().y;
        let mut out = vec![OFF; width * height];

        if self.merge_threads {
            for idy in 0..height {
				let inverse_y = height - (idy + 1);
                for (idx, chunk) in self.cpu_usages.chunks(2).enumerate() {
                    let usage = (chunk[0] + chunk[1]) / 2.0;
					if usage as usize >= inverse_y * 10 {
						out[(idy * width) + idx] = ON_FULL;
					}
                }
            }
        } else {
            for y in 0..16 {
                let bar_width_in_pixels = self.cpu_usages[y] / 100.0 * width as f32;
                for x in 0..width {
                    let percent_on = bar_width_in_pixels - x as f32;// this is a float telling how much the pixel should be on
                    if percent_on > 1.0 {//if we are more than 100% on
                        out[x + (y * width)] = ON_FULL;
                    }
                    else if percent_on > 0.0//if we are fractionally on - the end of the bar
                    {
                        out[x + (y * width)] = (ON_FULL as f32 * percent_on) as u8;
                    }
                }
            }
        }

        out
    }

    fn get_shape(&self) -> Shape {
        return match self.merge_threads {
            false => Shape {
                x: 9,
                y: self.cpu_usages.len(),
            },
            true => Shape { x: 8, y: 8 },
        };
    }
}

pub struct ClockWidget {
    time: chrono::DateTime<Local>,
}

impl ClockWidget {
    pub fn new() -> Self {
        println!("Initializing ClockWidget");
        let dt = chrono::offset::Local::now();
        Self { time: dt }
    }

    fn render_digit(num: u32) -> &'static [u8] {
        match num {
            0 => DIGIT_0,
            1 => DIGIT_1,
            2 => DIGIT_2,
            3 => DIGIT_3,
            4 => DIGIT_4,
            5 => DIGIT_5,
            6 => DIGIT_6,
            7 => DIGIT_7,
            8 => DIGIT_8,
            9 => DIGIT_9,
            _ => DIGIT_0,
        }
    }

    fn render_number(num: u32) -> Vec<u8> {
        let mut numrow = vec![0; 9 * 5];
        let first_digit = Self::render_digit(num / 10);
        let second_digit = Self::render_digit(num % 10);
        for idx in 0..(9 * 5) {
            let cell = match idx % 9 {
                1 | 2 | 3 => first_digit[((idx / 9) * 3) + (idx % 9) - 1],
                5 | 6 | 7 => second_digit[((idx / 9) * 3) + idx % 9 - 5],
                _ => OFF,
            };
            numrow[idx] = cell;
        }
        numrow
    }
}

impl UpdatableWidget for ClockWidget {
    fn update(&mut self) {
        self.time = chrono::offset::Local::now();
    }

    fn get_matrix(&self) -> Vec<u8> {
        let mut matrix = Vec::with_capacity(9 * 11);
        matrix.extend(Self::render_number(self.time.hour()));
        matrix.extend(vec![OFF; 9]);
        matrix.extend(Self::render_number(self.time.minute()));
        matrix
    }

    fn get_shape(&self) -> Shape {
        return Shape { x: 9, y: 11 };
    }
}
