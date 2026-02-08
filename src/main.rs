use std::fs::File;
use std::process::{Command, Stdio};
use std::env;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();


    if args.len() != 5 {
        eprintln!("Usage: {} <infile> \"<cmd1>\" \"<cmd2>\" <outfile>", args[0]);
        std::process::exit(1);
    }


    let infile_path = &args[1];
    let cmd1_str = &args[2];
    let cmd2_str = &args[3];
    let outfile_path = &args[4];


    // 1. Infile 열기 ('?'를 써서 에러 시 바로 리턴)
    let infile = File::open(infile_path)?;


    // 2. Outfile 열기
    let outfile = File::create(outfile_path)?;


    // 3. 첫번째 명령
    // 핵심: stdout을 'Stdio::piped()'로 설정
    // 이것이 C의 pipe() 생성 + dup2(fd[1], STDOUT_FILENO) 역할을 동시에 진행
    let cmd1: Vec<&str> = cmd1_str.split_whitespace().collect();
    let mut child1 = Command::new(cmd1[0])
            .args(&cmd1[1..])
            .stdin(Stdio::from(infile))
            .stdout(Stdio::piped())
            .spawn()?;


    // 4. 두번째 명령
    // 핵심: stdin에 child1의 stdout을 꽂아줌
    // 이것이 C의 dup2(fd[0], STDIN_FILENO) 역할
    let cmd2: Vec<&str> = cmd2_str.split_whitespace().collect();
    let mut child2 = Command::new(cmd2[0])
            .args(&cmd2[1..])
            .stdin(Stdio::from(child1.stdout.take().unwrap())) // 파이프 연결
            .stdout(Stdio::from(outfile))
            .spawn()?;


    // 5. 기다리기
    child1.wait()?;
    child2.wait()?;


    println!("Pipex 완료!");
    Ok(())
}
