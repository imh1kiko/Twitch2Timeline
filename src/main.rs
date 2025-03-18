use std::fs::File;
use std::io::{BufRead, Write};

// Custom implementation, as Chronos didn't cut.
struct ResolveEdlTime {
    start_time_code: u32,
    hours: u32,
    minutes: u32,
    seconds: u32,
}
impl ResolveEdlTime {
    // Create a new instance of ResolveEdlTime by splicing : into vector of u32
    fn new(time: &str, offset:u32) -> ResolveEdlTime {
        let time: Vec<&str> = time.split(":").collect();
        // start_time_code takes hours and divides by 60, hours is the remainder
        let start_time_code:u32 = (&time[0].parse::<u32>().unwrap()/60)+offset;
        let hours:u32 = &time[0].parse::<u32>().unwrap()%60;

        ResolveEdlTime {
            start_time_code,
            hours,
            minutes: time[1].parse().unwrap(),
            seconds: time[2].parse().unwrap(),
        }
    }
    fn add_seconds(&self, seconds:u32) -> ResolveEdlTime {
        let mut new_seconds = self.seconds + seconds;
        let mut new_minutes = self.minutes;
        let mut new_hours = self.hours;
        if new_seconds >= 60 {
            new_seconds -= 60;
            new_minutes += 1;
        }
        if new_minutes >= 60 {
            new_minutes -= 60;
            new_hours += 1;
        }
        ResolveEdlTime {
            start_time_code: self.start_time_code,
            hours: new_hours,
            minutes: new_minutes,
            seconds: new_seconds,
        }
    }
    fn marker_start(&self) -> String {
        format!("{:02}:{:02}:{:02}:{:02}", self.start_time_code, self.hours, self.minutes, self.seconds)
    }
    fn marker_end(&self) -> String {
        let end_time = self.add_seconds(1);
        format!("{:02}:{:02}:{:02}:{:02}", self.start_time_code, end_time.hours, end_time.minutes, end_time.seconds)
    }
}

fn main() {
    // Get CLi arguments, ignore first argument
    let args: Vec<String> = std::env::args().skip(1).collect();
    // if args aren't supplied, or not met with correct amount, error out
    if args.len() != 4 {
        eprintln!("Usage: Twitch2Timeline <INPUT FILE> <MARKER COLOR> <TIMECODE OFFSET> <OUTPUT NAME>");
        std::process::exit(1);
    }
    // supply the args to constructor
    construct_edl_section(
        args[0].clone(),
        args[1].clone(),
        args[2].clone().parse::<u32>().unwrap(),
        args[3].clone()
    );
}

fn construct_edl_section(input_file:String, color_name:String, timecode_offset:u32, output_name:String) {
    let header: &str = "TITLE: {}\nFCM: NON-DROP FRAME\n\n";
    // Keeps track of marker number
    let mut iterator:u32 = 1;
    // This holds the lines to prevent unnecessary writes to the file
    let mut edl_lines:String = String::new();
    // We open the file user gave through CLi
    let file = File::open(input_file).expect("Failed to open file");
    let reader = std::io::BufReader::new(file);
    // Iterate through the lines
    for line in reader.lines() {
        let line = line.unwrap();
        // Split the line by comma, resulting in "time, level, user and description"
        let split_line: Vec<&str> = line.split(",").collect();
        // We construct the data needed for EDL file.
        let edl_time = ResolveEdlTime::new(split_line[0], timecode_offset);
        // get proper color
        let color = get_color(color_name.as_str());
        // format the template using the data
        let edl_line = format!("{}  001      V     C        {} {} {} {}\n \
      |C:{} |M:{} by {} |D:1",
                               iterator,
                               edl_time.marker_start(),
                               edl_time.marker_end(),
                               edl_time.marker_start(),
                               edl_time.marker_end(),
                               color,
                               split_line[3],
                               split_line[2]
        );
        // Merge edl_line with edl_lines
        edl_lines.push_str(edl_line.as_str());
        // New line in the edl_lines
        edl_lines.push_str("\n");
        iterator += 1;
    }
    // Combine the header and edl_lines
    let mut combined = String::new();
    combined.push_str(&*header.replace("{}", "Twitch CSV to Resolve EDL"));
    combined.push_str(edl_lines.as_str());

    // Write to file
    let mut output_file = File::create(format!("{}.edl", output_name)).expect("Failed to create file");
    output_file.write_all(combined.as_bytes()).expect("Failed to write to file");
}

fn get_color(color:&str) -> &str {
    match color.to_uppercase().trim() {
        "BLUE" => "ResolveColorBlue",
        "CYAN" => "ResolveColorCyan",
        "GREEN" => "ResolveColorGreen",
        "YELLOW" => "ResolveColorYellow",
        "RED" => "ResolveColorRed",
        "PINK" => "ResolveColorPink",
        "PURPLE" => "ResolveColorPurple",
        "FUCHSIA" => "ResolveColorFuchsia",
        "ROSE" => "ResolveColorRose",
        "LAVENDER" => "ResolveColorLavender",
        "SKY" => "ResolveColorSky",
        "MINT" => "ResolveColorMint",
        "LEMON" => "ResolveColorLemon",
        "SAND" => "ResolveColorSand",
        "COCOA" => "ResolveColorCocoa",
        "CREAM" => "ResolveColorCream",
        _ => "ResolveColorBlue"
    }
}
