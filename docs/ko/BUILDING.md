[한국어](BUILDING.md) · [日本語](../BUILDING.md) · [English](../en/BUILDING.md)

<a id="ソースからのビルド"></a>

# 소스에서 빌드하기


Rust stable 툴체인과 C/C++ 빌드 환경이 필요합니다. 설치 방법은 [Rust 설치 안내](https://doc.rust-lang.org/book/ch01-01-installation.html)를 참고하세요. 소스를 내려받아 압축을 풀고 `Cargo.toml`이 있는 폴더에서 다음 명령을 실행합니다. 처음 빌드할 때는 의존 패키지를 받기 위한 인터넷 연결이 필요합니다.

화면 표시에는 GPU와 호환 드라이버가 필요합니다. 렌더링 기반은 [wgpu](https://docs.rs/wgpu/30.0.1/wgpu/struct.Backends.html)이며 Windows에서는 Direct3D 12 또는 Vulkan, Mac에서는 Metal, Linux에서는 Vulkan 등을 사용합니다. 앱 단독 실행에는 Unity가 필요하지 않습니다.

명령줄과 MCP 서버 (`yolupainter-cli`. [사용법](CLI.md))도 사용하려면 같은 명령에서 `-p yolu-app`을 `-p yolu-cli`로 바꾸어 빌드합니다 (화면 라이브러리를 사용하지 않으므로 빌드 시간이 짧음).

## Windows

64비트 Windows, Visual Studio Build Tools (2022의 17.11 이상)의 “C++를 사용한 데스크톱 개발” (Windows SDK 포함), 개별 구성 요소의 C++ ATL (ID `Microsoft.VisualStudio.Component.VC.ATL`), Rust MSVC 툴체인을 준비합니다. 셰이더 컴파일러 (DXC)를 실행 파일에 포함하기 위해 빌드할 때 GitHub에서 해당 라이브러리를 받습니다 (`curl.exe`와 `tar.exe`를 사용할 수 있어야 함). PowerShell에서 실행합니다.

```powershell
cargo build --release -p yolu-app --locked --target x86_64-pc-windows-msvc
.\target\x86_64-pc-windows-msvc\release\yolupainter.exe
```

펜 태블릿은 기본적으로 Windows Ink로 읽으므로 드라이버에서도 Windows Ink를 활성화하세요 (WinTab으로 읽으려면 “편집 → 설정…”의 “펜”에서 “펜 입력”을 선택). 브러시 도구 속성의 펜 버튼에서 필압으로 바꿀 항목을 선택할 수 있습니다. 필압이 너무 강하거나 약하면 “편집 → 설정…”의 “펜”에서 “필압 조정”으로 보정할 수 있습니다.

<a id="mac試用"></a>

## Mac (시험 사용)

Xcode Command Line Tools와 Rust를 준비하고 Metal을 사용할 수 있는 환경에서 실행합니다.

```sh
cargo build --release -p yolu-app --locked
./target/release/yolupainter
```

배포 형태 (Apple Silicon용과 Intel용 코드를 모두 포함하는 universal `YoluPainter.app`을 담은 zip)는 Xcode Command Line Tools (`lipo`·`codesign`·`iconutil`·`sips`·`ditto`·`plutil`), Python 3.10 이상, Rust 대상 2개를 준비한 뒤 저장소 루트에서 만듭니다.

```sh
rustup target add aarch64-apple-darwin x86_64-apple-darwin
cargo xtask build --target universal-apple-darwin --release
cargo xtask bundle --target universal-apple-darwin
```

`target/dist/yolupainter-<バージョン>-macos-universal-experimental.zip`이 생성됩니다. 2개 대상으로 빌드한 후 `lipo`로 하나로 합칩니다 (최소 macOS 버전은 11.0으로 통일). 기존 로고로 아이콘을 만들고 ad-hoc 서명 (`codesign -s -`)을 적용한 뒤 `ditto`로 zip을 만듭니다. Apple 개발자 서명과 공증은 하지 않습니다. 서명되지 않은 `.app`을 여는 방법은 [다운로드와 업데이트의 “Mac (시제품)”](INSTALL.md#mac試作)에 있습니다.

펜 태블릿 (Wacom·XP-Pen 등)은 드라이버가 macOS 표준 이벤트로 보내는 필압·기울기·지우개 쪽 끝·사이드 버튼을 읽습니다 (시험 기능). 제조사별 SDK는 사용하지 않습니다. 이상이 있으면 “편집 → 설정…”의 “펜”에서 “태블릿 필압 (시험)”을 끄세요. 그러면 펜이 마우스와 같은 방식으로 그립니다. 필압이 너무 강하거나 약하면 “편집 → 설정…”의 “펜”에서 “필압 조정”으로 보정할 수 있습니다.

<a id="linux試用"></a>

## Linux (시험 사용)

C/C++ 컴파일러, `pkg-config`, X11 데스크톱 환경, GPU 드라이버를 준비합니다. 창은 X11로 열리므로 Wayland 데스크톱에서는 XWayland 위에서 실행됩니다 (XWayland 없이 Wayland만 있는 환경에서는 실행할 수 없음). 파일 선택에는 D-Bus 세션, `xdg-desktop-portal`, 데스크톱에 맞는 포털 백엔드가 필요합니다. 확인 창 (예·아니요. 저장하지 않은 변경 사항을 버릴지 묻는 창 등)에는 `zenity`가 필요합니다 (없으면 확인이 필요한 작업은 취소됨). 화면 글꼴 (BIZ UDPGothic)은 실행 파일에 포함되므로 시스템에 일본어 글꼴이 없어도 됩니다.

Debian·Ubuntu 계열 패키지 이름의 예는 `build-essential`, `pkg-config`, `libxkbcommon-dev`, `libwayland-dev` (Wayland 구성 요소도 빌드에 포함되므로 빌드할 때 필요), `libvulkan1`, `xdg-desktop-portal`, `xdg-desktop-portal-gtk`, `zenity`입니다. GPU 드라이버는 장치에 맞는 것을 사용하세요.

```sh
cargo build --release -p yolu-app --locked
./target/release/yolupainter
```

마우스로 그릴 수 있습니다. 필압 수집은 데스크톱 환경과 입력 장치에 따라 달라집니다.

ARM (aarch64) Linux용 배포물은 시제품이며 실제 ARM 장치에서 실행하여 확인하지 않았습니다.
