use std::process::Command;
use std::env;

fn main() {
    // 1. 인자 수집
    let args: Vec<String> = env::args().collect();
  
    if args.len() < 2 {
        eprintln!("usage: cargo run -- <command> [args...]");
        std::process::exit(1);
    }

    // 2. 명령어 구성
    // &를 사용하여 인자를 빌려옴(복사 X)
    let mut cmd = Command::new(&args[1]);

    // 슬라이스로 한번에 넣기
    cmd.args(&args[2..]);

    // 3. 실행 및 대기
    let mut child = cmd.spawn().expect("명령 실행 실패");
    let res = child.wait().expect("자식 프로세스 대기 실패");

    println!("--- exec result ---");
    println!("Exit code: {}", res);
}
