use std::process::{Command, Stdio};
// use std::env;

fn main() {
    println!("--- Pipe 연결 시작: ls -l | wc -l ---");

    // 1. 첫번째 명령
    // 핵심: stdout을 'Stdio::piped()'로 설정
    // 이것이 C의 pipe() 생성 + dup2(fd[1], STDOUT_FILENO) 역할을 동시에 진행
    let mut child1 = Command::new("ls")
            .arg("-l")
            .stdout(Stdio::piped()) 
            .spawn()
            .expect("ls 실행 실패");

    // 2. pipe 연결 (C의 핵심 로직과 다른 부분)
    // child1이 가진 stdout의 '소유권'을 가져옴 (take)
    let child1_stdout = child1.stdout.take()
                            .expect("stdout을 가져올 수 없음");

    // 3. 두번째 명령
    // 핵심: stdin에 child1의 stdout을 꽂아줌
    // 이것이 C의 dup2(fd[0], STDIN_FILENO) 역할
    let mut child2 = Command::new("wc")
            .arg("-l")
            .stdin(Stdio::from(child1_stdout))
            .spawn()
            .expect("wc 실행 실패");

    // 4. 종료 대기
    child1.wait().expect("child1 wait 에러"); 
    let output = child2.wait().expect("child2 wait 에러");

    println!("--- 실행 종료 (Exit Code: {}) ---", output);

}
