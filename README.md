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

## 🗓 Week 2: Pipe Connection (cmd1 | cmd2)

### 🎯 Goal
- 두 개의 명령어를 실행하고 파이프(`|`)로 연결하기.
- C언어의 `pipe()`, `dup2()`, `close()` 로직을 Rust의 소유권 모델로 재구현하기.
- `split_whitespace`를 사용하여 공백이 포함된 명령어 문자열 파싱하기.

### 📝 Key Concepts & Learnings

#### 1. Piping in Rust (`Stdio`)
C에서는 파일 디스크립터(`int fd[2]`)를 직접 관리해야 했지만, Rust는 `Stdio` 추상화를 사용함.
- **cmd1:** `.stdout(Stdio::piped())`를 통해 쓰기 전용 파이프 생성.
- **cmd2:** `.stdin(Stdio::from(child1_stdout))`를 통해 읽기 전용 파이프 연결.

#### 2. Ownership & Resource Management
Rust의 소유권 시스템 덕분에 `close()`를 직접 호출할 필요가 없음.
- **`take()`:** `child1`의 `stdout` 소유권을 가져와서 `child2`에게 넘겨줌.
- 소유권이 이동(`Move`)되면 이전 스코프에서는 접근할 수 없으므로, 실수로 파이프를 닫지 않는 문제(Hang)를 원천 차단함.

#### 3. Execution Flow
- `spawn()`을 호출하여 두 프로세스를 동시에 실행(Concurrency)하고, `wait()`를 통해 좀비 프로세스 방지 및 종료 코드 회수.

### 🔄 C vs Rust Mapping
| C Concept (`pipex`) | Rust Implementation | Note |
|:---:|:---:|:---|
| `pipe(fd)` | `Stdio::piped()` | `Command` 설정 시 내부적으로 파이프 생성 |
| `dup2(fd[1], 1)` | `.stdout(Stdio::piped())` | 첫 번째 명령어의 출력을 파이프로 설정 |
| `dup2(fd[0], 0)` | `.stdin(Stdio::from(...))` | 두 번째 명령어의 입력을 첫 번째의 출력으로 설정 |
| `close(fd)` | `take()` / `Drop` | 소유권 이동 및 스코프 종료 시 자동 해제 |
| `ft_split` | `split_whitespace()` | 문자열 파싱 |

### 🛠 Usage
```bash
# Run with pipe
cargo run -- "ls -al" "wc -l"

# Result
# --- Pipe: "ls -al" | "wc -l" ---
# (Output of ls | wc)
# --- 실행 종료 (Exit Code: 0) ---
```

## 🗓 Week 3: File Redirection & Error Handling

### 🎯 Goal
- 파일 입출력을 프로세스의 표준 입출력으로 연결하기 (`Redirection`).
- 완성된 파이프라인 구조: `< infile cmd1 | cmd2 > outfile` 구현.
- `Result` 타입과 `?` 연산자를 사용하여 Rust다운 에러 처리 적용.

### 📝 Key Concepts & Learnings

#### 1. File Redirection (`Stdio::from`)
C언어의 복잡한 `open` + `dup2` + `close` 과정을 Rust의 객체 소유권 이동으로 단순화함.
- **Input:** `File::open()`으로 파일을 열고, `.stdin(Stdio::from(infile))`로 연결.
- **Output:** `File::create()`로 파일을 생성하고, `.stdout(Stdio::from(outfile))`로 연결.
- `Stdio::from()`이 파일 객체의 소유권을 가져가므로, 별도의 `close()` 호출이 필요 없음 (RAII).

#### 2. Error Propagation (`?` Operator)
`panic!`으로 프로그램을 강제 종료하는 대신, 에러를 호출자에게 전파하여 우아하게 종료함.
- **`Result<(), Box<dyn Error>>`**: 다양한 종류의 에러(`io::Error`, 등)를 하나의 `Box`(Trait Object)에 담아 처리.
- **`?` 연산자**: 에러 발생 시 즉시 함수를 리턴하고, 성공 시 값을 벗겨냄(`unwrap` 대체).

### 🔄 C vs Rust Mapping
| Feature | C (`pipex`) | Rust Implementation |
|:---:|:---:|:---|
| **Open Read** | `open("in", O_RDONLY)` | `File::open("in")` |
| **Open Write** | `open("out", O_CREAT \| O_TRUNC..)` | `File::create("out")` |
| **Redirect** | `dup2(fd, STDIN_FILENO)` | `.stdin(Stdio::from(file))` |
| **Error Check** | `if (ret < 0) return -1;` | `Result` + `?` operator |
| **Cleanup** | `close(fd);` | `Drop` (자동 해제) |

### 🛠 Usage
```bash
# Prepare input file
echo "Rust is safe" > infile.txt
echo "C is powerful" >> infile.txt

# Run pipex logic: grep "Rust" | wc -l
cargo run -- infile.txt "grep Rust" "wc -l" outfile.txt

# Check result
cat outfile.txt
# Output: 1
```
