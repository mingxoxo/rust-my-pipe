use std::process::Command;

fn main() {
    println!("-- Rust로 ls -l 실행 --");

    let mut child = Command::new("ls")  // 실행 프로그램 이름
            .arg("-l")                  // 인자
            .spawn()                    // fork + exec
            .expect("명령 실패");       // exec 실패 시

    let ecode = child.wait()
            .expect("자식 프로세스 기다리는 중 에러 발생");

    println!("-- 실행 종료 (Exit code: {}) --", ecode);
}
