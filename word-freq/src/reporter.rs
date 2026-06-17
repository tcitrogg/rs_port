pub fn save_report(content: String) -> String {
    // , files: &[String]
    let mut _file_name = String::from("wf-report.md");
    // generate the filename
    // if files.len() > 1 {
    // files.iter().fold(String::new(), |acc, s| acc+"_")
    //    _file_name = format!("{}and_other_files-wf-report.txt", files[0]);
    //} else {
    //    _file_name = format!("{}-wf-report.txt", files[0]);
    //}
    std::fs::write(&_file_name, content).expect("\x1b[31m(Err): File Not Found");
    _file_name
}
