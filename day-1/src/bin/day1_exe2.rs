const MAX_LINE: usize = 18;

fn level(line: &str) -> &str {
    match line.split_once(':') {
        Some((lvl, _msg)) => lvl,
        None => "UNKNOWN",
    }
}

fn show(line: &str) {
    println!("> {line}");
}

fn main() {
    let logs: Vec<&str> = vec![
        "INFO:Boot complete".into(),
        "WARN:Low battery".into(),
        "ERROR:Disk full".into(),
        "INFO:User login".into(),
        "WARN:High temp".into(),
        "BAD LINE WITHOUT COLON".into(),
    ];

    let mut warn_count: usize = 0;
    let mut error_count: usize = 0;
    let mut unknown_count: usize = 0;

    let mut report  =  String::new();
   

    for line in &logs {
      
        show(line);

        let lvl = level(&line);

        match lvl {
            "WARN" => warn_count += 1,
            "ERROR" => error_count += 1,
            "INFO" => {}, 
            _ => unknown_count += 1, // logical bug (compile ho jayega but wrong)
        }

        if line.len() > MAX_LINE {
            report.push_str("LONG\n");
        } else {
            report.push_str("OK\n");
        }
    }

    let total = logs.len();
    println!("total={total}, warn={warn_count}, error={error_count}, unknown={unknown_count}");
    println!("report:\n{report}");
}