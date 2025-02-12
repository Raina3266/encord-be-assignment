use crate::main_loop;

fn test_e2e(input: &str, expected_output: &str) {
    let input = format!("{input}\n");
    let mut output = Vec::new();

    main_loop(input.as_bytes(), &mut output, 10).unwrap();

    let output = String::from_utf8(output).unwrap();
    assert_eq!(output.trim(), expected_output);
}

#[test]
fn examples_from_readme() {
    test_e2e("Q0,Q0", "4");
    test_e2e("Q0,Q2", "2");
    test_e2e("I0,I4,Q8", "1");
}

#[test]
fn tests_from_sample_test_py() {
    test_e2e("Q0", "2");
    test_e2e(&["Q0"; 50].join(","), "100");
}

#[test]
fn multiple_input_lines() {
    test_e2e("Q0\nQ0\nQ0", "2\n2\n2");
}

#[test]
fn shapes_have_expected_height() {
    test_e2e("Q0", "2");
    test_e2e("Z0", "2");
    test_e2e("S0", "2");
    test_e2e("T0", "2");
    test_e2e("I0", "1");
    test_e2e("L0", "3");
    test_e2e("J0", "3");
}

#[test]
fn correct_answer_for_provided_test_case() {
    let provided_input = include_bytes!("../../tests/input.txt");
    let mut output = Vec::new();

    main_loop(&provided_input[..], &mut output, 10).unwrap();

    let output = String::from_utf8(output).unwrap();
    let output_lines: Vec<i32> = output.lines().map(|line| line.parse().unwrap()).collect();

    // calculated by hand
    let expected_output = [2, 4, 0, 2, 4, 1, 0, 2, 2, 2, 1, 1, 4, 3, 1, 2, 1, 8, 8, 0];

    assert_eq!(output_lines, expected_output)
}
