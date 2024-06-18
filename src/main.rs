mod ledmatrix;
mod matrix;
mod widget;
use std::{
    env::{args, args_os}, process::exit, thread, time::Duration
};

use clap::Parser;
use ledmatrix::LedMatrix;
use std::time::Instant;

use crate::widget::{AllCPUsWidget, BatteryWidgetUgly, RAMWidget, ClockWidget, UpdatableWidget};


const UPDATE_PERIOD:i32 = 100;


#[derive(Parser)]
#[command(version, about, long_about=None)]
struct Cli {
    // ======== Info about system ========
    #[arg(long)]
    /// List all connected matrix modules
    list_modules: bool,

    /// List all widgets available for placement
    #[arg(long)]
    list_widgets: bool, // ======== Program Control ========
                        // #[arg(long)]
                        // Start the background service updating the matrix
                        // start: bool,

                        // #[arg(long)]
                        // JSON config file path
                        // config: Option<String>,
}

enum Program {
    ListMod,
    ListWid,
    Default,
}

fn main() {
    // TODO possible options:
    // each widget + Y placement + LED module (both as default) (for now, x maybe later)
    // Overall brightness
    // update rate

    let matrix1 = "FRANKDEBZA1404101JE";
    let matrix2 = "FRAKDEBZA1404101J2";

    let mut program = Program::Default;

    if args_os().len() > 1 {
        let cli = Cli::parse();
        if cli.list_modules {
            program = Program::ListMod;
        } else if cli.list_widgets {
            program = Program::ListWid;
        }
    }
    fn detect_and_init_matrixes(mats: &mut Vec<LedMatrix>)
    {
        let detected_mats = LedMatrix::detect();
        for i in detected_mats
        {
            let mut found = false;
            for k in &mut *mats
            {
                if k.info.usb_info.serial_number == i.info.usb_info.serial_number
                {
                    found = true;
                    break;
                }
            }
            if !found
            {
                let mat = LedMatrix::connect(i);
                if mat.is_some()//sometimes we fail to connect. just dont initialize it
                {
                    let mut mat = mat.unwrap();
                    mat.draw_bool_matrix([[false; 9]; 34]);
                    mat.set_full_brightness(65);
                    mats.push(mat);
                }
            }
        }
    }
    fn error_mat(mats: &mut Vec<LedMatrix>, i: usize)
    {
        println!("Matrix on {} had a communication error, reconnecting...", mats[i].info.port_info.port_name);
        mats.remove(0);
        thread::sleep(Duration::from_millis(1000));
    }
    fn draw_on(mats: &mut Vec<LedMatrix>, i: usize, matrix: [[u8; 9]; 34])
    {
        let result = mats[i].draw_matrix(matrix);
        if result.is_err()
        {
            error_mat(mats, 0);
        }
    }
    match program {
        Program::Default => {
            let mut mats: Vec<LedMatrix> = vec![];
            detect_and_init_matrixes(&mut mats);
            // let num_mats = mats.len();
            // No arguments provided? Start the
            if args().len() <= 1 {
                let mut bat = BatteryWidgetUgly::new();
                let mut ram = RAMWidget::new();
                let mut cpu = AllCPUsWidget::new(false);
                let mut clock = ClockWidget::new();


                let mut ticker:u8 = 0;
                loop {
                    let start = Instant::now();
                    // do stuff

                    bat.update();
                    ram.update();
                    if ticker%5==0
                    {
                        cpu.update();
                        clock.update();
                    }
                    ticker = ticker.wrapping_add(1);

                    let mut matrix: [[u8; 9]; 34] = [[0; 9]; 34];
                    matrix = matrix::emplace(matrix, &bat, 0, 0);
                    matrix = matrix::emplace(matrix, &ram, 0, 3);
                    matrix = matrix::emplace(matrix, &cpu, 0, 6);
                    matrix = matrix::emplace(matrix, &clock, 0, 23);
                    

                    if mats.len() == 1//only one connected
                    {
                        draw_on(&mut mats, 0, matrix);
                    }
                    else if mats.len() == 2
                    {
                        if(mats[0].info.usb_info.serial_number.as_ref().unwrap() == matrix1)
                        {
                            draw_on(&mut mats, 0, matrix);
                            if mats.len()==2
                            {
                                draw_on(&mut mats, 1, [[0; 9]; 34]);
                            }
                        }
                        else {
                            draw_on(&mut mats, 0, [[0; 9]; 34]);
                            if mats.len()==2
                            {
                                draw_on(&mut mats, 1, matrix);
                            }
                        }
                    }
                    else 
                    {
                        println!("No Matrixes found, checking again soon");
                        thread::sleep(Duration::from_millis(1000));
                    }
                    detect_and_init_matrixes(&mut mats);




                    let elapsed = start.elapsed().as_millis();
                    let time_to_sleep = UPDATE_PERIOD-elapsed as i32;
                    // println!("time: {time_to_sleep}");
                    if time_to_sleep > 0
                    {
                        thread::sleep(Duration::from_millis(time_to_sleep.try_into().unwrap()));
                    }
                }
            }
        }
        Program::ListMod => {
            LedMatrix::detect();
        }
        Program::ListWid => {
            println!(
                "Battery Indicator:\n \
                A 9x4 widget in the shape of a battery, with an internal bar indicating remaining capacity.\n"
            );
            println!(
                "CPU Usage Indicator:\n \
                A 9x16 widget where each row of LEDs is a bar that represents the CPU usage of one core.\n"
            );
            println!(
                "Clock Widget:\n \
                A 9x11 widget that displays the system time in 24hr format.\n"
            );
        } // _ => {}
    }

    exit(0);
}
