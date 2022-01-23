use wrought::compile;

use std::fs;

#[test]
fn test_sample_programs() {
    for entry in fs::read_dir("./tests/sample_programs").unwrap() {
        let entry = entry.unwrap();
        // Skip any files that aren't *.wrt
        if !entry.file_name().to_str().unwrap().ends_with(".wrt") {
            continue;
        }

        // Read and compile the wrt program
        let name = entry.path()
            .file_name().expect("File name not available")
            .to_str().expect("File name not valid String").to_string();
        let input =  fs::read_to_string(entry.path()).unwrap();
        let output = compile(name.clone(), input);

        // Construct the name of the .wat expectation file
        let result_name = &name.split(".")
            .map(|s| s.to_string())
            .collect::<Vec<String>>()[0];
        let result_name = format!("{}.wat", result_name);

        // Compare the output to the expectation
        let result_file_path = entry.path()
            .with_file_name(result_name);
        let expected_output =  fs::read_to_string(result_file_path).unwrap();
        assert_eq!(output, Some(expected_output));
    }
}
