# rust-my-pipe

## 🗓 Week 1: Process Creation & Argument Parsing

### 🎯 Goal
- Rust의 `std::process::Command`를 사용하여 외부 프로그램 실행하기.
- C언어의 `fork`, `execve`, `waitpid` 흐름을 Rust로 구현하기.
- 하드코딩된 명령어가 아닌, 사용자 입력(`argv`)을 받아 동적으로 실행하기.

### 📝 Key Concepts & Learnings

#### 1. Builder Pattern for Process
C에서는 구조체를 초기화하고 함수에 포인터를 넘기는 방식이었지만, Rust는 **Builder Pattern**을 사용하여 직관적으로 프로세스를 설정함.
- **C:** `fork()` → `if (pid == 0) execve(...)`
- **Rust:** `Command::new(...).arg(...).spawn()`

#### 2. Ownership & Slices (Optimization)
초기 구현에서는 `String`을 복사(`clone`)하여 인자를 넘겼으나, 불필요한 메모리 할당을 방지하기 위해 **Reference(&)**와 **Slice**를 사용하는 방식으로 리팩토링함.
- **Before:** `for` loop를 돌며 `arg.clone()` 수행.
- **After:** `&args[2..]` 슬라이스를 사용하여 한 번에 인자 주입.

#### 3. Standard Streams
- **stdout:** `println!` (데이터 출력, 파이프 연결 대상)
- **stderr:** `eprintln!` (에러 로그, 파이프 타지 않음)

### 🔄 C vs Rust Mapping
| C Concept | Rust Implementation |
|:---:|:---:|
| `char *argv[]` | `std::env::args()` |
| `fork()` + `execve()` | `Command::spawn()` |
| `waitpid()` | `Child::wait()` |
| `int argc` check | `args.len()` |
