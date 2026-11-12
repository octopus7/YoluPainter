[한국어](THIRD_PARTY.ko.md) · [日本語](THIRD_PARTY.md)

<a id="第三者の許諾"></a>

# 제3자 라이선스

Windows MSVC·Windows GNU·Linux GNU·Linux ARM(aarch64)·macOS(universal)용 `yolu-app`(독립 실행형 앱, 업데이트 의존성과 명령줄 `yolu-cli`의 의존성 포함) 및
`yolu-cli`(Claude Desktop용 `.mcpb`. 앱과 같은 빌드의 하위 트리), 개발용 `xtask`의 의존성 목록입니다. 기본 기능과 아래 표에 기록한 Cargo.lock을 대상으로 합니다.
의존성을 업데이트하거나 다른 대상·기능으로 배포할 때는 목록과 라이선스 전문 묶음을 갱신합니다.
각 대상의 의존성을 나누어 기재합니다. Linux의 LGPL protocol XML 처리는 “배포 대상별 대조”에 설명합니다.

`tools/third-party.py`는 Cargo의 일반 의존성과 빌드 의존성을 제품별로 추적하며, 테스트용 의존성은 제외합니다.
프로시저 매크로와 빌드 의존성도 보수적으로 전문 묶음에 포함합니다. “실행 시”는 일반 의존성으로 도달하는 범위를 뜻하며,
최적화한 바이너리에 해당 크레이트의 모든 코드가 남는다는 뜻은 아닙니다.
`tools/licenses-reviewed.json`에 버전·선언된 라이선스·선택한 라이선스·원문 위치·SHA-256을 고정해 두었습니다.
크레이트에 원문이 없으면 해당 크레이트 발행 시점 커밋의 상위 프로젝트 원문을 가져옵니다.
같은 종류의 일반적인 라이선스 문구로 단순 대체하지 않으며, 저작권 표시와 NOTICE도 보존합니다.
크레이트와 상위 프로젝트 모두에 라이선스 본문이 없는 경우에만 상위 프로젝트의 기재 내용(`Cargo.toml`의 `license`와 `authors`)으로 구성한 MIT 문구를 `tools/license-texts/`에 놓고 고정합니다.
해당 문구의 앞부분에는 상위 프로젝트 원문이 아니라는 사실과 사용한 기재 내용을 적습니다. 연도는 상위 프로젝트 기재에 없으므로 적지 않습니다.

생성 방법은 [개발 절차](https://github.com/YozoraKurage/YoluPainter/blob/main/docs/DEVELOPMENT.md#配布用の許諾全文)를 참고하세요.
목록의 JSON과 Markdown, 성공한 제품의 `THIRD_PARTY_LICENSES.txt`는 `target/third-party/<target>/<クレート>/`에 생성됩니다(`--target`을 생략하면 Windows GNU용으로 `target/third-party/<クレート>/`에 출력).
생성물은 Git에 넣지 않습니다. 이 문서는 확인한 의존성 구성의 기록이며, 배포 시에는 생성한 전문을 포함합니다.

<a id="適用する許諾と配布時の表記"></a>

## 적용 라이선스와 배포 시 고지

- `clipboard-win 5.4.1`과 `error-code 3.4.0`은 **BSL-1.0(Boost Software License)**입니다.
  `egui-winit → arboard → clipboard-win → error-code`를 통해 들어옵니다.
  출처: [clipboard-win 원문](https://github.com/DoumanAsh/clipboard-win/blob/3b27cf2bfd1adcfa6e0264eb51c1025ddaf0f342/LICENSE),
  [error-code 원문](https://github.com/DoumanAsh/error-code/blob/e4615e514db3ff64f5f1b328b4865b20a3dcdbc3/LICENSE).
- `yolu-app`은 OS 클립보드 이미지를 위해 `arboard 3.6.1`을 `image-data` 및 `wayland-data-control` 기능으로 직접 사용합니다(Wayland의 data-control을 먼저 시도하고,
  사용할 수 없으면 X11 사용). `arboard` 자체는 이미 `egui-winit`을 통해 들어오던 것이며, 추가되는 것은 Linux 전용 `wl-clipboard-rs 0.9.4`
  (MIT OR Apache-2.0 중 MIT)와 그 의존성인 `os_pipe`·`tree_magic_mini`·`nom`(모두 MIT), `petgraph`·`fixedbitset`·
  `hashbrown 0.15.5`(MIT OR Apache-2.0 중 MIT), `foldhash 0.1.5`(Zlib)입니다. Windows 의존성은 변하지 않습니다. `tree_magic_mini`의 GPL 데이터
  (별도 크레이트 `tree_magic_db`, `with-gpl-data` 기능)는 활성화하지 않습니다. 활성화하면 GPL이 포함되므로 이 기능을 켜지 마세요.
- `yolu-app`은 화면 배치(도킹 탭 그룹·분할 방식·창 크기)를 저장하기 위해 `egui_dock`의 `serde` 기능을 활성화합니다.
  `egui`·`epaint`·`emath`·`ecolor`·`accesskit` 등이 이미 사용하는 `serde`만 사용하며, 추가되는 크레이트는 `accesskit`의 `serde` 기능이 가져오는
  프로시저 매크로 `enumn 0.1.14`(MIT OR Apache-2.0 중 MIT. 원문은 `LICENSE-MIT`) 1개뿐입니다.
- `yolu-app`은 화면 문구(`crates/yolu-app/locales/`의 .ftl)를 읽고 서식을 적용하기 위해 `fluent-bundle 0.16.0`·`fluent-syntax 0.12.0`·
  `unic-langid 0.9.6`을 직접 사용합니다. 추가되는 것은 이 3개와 그 의존성인 `fluent-langneg 0.13.1`·`intl-memoizer 0.5.3`·`intl_pluralrules 7.0.2`·
  `unic-langid-impl 0.9.6`(모두 Apache-2.0 OR MIT·MIT OR Apache-2.0 중 MIT), `tinystr 0.8.4`(Unicode-3.0)의 총 8개이며, 모든 대상 배포물에 포함됩니다.
  원문은 크레이트에 포함된 `LICENSE-MIT`(`tinystr`는 `LICENSE`)를 사용하며, 크레이트에 본문이 없는 `fluent-langneg`·`intl_pluralrules`는 발행 시점 커밋의 상위 프로젝트
  `LICENSE-MIT`를 SHA-256으로 대조합니다. `tinystr`가 가져오는 프로시저 매크로 `displaydoc 0.2.7`(MIT OR Apache-2.0 중 MIT. Cargo.lock에는 이전부터 있었음)도
  app 목록에 포함됩니다. `tinystr`에서 사용하지 않는 기능의 의존성인 `zerovec 0.11.8`·`zerofrom 0.1.8`(Unicode-3.0)은 Cargo.lock에 있지만 어떤 대상의 빌드에도 포함되지 않습니다
  (아래 lock 전체의 “대상 외”). 10개 전체 소스에서 GPL·LGPL·AGPL 표기를 검색했으며 해당 사항은 없었습니다.
- egui 기본 글꼴에 포함된 Hack 원문에는 **Bitstream Vera** 조건도 있습니다.
  OFL-1.1·Ubuntu Font Licence와 함께 전문을 보존합니다.
- `yolu-io`는 OS에 설치된 글꼴 목록(텍스트 레이어의 글꼴 선택)을 위해 `fontdb 0.24.0`(MIT)을 `std`·`fs`·`memmap` 기능으로 사용합니다(`default-features`는 끔).
  OS의 글꼴 파일을 읽기만 하며 OS 글꼴은 포함하지 않습니다. 포함하는 글꼴은 아래 BIZ UDPGothic과 egui 기본 글꼴(위 표)입니다.
- `yolu-app`은 Live Link 원본 이미지(JPG)를 읽기 위해 `image`의 `jpeg` 기능을 활성화합니다. 추가되는 것은 `zune-jpeg 0.5.15`와 `zune-core 0.5.3`
  (둘 다 MIT OR Apache-2.0 OR Zlib 중 MIT. 원문은 크레이트의 `LICENSE-MIT`) 2개이며, 둘 다 Cargo.lock에는 이전부터 테스트 의존성 `tiff`를 통해 들어 있었습니다.
  2개 전체 소스에서 GPL·LGPL·AGPL 표기를 검색했으며 해당 사항은 없었습니다. TGA 읽기(`tga` 기능)는 `image` 내부에서만 처리하므로 크레이트가 늘어나지 않습니다.
- 위 발행 시점 커밋의 원문과 SHA-256을 대조했으며, **Windows MSVC·Windows GNU·macOS(universal)는 app 크레이트 부분의 배포용 전문 묶음을 생성할 수 있습니다**.
  확인하지 않은 버전이나 원문 변경을 자동 승인하지 않으며, 의존성을 업데이트하면 다시 확인합니다.
- macOS(universal) 배포물에만 들어가는 의존성 32개(`objc2` 계열·`block2`·`dispatch2`·`dispatch`·`core-foundation` 계열·`core-graphics` 계열·`foreign-types` 계열·`accesskit_macos`·
  `accesskit_consumer 0.38.0`·`raw-window-metal`·`wgpu-core-deps-apple`·`tiff`·`weezl`·`fax`·`quick-error 2.0.1`·`bitflags 1.3.2`)를 `tools/licenses-reviewed.json`에 등록했습니다.
  라이선스는 MIT입니다(`OR` 조합은 MIT 선택). 원문은 크레이트에 포함된 것(15개)이나 해당 크레이트 발행 시점 커밋의 상위 프로젝트 원문(17개)을 SHA-256으로 고정합니다(그중 11개는 아래에서 구성한 문구를 앞에 추가).
  `objc2`의 0.6 계열 10개(`objc2`·`block2`·`dispatch2`·`objc2-app-kit`·`objc2-core-foundation`·`objc2-core-graphics`·`objc2-encode`·`objc2-foundation`·`objc2-metal`·`objc2-quartz-core`)와
  `dispatch 0.2.0`의 총 11개는 크레이트와 상위 저장소 모두에 라이선스 본문 파일이 없습니다(상위 프로젝트의 `LICENSE.md`는 라이선스 종류와 Apple SDK에서 만든 바인딩이라는 설명이며,
  저작권 행과 MIT 본문이 없음. `dispatch`는 상위 프로젝트 어디에도 라이선스 본문이 없음). 따라서 상위 프로젝트 기재 내용으로 구성한 MIT 문구(본문은 MIT 표준 문구, 저작권 행은 `Cargo.toml`의 `authors`)를
  `tools/license-texts/`에 놓고 전문 묶음 앞에 기재합니다. 작성자는 `objc2`·`block2`·`objc2-encode`가 Mads Marquart, `dispatch2`가 Mads Marquart와 Mary, `dispatch`가 Steven Sheldon입니다.
  `objc2-foundation`·`objc2-app-kit`·`objc2-core-foundation`·`objc2-core-graphics`·`objc2-metal`·`objc2-quartz-core`는 `Cargo.toml`에 `authors`가 없으므로 같은 저장소의 `objc2`에 있는 `authors`(Mads Marquart)를 사용합니다.
  상위 프로젝트의 `LICENSE.md`(`dispatch`는 `Cargo.toml`)는 구성한 문구 뒤에 이어서 고정합니다.
  `objc2`의 0.5 계열(`objc2`·`block2`·`objc-sys`·`objc2-app-kit`·`objc2-foundation`의 이전 버전)은 상위 프로젝트의 `LICENSE.txt`(MIT 본문)를 고정합니다.
- `self_cell`의 선언은 `Apache-2.0 OR GPL-2.0-only`입니다. 선택하는 라이선스는 Apache-2.0이며 GPL 조건은 선택하지 않습니다.
  `Unlicense OR MIT`도 MIT를 선택합니다. AND 조건은 모두 유지합니다.
- `bevy_mikktspace 1.0.0`(3D 뷰 노멀 맵의 탄젠트)은 **Zlib AND (MIT OR Apache-2.0)**입니다. MikkTSpace 참조 구현(Morten S. Mikkelsen)을
  Rust로 다시 작성한 것으로, Zlib 고지는 독립 라이선스 파일 없이 크레이트의 `src/lib.rs` 맨 앞에 있습니다. MIT의 `LICENSE-MIT`와 함께
  해당 원문(`lib.rs`)을 전문 묶음에 포함합니다. 선택하는 것은 MIT이며, Zlib 조건(출처를 허위로 밝히지 않음·변경 사항 명시·고지 삭제 금지)은 유지합니다.
- `yolu-ops`(외부에서 문서를 조작하는 명령 타입과 헤드리스 호스트. `yolu-cli`와 앱에서 사용)는 명령·응답·오류의 JSON Schema를 타입에서 생성하기 위해
  `schemars 1.2.2`(MIT. 원문은 크레이트의 `LICENSE`)를 사용합니다. 추가되는 크레이트는 `schemars_derive 1.2.2`(프로시저 매크로. MIT), `serde_derive_internals 0.30.0`·
  `dyn-clone 1.0.20`·`ref-cast 1.0.27`·`ref-cast-impl 1.0.27`(모두 MIT OR Apache-2.0 중 MIT. 원문은 `LICENSE-MIT`)의 총 6개이며, 모두 각 크레이트에 포함된 원문을
  SHA-256으로 대조합니다. 앱은 외부 조작을 받기 위해 `yolu-ops`를 직접 사용하고, 같은 배포물에 들어가는 `yolu-cli`도 사용하므로 이 6개는 `yolu-app` 목록에 포함됩니다.
- `yolu-cli`(명령줄 `yolupainter-cli` 및 표준 입출력 MCP를 실행 중인 앱에 연결하는 중계. 앱과 같은 배포물·같은 버전으로 배포하며 Claude Desktop용 `.mcpb`에도 포함)와
  `yolu-mcp`는 MCP를 위해 공식 Rust SDK `rmcp 3.5.1`과 `tokio`(MIT. 표준 입출력·실행 기반·127.0.0.1 통신 기능만)를 사용합니다. `rmcp`의 선언은 **Apache-2.0**이며, 크레이트 배포물에는 라이선스 전문이 없습니다. MCP 프로젝트는 MIT에서
  Apache-2.0으로 전환하는 도중이며 재라이선스 동의를 받지 않은 기여는 MIT로 남아 있으므로, 상위 프로젝트의 `LICENSE`(Apache-2.0 전문·MIT·문서에만 적용하는 CC-BY-4.0 안내)를
  발행 시점 commit(`79437f291b2c44053d00dcd5db969fd0cca7c887`)의 고정 URL에서 가져와 SHA-256으로 대조하고 Apache-2.0과 MIT를 모두 선택합니다(CC-BY-4.0은 문서 전용이며 코드에는 적용하지 않음).
  추가되는 크레이트는 `rmcp`, `rmcp`가 가져오는 `pastey 0.2.3`(MIT OR Apache-2.0 중 MIT), `tokio 1.53.2`·`tokio-macros 2.7.2`·`tokio-util 0.7.19`·`bytes 1.12.1`(모두 MIT),
  `futures`·`futures-channel`·`futures-executor`·`futures-sink`(0.3.34. MIT OR Apache-2.0 중 MIT), `chrono 0.4.45`(MIT OR Apache-2.0 중 MIT. 원문은 `LICENSE.txt`)의 11개입니다.
  `getrandom 0.4.3`(MIT OR Apache-2.0 중 MIT)은 테스트 전용에서 일반 의존성으로 이동합니다(`rmcp`가 가져오는 `uuid`에서 사용. 이미 등록됨). 모두 각 크레이트에 포함된 원문을 SHA-256으로 대조했으며,
  크레이트 전체 소스에서 GPL·LGPL·AGPL 표기를 검색했으나 해당 사항은 없었습니다. `rmcp`의 `macros`·`client`·OAuth 기능은 사용하지 않으므로 `reqwest` 등 통신 구성 요소는 들어가지 않습니다.
- `yolu-mcp`(MCP 구현 및 127.0.0.1의 HTTP 수신 지점·클라이언트. 앱이 수신 지점을 실행하고 `yolu-cli`가 중계 및 실행 중인 앱 명령에 사용)는 `rmcp`의
  Streamable HTTP 서버 기능(`transport-streamable-http-server`)과 이를 제공하는 `hyper 1.12.0`(`server`·`client`·`http1` 기능만. HTTP/2 제외)·
  `hyper-util 0.1.21`(`tokio` 기능만)·`http-body-util 0.1.5`(모두 MIT)를 사용합니다. 추가되는 크레이트는 `hyper`·`hyper-util`·`http-body-util`·`http-body 1.1.0`·
  `tokio-stream 0.1.19`·`tower-service 0.3.3`·`try-lock 0.2.5`·`want 0.3.2`·`mio 1.2.4`(모두 MIT. 원문은 `LICENSE`) 및 `http 1.5.0`·`httparse 1.10.1`·
  `httpdate 1.0.3`·`socket2 0.6.5`·`sse-stream 0.2.6`·`base64 0.23.1`·`rand 0.10.3`·`rand_core 0.10.1`·`chacha20 0.10.2`·`cpufeatures 0.3.1`(모두 MIT OR Apache-2.0
  중 MIT. 원문은 `LICENSE-MIT`, `rand`·`rand_core`는 `COPYRIGHT`도 포함)의 19개입니다. `rand` 이하 4개와 `sse-stream`·`base64 0.23.1`은 `rmcp`의 HTTP 서버 기능이,
  `mio`·`socket2`는 `tokio`의 `net` 기능이 가져옵니다. 모두 각 크레이트에 포함된 원문을 SHA-256으로 대조했으며, 크레이트 전체 소스에서 GPL·LGPL·AGPL 표기를 검색했으나 해당 사항은 없었습니다.
  TLS 구성 요소는 포함되지 않습니다(서버와 클라이언트 모두 127.0.0.1의 평문 HTTP만 사용).
- exe의 버전 정보(아이콘·제품명·버전)를 삽입하는 빌드용 `winresource`(MIT)는 실행 파일에 들어가지 않으며, 빌드 의존성으로 전문 묶음에 포함합니다.
- 업데이트 통신은 OS 구성 요소를 호출하기만 하며 통신 구성 요소를 함께 제공하지 않습니다. Windows는 OS에 포함된 WinHTTP, Linux는 사용자 환경의 `curl`을 호출합니다.
- Windows 설치 프로그램은 NSIS 3(zlib/libpng 라이선스)으로 만듭니다. 만든 설치 프로그램에는 NSIS 실행 구성 요소(stub)가 들어가며,
  압축 방식별 구성 요소(LZMA·bzip2·zlib)에는 각 라이선스와 NSIS 예외가 적용됩니다. 상위 프로젝트의 표기는
  [NSIS 라이선스](https://nsis.sourceforge.io/NSIS_License)를 참고하세요. 설치 프로그램에 넣는 파일은 zip과 같으며, 전문 묶음도 같은 것을 넣습니다.
- 자체 작성 부분의 배포 라이선스, Rust 표준 라이브러리, 실제로 링크하는 MinGW/GCC 런타임, 추가로 포함하는 DLL의 고지는
  최종 배포물과 사용한 도구 체인에 맞추어 별도로 확인합니다. 이 크레이트 목록만으로 제품 전체의 배포 가능 여부가 확정되지는 않습니다.
- `yolu-gpu`는 D3D12 셰이더 컴파일러 DXC를 실행 파일에 정적으로 포함합니다(wgpu의 `static-dxc`. Windows x86_64 msvc 빌드만.
  Windows GNU·Linux·macOS 의존성은 변하지 않음). 추가되는 크레이트는 `mach-dxcompiler-rs 0.1.6+2026.09.16-48d5a66.1`(MIT. 실행 시) 및 빌드용 `lhash 1.1.0`
  (BSL-1.0. 크레이트에 라이선스 본문이 없어 발행 시점 커밋의 상위 프로젝트 `LICENSE`를 SHA-256으로 대조) 2개입니다. DXC 본체는 크레이트 빌드(`build.rs`)가
  [mach-dxcompiler 릴리스 `2026.09.16+48d5a66.1`](https://github.com/DouglasDwyer/mach-dxcompiler/releases/tag/2026.09.16%2B48d5a66.1)(변경할 수 없는 릴리스로 공개된 것)의
  정적 라이브러리를 가져와 SHA-256을 대조한 뒤 링크합니다. 원본 소스는 `hexops/DirectXShaderCompiler`의 커밋 `4190bb0c90d374c6b4d0b0f2c7b45b604eda24b6`
  (mach-dxcompiler의 `build.zig`에 있는 `source_revision`)이며, SPIR-V 출력은 포함하지 않습니다(같은 `build.zig`의 `spirv` 선택은 기본값 false이고, 릴리스를 만드는 `ci.yml`의 `zig build`에도 지정 없음).
  적용되는 라이선스는 DXC의 **University of Illinois/NCSA**(LLVM Release License)와 해당 `LICENSE.TXT`에 나열된 LLVM 유래 구성 요소(`Files:` 행을 보면 miniz·Henry Spencer의 regex·ConvertUTF·MD5·OpenBSD의 `regstrlcpy`. 그 밖에 테스트용 googletest·YamlParser 및 빌드 보조 `autoconf/config.guess`(GPL 전문과 configure 예외) 항목도 있음. 이들이 라이브러리에 들어가는 구성 요소인지는 `Files:` 행으로 확인하지 않음),
  Mach 서명 부분(Apache-2.0 또는 MIT. MIT를 선택하고 크레이트의 `LICENSE`를 포함)입니다. DXC의 `LICENSE.TXT`와 `ThirdPartyNotices.txt`(둘 다 해당 커밋의 원문)를
  크레이트 라이선스와 함께 전문 묶음에 넣습니다(`NCSA`는 `tools/third-party.py`와 `deny.toml` 허용 목록에 추가). NCSA 조건은 저작권 표시·조건·면책을 함께 제공하는 자료에 기재하고,
  LLVM Team·일리노이 대학교·기여자의 이름을 광고에 사용하지 않는 것입니다. 이 빌드는 `dxil.dll`을 사용하지 않습니다. 빌드 시 GitHub 통신, `curl`·`tar`, Visual Studio 2022 17.11 이상의
  `cl.exe`, ATL 정적 라이브러리(`atls.lib`)가 필요합니다.

<a id="フォントとアイコン"></a>

## 글꼴과 아이콘

`epaint_default_fonts 0.36.2`의 원문은
[발행 시점의 fonts 디렉터리](https://github.com/emilk/egui/tree/49682f8baa058bf49e011035cfbd6e825f88a5ef/crates/epaint_default_fonts/fonts)에 있습니다.
글꼴을 변경하지 않고 포함하는 것을 전제로 다음 전문을 수집합니다.

| 글꼴 | 라이선스와 원문 |
|---|---|
| Noto Emoji | OFL-1.1, `fonts/OFL.txt` |
| Ubuntu Light | Ubuntu Font Licence 1.0, `fonts/UFL.txt` |
| Hack | MIT, DejaVu의 퍼블릭 도메인 고지, Bitstream Vera, `fonts/Hack-Regular.txt` |
| emoji-icon-font | MIT, `fonts/emoji-icon-font-mit-license.txt` |

기본 글꼴의 OFL-1.1과 Ubuntu Font Licence는 각 글꼴의 저작권·이름·조건을 원문 그대로 전문 묶음에 포함합니다.
글꼴을 변경해 배포하는 경우에는 예약된 글꼴 이름 등의 조건을 다시 확인합니다.
Fluent UI System Icons와 Phosphor Icons는 MIT입니다.
[기존 아이콘 고지](https://github.com/YozoraKurage/YoluPainter/blob/main/crates/yolu-app/assets/icons/THIRD-PARTY-NOTICES.md)도 app 전문 묶음에 포함합니다.
아이콘 120개는 아래 표의 크레이트 수에 포함하지 않습니다. 각 PNG와 Fluent·Phosphor 원본 이름·두께의 대응은 위 고지에 기재합니다.
Phosphor는 브러시·그라데이션·사각형 선택·타원 선택·다각형 선택·선택 펜·ID 선택·앵커·클리핑·생성기와 스냅 아이콘의 자석에 사용합니다.

<a id="同梱の画面のフォントbiz-udpgothic"></a>

### 내장 화면 글꼴(BIZ UDPGothic)

`yolu-app`은 화면 글꼴로 **BIZ UDPGothic**(Regular와 Bold. 모리사와의 BIZ UD 고딕을 Google Fonts에서 공개한 것)을 실행 파일에
내장하며 모든 OS에서 같은 글꼴을 사용합니다. 라이선스는 **SIL Open Font License 1.1**입니다.
배포 출처는 [googlefonts/morisawa-biz-ud-gothic](https://github.com/googlefonts/morisawa-biz-ud-gothic)의 릴리스 v1.051
(커밋 `18934af56b9c003ca58c54bffbf226848cb11032`)에 있는 TTF이며, Regular와 Bold는 변경하지 않고 `crates/yolu-app/assets/fonts/`에 둡니다.
라이선스 전문은 같은 위치의 `OFL.txt`(배포 출처의 같은 커밋 원문)와
[글꼴 제3자 고지](https://github.com/YozoraKurage/YoluPainter/blob/main/crates/yolu-app/assets/fonts/THIRD-PARTY-NOTICES.md)에 보관하고 app 전문 묶음에도 포함합니다. 글꼴은 크레이트가 아니므로 위 개수에 포함하지 않습니다
(아이콘과 같은 처리).

| 포함 파일 | SHA-256 |
|---|---|
| `BIZUDPGothic-Regular.ttf` | `595dfefa96dcb281c40f5d560841f3fc623b0d9d12064481a7daf78044462673` |
| `BIZUDPGothic-Bold.ttf` | `3a1adc5c062064d4e6fe20df1368ffd52d6bf969d5e0ac90f4083da8cd5597b8` |
| `BIZUDPGothic-Regular-Lines.ttf` | `75f2740ab18211e3e79c3373c11e3e3b187389a9942d78f0fecb8cc08eb509a8` |
| `BIZUDPGothic-Bold-Lines.ttf` | `8287377180647c950182bb0d078e71753bf5a80fb06bdce9fa767d17499c1170` |
| `OFL.txt` | `e753d7155d53c747d037a445e584c8ecfca6dd79846db610417e282a736b28bc` |

`*-Lines.ttf`는 Regular와 Bold에서 밑줄(_)·대시(– — ―)·빼기 기호(−)·윗줄(‾ ¯) 등 9개 글자의 윤곽만 힌팅 명령을 제거하여 추출한
파생 글꼴(각각 약 2.4 KB)이며, `tools/make-ui-font-lines.py`(fontTools 4.65.0)로 다시 만들 수 있습니다. 원본 명령은 가는 막대의 위아래 가장자리를 픽셀 격자에 각각 반올림하여
12 pt의 `_` 등을 두께 0으로 만들어 그려지지 않게 하므로, 화면 글꼴의 맨 앞에 두어 이 글자만 명령 없이 그립니다.
`OFL.txt`의 저작권 고지에는 예약된 글꼴 이름 선언이 없습니다. 원본의 `BIZ UDPGothic`은 Morisawa Inc.의 상표이므로 파생 글꼴 이름을 `YoluPainter UI Lines`로 바꾸고,
저작권 고지·라이선스 문구·상표 고지는 name 테이블에 유지하여 같은 SIL Open Font License 1.1로 포함합니다.

글꼴·아이콘·브러시 끝의 각 파일과 고지의 SHA-256 및 라이선스 종류는 `tools/licenses-reviewed.json`의 `bundled`에 고정하며, `tools/third-party.py`가 app을 대조할 때마다 확인합니다
(확인된 파일의 변경·누락, 대상 폴더에 미등록 파일 추가 시 전문 묶음을 만들지 않음). 글꼴을 변경해 배포하는 경우에는 예약된 글꼴 이름 등의 조건을 다시 확인합니다.

`libz-sys`의 Rust 부분은 MIT를 선택하며, 함께 제공하는 zlib의 Zlib 라이선스도 포함합니다.
`unicode-ident`의 Unicode-3.0,
`tracing-core` 안의 spin MIT 원문도 수집합니다. 사용하지 않는 zlib contrib·zlib-ng는 대상에서 제외합니다.
Linux 전용 `rfd` 백엔드의 window_identifier MIT 원문도 보존합니다(공통 설정이므로 Windows 묶음에도 보수적으로 포함).

<a id="liltoon-の再現式の移植"></a>

## lilToon 재현(수식 이식)

3D 뷰의 lilToon 외형(`crates/yolu-app/src/view3d/shaders/liltoon/` 구성 요소)과 외형 설정 속성 이름·기본값
(`crates/yolu-app/src/look/liltoon.rs`)은 [lilToon](https://github.com/lilxyzw/lilToon) 2.3.4(**MIT**, Copyright (c) 2020-present lilxyzw)의
셰이더와 인스펙터에서 수식과 값을 옮긴 것입니다. lilToon 파일·텍스처는 함께 제공하지 않습니다. 조명 수식은 lilToon에 포함된
OpenLit Library 1.0.2(**CC0 1.0**)에서 옮겼습니다. 반짝이의 난수는 lilToon의 `lilHashRGB4`에서 가져왔으며, 원 출처는 Shadertoy의
“Simplest Fastest 2D Hash”(MdcfDj, **Unlicense**. 원문 앞부분 고지에서 확인)입니다. MIT 원문과 출처는 [재현의 제3자 고지](https://github.com/YozoraKurage/YoluPainter/blob/main/crates/yolu-app/src/view3d/shaders/THIRD-PARTY-NOTICES.md)에
있으며, `tools/licenses-reviewed.json`의 `bundled`로 대조하여 app 전문 묶음에 포함합니다. 크레이트가 아니므로 위 개수에는 포함하지 않습니다.

<a id="グラデーションの混色oklab"></a>

## 그라데이션 혼색(Oklab)

그라데이션 맵의 “지각적” 혼색(`crates/yolu-core/src/generator/mixing.rs`)은 Oklab 색 공간(Björn Ottosson, 2020)의 sRGB 변환 수식
(<https://bottosson.github.io/posts/oklab/>. 작성자가 수식과 참조 코드를 **MIT**(Copyright (c) 2020 Björn Ottosson)로 공개)의 계수를 사용합니다.
변환 행렬 계수만 사용하며 참조 코드는 옮기지 않았습니다. 크레이트가 아니므로 위 개수에는 포함하지 않습니다. 휘도 보정 수식(줄어든 채도에 비례해 밝기를 높임)은 이 앱의 자체 수식입니다.

<a id="生成テストに使うツール"></a>

## 생성·테스트에 사용하는 도구

라이선스 전문 생성 도구는 Python 3.10 이상 표준 라이브러리만 사용하며, cargo-about / cargo-deny / pip 추가 패키지는 필요하지 않습니다.
아래는 개발 환경에서 사용하는 도구의 라이선스이며, 도구 본체는 제품에 포함하지 않습니다.

| 도구 | 라이선스·참조 출처 |
|---|---|
| Python | PSF License 및 부수 라이선스. [Python 고지](https://docs.python.org/3/license.html) |
| Rust / Cargo | 주로 MIT OR Apache-2.0. [Rust](https://github.com/rust-lang/rust/blob/master/COPYRIGHT)·[Cargo](https://github.com/rust-lang/cargo/blob/master/LICENSE-MIT)의 제3자 고지도 참조 |
| Bash | GPL-3.0-or-later. [Bash](https://www.gnu.org/software/bash/) |
| Wine | LGPL-2.1-or-later. 사용 환경의 `/usr/share/doc/wine/copyright` 및 [상위 프로젝트의 COPYING.LIB](https://gitlab.winehq.org/wine/wine/-/blob/master/COPYING.LIB) |
| MinGW-w64의 GCC·binutils | 도구 본체는 GPL 계열. 런타임은 별도의 라이선스·예외가 있으므로 [MinGW-w64](https://www.mingw-w64.org/) 및 각 설치의 copyright 확인 |

<a id="同梱の筆先とブラシ形式"></a>

## 내장 브러시 끝과 브러시 형식

`crates/yolu-brush-sets/data/krita4/brushes/`는 Krita의 `Krita_4_Default_Resources.bundle`에 있는 브러시 끝 76개(`.png`·`.gih`·`.gbr`)를
1바이트도 바꾸지 않고 보관합니다. 원본 묶음의 `meta.xml`(**CC0 1.0** 라이선스 선언)과 가져올 때 만든
`SHA256SUMS`·출처 및 묶음의 SHA-256을 적은 `README.md`는 `crates/yolu-brush-sets/data/krita4/`에 있습니다.
작성자 표시(CC0에서는 필수 아님): David Revoy(Deevad) 및 Ramon Miranda·Razvanc·Radian·Wolthera·Storm·Scottyp 등의 파생 작업, Krita 프로젝트.
Krita 프리셋(`.kpp`)·SVG 브러시 끝·패턴은 포함하지 않았습니다. 이 앱의 설정(반경·간격 등)은 자체 설정입니다.
[브러시 끝 제3자 고지](https://github.com/YozoraKurage/YoluPainter/blob/main/crates/yolu-app/assets/brushes/THIRD-PARTY-NOTICES.md)에 출처·작성자·원래 라이선스 선언과 CC0 전문을 모았으며, app 전문 묶음에도 포함합니다.
`tools/licenses-reviewed.json`으로 브러시 끝 76개·원래 선언·가져오기 기록·고지의 해시를 대조합니다.
로고를 제외한 앱 내장 이미지는 이 브러시 끝과 앞서 설명한 아이콘이며, 테스트의 fixtures·snapshots는 배포에 포함하지 않습니다.

브러시 형식 리더는 공개된 형식 설명을 바탕으로 작성했습니다. 다른 프로젝트의 코드는 사용하지 않았습니다:
GIMP의 `gbr.txt`·`gih.txt`·`vbr.txt`(devel-docs), Photoshop File Formats Specification(버전 1·2의 브러시, 디스크립터, 패턴),
버전 6 이후의 `samp`·`desc` 배열에 대한 공개 설명.

CLIP STUDIO PAINT의 `.sut`(SQLite) 리더 역시 공개 분석에서 테이블·열·BLOB 형식이라는 사실만 가져와 작성했습니다(코드는 옮기지 않음):
Kenneth Evans의 VS-CSPBrushInfo(MIT. `Node`·`Variant`·영향 원인과 소재 참조의 BLOB), Leon Schoenbrunn의 CSP2PC(MIT. 열 이름),
Mari Paint 가져오기 기능의 주석 달린 소스와 조사 노트(Apache-2.0. 영향 원인 헤더·소재 PNG 위치·열 별칭), Brushfactory의 공개 설명
(`MaterialFile.FileData`가 비압축 tar라는 사실). AGPL-3.0인 CSPBrushExtract, GPL-3.0 변환기(Krita용), 라이선스 없는 구현은 읽지 않았습니다.
파악한 형식과 실제 파일로 확인하지 않은 부분은 [브러시 가져오기](docs/ko/BRUSH_IMPORT.md)에 적습니다.

SQLite 읽기에는 `rusqlite 0.40.2`(MIT)를 `bundled`·`serialize`·`limits`·`hooks` 기능으로 사용합니다. `libsqlite3-sys 0.38.2`(MIT)가
SQLite 3.53.2의 amalgamation(`sqlite3.c`·`sqlite3.h`)을 포함해 컴파일합니다. **SQLite는 퍼블릭 도메인**입니다(저작권 포기와 축복 문구가
`sqlite3.c` 맨 앞에 있으며, 라이선스 전문은 없음). 이 크레이트에는 SQLCipher 소스(`sqlcipher/`)도 들어 있지만
`bundled-sqlcipher` 기능은 활성화하지 않아 포함하지 않습니다. SQLite 확장 로드(`load_extension`)도 활성화하지 않습니다. 실행 시 의존성으로
`fallible-iterator 0.3.0`·`fallible-streaming-iterator 0.1.9`(둘 다 MIT/Apache-2.0 중 MIT)가 추가됩니다.
SQLite의 C 컴파일로 클린 빌드가 무거워집니다. 이 환경(gcc, 최적화 3, 1코어)에서는 약 40초입니다. 2회째부터는 재사용합니다.

| 확인한 소스(`libsqlite3-sys 0.38.2`에 포함) | SHA-256 |
|---|---|
| `sqlite3/sqlite3.c`(SQLite 3.53.2) | `0a409f1633283fa31a9126b11fbfd64a1991c5d30defad07e5745d4667f5e23d` |
| `sqlite3/sqlite3.h` | `9e69a1353a4288450b0d5239ede11fc7f1f4c8e5eb07491fc8317eacb5b7de7e` |
| `rusqlite`·`libsqlite3-sys`의 `LICENSE`(같은 MIT) | `c10c1f27337546471e5f7e4e97fdd398b35b9d4e126115dcd22de8d8e65abf6f` |

<a id="fbx-の読み込みufbx"></a>

## FBX 읽기(ufbx)

`yolu-model`이 사용하는 `ufbx 0.11.5`는 `MIT OR Unlicense` 중 **MIT**를 선택합니다.
크레이트에 LICENSE가 포함되어 있지 않으므로 `.cargo_vcs_info.json`이 가리키는 발행 시점 커밋
`856c76840977cd504f053bf17ab4948b675b981a`의
[ufbx-rust 원문](https://github.com/ufbx/ufbx-rust/blob/856c76840977cd504f053bf17ab4948b675b981a/LICENSE)을 사용합니다.

함께 제공하는 C ufbx 0.23.1도 MIT를 선택합니다. 위 커밋의
[sfs-deps.json.lock](https://github.com/ufbx/ufbx-rust/blob/856c76840977cd504f053bf17ab4948b675b981a/sfs-deps.json.lock)이 가리키는
`48d2bb4114905c51347a91b8b6a276563daaa3f1`의
[C 본체 원문](https://github.com/ufbx/ufbx/blob/48d2bb4114905c51347a91b8b6a276563daaa3f1/LICENSE)을 별도 출처로 수집합니다.
이 커밋의 `ufbx.c`·`ufbx.h`가 크레이트에 포함된 파일 및 ufbx-rust 발행 시점 커밋의 파일과 바이트가 일치함을 확인했습니다.

| 확인한 원문·소스 | SHA-256 |
|---|---|
| ufbx-rust와 C 본체의 LICENSE(둘 다 동일) | `0dd48ebadf52273c736256325c8f078c03c8bb4facee22a4122de0ad3f615391` |
| 포함된 `ufbx/ufbx.c` | `4a956c26a708e40d82ecb0aeaee54da709d5ba7c144c9642174970059bda7e1d` |
| 포함된 `ufbx/ufbx.h` | `f787be529af577efc04f3e3e735844a08556718dbb613d1211c2b313d72895dc` |

원문은 대체 라이선스인 Unlicense도 포함한 형태 그대로 보존하지만, 선택하는 조건은 MIT뿐입니다.
Rust와 C의 저작권 표시(2020 Samuli Raivio)를 전문 묶음에 포함하며, C 본체를 별도 크레이트로 중복 계산하지 않습니다.

<a id="配布対象別の照合"></a>

## 배포 대상별 대조

`cargo xtask bundle`과 마찬가지로 app은 `--include-update --include-cli`를 붙인 집합을 기재합니다.
`yolu-update`는 앱의 자동 업데이트로 이미 포함되어 있으며, 그 의존성도 app 목록에 들어 있습니다.
`yolu-cli`(`yolupainter-cli`)는 app과 같은 배포물(zip·tar.gz·설치 프로그램)에 들어가므로, 그 의존성도 app 목록과 전문 묶음에 포함하고,
`.mcpb`에는 `yolu-cli` 목록과 전문 묶음(`--package yolu-cli --built-with yolu-app`)을 넣습니다. 실행 파일은 `-p yolu-app -p yolu-cli`의 1회 빌드로 만들며, 2개 크레이트의 기능이 합쳐져 `yolu-cli` 단독 트리에는 없는 의존성(`libm`·`foldhash`)이 들어가므로 해당 빌드의 `yolu-cli` 하위 트리를 셉니다.
개발용 `xtask` 의존성은 뒷부분에 따로 적습니다.
이름이 같아도 버전이 다른 크레이트는 별개로 셉니다. 제품 간·대상 간 개수에는 중복이 있습니다.

```sh
python3 tools/third-party.py --target x86_64-pc-windows-msvc --package yolu-app --include-update --include-cli --bundle
python3 tools/third-party.py --target x86_64-pc-windows-gnu --package yolu-app --include-update --include-cli --bundle
python3 tools/third-party.py --target x86_64-pc-windows-msvc --package yolu-cli --built-with yolu-app --bundle
python3 tools/third-party.py --target x86_64-unknown-linux-gnu --package yolu-app --include-update --include-cli --bundle
# Linux の ARM（aarch64、試作）。x86_64 の Linux の部分集合で、新しい許諾は無い
python3 tools/third-party.py --target aarch64-unknown-linux-gnu --package yolu-app --include-update --include-cli --bundle
# macOS の universal は Apple Silicon と Intel の 2 つのターゲットの木の和（どちらか単独の aarch64-apple-darwin・x86_64-apple-darwin も指せる）。.mcpb は Windows だけなので cli の一覧は無い
python3 tools/third-party.py --target universal-apple-darwin --package yolu-app --include-update --include-cli --bundle
```

원문을 받은 뒤에는 `--offline`으로도 같은 대조가 가능합니다.
배포물에는 대상별 `THIRD_PARTY.md`를 `DEPENDENCIES.md`로, 전문을 `THIRD_PARTY_LICENSES.txt`로 포함합니다.
`LICENSE`·이 `THIRD_PARTY.md`·`README.md`·`README.en.md` 및 사용자용 `docs/`(일본어·영어. 개발·릴리스 절차는 제외)도 포함합니다.
zip·tar.gz와 설치 프로그램은 `xtask`의 공통 패키징 목록을 사용하며 NSIS에도 같은 문서를 넣습니다. [배포 절차](https://github.com/YozoraKurage/YoluPainter/blob/main/docs/RELEASING.md)와 일치합니다.
다른 대상의 목록을 재사용하지 않습니다. 결과는 `target/third-party/<target>/<クレート>/`에 출력합니다.

| 대상 | app(업데이트·명령줄 포함) | cli(app과 같은 빌드) | update 단독 | xtask | app 전문 묶음 |
|---|---:|---:|---:|---:|---|
| Windows MSVC | 262 | 132 | 31 | 48 | 생성 성공 |
| Windows GNU | 260 | 132 | 31 | 48 | 생성 성공 |
| Linux GNU | 328 | 131 | 31 | 53 | 생성 성공 |
| Linux ARM(aarch64) | 326 | 130 | 30 | 51 | 생성 성공 |
| macOS universal | 255 | — | — | — | 생성 성공 |

서명 검증에 사용하는 `ed25519-dalek`·`curve25519-dalek`·`subtle`은 BSD-3-Clause입니다.
업데이트·패키징 크레이트까지 포함해 원문을 대조했으며, 확인하지 않은 크레이트가 없음을 확인했습니다.

Linux ARM(aarch64, 시험 배포물)의 app 326개는 x86_64 Linux의 328개에서 x86_64에만 들어가는 `cpufeatures 0.3.1`과
`curve25519-dalek-derive 0.1.1`을 뺀 부분집합입니다. ARM에만 들어가는 크레이트는 없으며 새로운 라이선스 종류도 없습니다. LGPL protocol XML 처리는 x86_64 Linux와 같습니다.

Linux의 `wayland-protocols-plasma 0.3.12`와 `wayland-protocols-misc 0.3.12`는 크레이트 선언은 MIT이지만 함께 제공하는 protocol XML
(plasma의 `plasma-wayland-protocols/src/protocols/` 31개 중 17개 및 misc의 `protocols/server-decoration.xml`)은 **LGPL-2.1-or-later**입니다.
Linux 실행 파일에는 이 XML에서 생성한 바인딩(프로토콜·메시지 이름과 인수 형식)이 들어갑니다
(`cargo tree -i wayland-protocols-plasma -e features --target x86_64-unknown-linux-gnu`와 misc로 확인한 의존 경로: plasma는 `winit`의 Wayland 경로가 직접 가져오고,
misc는 `egui-winit`의 클립보드(`smithay-clipboard`)가 `smithay-client-toolkit 0.20`을 거쳐 가져옴. 접근성 구성 요소가 가져오는 것은 아님).
창은 X11로 열어 Wayland 경로를 사용하지 않지만, `eframe`의 `wayland` 기능(`crates/yolu-app/Cargo.toml`)이 `winit`의 Wayland 경로와
클립보드의 Wayland 지원을 활성화하므로 이 2개는 제외할 수 없습니다.
Linux 배포물의 `THIRD_PARTY_LICENSES.txt`에는 LGPL-2.1 전문(plasma의 `COPYING.LIB`)과 `server-decoration.xml`의 고지를 넣습니다.
LGPL 부분의 원본은 crates.io의 두 크레이트와 상위 프로젝트([plasma-wayland-protocols](https://invent.kde.org/libraries/plasma-wayland-protocols))에 있으며,
YoluPainter 소스는 이 저장소에 공개하므로 사용자가 해당 구성 요소를 교체하여 다시 빌드할 수 있습니다.

<a id="cargolock-全体とテスト専用の依存"></a>

## Cargo.lock 전체와 테스트 전용 의존성

`python3 tools/third-party.py --audit-lock --offline`은 Windows MSVC·Windows GNU·Linux GNU·Linux ARM(aarch64)·macOS(Apple Silicon과 Intel)의 workspace 전체를 조사합니다.
일반·빌드 의존성 외에 테스트 의존성의 원문도 대조하며, 다른 OS·비활성 기능의 의존성을 포함한 lock 전체 항목을
`target/third-party/lock-inventory.json`에 분류합니다. 오래된 등록·확인하지 않은 버전·미승인 조건이 있으면 종료 코드 1을 반환합니다.
배포용 대조와는 별개이며, 대상 외 크레이트를 승인된 것으로 취급하지 않습니다.

대조한 Cargo.lock SHA-256: `0daa72a781beb6e19705f5836f8aa24b3ff16b15c6ea6d6899968a0ff3398990`. 외부 크레이트 508개의 내역은 다음과 같습니다.

| 범위(6개 대상의 합집합, 이름이 같아도 버전이 다르면 별개) | 개수 | 결과 |
|---|---:|---|
| 일반·빌드 의존성 | 400 | 원문 대조. Linux protocol XML 2개는 LGPL-2.1-or-later 전문과 고지 포함 |
| 테스트 전용 | 12 | MIT 11개는 원문 대조. MPL-2.0 1개는 미승인 |
| 대상 외(다른 OS·현재 비활성 기능) | 96 | 선언과 대상 외라는 사실 기록. 배포용 원문 대조·승인은 하지 않음 |
| Cargo.lock에 없는 오래된 등록 | 0 | 삭제 없음 |

테스트 전용 목록(배포용 전문 묶음에는 포함하지 않음):

| 크레이트 | 버전 | 선언 | 선택·상태 |
|---|---|---|---|
| anyhow | 1.0.104 | MIT OR Apache-2.0 | MIT |
| colored | 2.2.0 | MPL-2.0 | 미승인 |
| dify | 0.8.0 | MIT | MIT |
| egui_kittest | 0.36.2 | MIT OR Apache-2.0 | MIT |
| getopts | 0.2.24 | MIT OR Apache-2.0 | MIT |
| is-docker | 0.2.0 | MIT | MIT |
| is-wsl | 0.4.0 | MIT | MIT |
| kittest | 0.4.0 | MIT OR Apache-2.0 | MIT |
| lazy_static | 1.5.1 | MIT OR Apache-2.0 | MIT |
| open | 5.4.4 | MIT | MIT |
| tempfile | 3.27.0 | MIT OR Apache-2.0 | MIT |
| windows-sys | 0.59.0 | MIT OR Apache-2.0 | MIT |

`egui_kittest → dify → colored 2.2.0`은 MPL-2.0입니다. 허용 목록에 자동 추가하지 않으며 원문과 미승인 이유만 기록합니다.
현재 제품 의존성에는 포함되지 않습니다. 테스트 전용 12개는 배포하지 않는 개발용 크레이트 `yolu-mock`(화면 구성 요소 촬영)이 일반 의존성으로 가져오므로, `--audit-lock` 출력에서는
“일반·빌드”(412개)로 셉니다. 표의 400개는 `yolu-mock`을 제외한 수이며, 배포하는 제품(`yolu-app`·`yolu-cli`)의 트리에는 12개 모두 들어가지 않습니다. 위 lock 감사는 이 1개 때문에 종료 코드 1을 반환합니다(Linux protocol XML 2개는 LGPL-2.1-or-later 전문과 고지를 포함하여 이미 승인됨).

전체 크레이트에 포함된 파일에서 GPL·LGPL·AGPL 표기도 검색했습니다(0.6.1의 Cargo.lock 496개와 이후 추가된 15개에서 `gpl` 부분 일치 및 `General Public License`를 대소문자 구분 없이 검색했으며, 단어로 일치한 것은 아래에 든 항목뿐이었음. 부분 일치만 한 항목은 `eggplant`·`SIGPLAN94`·`Placement`·`WindowsFilteringPlatform` 등 다른 단어나 식별자의 일부)(`rusqlite` 계열 4개와 `schemars` 계열 6개에는 없으며, `libsqlite3-sys`의 `sqlite3.c`에서 일치한 것은 Eggplant라는 단어). Linux에서 이미 알려진 2개 외에,
대상 외인 `orbclient 0.3.55`의 `res/unifont-license.txt`에 GPL-2.0-or-later와 글꼴 임베딩 예외가 있습니다.
현재 Windows·Linux 의존성에는 포함되지 않지만, 이 크레이트를 사용하는 대상·기능을 추가할 때는 판단이 필요합니다.
`self_cell`의 GPL은 Apache-2.0과 선택할 수 있는 조건이며, 대상 외인 `r-efi 5.3.0`·`6.0.0`의 LGPL은 MIT·Apache-2.0과 선택할 수 있는 조건입니다.
`colored` 원문의 GPL 언급은 MPL의 Secondary License 정의이며, GPL을 선택했다는 기록이 아닙니다.
`x11rb 0.13.2`의 `examples/record.rs`는 GPLv3 샘플을 바탕으로 했다는 고지가 있지만, 제품에서 사용하는 라이브러리에 이 examples는 포함하지 않습니다.
`nix 0.31.3`(개발용 `xtask`의 `zip`이 macOS에서만 가져옴)의 `test/test_kmod/hello_mod/hello.c`는 테스트용 커널 모듈이며 `GPL-2.0+ or MIT`라는 이중 표기가 있습니다.
라이브러리 빌드에는 들어가지 않으며 배포 제품의 의존성도 아닙니다.
다른 검색 결과는 LLVM 예외에서 GPLv2를 언급한 부분, `slotmap`의 호환성 설명, `vcpkg`의 테스트용 파일명 목록,
앞서 설명한 `tree_magic_mini`의 비활성 `with-gpl-data` 기능과 변경 기록입니다.
AGPL 표기는 발견하지 못했습니다. 문자열 검색만으로 해당 라이선스가 없음을 보장하는 것은 아닙니다.

<a id="windows-msvc-の製品別一覧"></a>

## Windows MSVC 제품별 목록

<a id="yolu-appyolu-update-と-yolu-cli-の依存を含む-の依存一覧"></a>

### yolu-app(yolu-update와 yolu-cli 의존성 포함) 의존성 목록

대상: `x86_64-pc-windows-msvc`, 일반 기능. Cargo.lock SHA-256: `0daa72a781beb6e19705f5836f8aa24b3ff16b15c6ea6d6899968a0ff3398990`.

외부 크레이트 262개(이름이 같아도 버전이 다르면 별개). 실행 시 216개.

빌드용·프로시저 매크로용도 누락을 방지하기 위해 전문 묶음에 포함합니다. 테스트용 의존성은 제외합니다.

| 선택한 라이선스(추가 조건 포함) | 개수 |
|---|---:|
| Apache-2.0 | 11 |
| Apache-2.0 AND MIT | 2 |
| BSD-3-Clause | 3 |
| BSL-1.0 | 3 |
| ISC | 1 |
| MIT | 234 |
| MIT AND NCSA | 1 |
| MIT AND OFL-1.1 AND Ubuntu-font-1.0 AND Bitstream-Vera | 1 |
| MIT AND Unicode-3.0 | 1 |
| MIT AND Zlib | 2 |
| Unicode-3.0 | 1 |
| Zlib | 2 |

상태: 크레이트 라이선스 대조 성공.

| 크레이트 | 버전 | 용도 | 선언된 라이선스 | 선택·추가 조건 | 확인 |
|---|---|---|---|---|---|
| accesskit | 0.24.1 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| accesskit_consumer | 0.35.0 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| accesskit_windows | 0.32.1 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| accesskit_winit | 0.32.2 | 실행 시 | Apache-2.0 | Apache-2.0 | 확인됨 |
| adler2 | 2.0.1 | 실행 시 | 0BSD OR MIT OR Apache-2.0 | MIT | 확인됨 |
| ahash | 0.8.12 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| allocator-api2 | 0.2.21 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| arboard | 3.6.1 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| arrayvec | 0.7.8 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| ash | 0.38.0+1.3.281 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| async-trait | 0.1.92 | 빌드·매크로용 | MIT OR Apache-2.0 | MIT | 확인됨 |
| atomic-waker | 1.1.2 | 실행 시 | Apache-2.0 OR MIT | MIT | 확인됨 |
| autocfg | 1.5.1 | 빌드·매크로용 | Apache-2.0 OR MIT | MIT | 확인됨 |
| base64 | 0.23.1 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| bevy_mikktspace | 1.0.0 | 실행 시 | Zlib AND (MIT OR Apache-2.0) | MIT AND Zlib | 확인됨 |
| bit-set | 0.10.0 | 실행 시 | Apache-2.0 OR MIT | MIT | 확인됨 |
| bit-vec | 0.9.1 | 실행 시 | Apache-2.0 OR MIT | MIT | 확인됨 |
| bitflags | 2.13.2 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| block-buffer | 0.10.4 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| bytemuck | 1.25.2 | 실행 시 | Zlib OR Apache-2.0 OR MIT | MIT | 확인됨 |
| bytemuck_derive | 1.12.1 | 빌드·매크로용 | Zlib OR Apache-2.0 OR MIT | MIT | 확인됨 |
| byteorder-lite | 0.1.0 | 실행 시 | Unlicense OR MIT | MIT | 확인됨 |
| bytes | 1.12.1 | 실행 시 | MIT | MIT | 확인됨 |
| cc | 1.6.0 | 빌드·매크로용 | MIT OR Apache-2.0 | MIT | 확인됨 |
| cfg-if | 1.0.5 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| cfg_aliases | 0.2.2 | 빌드·매크로용 | MIT | MIT | 확인됨 |
| chacha20 | 0.10.2 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| chrono | 0.4.45 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| clipboard-win | 5.4.1 | 실행 시 | BSL-1.0 | BSL-1.0 | 확인됨 |
| codespan-reporting | 0.13.1 | 실행 시 | Apache-2.0 | Apache-2.0 | 확인됨 |
| color | 0.3.3 | 실행 시 | Apache-2.0 OR MIT | MIT | 확인됨 |
| cpufeatures | 0.2.17 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| cpufeatures | 0.3.1 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| crc32fast | 1.5.2 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| crossbeam-deque | 0.8.8 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| crossbeam-epoch | 0.9.21 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| crossbeam-utils | 0.8.23 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| crypto-common | 0.1.7 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| cursor-icon | 1.2.0 | 실행 시 | MIT OR Apache-2.0 OR Zlib | MIT | 확인됨 |
| curve25519-dalek-derive | 0.1.1 | 빌드·매크로용 | MIT/Apache-2.0 | MIT | 확인됨 |
| curve25519-dalek | 4.1.3 | 실행 시 | BSD-3-Clause | BSD-3-Clause | 확인됨 |
| digest | 0.10.7 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| displaydoc | 0.2.7 | 빌드·매크로용 | MIT OR Apache-2.0 | MIT | 확인됨 |
| document-features | 0.2.12 | 빌드·매크로용 | MIT OR Apache-2.0 | MIT | 확인됨 |
| dpi | 0.1.2 | 실행 시 | Apache-2.0 AND MIT | Apache-2.0 AND MIT | 확인됨 |
| duplicate | 2.0.1 | 빌드·매크로용 | MIT OR Apache-2.0 | MIT | 확인됨 |
| dyn-clone | 1.0.20 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| ecolor | 0.36.2 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| ed25519-dalek | 2.2.0 | 실행 시 | BSD-3-Clause | BSD-3-Clause | 확인됨 |
| ed25519 | 2.2.3 | 실행 시 | Apache-2.0 OR MIT | MIT | 확인됨 |
| eframe | 0.36.2 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| egui-wgpu | 0.36.2 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| egui-winit | 0.36.2 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| egui | 0.36.2 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| egui_dock | 0.21.1 | 실행 시 | MIT | MIT | 확인됨 |
| either | 1.18.0 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| emath | 0.36.2 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| enumn | 0.1.14 | 빌드·매크로용 | MIT OR Apache-2.0 | MIT | 확인됨 |
| epaint | 0.36.2 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| epaint_default_fonts | 0.36.2 | 실행 시 | (MIT OR Apache-2.0) AND OFL-1.1 AND Ubuntu-font-1.0 | MIT AND OFL-1.1 AND Ubuntu-font-1.0 AND Bitstream-Vera | 확인됨 |
| equivalent | 1.0.2 | 실행 시 | Apache-2.0 OR MIT | MIT | 확인됨 |
| error-code | 3.4.0 | 실행 시 | BSL-1.0 | BSL-1.0 | 확인됨 |
| euclid | 0.22.14 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| fallible-iterator | 0.3.0 | 실행 시 | MIT/Apache-2.0 | MIT | 확인됨 |
| fallible-streaming-iterator | 0.1.9 | 실행 시 | MIT/Apache-2.0 | MIT | 확인됨 |
| fdeflate | 0.3.7 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| fearless_simd | 0.4.1 | 실행 시 | Apache-2.0 OR MIT | MIT | 확인됨 |
| find-msvc-tools | 0.1.14 | 빌드·매크로용 | MIT OR Apache-2.0 | MIT | 확인됨 |
| flate2 | 1.1.10 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| fluent-bundle | 0.16.0 | 실행 시 | Apache-2.0 OR MIT | MIT | 확인됨 |
| fluent-langneg | 0.13.1 | 실행 시 | Apache-2.0 OR MIT | MIT | 확인됨 |
| fluent-syntax | 0.12.0 | 실행 시 | Apache-2.0 OR MIT | MIT | 확인됨 |
| foldhash | 0.2.0 | 실행 시 | Zlib | Zlib | 확인됨 |
| font-types | 0.12.6 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| fontdb | 0.24.0 | 실행 시 | MIT | MIT | 확인됨 |
| futures-channel | 0.3.34 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| futures-core | 0.3.34 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| futures-executor | 0.3.34 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| futures-io | 0.3.34 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| futures-macro | 0.3.34 | 빌드·매크로용 | MIT OR Apache-2.0 | MIT | 확인됨 |
| futures-sink | 0.3.34 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| futures-task | 0.3.34 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| futures-util | 0.3.34 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| futures | 0.3.34 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| generic-array | 0.14.7 | 실행 시 | MIT | MIT | 확인됨 |
| getrandom | 0.4.3 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| gl_generator | 0.14.0 | 빌드·매크로용 | Apache-2.0 | Apache-2.0 | 확인됨 |
| glam | 0.33.12 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| glow | 0.17.0 | 실행 시 | MIT OR Apache-2.0 OR Zlib | MIT | 확인됨 |
| glutin_wgl_sys | 0.6.1 | 실행 시 | Apache-2.0 | Apache-2.0 | 확인됨 |
| gpu-allocator | 0.28.0 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| guillotiere | 0.7.0 | 실행 시 | MIT/Apache-2.0 | MIT | 확인됨 |
| half | 2.7.1 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| harfrust | 0.12.0 | 실행 시 | MIT | MIT | 확인됨 |
| hashbrown | 0.16.1 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| hashbrown | 0.17.1 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| heck | 0.5.0 | 빌드·매크로용 | MIT OR Apache-2.0 | MIT | 확인됨 |
| hex | 0.4.3 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| http-body-util | 0.1.5 | 실행 시 | MIT | MIT | 확인됨 |
| http-body | 1.1.0 | 실행 시 | MIT | MIT | 확인됨 |
| http | 1.5.0 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| httparse | 1.10.1 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| httpdate | 1.0.3 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| hyper-util | 0.1.21 | 실행 시 | MIT | MIT | 확인됨 |
| hyper | 1.12.0 | 실행 시 | MIT | MIT | 확인됨 |
| image | 0.25.10 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| indexmap | 2.14.2 | 실행 시 | Apache-2.0 OR MIT | MIT | 확인됨 |
| intl-memoizer | 0.5.3 | 실행 시 | Apache-2.0 OR MIT | MIT | 확인됨 |
| intl_pluralrules | 7.0.2 | 실행 시 | Apache-2.0/MIT | MIT | 확인됨 |
| itertools | 0.15.0 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| itoa | 1.0.18 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| khronos-egl | 6.0.0 | 실행 시 | MIT/Apache-2.0 | MIT | 확인됨 |
| khronos_api | 3.1.0 | 빌드·매크로용 | Apache-2.0 | Apache-2.0 | 확인됨 |
| kurbo | 0.13.1 | 실행 시 | Apache-2.0 OR MIT | MIT | 확인됨 |
| lhash | 1.1.0 | 빌드·매크로용 | BSL-1.0 | BSL-1.0 | 확인됨 |
| libc | 0.2.190 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| libloading | 0.8.9 | 실행 시 | ISC | ISC | 확인됨 |
| libm | 0.2.16 | 실행 시 | MIT | MIT | 확인됨 |
| libsqlite3-sys | 0.38.2 | 실행 시 | MIT | MIT | 확인됨 |
| libz-sys | 1.1.29 | 실행 시 | MIT OR Apache-2.0 | MIT AND Zlib | 확인됨 |
| linebender_resource_handle | 0.1.1 | 실행 시 | Apache-2.0 OR MIT | MIT | 확인됨 |
| litrs | 1.0.0 | 빌드·매크로용 | MIT OR Apache-2.0 | MIT | 확인됨 |
| lock_api | 0.4.14 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| log | 0.4.34 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| mach-dxcompiler-rs | 0.1.6+2026.09.16-48d5a66.1 | 실행 시 | MIT | MIT AND NCSA | 확인됨 |
| memchr | 2.8.3 | 실행 시 | Unlicense OR MIT | MIT | 확인됨 |
| memmap2 | 0.9.11 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| miniz_oxide | 0.8.9 | 실행 시 | MIT OR Zlib OR Apache-2.0 | MIT | 확인됨 |
| miniz_oxide | 0.9.1 | 실행 시 | MIT OR Zlib OR Apache-2.0 | MIT | 확인됨 |
| mio | 1.2.4 | 실행 시 | MIT | MIT | 확인됨 |
| moxcms | 0.8.1 | 실행 시 | BSD-3-Clause OR Apache-2.0 | Apache-2.0 | 확인됨 |
| naga-types | 30.0.1 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| naga | 30.0.1 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| nohash-hasher | 0.2.0 | 실행 시 | Apache-2.0 OR MIT | MIT | 확인됨 |
| num-traits | 0.2.19 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| once_cell | 1.21.4 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| ordered-float | 5.5.0 | 실행 시 | MIT | MIT | 확인됨 |
| parking_lot | 0.12.5 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| parking_lot_core | 0.9.12 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| paste | 1.0.15 | 빌드·매크로용 | MIT OR Apache-2.0 | MIT | 확인됨 |
| pastey | 0.2.3 | 빌드·매크로용 | MIT OR Apache-2.0 | MIT | 확인됨 |
| peniko | 0.6.1 | 실행 시 | Apache-2.0 OR MIT | MIT | 확인됨 |
| pin-project-lite | 0.2.17 | 실행 시 | Apache-2.0 OR MIT | MIT | 확인됨 |
| pkg-config | 0.3.34 | 빌드·매크로용 | MIT OR Apache-2.0 | MIT | 확인됨 |
| png | 0.18.1 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| pollster | 1.0.1 | 실행 시 | Apache-2.0/MIT | MIT | 확인됨 |
| polycool | 0.4.0 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| presser | 0.3.1 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| proc-macro2-diagnostics | 0.10.1 | 빌드·매크로용 | MIT/Apache-2.0 | MIT | 확인됨 |
| proc-macro2 | 1.0.107 | 빌드·매크로용 | MIT OR Apache-2.0 | MIT | 확인됨 |
| profiling | 1.0.18 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| pxfm | 0.1.30 | 실행 시 | BSD-3-Clause OR Apache-2.0 | Apache-2.0 | 확인됨 |
| quote | 1.0.47 | 빌드·매크로용 | MIT OR Apache-2.0 | MIT | 확인됨 |
| rand | 0.10.3 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| rand_core | 0.10.1 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| range-alloc | 0.1.5 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| raw-window-handle | 0.6.2 | 실행 시 | MIT OR Apache-2.0 OR Zlib | MIT | 확인됨 |
| rayon-core | 1.13.0 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| rayon | 1.12.0 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| read-fonts | 0.41.0 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| ref-cast-impl | 1.0.27 | 빌드·매크로용 | MIT OR Apache-2.0 | MIT | 확인됨 |
| ref-cast | 1.0.27 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| renderdoc-sys | 1.1.0 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| rfd | 0.17.2 | 실행 시 | MIT | MIT | 확인됨 |
| rmcp | 3.5.1 | 실행 시 | Apache-2.0 | Apache-2.0 AND MIT | 확인됨 |
| rusqlite | 0.40.2 | 실행 시 | MIT | MIT | 확인됨 |
| rustc-hash | 1.1.0 | 실행 시 | Apache-2.0/MIT | MIT | 확인됨 |
| rustc-hash | 2.1.3 | 실행 시 | Apache-2.0 OR MIT | MIT | 확인됨 |
| rustc_version | 0.4.1 | 빌드·매크로용 | MIT OR Apache-2.0 | MIT | 확인됨 |
| schemars | 1.2.2 | 실행 시 | MIT | MIT | 확인됨 |
| schemars_derive | 1.2.2 | 빌드·매크로용 | MIT | MIT | 확인됨 |
| scopeguard | 1.2.0 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| self_cell | 1.3.0 | 실행 시 | Apache-2.0 OR GPL-2.0-only | Apache-2.0 | 확인됨 |
| semver | 1.0.28 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| serde | 1.0.229 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| serde_core | 1.0.229 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| serde_derive | 1.0.229 | 빌드·매크로용 | MIT OR Apache-2.0 | MIT | 확인됨 |
| serde_derive_internals | 0.30.0 | 빌드·매크로용 | MIT OR Apache-2.0 | MIT | 확인됨 |
| serde_json | 1.0.151 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| serde_spanned | 1.1.1 | 빌드·매크로용 | MIT OR Apache-2.0 | MIT | 확인됨 |
| sha2 | 0.10.9 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| shlex | 2.0.1 | 빌드·매크로용 | MIT OR Apache-2.0 | MIT | 확인됨 |
| signature | 2.2.0 | 실행 시 | Apache-2.0 OR MIT | MIT | 확인됨 |
| simd-adler32 | 0.3.10 | 실행 시 | MIT | MIT | 확인됨 |
| skrifa | 0.44.0 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| slab | 0.4.12 | 실행 시 | MIT | MIT | 확인됨 |
| slotmap | 1.1.1 | 실행 시 | Zlib | Zlib | 확인됨 |
| smallvec | 1.16.2 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| smol_str | 0.2.2 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| socket2 | 0.6.5 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| spirv | 0.4.0+sdk-1.4.341.0 | 실행 시 | Apache-2.0 | Apache-2.0 | 확인됨 |
| sse-stream | 0.2.6 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| static_assertions | 1.1.0 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| subtle | 2.6.1 | 실행 시 | BSD-3-Clause | BSD-3-Clause | 확인됨 |
| syn | 2.0.119 | 빌드·매크로용 | MIT OR Apache-2.0 | MIT | 확인됨 |
| syn | 3.0.6 | 빌드·매크로용 | MIT OR Apache-2.0 | MIT | 확인됨 |
| thiserror-impl | 2.0.21 | 빌드·매크로용 | MIT OR Apache-2.0 | MIT | 확인됨 |
| thiserror | 2.0.21 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| tinystr | 0.8.4 | 실행 시 | Unicode-3.0 | Unicode-3.0 | 확인됨 |
| tinyvec | 1.13.3 | 실행 시 | Zlib OR Apache-2.0 OR MIT | MIT | 확인됨 |
| tokio-macros | 2.7.2 | 빌드·매크로용 | MIT | MIT | 확인됨 |
| tokio-stream | 0.1.19 | 실행 시 | MIT | MIT | 확인됨 |
| tokio-util | 0.7.19 | 실행 시 | MIT | MIT | 확인됨 |
| tokio | 1.53.2 | 실행 시 | MIT | MIT | 확인됨 |
| toml | 1.1.6+spec-1.1.0 | 빌드·매크로용 | MIT OR Apache-2.0 | MIT | 확인됨 |
| toml_datetime | 1.1.1+spec-1.1.0 | 빌드·매크로용 | MIT OR Apache-2.0 | MIT | 확인됨 |
| toml_parser | 1.1.3+spec-1.1.0 | 빌드·매크로용 | MIT OR Apache-2.0 | MIT | 확인됨 |
| tower-service | 0.3.3 | 실행 시 | MIT | MIT | 확인됨 |
| tracing-attributes | 0.1.31 | 빌드·매크로용 | MIT | MIT | 확인됨 |
| tracing-core | 0.1.36 | 실행 시 | MIT | MIT | 확인됨 |
| tracing | 0.1.44 | 실행 시 | MIT | MIT | 확인됨 |
| try-lock | 0.2.5 | 실행 시 | MIT | MIT | 확인됨 |
| type-map | 0.5.1 | 실행 시 | MIT/Apache-2.0 | MIT | 확인됨 |
| typenum | 1.20.1 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| ufbx | 0.11.5 | 실행 시 | MIT OR Unlicense | MIT | 확인됨 |
| unic-langid-impl | 0.9.6 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| unic-langid | 0.9.6 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| unicode-general-category | 1.1.0 | 실행 시 | Apache-2.0 | Apache-2.0 | 확인됨 |
| unicode-ident | 1.0.26 | 실행 시 | (MIT OR Apache-2.0) AND Unicode-3.0 | MIT AND Unicode-3.0 | 확인됨 |
| unicode-segmentation | 1.13.3 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| unicode-width | 0.2.2 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| uuid | 1.27.0 | 실행 시 | Apache-2.0 OR MIT | MIT | 확인됨 |
| vcpkg | 0.2.15 | 빌드·매크로용 | MIT/Apache-2.0 | MIT | 확인됨 |
| vello_common | 0.1.0 | 실행 시 | Apache-2.0 OR MIT | MIT | 확인됨 |
| vello_cpu | 0.1.0 | 실행 시 | Apache-2.0 OR MIT | MIT | 확인됨 |
| version_check | 0.9.5 | 빌드·매크로용 | MIT/Apache-2.0 | MIT | 확인됨 |
| want | 0.3.2 | 실행 시 | MIT | MIT | 확인됨 |
| web-time | 1.1.0 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| wgpu-core-deps-windows-linux-android | 30.0.1 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| wgpu-core | 30.0.1 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| wgpu-hal | 30.0.1 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| wgpu-naga-bridge | 30.0.1 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| wgpu-types | 30.0.1 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| wgpu | 30.0.1 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| windows-collections | 0.3.2 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| windows-core | 0.62.2 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| windows-future | 0.3.2 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| windows-implement | 0.60.2 | 빌드·매크로용 | MIT OR Apache-2.0 | MIT | 확인됨 |
| windows-interface | 0.59.3 | 빌드·매크로용 | MIT OR Apache-2.0 | MIT | 확인됨 |
| windows-link | 0.2.1 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| windows-numerics | 0.3.1 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| windows-result | 0.4.1 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| windows-strings | 0.5.1 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| windows-sys | 0.52.0 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| windows-sys | 0.60.2 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| windows-sys | 0.61.2 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| windows-targets | 0.52.6 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| windows-targets | 0.53.5 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| windows-threading | 0.2.1 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| windows | 0.62.2 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| windows_x86_64_msvc | 0.52.6 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| windows_x86_64_msvc | 0.53.1 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| winit | 0.30.13 | 실행 시 | Apache-2.0 | Apache-2.0 | 확인됨 |
| winnow | 1.0.4 | 빌드·매크로용 | MIT | MIT | 확인됨 |
| winresource | 0.1.31 | 빌드·매크로용 | MIT | MIT | 확인됨 |
| xml-rs | 0.8.29 | 빌드·매크로용 | MIT | MIT | 확인됨 |
| zerocopy-derive | 0.8.59 | 빌드·매크로용 | BSD-2-Clause OR Apache-2.0 OR MIT | MIT | 확인됨 |
| zerocopy | 0.8.59 | 실행 시 | BSD-2-Clause OR Apache-2.0 OR MIT | MIT | 확인됨 |
| zeroize | 1.9.0 | 실행 시 | Apache-2.0 OR MIT | MIT | 확인됨 |
| zmij | 1.0.23 | 실행 시 | MIT | MIT | 확인됨 |
| zune-core | 0.5.3 | 실행 시 | MIT OR Apache-2.0 OR Zlib | MIT | 확인됨 |
| zune-jpeg | 0.5.15 | 실행 시 | MIT OR Apache-2.0 OR Zlib | MIT | 확인됨 |

<a id="yolu-cli-の依存一覧"></a>

### yolu-cli 의존성 목록

대상: `x86_64-pc-windows-msvc`, 일반 기능. Cargo.lock SHA-256: `0daa72a781beb6e19705f5836f8aa24b3ff16b15c6ea6d6899968a0ff3398990`.

외부 크레이트 132개(이름이 같아도 버전이 다르면 별개). 실행 시 109개.

빌드용·프로시저 매크로용도 누락을 방지하기 위해 전문 묶음에 포함합니다. 테스트용 의존성은 제외합니다.

`yolu-app`과 같은 cargo 빌드(`-p yolu-app -p yolu-cli`)에서 기능이 합쳐진 `yolu-cli` 하위 트리입니다.

| 선택한 라이선스(추가 조건 포함) | 개수 |
|---|---:|
| Apache-2.0 AND MIT | 1 |
| MIT | 127 |
| MIT AND Unicode-3.0 | 1 |
| MIT AND Zlib | 1 |
| Zlib | 2 |

상태: 크레이트 라이선스 대조 성공.

| 크레이트 | 버전 | 용도 | 선언된 라이선스 | 선택·추가 조건 | 확인 |
|---|---|---|---|---|---|
| adler2 | 2.0.1 | 실행 시 | 0BSD OR MIT OR Apache-2.0 | MIT | 확인됨 |
| arrayvec | 0.7.8 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| async-trait | 0.1.92 | 빌드·매크로용 | MIT OR Apache-2.0 | MIT | 확인됨 |
| atomic-waker | 1.1.2 | 실행 시 | Apache-2.0 OR MIT | MIT | 확인됨 |
| autocfg | 1.5.1 | 빌드·매크로용 | Apache-2.0 OR MIT | MIT | 확인됨 |
| base64 | 0.23.1 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| bitflags | 2.13.2 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| block-buffer | 0.10.4 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| bytemuck | 1.25.2 | 실행 시 | Zlib OR Apache-2.0 OR MIT | MIT | 확인됨 |
| bytemuck_derive | 1.12.1 | 빌드·매크로용 | Zlib OR Apache-2.0 OR MIT | MIT | 확인됨 |
| bytes | 1.12.1 | 실행 시 | MIT | MIT | 확인됨 |
| cc | 1.6.0 | 빌드·매크로용 | MIT OR Apache-2.0 | MIT | 확인됨 |
| cfg-if | 1.0.5 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| chacha20 | 0.10.2 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| chrono | 0.4.45 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| color | 0.3.3 | 실행 시 | Apache-2.0 OR MIT | MIT | 확인됨 |
| cpufeatures | 0.2.17 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| cpufeatures | 0.3.1 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| crc32fast | 1.5.2 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| crossbeam-deque | 0.8.8 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| crossbeam-epoch | 0.9.21 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| crossbeam-utils | 0.8.23 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| crypto-common | 0.1.7 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| digest | 0.10.7 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| dyn-clone | 1.0.20 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| either | 1.18.0 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| equivalent | 1.0.2 | 실행 시 | Apache-2.0 OR MIT | MIT | 확인됨 |
| euclid | 0.22.14 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| fallible-iterator | 0.3.0 | 실행 시 | MIT/Apache-2.0 | MIT | 확인됨 |
| fallible-streaming-iterator | 0.1.9 | 실행 시 | MIT/Apache-2.0 | MIT | 확인됨 |
| fdeflate | 0.3.7 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| fearless_simd | 0.4.1 | 실행 시 | Apache-2.0 OR MIT | MIT | 확인됨 |
| find-msvc-tools | 0.1.14 | 빌드·매크로용 | MIT OR Apache-2.0 | MIT | 확인됨 |
| flate2 | 1.1.10 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| foldhash | 0.2.0 | 실행 시 | Zlib | Zlib | 확인됨 |
| font-types | 0.12.6 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| fontdb | 0.24.0 | 실행 시 | MIT | MIT | 확인됨 |
| futures-channel | 0.3.34 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| futures-core | 0.3.34 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| futures-executor | 0.3.34 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| futures-io | 0.3.34 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| futures-macro | 0.3.34 | 빌드·매크로용 | MIT OR Apache-2.0 | MIT | 확인됨 |
| futures-sink | 0.3.34 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| futures-task | 0.3.34 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| futures-util | 0.3.34 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| futures | 0.3.34 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| generic-array | 0.14.7 | 실행 시 | MIT | MIT | 확인됨 |
| getrandom | 0.4.3 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| glam | 0.33.12 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| guillotiere | 0.7.0 | 실행 시 | MIT/Apache-2.0 | MIT | 확인됨 |
| harfrust | 0.12.0 | 실행 시 | MIT | MIT | 확인됨 |
| hashbrown | 0.17.1 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| http-body-util | 0.1.5 | 실행 시 | MIT | MIT | 확인됨 |
| http-body | 1.1.0 | 실행 시 | MIT | MIT | 확인됨 |
| http | 1.5.0 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| httparse | 1.10.1 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| httpdate | 1.0.3 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| hyper-util | 0.1.21 | 실행 시 | MIT | MIT | 확인됨 |
| hyper | 1.12.0 | 실행 시 | MIT | MIT | 확인됨 |
| indexmap | 2.14.2 | 실행 시 | Apache-2.0 OR MIT | MIT | 확인됨 |
| itoa | 1.0.18 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| kurbo | 0.13.1 | 실행 시 | Apache-2.0 OR MIT | MIT | 확인됨 |
| libm | 0.2.16 | 실행 시 | MIT | MIT | 확인됨 |
| libsqlite3-sys | 0.38.2 | 실행 시 | MIT | MIT | 확인됨 |
| libz-sys | 1.1.29 | 실행 시 | MIT OR Apache-2.0 | MIT AND Zlib | 확인됨 |
| linebender_resource_handle | 0.1.1 | 실행 시 | Apache-2.0 OR MIT | MIT | 확인됨 |
| log | 0.4.34 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| memchr | 2.8.3 | 실행 시 | Unlicense OR MIT | MIT | 확인됨 |
| memmap2 | 0.9.11 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| miniz_oxide | 0.8.9 | 실행 시 | MIT OR Zlib OR Apache-2.0 | MIT | 확인됨 |
| miniz_oxide | 0.9.1 | 실행 시 | MIT OR Zlib OR Apache-2.0 | MIT | 확인됨 |
| mio | 1.2.4 | 실행 시 | MIT | MIT | 확인됨 |
| num-traits | 0.2.19 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| once_cell | 1.21.4 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| pastey | 0.2.3 | 빌드·매크로용 | MIT OR Apache-2.0 | MIT | 확인됨 |
| peniko | 0.6.1 | 실행 시 | Apache-2.0 OR MIT | MIT | 확인됨 |
| pin-project-lite | 0.2.17 | 실행 시 | Apache-2.0 OR MIT | MIT | 확인됨 |
| pkg-config | 0.3.34 | 빌드·매크로용 | MIT OR Apache-2.0 | MIT | 확인됨 |
| png | 0.18.1 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| polycool | 0.4.0 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| proc-macro2 | 1.0.107 | 빌드·매크로용 | MIT OR Apache-2.0 | MIT | 확인됨 |
| quote | 1.0.47 | 빌드·매크로용 | MIT OR Apache-2.0 | MIT | 확인됨 |
| rand | 0.10.3 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| rand_core | 0.10.1 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| rayon-core | 1.13.0 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| rayon | 1.12.0 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| read-fonts | 0.41.0 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| ref-cast-impl | 1.0.27 | 빌드·매크로용 | MIT OR Apache-2.0 | MIT | 확인됨 |
| ref-cast | 1.0.27 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| rmcp | 3.5.1 | 실행 시 | Apache-2.0 | Apache-2.0 AND MIT | 확인됨 |
| rusqlite | 0.40.2 | 실행 시 | MIT | MIT | 확인됨 |
| schemars | 1.2.2 | 실행 시 | MIT | MIT | 확인됨 |
| schemars_derive | 1.2.2 | 빌드·매크로용 | MIT | MIT | 확인됨 |
| serde | 1.0.229 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| serde_core | 1.0.229 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| serde_derive | 1.0.229 | 빌드·매크로용 | MIT OR Apache-2.0 | MIT | 확인됨 |
| serde_derive_internals | 0.30.0 | 빌드·매크로용 | MIT OR Apache-2.0 | MIT | 확인됨 |
| serde_json | 1.0.151 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| sha2 | 0.10.9 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| shlex | 2.0.1 | 빌드·매크로용 | MIT OR Apache-2.0 | MIT | 확인됨 |
| simd-adler32 | 0.3.10 | 실행 시 | MIT | MIT | 확인됨 |
| skrifa | 0.44.0 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| slab | 0.4.12 | 실행 시 | MIT | MIT | 확인됨 |
| slotmap | 1.1.1 | 실행 시 | Zlib | Zlib | 확인됨 |
| smallvec | 1.16.2 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| socket2 | 0.6.5 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| sse-stream | 0.2.6 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| syn | 2.0.119 | 빌드·매크로용 | MIT OR Apache-2.0 | MIT | 확인됨 |
| syn | 3.0.6 | 빌드·매크로용 | MIT OR Apache-2.0 | MIT | 확인됨 |
| thiserror-impl | 2.0.21 | 빌드·매크로용 | MIT OR Apache-2.0 | MIT | 확인됨 |
| thiserror | 2.0.21 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| tinyvec | 1.13.3 | 실행 시 | Zlib OR Apache-2.0 OR MIT | MIT | 확인됨 |
| tokio-macros | 2.7.2 | 빌드·매크로용 | MIT | MIT | 확인됨 |
| tokio-stream | 0.1.19 | 실행 시 | MIT | MIT | 확인됨 |
| tokio-util | 0.7.19 | 실행 시 | MIT | MIT | 확인됨 |
| tokio | 1.53.2 | 실행 시 | MIT | MIT | 확인됨 |
| tower-service | 0.3.3 | 실행 시 | MIT | MIT | 확인됨 |
| tracing-attributes | 0.1.31 | 빌드·매크로용 | MIT | MIT | 확인됨 |
| tracing-core | 0.1.36 | 실행 시 | MIT | MIT | 확인됨 |
| tracing | 0.1.44 | 실행 시 | MIT | MIT | 확인됨 |
| try-lock | 0.2.5 | 실행 시 | MIT | MIT | 확인됨 |
| typenum | 1.20.1 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| unicode-ident | 1.0.26 | 빌드·매크로용 | (MIT OR Apache-2.0) AND Unicode-3.0 | MIT AND Unicode-3.0 | 확인됨 |
| uuid | 1.27.0 | 실행 시 | Apache-2.0 OR MIT | MIT | 확인됨 |
| vcpkg | 0.2.15 | 빌드·매크로용 | MIT/Apache-2.0 | MIT | 확인됨 |
| vello_common | 0.1.0 | 실행 시 | Apache-2.0 OR MIT | MIT | 확인됨 |
| vello_cpu | 0.1.0 | 실행 시 | Apache-2.0 OR MIT | MIT | 확인됨 |
| version_check | 0.9.5 | 빌드·매크로용 | MIT/Apache-2.0 | MIT | 확인됨 |
| want | 0.3.2 | 실행 시 | MIT | MIT | 확인됨 |
| windows-link | 0.2.1 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| windows-sys | 0.61.2 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| zmij | 1.0.23 | 실행 시 | MIT | MIT | 확인됨 |

<a id="windows-gnu-の製品別一覧"></a>

## Windows GNU 제품별 목록

<a id="yolu-appyolu-update-と-yolu-cli-の依存を含む-の依存一覧-1"></a>

### yolu-app(yolu-update와 yolu-cli 의존성 포함) 의존성 목록

대상: `x86_64-pc-windows-gnu`, 일반 기능. Cargo.lock SHA-256: `0daa72a781beb6e19705f5836f8aa24b3ff16b15c6ea6d6899968a0ff3398990`.

외부 크레이트 260개(이름이 같아도 버전이 다르면 별개). 실행 시 215개.

빌드용·프로시저 매크로용도 누락을 방지하기 위해 전문 묶음에 포함합니다. 테스트용 의존성은 제외합니다.

| 선택한 라이선스(추가 조건 포함) | 개수 |
|---|---:|
| Apache-2.0 | 11 |
| Apache-2.0 AND MIT | 2 |
| BSD-3-Clause | 3 |
| BSL-1.0 | 2 |
| ISC | 1 |
| MIT | 234 |
| MIT AND OFL-1.1 AND Ubuntu-font-1.0 AND Bitstream-Vera | 1 |
| MIT AND Unicode-3.0 | 1 |
| MIT AND Zlib | 2 |
| Unicode-3.0 | 1 |
| Zlib | 2 |

상태: 크레이트 라이선스 대조 성공.

| 크레이트 | 버전 | 용도 | 선언된 라이선스 | 선택·추가 조건 | 확인 |
|---|---|---|---|---|---|
| accesskit | 0.24.1 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| accesskit_consumer | 0.35.0 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| accesskit_windows | 0.32.1 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| accesskit_winit | 0.32.2 | 실행 시 | Apache-2.0 | Apache-2.0 | 확인됨 |
| adler2 | 2.0.1 | 실행 시 | 0BSD OR MIT OR Apache-2.0 | MIT | 확인됨 |
| ahash | 0.8.12 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| allocator-api2 | 0.2.21 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| arboard | 3.6.1 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| arrayvec | 0.7.8 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| ash | 0.38.0+1.3.281 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| async-trait | 0.1.92 | 빌드·매크로용 | MIT OR Apache-2.0 | MIT | 확인됨 |
| atomic-waker | 1.1.2 | 실행 시 | Apache-2.0 OR MIT | MIT | 확인됨 |
| autocfg | 1.5.1 | 빌드·매크로용 | Apache-2.0 OR MIT | MIT | 확인됨 |
| base64 | 0.23.1 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| bevy_mikktspace | 1.0.0 | 실행 시 | Zlib AND (MIT OR Apache-2.0) | MIT AND Zlib | 확인됨 |
| bit-set | 0.10.0 | 실행 시 | Apache-2.0 OR MIT | MIT | 확인됨 |
| bit-vec | 0.9.1 | 실행 시 | Apache-2.0 OR MIT | MIT | 확인됨 |
| bitflags | 2.13.2 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| block-buffer | 0.10.4 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| bytemuck | 1.25.2 | 실행 시 | Zlib OR Apache-2.0 OR MIT | MIT | 확인됨 |
| bytemuck_derive | 1.12.1 | 빌드·매크로용 | Zlib OR Apache-2.0 OR MIT | MIT | 확인됨 |
| byteorder-lite | 0.1.0 | 실행 시 | Unlicense OR MIT | MIT | 확인됨 |
| bytes | 1.12.1 | 실행 시 | MIT | MIT | 확인됨 |
| cc | 1.6.0 | 빌드·매크로용 | MIT OR Apache-2.0 | MIT | 확인됨 |
| cfg-if | 1.0.5 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| cfg_aliases | 0.2.2 | 빌드·매크로용 | MIT | MIT | 확인됨 |
| chacha20 | 0.10.2 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| chrono | 0.4.45 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| clipboard-win | 5.4.1 | 실행 시 | BSL-1.0 | BSL-1.0 | 확인됨 |
| codespan-reporting | 0.13.1 | 실행 시 | Apache-2.0 | Apache-2.0 | 확인됨 |
| color | 0.3.3 | 실행 시 | Apache-2.0 OR MIT | MIT | 확인됨 |
| cpufeatures | 0.2.17 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| cpufeatures | 0.3.1 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| crc32fast | 1.5.2 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| crossbeam-deque | 0.8.8 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| crossbeam-epoch | 0.9.21 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| crossbeam-utils | 0.8.23 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| crypto-common | 0.1.7 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| cursor-icon | 1.2.0 | 실행 시 | MIT OR Apache-2.0 OR Zlib | MIT | 확인됨 |
| curve25519-dalek-derive | 0.1.1 | 빌드·매크로용 | MIT/Apache-2.0 | MIT | 확인됨 |
| curve25519-dalek | 4.1.3 | 실행 시 | BSD-3-Clause | BSD-3-Clause | 확인됨 |
| digest | 0.10.7 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| displaydoc | 0.2.7 | 빌드·매크로용 | MIT OR Apache-2.0 | MIT | 확인됨 |
| document-features | 0.2.12 | 빌드·매크로용 | MIT OR Apache-2.0 | MIT | 확인됨 |
| dpi | 0.1.2 | 실행 시 | Apache-2.0 AND MIT | Apache-2.0 AND MIT | 확인됨 |
| duplicate | 2.0.1 | 빌드·매크로용 | MIT OR Apache-2.0 | MIT | 확인됨 |
| dyn-clone | 1.0.20 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| ecolor | 0.36.2 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| ed25519-dalek | 2.2.0 | 실행 시 | BSD-3-Clause | BSD-3-Clause | 확인됨 |
| ed25519 | 2.2.3 | 실행 시 | Apache-2.0 OR MIT | MIT | 확인됨 |
| eframe | 0.36.2 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| egui-wgpu | 0.36.2 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| egui-winit | 0.36.2 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| egui | 0.36.2 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| egui_dock | 0.21.1 | 실행 시 | MIT | MIT | 확인됨 |
| either | 1.18.0 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| emath | 0.36.2 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| enumn | 0.1.14 | 빌드·매크로용 | MIT OR Apache-2.0 | MIT | 확인됨 |
| epaint | 0.36.2 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| epaint_default_fonts | 0.36.2 | 실행 시 | (MIT OR Apache-2.0) AND OFL-1.1 AND Ubuntu-font-1.0 | MIT AND OFL-1.1 AND Ubuntu-font-1.0 AND Bitstream-Vera | 확인됨 |
| equivalent | 1.0.2 | 실행 시 | Apache-2.0 OR MIT | MIT | 확인됨 |
| error-code | 3.4.0 | 실행 시 | BSL-1.0 | BSL-1.0 | 확인됨 |
| euclid | 0.22.14 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| fallible-iterator | 0.3.0 | 실행 시 | MIT/Apache-2.0 | MIT | 확인됨 |
| fallible-streaming-iterator | 0.1.9 | 실행 시 | MIT/Apache-2.0 | MIT | 확인됨 |
| fdeflate | 0.3.7 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| fearless_simd | 0.4.1 | 실행 시 | Apache-2.0 OR MIT | MIT | 확인됨 |
| find-msvc-tools | 0.1.14 | 빌드·매크로용 | MIT OR Apache-2.0 | MIT | 확인됨 |
| flate2 | 1.1.10 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| fluent-bundle | 0.16.0 | 실행 시 | Apache-2.0 OR MIT | MIT | 확인됨 |
| fluent-langneg | 0.13.1 | 실행 시 | Apache-2.0 OR MIT | MIT | 확인됨 |
| fluent-syntax | 0.12.0 | 실행 시 | Apache-2.0 OR MIT | MIT | 확인됨 |
| foldhash | 0.2.0 | 실행 시 | Zlib | Zlib | 확인됨 |
| font-types | 0.12.6 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| fontdb | 0.24.0 | 실행 시 | MIT | MIT | 확인됨 |
| futures-channel | 0.3.34 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| futures-core | 0.3.34 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| futures-executor | 0.3.34 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| futures-io | 0.3.34 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| futures-macro | 0.3.34 | 빌드·매크로용 | MIT OR Apache-2.0 | MIT | 확인됨 |
| futures-sink | 0.3.34 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| futures-task | 0.3.34 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| futures-util | 0.3.34 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| futures | 0.3.34 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| generic-array | 0.14.7 | 실행 시 | MIT | MIT | 확인됨 |
| getrandom | 0.4.3 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| gl_generator | 0.14.0 | 빌드·매크로용 | Apache-2.0 | Apache-2.0 | 확인됨 |
| glam | 0.33.12 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| glow | 0.17.0 | 실행 시 | MIT OR Apache-2.0 OR Zlib | MIT | 확인됨 |
| glutin_wgl_sys | 0.6.1 | 실행 시 | Apache-2.0 | Apache-2.0 | 확인됨 |
| gpu-allocator | 0.28.0 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| guillotiere | 0.7.0 | 실행 시 | MIT/Apache-2.0 | MIT | 확인됨 |
| half | 2.7.1 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| harfrust | 0.12.0 | 실행 시 | MIT | MIT | 확인됨 |
| hashbrown | 0.16.1 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| hashbrown | 0.17.1 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| heck | 0.5.0 | 빌드·매크로용 | MIT OR Apache-2.0 | MIT | 확인됨 |
| hex | 0.4.3 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| http-body-util | 0.1.5 | 실행 시 | MIT | MIT | 확인됨 |
| http-body | 1.1.0 | 실행 시 | MIT | MIT | 확인됨 |
| http | 1.5.0 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| httparse | 1.10.1 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| httpdate | 1.0.3 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| hyper-util | 0.1.21 | 실행 시 | MIT | MIT | 확인됨 |
| hyper | 1.12.0 | 실행 시 | MIT | MIT | 확인됨 |
| image | 0.25.10 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| indexmap | 2.14.2 | 실행 시 | Apache-2.0 OR MIT | MIT | 확인됨 |
| intl-memoizer | 0.5.3 | 실행 시 | Apache-2.0 OR MIT | MIT | 확인됨 |
| intl_pluralrules | 7.0.2 | 실행 시 | Apache-2.0/MIT | MIT | 확인됨 |
| itertools | 0.15.0 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| itoa | 1.0.18 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| khronos-egl | 6.0.0 | 실행 시 | MIT/Apache-2.0 | MIT | 확인됨 |
| khronos_api | 3.1.0 | 빌드·매크로용 | Apache-2.0 | Apache-2.0 | 확인됨 |
| kurbo | 0.13.1 | 실행 시 | Apache-2.0 OR MIT | MIT | 확인됨 |
| libc | 0.2.190 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| libloading | 0.8.9 | 실행 시 | ISC | ISC | 확인됨 |
| libm | 0.2.16 | 실행 시 | MIT | MIT | 확인됨 |
| libsqlite3-sys | 0.38.2 | 실행 시 | MIT | MIT | 확인됨 |
| libz-sys | 1.1.29 | 실행 시 | MIT OR Apache-2.0 | MIT AND Zlib | 확인됨 |
| linebender_resource_handle | 0.1.1 | 실행 시 | Apache-2.0 OR MIT | MIT | 확인됨 |
| litrs | 1.0.0 | 빌드·매크로용 | MIT OR Apache-2.0 | MIT | 확인됨 |
| lock_api | 0.4.14 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| log | 0.4.34 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| memchr | 2.8.3 | 실행 시 | Unlicense OR MIT | MIT | 확인됨 |
| memmap2 | 0.9.11 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| miniz_oxide | 0.8.9 | 실행 시 | MIT OR Zlib OR Apache-2.0 | MIT | 확인됨 |
| miniz_oxide | 0.9.1 | 실행 시 | MIT OR Zlib OR Apache-2.0 | MIT | 확인됨 |
| mio | 1.2.4 | 실행 시 | MIT | MIT | 확인됨 |
| moxcms | 0.8.1 | 실행 시 | BSD-3-Clause OR Apache-2.0 | Apache-2.0 | 확인됨 |
| naga-types | 30.0.1 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| naga | 30.0.1 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| nohash-hasher | 0.2.0 | 실행 시 | Apache-2.0 OR MIT | MIT | 확인됨 |
| num-traits | 0.2.19 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| once_cell | 1.21.4 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| ordered-float | 5.5.0 | 실행 시 | MIT | MIT | 확인됨 |
| parking_lot | 0.12.5 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| parking_lot_core | 0.9.12 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| paste | 1.0.15 | 빌드·매크로용 | MIT OR Apache-2.0 | MIT | 확인됨 |
| pastey | 0.2.3 | 빌드·매크로용 | MIT OR Apache-2.0 | MIT | 확인됨 |
| peniko | 0.6.1 | 실행 시 | Apache-2.0 OR MIT | MIT | 확인됨 |
| pin-project-lite | 0.2.17 | 실행 시 | Apache-2.0 OR MIT | MIT | 확인됨 |
| pkg-config | 0.3.34 | 빌드·매크로용 | MIT OR Apache-2.0 | MIT | 확인됨 |
| png | 0.18.1 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| pollster | 1.0.1 | 실행 시 | Apache-2.0/MIT | MIT | 확인됨 |
| polycool | 0.4.0 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| presser | 0.3.1 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| proc-macro2-diagnostics | 0.10.1 | 빌드·매크로용 | MIT/Apache-2.0 | MIT | 확인됨 |
| proc-macro2 | 1.0.107 | 빌드·매크로용 | MIT OR Apache-2.0 | MIT | 확인됨 |
| profiling | 1.0.18 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| pxfm | 0.1.30 | 실행 시 | BSD-3-Clause OR Apache-2.0 | Apache-2.0 | 확인됨 |
| quote | 1.0.47 | 빌드·매크로용 | MIT OR Apache-2.0 | MIT | 확인됨 |
| rand | 0.10.3 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| rand_core | 0.10.1 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| range-alloc | 0.1.5 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| raw-window-handle | 0.6.2 | 실행 시 | MIT OR Apache-2.0 OR Zlib | MIT | 확인됨 |
| rayon-core | 1.13.0 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| rayon | 1.12.0 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| read-fonts | 0.41.0 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| ref-cast-impl | 1.0.27 | 빌드·매크로용 | MIT OR Apache-2.0 | MIT | 확인됨 |
| ref-cast | 1.0.27 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| renderdoc-sys | 1.1.0 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| rfd | 0.17.2 | 실행 시 | MIT | MIT | 확인됨 |
| rmcp | 3.5.1 | 실행 시 | Apache-2.0 | Apache-2.0 AND MIT | 확인됨 |
| rusqlite | 0.40.2 | 실행 시 | MIT | MIT | 확인됨 |
| rustc-hash | 1.1.0 | 실행 시 | Apache-2.0/MIT | MIT | 확인됨 |
| rustc-hash | 2.1.3 | 실행 시 | Apache-2.0 OR MIT | MIT | 확인됨 |
| rustc_version | 0.4.1 | 빌드·매크로용 | MIT OR Apache-2.0 | MIT | 확인됨 |
| schemars | 1.2.2 | 실행 시 | MIT | MIT | 확인됨 |
| schemars_derive | 1.2.2 | 빌드·매크로용 | MIT | MIT | 확인됨 |
| scopeguard | 1.2.0 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| self_cell | 1.3.0 | 실행 시 | Apache-2.0 OR GPL-2.0-only | Apache-2.0 | 확인됨 |
| semver | 1.0.28 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| serde | 1.0.229 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| serde_core | 1.0.229 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| serde_derive | 1.0.229 | 빌드·매크로용 | MIT OR Apache-2.0 | MIT | 확인됨 |
| serde_derive_internals | 0.30.0 | 빌드·매크로용 | MIT OR Apache-2.0 | MIT | 확인됨 |
| serde_json | 1.0.151 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| serde_spanned | 1.1.1 | 빌드·매크로용 | MIT OR Apache-2.0 | MIT | 확인됨 |
| sha2 | 0.10.9 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| shlex | 2.0.1 | 빌드·매크로용 | MIT OR Apache-2.0 | MIT | 확인됨 |
| signature | 2.2.0 | 실행 시 | Apache-2.0 OR MIT | MIT | 확인됨 |
| simd-adler32 | 0.3.10 | 실행 시 | MIT | MIT | 확인됨 |
| skrifa | 0.44.0 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| slab | 0.4.12 | 실행 시 | MIT | MIT | 확인됨 |
| slotmap | 1.1.1 | 실행 시 | Zlib | Zlib | 확인됨 |
| smallvec | 1.16.2 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| smol_str | 0.2.2 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| socket2 | 0.6.5 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| spirv | 0.4.0+sdk-1.4.341.0 | 실행 시 | Apache-2.0 | Apache-2.0 | 확인됨 |
| sse-stream | 0.2.6 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| static_assertions | 1.1.0 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| subtle | 2.6.1 | 실행 시 | BSD-3-Clause | BSD-3-Clause | 확인됨 |
| syn | 2.0.119 | 빌드·매크로용 | MIT OR Apache-2.0 | MIT | 확인됨 |
| syn | 3.0.6 | 빌드·매크로용 | MIT OR Apache-2.0 | MIT | 확인됨 |
| thiserror-impl | 2.0.21 | 빌드·매크로용 | MIT OR Apache-2.0 | MIT | 확인됨 |
| thiserror | 2.0.21 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| tinystr | 0.8.4 | 실행 시 | Unicode-3.0 | Unicode-3.0 | 확인됨 |
| tinyvec | 1.13.3 | 실행 시 | Zlib OR Apache-2.0 OR MIT | MIT | 확인됨 |
| tokio-macros | 2.7.2 | 빌드·매크로용 | MIT | MIT | 확인됨 |
| tokio-stream | 0.1.19 | 실행 시 | MIT | MIT | 확인됨 |
| tokio-util | 0.7.19 | 실행 시 | MIT | MIT | 확인됨 |
| tokio | 1.53.2 | 실행 시 | MIT | MIT | 확인됨 |
| toml | 1.1.6+spec-1.1.0 | 빌드·매크로용 | MIT OR Apache-2.0 | MIT | 확인됨 |
| toml_datetime | 1.1.1+spec-1.1.0 | 빌드·매크로용 | MIT OR Apache-2.0 | MIT | 확인됨 |
| toml_parser | 1.1.3+spec-1.1.0 | 빌드·매크로용 | MIT OR Apache-2.0 | MIT | 확인됨 |
| tower-service | 0.3.3 | 실행 시 | MIT | MIT | 확인됨 |
| tracing-attributes | 0.1.31 | 빌드·매크로용 | MIT | MIT | 확인됨 |
| tracing-core | 0.1.36 | 실행 시 | MIT | MIT | 확인됨 |
| tracing | 0.1.44 | 실행 시 | MIT | MIT | 확인됨 |
| try-lock | 0.2.5 | 실행 시 | MIT | MIT | 확인됨 |
| type-map | 0.5.1 | 실행 시 | MIT/Apache-2.0 | MIT | 확인됨 |
| typenum | 1.20.1 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| ufbx | 0.11.5 | 실행 시 | MIT OR Unlicense | MIT | 확인됨 |
| unic-langid-impl | 0.9.6 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| unic-langid | 0.9.6 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| unicode-general-category | 1.1.0 | 실행 시 | Apache-2.0 | Apache-2.0 | 확인됨 |
| unicode-ident | 1.0.26 | 실행 시 | (MIT OR Apache-2.0) AND Unicode-3.0 | MIT AND Unicode-3.0 | 확인됨 |
| unicode-segmentation | 1.13.3 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| unicode-width | 0.2.2 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| uuid | 1.27.0 | 실행 시 | Apache-2.0 OR MIT | MIT | 확인됨 |
| vcpkg | 0.2.15 | 빌드·매크로용 | MIT/Apache-2.0 | MIT | 확인됨 |
| vello_common | 0.1.0 | 실행 시 | Apache-2.0 OR MIT | MIT | 확인됨 |
| vello_cpu | 0.1.0 | 실행 시 | Apache-2.0 OR MIT | MIT | 확인됨 |
| version_check | 0.9.5 | 빌드·매크로용 | MIT/Apache-2.0 | MIT | 확인됨 |
| want | 0.3.2 | 실행 시 | MIT | MIT | 확인됨 |
| web-time | 1.1.0 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| wgpu-core-deps-windows-linux-android | 30.0.1 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| wgpu-core | 30.0.1 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| wgpu-hal | 30.0.1 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| wgpu-naga-bridge | 30.0.1 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| wgpu-types | 30.0.1 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| wgpu | 30.0.1 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| windows-collections | 0.3.2 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| windows-core | 0.62.2 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| windows-future | 0.3.2 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| windows-implement | 0.60.2 | 빌드·매크로용 | MIT OR Apache-2.0 | MIT | 확인됨 |
| windows-interface | 0.59.3 | 빌드·매크로용 | MIT OR Apache-2.0 | MIT | 확인됨 |
| windows-link | 0.2.1 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| windows-numerics | 0.3.1 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| windows-result | 0.4.1 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| windows-strings | 0.5.1 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| windows-sys | 0.52.0 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| windows-sys | 0.60.2 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| windows-sys | 0.61.2 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| windows-targets | 0.52.6 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| windows-targets | 0.53.5 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| windows-threading | 0.2.1 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| windows | 0.62.2 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| windows_x86_64_gnu | 0.52.6 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| windows_x86_64_gnu | 0.53.1 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| winit | 0.30.13 | 실행 시 | Apache-2.0 | Apache-2.0 | 확인됨 |
| winnow | 1.0.4 | 빌드·매크로용 | MIT | MIT | 확인됨 |
| winresource | 0.1.31 | 빌드·매크로용 | MIT | MIT | 확인됨 |
| xml-rs | 0.8.29 | 빌드·매크로용 | MIT | MIT | 확인됨 |
| zerocopy-derive | 0.8.59 | 빌드·매크로용 | BSD-2-Clause OR Apache-2.0 OR MIT | MIT | 확인됨 |
| zerocopy | 0.8.59 | 실행 시 | BSD-2-Clause OR Apache-2.0 OR MIT | MIT | 확인됨 |
| zeroize | 1.9.0 | 실행 시 | Apache-2.0 OR MIT | MIT | 확인됨 |
| zmij | 1.0.23 | 실행 시 | MIT | MIT | 확인됨 |
| zune-core | 0.5.3 | 실행 시 | MIT OR Apache-2.0 OR Zlib | MIT | 확인됨 |
| zune-jpeg | 0.5.15 | 실행 시 | MIT OR Apache-2.0 OR Zlib | MIT | 확인됨 |

<a id="yolu-cli-の依存一覧-1"></a>

### yolu-cli 의존성 목록

대상: `x86_64-pc-windows-gnu`, 일반 기능. Cargo.lock SHA-256: `0daa72a781beb6e19705f5836f8aa24b3ff16b15c6ea6d6899968a0ff3398990`.

외부 크레이트 132개(이름이 같아도 버전이 다르면 별개). 실행 시 109개.

빌드용·프로시저 매크로용도 누락을 방지하기 위해 전문 묶음에 포함합니다. 테스트용 의존성은 제외합니다.

`yolu-app`과 같은 cargo 빌드(`-p yolu-app -p yolu-cli`)에서 기능이 합쳐진 `yolu-cli` 하위 트리입니다.

| 선택한 라이선스(추가 조건 포함) | 개수 |
|---|---:|
| Apache-2.0 AND MIT | 1 |
| MIT | 127 |
| MIT AND Unicode-3.0 | 1 |
| MIT AND Zlib | 1 |
| Zlib | 2 |

상태: 크레이트 라이선스 대조 성공.

| 크레이트 | 버전 | 용도 | 선언된 라이선스 | 선택·추가 조건 | 확인 |
|---|---|---|---|---|---|
| adler2 | 2.0.1 | 실행 시 | 0BSD OR MIT OR Apache-2.0 | MIT | 확인됨 |
| arrayvec | 0.7.8 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| async-trait | 0.1.92 | 빌드·매크로용 | MIT OR Apache-2.0 | MIT | 확인됨 |
| atomic-waker | 1.1.2 | 실행 시 | Apache-2.0 OR MIT | MIT | 확인됨 |
| autocfg | 1.5.1 | 빌드·매크로용 | Apache-2.0 OR MIT | MIT | 확인됨 |
| base64 | 0.23.1 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| bitflags | 2.13.2 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| block-buffer | 0.10.4 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| bytemuck | 1.25.2 | 실행 시 | Zlib OR Apache-2.0 OR MIT | MIT | 확인됨 |
| bytemuck_derive | 1.12.1 | 빌드·매크로용 | Zlib OR Apache-2.0 OR MIT | MIT | 확인됨 |
| bytes | 1.12.1 | 실행 시 | MIT | MIT | 확인됨 |
| cc | 1.6.0 | 빌드·매크로용 | MIT OR Apache-2.0 | MIT | 확인됨 |
| cfg-if | 1.0.5 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| chacha20 | 0.10.2 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| chrono | 0.4.45 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| color | 0.3.3 | 실행 시 | Apache-2.0 OR MIT | MIT | 확인됨 |
| cpufeatures | 0.2.17 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| cpufeatures | 0.3.1 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| crc32fast | 1.5.2 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| crossbeam-deque | 0.8.8 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| crossbeam-epoch | 0.9.21 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| crossbeam-utils | 0.8.23 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| crypto-common | 0.1.7 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| digest | 0.10.7 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| dyn-clone | 1.0.20 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| either | 1.18.0 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| equivalent | 1.0.2 | 실행 시 | Apache-2.0 OR MIT | MIT | 확인됨 |
| euclid | 0.22.14 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| fallible-iterator | 0.3.0 | 실행 시 | MIT/Apache-2.0 | MIT | 확인됨 |
| fallible-streaming-iterator | 0.1.9 | 실행 시 | MIT/Apache-2.0 | MIT | 확인됨 |
| fdeflate | 0.3.7 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| fearless_simd | 0.4.1 | 실행 시 | Apache-2.0 OR MIT | MIT | 확인됨 |
| find-msvc-tools | 0.1.14 | 빌드·매크로용 | MIT OR Apache-2.0 | MIT | 확인됨 |
| flate2 | 1.1.10 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| foldhash | 0.2.0 | 실행 시 | Zlib | Zlib | 확인됨 |
| font-types | 0.12.6 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| fontdb | 0.24.0 | 실행 시 | MIT | MIT | 확인됨 |
| futures-channel | 0.3.34 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| futures-core | 0.3.34 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| futures-executor | 0.3.34 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| futures-io | 0.3.34 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| futures-macro | 0.3.34 | 빌드·매크로용 | MIT OR Apache-2.0 | MIT | 확인됨 |
| futures-sink | 0.3.34 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| futures-task | 0.3.34 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| futures-util | 0.3.34 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| futures | 0.3.34 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| generic-array | 0.14.7 | 실행 시 | MIT | MIT | 확인됨 |
| getrandom | 0.4.3 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| glam | 0.33.12 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| guillotiere | 0.7.0 | 실행 시 | MIT/Apache-2.0 | MIT | 확인됨 |
| harfrust | 0.12.0 | 실행 시 | MIT | MIT | 확인됨 |
| hashbrown | 0.17.1 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| http-body-util | 0.1.5 | 실행 시 | MIT | MIT | 확인됨 |
| http-body | 1.1.0 | 실행 시 | MIT | MIT | 확인됨 |
| http | 1.5.0 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| httparse | 1.10.1 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| httpdate | 1.0.3 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| hyper-util | 0.1.21 | 실행 시 | MIT | MIT | 확인됨 |
| hyper | 1.12.0 | 실행 시 | MIT | MIT | 확인됨 |
| indexmap | 2.14.2 | 실행 시 | Apache-2.0 OR MIT | MIT | 확인됨 |
| itoa | 1.0.18 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| kurbo | 0.13.1 | 실행 시 | Apache-2.0 OR MIT | MIT | 확인됨 |
| libm | 0.2.16 | 실행 시 | MIT | MIT | 확인됨 |
| libsqlite3-sys | 0.38.2 | 실행 시 | MIT | MIT | 확인됨 |
| libz-sys | 1.1.29 | 실행 시 | MIT OR Apache-2.0 | MIT AND Zlib | 확인됨 |
| linebender_resource_handle | 0.1.1 | 실행 시 | Apache-2.0 OR MIT | MIT | 확인됨 |
| log | 0.4.34 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| memchr | 2.8.3 | 실행 시 | Unlicense OR MIT | MIT | 확인됨 |
| memmap2 | 0.9.11 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| miniz_oxide | 0.8.9 | 실행 시 | MIT OR Zlib OR Apache-2.0 | MIT | 확인됨 |
| miniz_oxide | 0.9.1 | 실행 시 | MIT OR Zlib OR Apache-2.0 | MIT | 확인됨 |
| mio | 1.2.4 | 실행 시 | MIT | MIT | 확인됨 |
| num-traits | 0.2.19 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| once_cell | 1.21.4 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| pastey | 0.2.3 | 빌드·매크로용 | MIT OR Apache-2.0 | MIT | 확인됨 |
| peniko | 0.6.1 | 실행 시 | Apache-2.0 OR MIT | MIT | 확인됨 |
| pin-project-lite | 0.2.17 | 실행 시 | Apache-2.0 OR MIT | MIT | 확인됨 |
| pkg-config | 0.3.34 | 빌드·매크로용 | MIT OR Apache-2.0 | MIT | 확인됨 |
| png | 0.18.1 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| polycool | 0.4.0 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| proc-macro2 | 1.0.107 | 빌드·매크로용 | MIT OR Apache-2.0 | MIT | 확인됨 |
| quote | 1.0.47 | 빌드·매크로용 | MIT OR Apache-2.0 | MIT | 확인됨 |
| rand | 0.10.3 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| rand_core | 0.10.1 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| rayon-core | 1.13.0 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| rayon | 1.12.0 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| read-fonts | 0.41.0 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| ref-cast-impl | 1.0.27 | 빌드·매크로용 | MIT OR Apache-2.0 | MIT | 확인됨 |
| ref-cast | 1.0.27 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| rmcp | 3.5.1 | 실행 시 | Apache-2.0 | Apache-2.0 AND MIT | 확인됨 |
| rusqlite | 0.40.2 | 실행 시 | MIT | MIT | 확인됨 |
| schemars | 1.2.2 | 실행 시 | MIT | MIT | 확인됨 |
| schemars_derive | 1.2.2 | 빌드·매크로용 | MIT | MIT | 확인됨 |
| serde | 1.0.229 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| serde_core | 1.0.229 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| serde_derive | 1.0.229 | 빌드·매크로용 | MIT OR Apache-2.0 | MIT | 확인됨 |
| serde_derive_internals | 0.30.0 | 빌드·매크로용 | MIT OR Apache-2.0 | MIT | 확인됨 |
| serde_json | 1.0.151 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| sha2 | 0.10.9 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| shlex | 2.0.1 | 빌드·매크로용 | MIT OR Apache-2.0 | MIT | 확인됨 |
| simd-adler32 | 0.3.10 | 실행 시 | MIT | MIT | 확인됨 |
| skrifa | 0.44.0 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| slab | 0.4.12 | 실행 시 | MIT | MIT | 확인됨 |
| slotmap | 1.1.1 | 실행 시 | Zlib | Zlib | 확인됨 |
| smallvec | 1.16.2 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| socket2 | 0.6.5 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| sse-stream | 0.2.6 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| syn | 2.0.119 | 빌드·매크로용 | MIT OR Apache-2.0 | MIT | 확인됨 |
| syn | 3.0.6 | 빌드·매크로용 | MIT OR Apache-2.0 | MIT | 확인됨 |
| thiserror-impl | 2.0.21 | 빌드·매크로용 | MIT OR Apache-2.0 | MIT | 확인됨 |
| thiserror | 2.0.21 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| tinyvec | 1.13.3 | 실행 시 | Zlib OR Apache-2.0 OR MIT | MIT | 확인됨 |
| tokio-macros | 2.7.2 | 빌드·매크로용 | MIT | MIT | 확인됨 |
| tokio-stream | 0.1.19 | 실행 시 | MIT | MIT | 확인됨 |
| tokio-util | 0.7.19 | 실행 시 | MIT | MIT | 확인됨 |
| tokio | 1.53.2 | 실행 시 | MIT | MIT | 확인됨 |
| tower-service | 0.3.3 | 실행 시 | MIT | MIT | 확인됨 |
| tracing-attributes | 0.1.31 | 빌드·매크로용 | MIT | MIT | 확인됨 |
| tracing-core | 0.1.36 | 실행 시 | MIT | MIT | 확인됨 |
| tracing | 0.1.44 | 실행 시 | MIT | MIT | 확인됨 |
| try-lock | 0.2.5 | 실행 시 | MIT | MIT | 확인됨 |
| typenum | 1.20.1 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| unicode-ident | 1.0.26 | 빌드·매크로용 | (MIT OR Apache-2.0) AND Unicode-3.0 | MIT AND Unicode-3.0 | 확인됨 |
| uuid | 1.27.0 | 실행 시 | Apache-2.0 OR MIT | MIT | 확인됨 |
| vcpkg | 0.2.15 | 빌드·매크로용 | MIT/Apache-2.0 | MIT | 확인됨 |
| vello_common | 0.1.0 | 실행 시 | Apache-2.0 OR MIT | MIT | 확인됨 |
| vello_cpu | 0.1.0 | 실행 시 | Apache-2.0 OR MIT | MIT | 확인됨 |
| version_check | 0.9.5 | 빌드·매크로용 | MIT/Apache-2.0 | MIT | 확인됨 |
| want | 0.3.2 | 실행 시 | MIT | MIT | 확인됨 |
| windows-link | 0.2.1 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| windows-sys | 0.61.2 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| zmij | 1.0.23 | 실행 시 | MIT | MIT | 확인됨 |

<a id="linux-gnu-の製品別一覧"></a>

## Linux GNU 제품별 목록

<a id="yolu-appyolu-update-と-yolu-cli-の依存を含む-の依存一覧-2"></a>

### yolu-app(yolu-update와 yolu-cli 의존성 포함) 의존성 목록

대상: `x86_64-unknown-linux-gnu`, 일반 기능. Cargo.lock SHA-256: `0daa72a781beb6e19705f5836f8aa24b3ff16b15c6ea6d6899968a0ff3398990`.

외부 크레이트 328개(이름이 같아도 버전이 다르면 별개). 실행 시 279개.

빌드용·프로시저 매크로용도 누락을 방지하기 위해 전문 묶음에 포함합니다. 테스트용 의존성은 제외합니다.

| 선택한 라이선스(추가 조건 포함) | 개수 |
|---|---:|
| Apache-2.0 | 9 |
| Apache-2.0 AND MIT | 2 |
| BSD-3-Clause | 3 |
| ISC | 1 |
| MIT | 303 |
| MIT AND LGPL-2.1-or-later | 2 |
| MIT AND OFL-1.1 AND Ubuntu-font-1.0 AND Bitstream-Vera | 1 |
| MIT AND Unicode-3.0 | 1 |
| MIT AND Zlib | 2 |
| Unicode-3.0 | 1 |
| Zlib | 3 |

상태: 크레이트 라이선스 대조 성공.

| 크레이트 | 버전 | 용도 | 선언된 라이선스 | 선택·추가 조건 | 확인 |
|---|---|---|---|---|---|
| accesskit | 0.24.1 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| accesskit_atspi_common | 0.18.1 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| accesskit_consumer | 0.36.0 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| accesskit_unix | 0.21.1 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| accesskit_winit | 0.32.2 | 실행 시 | Apache-2.0 | Apache-2.0 | 확인됨 |
| adler2 | 2.0.1 | 실행 시 | 0BSD OR MIT OR Apache-2.0 | MIT | 확인됨 |
| ahash | 0.8.12 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| allocator-api2 | 0.2.21 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| arboard | 3.6.1 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| arrayvec | 0.7.8 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| as-raw-xcb-connection | 1.0.1 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| ash | 0.38.0+1.3.281 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| async-broadcast | 0.7.2 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| async-channel | 2.5.0 | 실행 시 | Apache-2.0 OR MIT | MIT | 확인됨 |
| async-executor | 1.14.0 | 실행 시 | Apache-2.0 OR MIT | MIT | 확인됨 |
| async-io | 2.6.0 | 실행 시 | Apache-2.0 OR MIT | MIT | 확인됨 |
| async-lock | 3.4.2 | 실행 시 | Apache-2.0 OR MIT | MIT | 확인됨 |
| async-process | 2.5.0 | 실행 시 | Apache-2.0 OR MIT | MIT | 확인됨 |
| async-recursion | 1.2.0 | 빌드·매크로용 | MIT OR Apache-2.0 | MIT | 확인됨 |
| async-signal | 0.2.14 | 실행 시 | Apache-2.0 OR MIT | MIT | 확인됨 |
| async-task | 4.7.1 | 실행 시 | Apache-2.0 OR MIT | MIT | 확인됨 |
| async-trait | 0.1.92 | 빌드·매크로용 | MIT OR Apache-2.0 | MIT | 확인됨 |
| atomic-waker | 1.1.2 | 실행 시 | Apache-2.0 OR MIT | MIT | 확인됨 |
| atspi-common | 0.13.0 | 실행 시 | Apache-2.0 OR MIT | MIT | 확인됨 |
| atspi-proxies | 0.13.0 | 실행 시 | Apache-2.0 OR MIT | MIT | 확인됨 |
| atspi | 0.29.0 | 실행 시 | Apache-2.0 OR MIT | MIT | 확인됨 |
| autocfg | 1.5.1 | 빌드·매크로용 | Apache-2.0 OR MIT | MIT | 확인됨 |
| base64 | 0.23.1 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| bevy_mikktspace | 1.0.0 | 실행 시 | Zlib AND (MIT OR Apache-2.0) | MIT AND Zlib | 확인됨 |
| bit-set | 0.10.0 | 실행 시 | Apache-2.0 OR MIT | MIT | 확인됨 |
| bit-vec | 0.9.1 | 실행 시 | Apache-2.0 OR MIT | MIT | 확인됨 |
| bitflags | 2.13.2 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| block-buffer | 0.10.4 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| blocking | 1.7.0 | 실행 시 | Apache-2.0 OR MIT | MIT | 확인됨 |
| bytemuck | 1.25.2 | 실행 시 | Zlib OR Apache-2.0 OR MIT | MIT | 확인됨 |
| bytemuck_derive | 1.12.1 | 빌드·매크로용 | Zlib OR Apache-2.0 OR MIT | MIT | 확인됨 |
| byteorder-lite | 0.1.0 | 실행 시 | Unlicense OR MIT | MIT | 확인됨 |
| bytes | 1.12.1 | 실행 시 | MIT | MIT | 확인됨 |
| calloop-wayland-source | 0.3.0 | 실행 시 | MIT | MIT | 확인됨 |
| calloop-wayland-source | 0.4.1 | 실행 시 | MIT | MIT | 확인됨 |
| calloop | 0.13.0 | 실행 시 | MIT | MIT | 확인됨 |
| calloop | 0.14.5 | 실행 시 | MIT | MIT | 확인됨 |
| cc | 1.6.0 | 빌드·매크로용 | MIT OR Apache-2.0 | MIT | 확인됨 |
| cfg-if | 1.0.5 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| cfg_aliases | 0.2.2 | 빌드·매크로용 | MIT | MIT | 확인됨 |
| chacha20 | 0.10.2 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| chrono | 0.4.45 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| codespan-reporting | 0.13.1 | 실행 시 | Apache-2.0 | Apache-2.0 | 확인됨 |
| color | 0.3.3 | 실행 시 | Apache-2.0 OR MIT | MIT | 확인됨 |
| concurrent-queue | 2.5.0 | 실행 시 | Apache-2.0 OR MIT | MIT | 확인됨 |
| cpufeatures | 0.2.17 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| cpufeatures | 0.3.1 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| crc32fast | 1.5.2 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| crossbeam-deque | 0.8.8 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| crossbeam-epoch | 0.9.21 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| crossbeam-utils | 0.8.23 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| crypto-common | 0.1.7 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| cursor-icon | 1.2.0 | 실행 시 | MIT OR Apache-2.0 OR Zlib | MIT | 확인됨 |
| curve25519-dalek-derive | 0.1.1 | 빌드·매크로용 | MIT/Apache-2.0 | MIT | 확인됨 |
| curve25519-dalek | 4.1.3 | 실행 시 | BSD-3-Clause | BSD-3-Clause | 확인됨 |
| digest | 0.10.7 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| displaydoc | 0.2.7 | 빌드·매크로용 | MIT OR Apache-2.0 | MIT | 확인됨 |
| dlib | 0.5.3 | 실행 시 | MIT | MIT | 확인됨 |
| document-features | 0.2.12 | 빌드·매크로용 | MIT OR Apache-2.0 | MIT | 확인됨 |
| downcast-rs | 1.2.1 | 실행 시 | MIT/Apache-2.0 | MIT | 확인됨 |
| dpi | 0.1.2 | 실행 시 | Apache-2.0 AND MIT | Apache-2.0 AND MIT | 확인됨 |
| duplicate | 2.0.1 | 빌드·매크로용 | MIT OR Apache-2.0 | MIT | 확인됨 |
| dyn-clone | 1.0.20 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| ecolor | 0.36.2 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| ed25519-dalek | 2.2.0 | 실행 시 | BSD-3-Clause | BSD-3-Clause | 확인됨 |
| ed25519 | 2.2.3 | 실행 시 | Apache-2.0 OR MIT | MIT | 확인됨 |
| eframe | 0.36.2 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| egui-wgpu | 0.36.2 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| egui-winit | 0.36.2 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| egui | 0.36.2 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| egui_dock | 0.21.1 | 실행 시 | MIT | MIT | 확인됨 |
| either | 1.18.0 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| emath | 0.36.2 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| endi | 1.1.1 | 실행 시 | MIT | MIT | 확인됨 |
| enumflags2 | 0.7.12 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| enumflags2_derive | 0.7.12 | 빌드·매크로용 | MIT OR Apache-2.0 | MIT | 확인됨 |
| enumn | 0.1.14 | 빌드·매크로용 | MIT OR Apache-2.0 | MIT | 확인됨 |
| epaint | 0.36.2 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| epaint_default_fonts | 0.36.2 | 실행 시 | (MIT OR Apache-2.0) AND OFL-1.1 AND Ubuntu-font-1.0 | MIT AND OFL-1.1 AND Ubuntu-font-1.0 AND Bitstream-Vera | 확인됨 |
| equivalent | 1.0.2 | 실행 시 | Apache-2.0 OR MIT | MIT | 확인됨 |
| errno | 0.3.14 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| euclid | 0.22.14 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| event-listener-strategy | 0.5.4 | 실행 시 | Apache-2.0 OR MIT | MIT | 확인됨 |
| event-listener | 5.4.2 | 실행 시 | Apache-2.0 OR MIT | MIT | 확인됨 |
| fallible-iterator | 0.3.0 | 실행 시 | MIT/Apache-2.0 | MIT | 확인됨 |
| fallible-streaming-iterator | 0.1.9 | 실행 시 | MIT/Apache-2.0 | MIT | 확인됨 |
| fastrand | 2.5.0 | 실행 시 | Apache-2.0 OR MIT | MIT | 확인됨 |
| fdeflate | 0.3.7 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| fearless_simd | 0.4.1 | 실행 시 | Apache-2.0 OR MIT | MIT | 확인됨 |
| find-msvc-tools | 0.1.14 | 빌드·매크로용 | MIT OR Apache-2.0 | MIT | 확인됨 |
| fixedbitset | 0.5.7 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| flate2 | 1.1.10 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| fluent-bundle | 0.16.0 | 실행 시 | Apache-2.0 OR MIT | MIT | 확인됨 |
| fluent-langneg | 0.13.1 | 실행 시 | Apache-2.0 OR MIT | MIT | 확인됨 |
| fluent-syntax | 0.12.0 | 실행 시 | Apache-2.0 OR MIT | MIT | 확인됨 |
| foldhash | 0.1.5 | 실행 시 | Zlib | Zlib | 확인됨 |
| foldhash | 0.2.0 | 실행 시 | Zlib | Zlib | 확인됨 |
| font-types | 0.12.6 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| fontdb | 0.24.0 | 실행 시 | MIT | MIT | 확인됨 |
| futures-channel | 0.3.34 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| futures-core | 0.3.34 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| futures-executor | 0.3.34 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| futures-io | 0.3.34 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| futures-lite | 2.6.1 | 실행 시 | Apache-2.0 OR MIT | MIT | 확인됨 |
| futures-macro | 0.3.34 | 빌드·매크로용 | MIT OR Apache-2.0 | MIT | 확인됨 |
| futures-sink | 0.3.34 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| futures-task | 0.3.34 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| futures-util | 0.3.34 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| futures | 0.3.34 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| generic-array | 0.14.7 | 실행 시 | MIT | MIT | 확인됨 |
| gethostname | 1.1.0 | 실행 시 | Apache-2.0 | Apache-2.0 | 확인됨 |
| getrandom | 0.3.4 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| getrandom | 0.4.3 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| glam | 0.33.12 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| glow | 0.17.0 | 실행 시 | MIT OR Apache-2.0 OR Zlib | MIT | 확인됨 |
| gpu-allocator | 0.28.0 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| guillotiere | 0.7.0 | 실행 시 | MIT/Apache-2.0 | MIT | 확인됨 |
| half | 2.7.1 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| harfrust | 0.12.0 | 실행 시 | MIT | MIT | 확인됨 |
| hashbrown | 0.15.5 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| hashbrown | 0.16.1 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| hashbrown | 0.17.1 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| heck | 0.5.0 | 빌드·매크로용 | MIT OR Apache-2.0 | MIT | 확인됨 |
| hex | 0.4.3 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| http-body-util | 0.1.5 | 실행 시 | MIT | MIT | 확인됨 |
| http-body | 1.1.0 | 실행 시 | MIT | MIT | 확인됨 |
| http | 1.5.0 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| httparse | 1.10.1 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| httpdate | 1.0.3 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| hyper-util | 0.1.21 | 실행 시 | MIT | MIT | 확인됨 |
| hyper | 1.12.0 | 실행 시 | MIT | MIT | 확인됨 |
| image | 0.25.10 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| indexmap | 2.14.2 | 실행 시 | Apache-2.0 OR MIT | MIT | 확인됨 |
| intl-memoizer | 0.5.3 | 실행 시 | Apache-2.0 OR MIT | MIT | 확인됨 |
| intl_pluralrules | 7.0.2 | 실행 시 | Apache-2.0/MIT | MIT | 확인됨 |
| itertools | 0.15.0 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| itoa | 1.0.18 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| khronos-egl | 6.0.0 | 실행 시 | MIT/Apache-2.0 | MIT | 확인됨 |
| kurbo | 0.13.1 | 실행 시 | Apache-2.0 OR MIT | MIT | 확인됨 |
| libc | 0.2.190 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| libloading | 0.8.9 | 실행 시 | ISC | ISC | 확인됨 |
| libm | 0.2.16 | 실행 시 | MIT | MIT | 확인됨 |
| libsqlite3-sys | 0.38.2 | 실행 시 | MIT | MIT | 확인됨 |
| libz-sys | 1.1.29 | 실행 시 | MIT OR Apache-2.0 | MIT AND Zlib | 확인됨 |
| linebender_resource_handle | 0.1.1 | 실행 시 | Apache-2.0 OR MIT | MIT | 확인됨 |
| linux-raw-sys | 0.12.1 | 실행 시 | Apache-2.0 WITH LLVM-exception OR Apache-2.0 OR MIT | MIT | 확인됨 |
| linux-raw-sys | 0.4.15 | 실행 시 | Apache-2.0 WITH LLVM-exception OR Apache-2.0 OR MIT | MIT | 확인됨 |
| litrs | 1.0.0 | 빌드·매크로용 | MIT OR Apache-2.0 | MIT | 확인됨 |
| lock_api | 0.4.14 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| log | 0.4.34 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| memchr | 2.8.3 | 실행 시 | Unlicense OR MIT | MIT | 확인됨 |
| memmap2 | 0.9.11 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| miniz_oxide | 0.8.9 | 실행 시 | MIT OR Zlib OR Apache-2.0 | MIT | 확인됨 |
| miniz_oxide | 0.9.1 | 실행 시 | MIT OR Zlib OR Apache-2.0 | MIT | 확인됨 |
| mio | 1.2.4 | 실행 시 | MIT | MIT | 확인됨 |
| moxcms | 0.8.1 | 실행 시 | BSD-3-Clause OR Apache-2.0 | Apache-2.0 | 확인됨 |
| naga-types | 30.0.1 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| naga | 30.0.1 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| nohash-hasher | 0.2.0 | 실행 시 | Apache-2.0 OR MIT | MIT | 확인됨 |
| nom | 8.0.0 | 실행 시 | MIT | MIT | 확인됨 |
| num-traits | 0.2.19 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| once_cell | 1.21.4 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| ordered-float | 5.5.0 | 실행 시 | MIT | MIT | 확인됨 |
| ordered-stream | 0.2.0 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| os_pipe | 1.2.3 | 실행 시 | MIT | MIT | 확인됨 |
| parking | 2.2.1 | 실행 시 | Apache-2.0 OR MIT | MIT | 확인됨 |
| parking_lot | 0.12.5 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| parking_lot_core | 0.9.12 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| paste | 1.0.15 | 빌드·매크로용 | MIT OR Apache-2.0 | MIT | 확인됨 |
| pastey | 0.2.3 | 빌드·매크로용 | MIT OR Apache-2.0 | MIT | 확인됨 |
| peniko | 0.6.1 | 실행 시 | Apache-2.0 OR MIT | MIT | 확인됨 |
| percent-encoding | 2.3.2 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| petgraph | 0.8.3 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| phf | 0.13.1 | 실행 시 | MIT | MIT | 확인됨 |
| phf_generator | 0.13.1 | 빌드·매크로용 | MIT | MIT | 확인됨 |
| phf_macros | 0.13.1 | 빌드·매크로용 | MIT | MIT | 확인됨 |
| phf_shared | 0.13.1 | 실행 시 | MIT | MIT | 확인됨 |
| pin-project-lite | 0.2.17 | 실행 시 | Apache-2.0 OR MIT | MIT | 확인됨 |
| piper | 0.2.5 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| pkg-config | 0.3.34 | 빌드·매크로용 | MIT OR Apache-2.0 | MIT | 확인됨 |
| png | 0.18.1 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| polling | 3.11.0 | 실행 시 | Apache-2.0 OR MIT | MIT | 확인됨 |
| pollster | 0.4.0 | 실행 시 | Apache-2.0/MIT | MIT | 확인됨 |
| pollster | 1.0.1 | 실행 시 | Apache-2.0/MIT | MIT | 확인됨 |
| polycool | 0.4.0 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| presser | 0.3.1 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| proc-macro-crate | 3.5.0 | 빌드·매크로용 | MIT OR Apache-2.0 | MIT | 확인됨 |
| proc-macro2-diagnostics | 0.10.1 | 빌드·매크로용 | MIT/Apache-2.0 | MIT | 확인됨 |
| proc-macro2 | 1.0.107 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| profiling | 1.0.18 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| pxfm | 0.1.30 | 실행 시 | BSD-3-Clause OR Apache-2.0 | Apache-2.0 | 확인됨 |
| quick-xml | 0.41.0 | 빌드·매크로용 | MIT | MIT | 확인됨 |
| quote | 1.0.47 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| rand | 0.10.3 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| rand_core | 0.10.1 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| raw-window-handle | 0.6.2 | 실행 시 | MIT OR Apache-2.0 OR Zlib | MIT | 확인됨 |
| rayon-core | 1.13.0 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| rayon | 1.12.0 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| read-fonts | 0.41.0 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| ref-cast-impl | 1.0.27 | 빌드·매크로용 | MIT OR Apache-2.0 | MIT | 확인됨 |
| ref-cast | 1.0.27 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| renderdoc-sys | 1.1.0 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| rfd | 0.17.2 | 실행 시 | MIT | MIT | 확인됨 |
| rmcp | 3.5.1 | 실행 시 | Apache-2.0 | Apache-2.0 AND MIT | 확인됨 |
| rusqlite | 0.40.2 | 실행 시 | MIT | MIT | 확인됨 |
| rustc-hash | 1.1.0 | 실행 시 | Apache-2.0/MIT | MIT | 확인됨 |
| rustc-hash | 2.1.3 | 실행 시 | Apache-2.0 OR MIT | MIT | 확인됨 |
| rustc_version | 0.4.1 | 빌드·매크로용 | MIT OR Apache-2.0 | MIT | 확인됨 |
| rustix | 0.38.44 | 실행 시 | Apache-2.0 WITH LLVM-exception OR Apache-2.0 OR MIT | MIT | 확인됨 |
| rustix | 1.1.5 | 실행 시 | Apache-2.0 WITH LLVM-exception OR Apache-2.0 OR MIT | MIT | 확인됨 |
| schemars | 1.2.2 | 실행 시 | MIT | MIT | 확인됨 |
| schemars_derive | 1.2.2 | 빌드·매크로용 | MIT | MIT | 확인됨 |
| scoped-tls | 1.0.1 | 실행 시 | MIT/Apache-2.0 | MIT | 확인됨 |
| scopeguard | 1.2.0 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| self_cell | 1.3.0 | 실행 시 | Apache-2.0 OR GPL-2.0-only | Apache-2.0 | 확인됨 |
| semver | 1.0.28 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| serde | 1.0.229 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| serde_core | 1.0.229 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| serde_derive | 1.0.229 | 빌드·매크로용 | MIT OR Apache-2.0 | MIT | 확인됨 |
| serde_derive_internals | 0.30.0 | 빌드·매크로용 | MIT OR Apache-2.0 | MIT | 확인됨 |
| serde_json | 1.0.151 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| serde_repr | 0.1.21 | 빌드·매크로용 | MIT OR Apache-2.0 | MIT | 확인됨 |
| serde_spanned | 1.1.1 | 빌드·매크로용 | MIT OR Apache-2.0 | MIT | 확인됨 |
| sha2 | 0.10.9 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| shlex | 2.0.1 | 빌드·매크로용 | MIT OR Apache-2.0 | MIT | 확인됨 |
| signal-hook-registry | 1.4.8 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| signature | 2.2.0 | 실행 시 | Apache-2.0 OR MIT | MIT | 확인됨 |
| simd-adler32 | 0.3.10 | 실행 시 | MIT | MIT | 확인됨 |
| siphasher | 1.0.4 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| skrifa | 0.44.0 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| slab | 0.4.12 | 실행 시 | MIT | MIT | 확인됨 |
| slotmap | 1.1.1 | 실행 시 | Zlib | Zlib | 확인됨 |
| smallvec | 1.16.2 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| smithay-client-toolkit | 0.19.2 | 실행 시 | MIT | MIT | 확인됨 |
| smithay-client-toolkit | 0.20.0 | 실행 시 | MIT | MIT | 확인됨 |
| smithay-clipboard | 0.7.3 | 실행 시 | MIT | MIT | 확인됨 |
| smol_str | 0.2.2 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| socket2 | 0.6.5 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| spirv | 0.4.0+sdk-1.4.341.0 | 실행 시 | Apache-2.0 | Apache-2.0 | 확인됨 |
| sse-stream | 0.2.6 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| static_assertions | 1.1.0 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| subtle | 2.6.1 | 실행 시 | BSD-3-Clause | BSD-3-Clause | 확인됨 |
| syn | 2.0.119 | 빌드·매크로용 | MIT OR Apache-2.0 | MIT | 확인됨 |
| syn | 3.0.6 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| thiserror-impl | 1.0.69 | 빌드·매크로용 | MIT OR Apache-2.0 | MIT | 확인됨 |
| thiserror-impl | 2.0.21 | 빌드·매크로용 | MIT OR Apache-2.0 | MIT | 확인됨 |
| thiserror | 1.0.69 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| thiserror | 2.0.21 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| tinystr | 0.8.4 | 실행 시 | Unicode-3.0 | Unicode-3.0 | 확인됨 |
| tinyvec | 1.13.3 | 실행 시 | Zlib OR Apache-2.0 OR MIT | MIT | 확인됨 |
| tokio-macros | 2.7.2 | 빌드·매크로용 | MIT | MIT | 확인됨 |
| tokio-stream | 0.1.19 | 실행 시 | MIT | MIT | 확인됨 |
| tokio-util | 0.7.19 | 실행 시 | MIT | MIT | 확인됨 |
| tokio | 1.53.2 | 실행 시 | MIT | MIT | 확인됨 |
| toml | 1.1.6+spec-1.1.0 | 빌드·매크로용 | MIT OR Apache-2.0 | MIT | 확인됨 |
| toml_datetime | 1.1.1+spec-1.1.0 | 빌드·매크로용 | MIT OR Apache-2.0 | MIT | 확인됨 |
| toml_edit | 0.25.15+spec-1.1.0 | 빌드·매크로용 | MIT OR Apache-2.0 | MIT | 확인됨 |
| toml_parser | 1.1.3+spec-1.1.0 | 빌드·매크로용 | MIT OR Apache-2.0 | MIT | 확인됨 |
| tower-service | 0.3.3 | 실행 시 | MIT | MIT | 확인됨 |
| tracing-attributes | 0.1.31 | 빌드·매크로용 | MIT | MIT | 확인됨 |
| tracing-core | 0.1.36 | 실행 시 | MIT | MIT | 확인됨 |
| tracing | 0.1.44 | 실행 시 | MIT | MIT | 확인됨 |
| tree_magic_mini | 3.2.2 | 실행 시 | MIT | MIT | 확인됨 |
| try-lock | 0.2.5 | 실행 시 | MIT | MIT | 확인됨 |
| type-map | 0.5.1 | 실행 시 | MIT/Apache-2.0 | MIT | 확인됨 |
| typenum | 1.20.1 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| ufbx | 0.11.5 | 실행 시 | MIT OR Unlicense | MIT | 확인됨 |
| unic-langid-impl | 0.9.6 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| unic-langid | 0.9.6 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| unicode-general-category | 1.1.0 | 실행 시 | Apache-2.0 | Apache-2.0 | 확인됨 |
| unicode-ident | 1.0.26 | 실행 시 | (MIT OR Apache-2.0) AND Unicode-3.0 | MIT AND Unicode-3.0 | 확인됨 |
| unicode-segmentation | 1.13.3 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| unicode-width | 0.2.2 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| uuid | 1.27.0 | 실행 시 | Apache-2.0 OR MIT | MIT | 확인됨 |
| vcpkg | 0.2.15 | 빌드·매크로용 | MIT/Apache-2.0 | MIT | 확인됨 |
| vello_common | 0.1.0 | 실행 시 | Apache-2.0 OR MIT | MIT | 확인됨 |
| vello_cpu | 0.1.0 | 실행 시 | Apache-2.0 OR MIT | MIT | 확인됨 |
| version_check | 0.9.5 | 빌드·매크로용 | MIT/Apache-2.0 | MIT | 확인됨 |
| want | 0.3.2 | 실행 시 | MIT | MIT | 확인됨 |
| wayland-backend | 0.3.17 | 실행 시 | MIT | MIT | 확인됨 |
| wayland-client | 0.31.15 | 실행 시 | MIT | MIT | 확인됨 |
| wayland-csd-frame | 0.3.0 | 실행 시 | MIT | MIT | 확인됨 |
| wayland-cursor | 0.31.14 | 실행 시 | MIT | MIT | 확인됨 |
| wayland-protocols-experimental | 20250721.0.1 | 실행 시 | MIT | MIT | 확인됨 |
| wayland-protocols-misc | 0.3.12 | 실행 시 | MIT | MIT AND LGPL-2.1-or-later | 확인됨 |
| wayland-protocols-plasma | 0.3.12 | 실행 시 | MIT | MIT AND LGPL-2.1-or-later | 확인됨 |
| wayland-protocols-wlr | 0.3.12 | 실행 시 | MIT | MIT | 확인됨 |
| wayland-protocols | 0.32.13 | 실행 시 | MIT | MIT | 확인됨 |
| wayland-scanner | 0.31.11 | 빌드·매크로용 | MIT | MIT | 확인됨 |
| wayland-sys | 0.31.11 | 실행 시 | MIT | MIT | 확인됨 |
| web-time | 1.1.0 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| wgpu-core-deps-windows-linux-android | 30.0.1 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| wgpu-core | 30.0.1 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| wgpu-hal | 30.0.1 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| wgpu-naga-bridge | 30.0.1 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| wgpu-types | 30.0.1 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| wgpu | 30.0.1 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| winit | 0.30.13 | 실행 시 | Apache-2.0 | Apache-2.0 | 확인됨 |
| winnow | 1.0.4 | 실행 시 | MIT | MIT | 확인됨 |
| winresource | 0.1.31 | 빌드·매크로용 | MIT | MIT | 확인됨 |
| wl-clipboard-rs | 0.9.4 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| x11-dl | 2.21.0 | 실행 시 | MIT | MIT | 확인됨 |
| x11rb-protocol | 0.13.2 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| x11rb | 0.13.2 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| xcursor | 0.3.11 | 실행 시 | MIT | MIT | 확인됨 |
| xkbcommon-dl | 0.4.2 | 실행 시 | MIT | MIT | 확인됨 |
| xkeysym | 0.2.1 | 실행 시 | MIT OR Apache-2.0 OR Zlib | MIT | 확인됨 |
| zbus-lockstep-macros | 0.5.2 | 빌드·매크로용 | MIT | MIT | 확인됨 |
| zbus-lockstep | 0.5.2 | 실행 시 | MIT | MIT | 확인됨 |
| zbus | 5.19.0 | 실행 시 | MIT | MIT | 확인됨 |
| zbus_macros | 5.19.0 | 빌드·매크로용 | MIT | MIT | 확인됨 |
| zbus_names | 4.3.4 | 실행 시 | MIT | MIT | 확인됨 |
| zbus_xml | 5.2.1 | 실행 시 | MIT | MIT | 확인됨 |
| zcheapstr | 1.1.0 | 실행 시 | MIT | MIT | 확인됨 |
| zerocopy-derive | 0.8.59 | 빌드·매크로용 | BSD-2-Clause OR Apache-2.0 OR MIT | MIT | 확인됨 |
| zerocopy | 0.8.59 | 실행 시 | BSD-2-Clause OR Apache-2.0 OR MIT | MIT | 확인됨 |
| zeroize | 1.9.0 | 실행 시 | Apache-2.0 OR MIT | MIT | 확인됨 |
| zmij | 1.0.23 | 실행 시 | MIT | MIT | 확인됨 |
| zune-core | 0.5.3 | 실행 시 | MIT OR Apache-2.0 OR Zlib | MIT | 확인됨 |
| zune-jpeg | 0.5.15 | 실행 시 | MIT OR Apache-2.0 OR Zlib | MIT | 확인됨 |
| zvariant | 5.15.0 | 실행 시 | MIT | MIT | 확인됨 |
| zvariant_derive | 5.15.0 | 빌드·매크로용 | MIT | MIT | 확인됨 |
| zvariant_utils | 4.2.0 | 실행 시 | MIT | MIT | 확인됨 |

<a id="yolu-cli-の依存一覧-2"></a>

### yolu-cli 의존성 목록

대상: `x86_64-unknown-linux-gnu`, 일반 기능. Cargo.lock SHA-256: `0daa72a781beb6e19705f5836f8aa24b3ff16b15c6ea6d6899968a0ff3398990`.

외부 크레이트 131개(이름이 같아도 버전이 다르면 별개). 실행 시 108개.

빌드용·프로시저 매크로용도 누락을 방지하기 위해 전문 묶음에 포함합니다. 테스트용 의존성은 제외합니다.

`yolu-app`과 같은 cargo 빌드(`-p yolu-app -p yolu-cli`)에서 기능이 합쳐진 `yolu-cli` 하위 트리입니다.

| 선택한 라이선스(추가 조건 포함) | 개수 |
|---|---:|
| Apache-2.0 AND MIT | 1 |
| MIT | 126 |
| MIT AND Unicode-3.0 | 1 |
| MIT AND Zlib | 1 |
| Zlib | 2 |

상태: 크레이트 라이선스 대조 성공.

| 크레이트 | 버전 | 용도 | 선언된 라이선스 | 선택·추가 조건 | 확인 |
|---|---|---|---|---|---|
| adler2 | 2.0.1 | 실행 시 | 0BSD OR MIT OR Apache-2.0 | MIT | 확인됨 |
| arrayvec | 0.7.8 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| async-trait | 0.1.92 | 빌드·매크로용 | MIT OR Apache-2.0 | MIT | 확인됨 |
| atomic-waker | 1.1.2 | 실행 시 | Apache-2.0 OR MIT | MIT | 확인됨 |
| autocfg | 1.5.1 | 빌드·매크로용 | Apache-2.0 OR MIT | MIT | 확인됨 |
| base64 | 0.23.1 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| bitflags | 2.13.2 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| block-buffer | 0.10.4 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| bytemuck | 1.25.2 | 실행 시 | Zlib OR Apache-2.0 OR MIT | MIT | 확인됨 |
| bytemuck_derive | 1.12.1 | 빌드·매크로용 | Zlib OR Apache-2.0 OR MIT | MIT | 확인됨 |
| bytes | 1.12.1 | 실행 시 | MIT | MIT | 확인됨 |
| cc | 1.6.0 | 빌드·매크로용 | MIT OR Apache-2.0 | MIT | 확인됨 |
| cfg-if | 1.0.5 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| chacha20 | 0.10.2 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| chrono | 0.4.45 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| color | 0.3.3 | 실행 시 | Apache-2.0 OR MIT | MIT | 확인됨 |
| cpufeatures | 0.2.17 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| cpufeatures | 0.3.1 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| crc32fast | 1.5.2 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| crossbeam-deque | 0.8.8 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| crossbeam-epoch | 0.9.21 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| crossbeam-utils | 0.8.23 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| crypto-common | 0.1.7 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| digest | 0.10.7 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| dyn-clone | 1.0.20 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| either | 1.18.0 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| equivalent | 1.0.2 | 실행 시 | Apache-2.0 OR MIT | MIT | 확인됨 |
| euclid | 0.22.14 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| fallible-iterator | 0.3.0 | 실행 시 | MIT/Apache-2.0 | MIT | 확인됨 |
| fallible-streaming-iterator | 0.1.9 | 실행 시 | MIT/Apache-2.0 | MIT | 확인됨 |
| fdeflate | 0.3.7 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| fearless_simd | 0.4.1 | 실행 시 | Apache-2.0 OR MIT | MIT | 확인됨 |
| find-msvc-tools | 0.1.14 | 빌드·매크로용 | MIT OR Apache-2.0 | MIT | 확인됨 |
| flate2 | 1.1.10 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| foldhash | 0.2.0 | 실행 시 | Zlib | Zlib | 확인됨 |
| font-types | 0.12.6 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| fontdb | 0.24.0 | 실행 시 | MIT | MIT | 확인됨 |
| futures-channel | 0.3.34 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| futures-core | 0.3.34 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| futures-executor | 0.3.34 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| futures-io | 0.3.34 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| futures-macro | 0.3.34 | 빌드·매크로용 | MIT OR Apache-2.0 | MIT | 확인됨 |
| futures-sink | 0.3.34 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| futures-task | 0.3.34 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| futures-util | 0.3.34 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| futures | 0.3.34 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| generic-array | 0.14.7 | 실행 시 | MIT | MIT | 확인됨 |
| getrandom | 0.4.3 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| glam | 0.33.12 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| guillotiere | 0.7.0 | 실행 시 | MIT/Apache-2.0 | MIT | 확인됨 |
| harfrust | 0.12.0 | 실행 시 | MIT | MIT | 확인됨 |
| hashbrown | 0.17.1 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| http-body-util | 0.1.5 | 실행 시 | MIT | MIT | 확인됨 |
| http-body | 1.1.0 | 실행 시 | MIT | MIT | 확인됨 |
| http | 1.5.0 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| httparse | 1.10.1 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| httpdate | 1.0.3 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| hyper-util | 0.1.21 | 실행 시 | MIT | MIT | 확인됨 |
| hyper | 1.12.0 | 실행 시 | MIT | MIT | 확인됨 |
| indexmap | 2.14.2 | 실행 시 | Apache-2.0 OR MIT | MIT | 확인됨 |
| itoa | 1.0.18 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| kurbo | 0.13.1 | 실행 시 | Apache-2.0 OR MIT | MIT | 확인됨 |
| libc | 0.2.190 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| libm | 0.2.16 | 실행 시 | MIT | MIT | 확인됨 |
| libsqlite3-sys | 0.38.2 | 실행 시 | MIT | MIT | 확인됨 |
| libz-sys | 1.1.29 | 실행 시 | MIT OR Apache-2.0 | MIT AND Zlib | 확인됨 |
| linebender_resource_handle | 0.1.1 | 실행 시 | Apache-2.0 OR MIT | MIT | 확인됨 |
| log | 0.4.34 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| memchr | 2.8.3 | 실행 시 | Unlicense OR MIT | MIT | 확인됨 |
| memmap2 | 0.9.11 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| miniz_oxide | 0.8.9 | 실행 시 | MIT OR Zlib OR Apache-2.0 | MIT | 확인됨 |
| miniz_oxide | 0.9.1 | 실행 시 | MIT OR Zlib OR Apache-2.0 | MIT | 확인됨 |
| mio | 1.2.4 | 실행 시 | MIT | MIT | 확인됨 |
| num-traits | 0.2.19 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| once_cell | 1.21.4 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| pastey | 0.2.3 | 빌드·매크로용 | MIT OR Apache-2.0 | MIT | 확인됨 |
| peniko | 0.6.1 | 실행 시 | Apache-2.0 OR MIT | MIT | 확인됨 |
| pin-project-lite | 0.2.17 | 실행 시 | Apache-2.0 OR MIT | MIT | 확인됨 |
| pkg-config | 0.3.34 | 빌드·매크로용 | MIT OR Apache-2.0 | MIT | 확인됨 |
| png | 0.18.1 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| polycool | 0.4.0 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| proc-macro2 | 1.0.107 | 빌드·매크로용 | MIT OR Apache-2.0 | MIT | 확인됨 |
| quote | 1.0.47 | 빌드·매크로용 | MIT OR Apache-2.0 | MIT | 확인됨 |
| rand | 0.10.3 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| rand_core | 0.10.1 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| rayon-core | 1.13.0 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| rayon | 1.12.0 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| read-fonts | 0.41.0 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| ref-cast-impl | 1.0.27 | 빌드·매크로용 | MIT OR Apache-2.0 | MIT | 확인됨 |
| ref-cast | 1.0.27 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| rmcp | 3.5.1 | 실행 시 | Apache-2.0 | Apache-2.0 AND MIT | 확인됨 |
| rusqlite | 0.40.2 | 실행 시 | MIT | MIT | 확인됨 |
| schemars | 1.2.2 | 실행 시 | MIT | MIT | 확인됨 |
| schemars_derive | 1.2.2 | 빌드·매크로용 | MIT | MIT | 확인됨 |
| serde | 1.0.229 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| serde_core | 1.0.229 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| serde_derive | 1.0.229 | 빌드·매크로용 | MIT OR Apache-2.0 | MIT | 확인됨 |
| serde_derive_internals | 0.30.0 | 빌드·매크로용 | MIT OR Apache-2.0 | MIT | 확인됨 |
| serde_json | 1.0.151 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| sha2 | 0.10.9 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| shlex | 2.0.1 | 빌드·매크로용 | MIT OR Apache-2.0 | MIT | 확인됨 |
| simd-adler32 | 0.3.10 | 실행 시 | MIT | MIT | 확인됨 |
| skrifa | 0.44.0 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| slab | 0.4.12 | 실행 시 | MIT | MIT | 확인됨 |
| slotmap | 1.1.1 | 실행 시 | Zlib | Zlib | 확인됨 |
| smallvec | 1.16.2 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| socket2 | 0.6.5 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| sse-stream | 0.2.6 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| syn | 2.0.119 | 빌드·매크로용 | MIT OR Apache-2.0 | MIT | 확인됨 |
| syn | 3.0.6 | 빌드·매크로용 | MIT OR Apache-2.0 | MIT | 확인됨 |
| thiserror-impl | 2.0.21 | 빌드·매크로용 | MIT OR Apache-2.0 | MIT | 확인됨 |
| thiserror | 2.0.21 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| tinyvec | 1.13.3 | 실행 시 | Zlib OR Apache-2.0 OR MIT | MIT | 확인됨 |
| tokio-macros | 2.7.2 | 빌드·매크로용 | MIT | MIT | 확인됨 |
| tokio-stream | 0.1.19 | 실행 시 | MIT | MIT | 확인됨 |
| tokio-util | 0.7.19 | 실행 시 | MIT | MIT | 확인됨 |
| tokio | 1.53.2 | 실행 시 | MIT | MIT | 확인됨 |
| tower-service | 0.3.3 | 실행 시 | MIT | MIT | 확인됨 |
| tracing-attributes | 0.1.31 | 빌드·매크로용 | MIT | MIT | 확인됨 |
| tracing-core | 0.1.36 | 실행 시 | MIT | MIT | 확인됨 |
| tracing | 0.1.44 | 실행 시 | MIT | MIT | 확인됨 |
| try-lock | 0.2.5 | 실행 시 | MIT | MIT | 확인됨 |
| typenum | 1.20.1 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| unicode-ident | 1.0.26 | 빌드·매크로용 | (MIT OR Apache-2.0) AND Unicode-3.0 | MIT AND Unicode-3.0 | 확인됨 |
| uuid | 1.27.0 | 실행 시 | Apache-2.0 OR MIT | MIT | 확인됨 |
| vcpkg | 0.2.15 | 빌드·매크로용 | MIT/Apache-2.0 | MIT | 확인됨 |
| vello_common | 0.1.0 | 실행 시 | Apache-2.0 OR MIT | MIT | 확인됨 |
| vello_cpu | 0.1.0 | 실행 시 | Apache-2.0 OR MIT | MIT | 확인됨 |
| version_check | 0.9.5 | 빌드·매크로용 | MIT/Apache-2.0 | MIT | 확인됨 |
| want | 0.3.2 | 실행 시 | MIT | MIT | 확인됨 |
| zmij | 1.0.23 | 실행 시 | MIT | MIT | 확인됨 |

<a id="macosuniversalの製品別一覧"></a>

## macOS(universal) 제품별 목록

<a id="yolu-appyolu-update-と-yolu-cli-の依存を含む-の依存一覧-3"></a>

### yolu-app(yolu-update와 yolu-cli 의존성 포함) 의존성 목록

대상: `universal-apple-darwin`, 일반 기능. Cargo.lock SHA-256: `0daa72a781beb6e19705f5836f8aa24b3ff16b15c6ea6d6899968a0ff3398990`.

외부 크레이트 255개(이름이 같아도 버전이 다르면 별개). 실행 시 214개.

빌드용·프로시저 매크로용도 누락을 방지하기 위해 전문 묶음에 포함합니다. 테스트용 의존성은 제외합니다.

| 선택한 라이선스(추가 조건 포함) | 개수 |
|---|---:|
| Apache-2.0 | 7 |
| Apache-2.0 AND MIT | 2 |
| BSD-3-Clause | 3 |
| ISC | 1 |
| MIT | 235 |
| MIT AND OFL-1.1 AND Ubuntu-font-1.0 AND Bitstream-Vera | 1 |
| MIT AND Unicode-3.0 | 1 |
| MIT AND Zlib | 2 |
| Unicode-3.0 | 1 |
| Zlib | 2 |

상태: 크레이트 라이선스 대조 성공.

| 크레이트 | 버전 | 용도 | 선언된 라이선스 | 선택·추가 조건 | 확인 |
|---|---|---|---|---|---|
| accesskit | 0.24.1 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| accesskit_consumer | 0.38.0 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| accesskit_macos | 0.26.3 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| accesskit_winit | 0.32.2 | 실행 시 | Apache-2.0 | Apache-2.0 | 확인됨 |
| adler2 | 2.0.1 | 실행 시 | 0BSD OR MIT OR Apache-2.0 | MIT | 확인됨 |
| ahash | 0.8.12 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| arboard | 3.6.1 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| arrayvec | 0.7.8 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| async-trait | 0.1.92 | 빌드·매크로용 | MIT OR Apache-2.0 | MIT | 확인됨 |
| atomic-waker | 1.1.2 | 실행 시 | Apache-2.0 OR MIT | MIT | 확인됨 |
| autocfg | 1.5.1 | 빌드·매크로용 | Apache-2.0 OR MIT | MIT | 확인됨 |
| base64 | 0.23.1 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| bevy_mikktspace | 1.0.0 | 실행 시 | Zlib AND (MIT OR Apache-2.0) | MIT AND Zlib | 확인됨 |
| bit-set | 0.10.0 | 실행 시 | Apache-2.0 OR MIT | MIT | 확인됨 |
| bit-vec | 0.9.1 | 실행 시 | Apache-2.0 OR MIT | MIT | 확인됨 |
| bitflags | 1.3.2 | 실행 시 | MIT/Apache-2.0 | MIT | 확인됨 |
| bitflags | 2.13.2 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| block-buffer | 0.10.4 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| block2 | 0.5.1 | 실행 시 | MIT | MIT | 확인됨 |
| block2 | 0.6.2 | 실행 시 | MIT | MIT | 확인됨 |
| bytemuck | 1.25.2 | 실행 시 | Zlib OR Apache-2.0 OR MIT | MIT | 확인됨 |
| bytemuck_derive | 1.12.1 | 빌드·매크로용 | Zlib OR Apache-2.0 OR MIT | MIT | 확인됨 |
| byteorder-lite | 0.1.0 | 실행 시 | Unlicense OR MIT | MIT | 확인됨 |
| bytes | 1.12.1 | 실행 시 | MIT | MIT | 확인됨 |
| cc | 1.6.0 | 빌드·매크로용 | MIT OR Apache-2.0 | MIT | 확인됨 |
| cfg-if | 1.0.5 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| cfg_aliases | 0.2.2 | 빌드·매크로용 | MIT | MIT | 확인됨 |
| chacha20 | 0.10.2 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| chrono | 0.4.45 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| codespan-reporting | 0.13.1 | 실행 시 | Apache-2.0 | Apache-2.0 | 확인됨 |
| color | 0.3.3 | 실행 시 | Apache-2.0 OR MIT | MIT | 확인됨 |
| core-foundation-sys | 0.8.7 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| core-foundation | 0.9.4 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| core-graphics-types | 0.1.3 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| core-graphics | 0.23.2 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| cpufeatures | 0.2.17 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| cpufeatures | 0.3.1 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| crc32fast | 1.5.2 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| crossbeam-deque | 0.8.8 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| crossbeam-epoch | 0.9.21 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| crossbeam-utils | 0.8.23 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| crypto-common | 0.1.7 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| cursor-icon | 1.2.0 | 실행 시 | MIT OR Apache-2.0 OR Zlib | MIT | 확인됨 |
| curve25519-dalek-derive | 0.1.1 | 빌드·매크로용 | MIT/Apache-2.0 | MIT | 확인됨 |
| curve25519-dalek | 4.1.3 | 실행 시 | BSD-3-Clause | BSD-3-Clause | 확인됨 |
| digest | 0.10.7 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| dispatch2 | 0.3.1 | 실행 시 | Zlib OR Apache-2.0 OR MIT | MIT | 확인됨 |
| dispatch | 0.2.0 | 실행 시 | MIT | MIT | 확인됨 |
| displaydoc | 0.2.7 | 빌드·매크로용 | MIT OR Apache-2.0 | MIT | 확인됨 |
| document-features | 0.2.12 | 빌드·매크로용 | MIT OR Apache-2.0 | MIT | 확인됨 |
| dpi | 0.1.2 | 실행 시 | Apache-2.0 AND MIT | Apache-2.0 AND MIT | 확인됨 |
| duplicate | 2.0.1 | 빌드·매크로용 | MIT OR Apache-2.0 | MIT | 확인됨 |
| dyn-clone | 1.0.20 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| ecolor | 0.36.2 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| ed25519-dalek | 2.2.0 | 실행 시 | BSD-3-Clause | BSD-3-Clause | 확인됨 |
| ed25519 | 2.2.3 | 실행 시 | Apache-2.0 OR MIT | MIT | 확인됨 |
| eframe | 0.36.2 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| egui-wgpu | 0.36.2 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| egui-winit | 0.36.2 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| egui | 0.36.2 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| egui_dock | 0.21.1 | 실행 시 | MIT | MIT | 확인됨 |
| either | 1.18.0 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| emath | 0.36.2 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| enumn | 0.1.14 | 빌드·매크로용 | MIT OR Apache-2.0 | MIT | 확인됨 |
| epaint | 0.36.2 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| epaint_default_fonts | 0.36.2 | 실행 시 | (MIT OR Apache-2.0) AND OFL-1.1 AND Ubuntu-font-1.0 | MIT AND OFL-1.1 AND Ubuntu-font-1.0 AND Bitstream-Vera | 확인됨 |
| equivalent | 1.0.2 | 실행 시 | Apache-2.0 OR MIT | MIT | 확인됨 |
| euclid | 0.22.14 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| fallible-iterator | 0.3.0 | 실행 시 | MIT/Apache-2.0 | MIT | 확인됨 |
| fallible-streaming-iterator | 0.1.9 | 실행 시 | MIT/Apache-2.0 | MIT | 확인됨 |
| fax | 0.2.7 | 실행 시 | MIT | MIT | 확인됨 |
| fdeflate | 0.3.7 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| fearless_simd | 0.4.1 | 실행 시 | Apache-2.0 OR MIT | MIT | 확인됨 |
| find-msvc-tools | 0.1.14 | 빌드·매크로용 | MIT OR Apache-2.0 | MIT | 확인됨 |
| flate2 | 1.1.10 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| fluent-bundle | 0.16.0 | 실행 시 | Apache-2.0 OR MIT | MIT | 확인됨 |
| fluent-langneg | 0.13.1 | 실행 시 | Apache-2.0 OR MIT | MIT | 확인됨 |
| fluent-syntax | 0.12.0 | 실행 시 | Apache-2.0 OR MIT | MIT | 확인됨 |
| foldhash | 0.2.0 | 실행 시 | Zlib | Zlib | 확인됨 |
| font-types | 0.12.6 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| fontdb | 0.24.0 | 실행 시 | MIT | MIT | 확인됨 |
| foreign-types-macros | 0.2.4 | 빌드·매크로용 | MIT/Apache-2.0 | MIT | 확인됨 |
| foreign-types-shared | 0.3.1 | 실행 시 | MIT/Apache-2.0 | MIT | 확인됨 |
| foreign-types | 0.5.0 | 실행 시 | MIT/Apache-2.0 | MIT | 확인됨 |
| futures-channel | 0.3.34 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| futures-core | 0.3.34 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| futures-executor | 0.3.34 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| futures-io | 0.3.34 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| futures-macro | 0.3.34 | 빌드·매크로용 | MIT OR Apache-2.0 | MIT | 확인됨 |
| futures-sink | 0.3.34 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| futures-task | 0.3.34 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| futures-util | 0.3.34 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| futures | 0.3.34 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| generic-array | 0.14.7 | 실행 시 | MIT | MIT | 확인됨 |
| getrandom | 0.4.3 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| glam | 0.33.12 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| guillotiere | 0.7.0 | 실행 시 | MIT/Apache-2.0 | MIT | 확인됨 |
| half | 2.7.1 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| harfrust | 0.12.0 | 실행 시 | MIT | MIT | 확인됨 |
| hashbrown | 0.16.1 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| hashbrown | 0.17.1 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| heck | 0.5.0 | 빌드·매크로용 | MIT OR Apache-2.0 | MIT | 확인됨 |
| hex | 0.4.3 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| http-body-util | 0.1.5 | 실행 시 | MIT | MIT | 확인됨 |
| http-body | 1.1.0 | 실행 시 | MIT | MIT | 확인됨 |
| http | 1.5.0 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| httparse | 1.10.1 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| httpdate | 1.0.3 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| hyper-util | 0.1.21 | 실행 시 | MIT | MIT | 확인됨 |
| hyper | 1.12.0 | 실행 시 | MIT | MIT | 확인됨 |
| image | 0.25.10 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| indexmap | 2.14.2 | 실행 시 | Apache-2.0 OR MIT | MIT | 확인됨 |
| intl-memoizer | 0.5.3 | 실행 시 | Apache-2.0 OR MIT | MIT | 확인됨 |
| intl_pluralrules | 7.0.2 | 실행 시 | Apache-2.0/MIT | MIT | 확인됨 |
| itertools | 0.15.0 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| itoa | 1.0.18 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| kurbo | 0.13.1 | 실행 시 | Apache-2.0 OR MIT | MIT | 확인됨 |
| libc | 0.2.190 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| libloading | 0.8.9 | 실행 시 | ISC | ISC | 확인됨 |
| libm | 0.2.16 | 실행 시 | MIT | MIT | 확인됨 |
| libsqlite3-sys | 0.38.2 | 실행 시 | MIT | MIT | 확인됨 |
| libz-sys | 1.1.29 | 실행 시 | MIT OR Apache-2.0 | MIT AND Zlib | 확인됨 |
| linebender_resource_handle | 0.1.1 | 실행 시 | Apache-2.0 OR MIT | MIT | 확인됨 |
| litrs | 1.0.0 | 빌드·매크로용 | MIT OR Apache-2.0 | MIT | 확인됨 |
| lock_api | 0.4.14 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| log | 0.4.34 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| memchr | 2.8.3 | 실행 시 | Unlicense OR MIT | MIT | 확인됨 |
| memmap2 | 0.9.11 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| miniz_oxide | 0.8.9 | 실행 시 | MIT OR Zlib OR Apache-2.0 | MIT | 확인됨 |
| miniz_oxide | 0.9.1 | 실행 시 | MIT OR Zlib OR Apache-2.0 | MIT | 확인됨 |
| mio | 1.2.4 | 실행 시 | MIT | MIT | 확인됨 |
| moxcms | 0.8.1 | 실행 시 | BSD-3-Clause OR Apache-2.0 | Apache-2.0 | 확인됨 |
| naga-types | 30.0.1 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| naga | 30.0.1 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| nohash-hasher | 0.2.0 | 실행 시 | Apache-2.0 OR MIT | MIT | 확인됨 |
| num-traits | 0.2.19 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| objc-sys | 0.3.5 | 실행 시 | MIT | MIT | 확인됨 |
| objc2-app-kit | 0.2.2 | 실행 시 | MIT | MIT | 확인됨 |
| objc2-app-kit | 0.3.2 | 실행 시 | Zlib OR Apache-2.0 OR MIT | MIT | 확인됨 |
| objc2-core-foundation | 0.3.2 | 실행 시 | Zlib OR Apache-2.0 OR MIT | MIT | 확인됨 |
| objc2-core-graphics | 0.3.2 | 실행 시 | Zlib OR Apache-2.0 OR MIT | MIT | 확인됨 |
| objc2-encode | 4.1.0 | 실행 시 | MIT | MIT | 확인됨 |
| objc2-foundation | 0.2.2 | 실행 시 | MIT | MIT | 확인됨 |
| objc2-foundation | 0.3.2 | 실행 시 | MIT | MIT | 확인됨 |
| objc2-metal | 0.3.2 | 실행 시 | Zlib OR Apache-2.0 OR MIT | MIT | 확인됨 |
| objc2-quartz-core | 0.3.2 | 실행 시 | Zlib OR Apache-2.0 OR MIT | MIT | 확인됨 |
| objc2 | 0.5.2 | 실행 시 | MIT | MIT | 확인됨 |
| objc2 | 0.6.4 | 실행 시 | MIT | MIT | 확인됨 |
| once_cell | 1.21.4 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| parking_lot | 0.12.5 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| parking_lot_core | 0.9.12 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| paste | 1.0.15 | 빌드·매크로용 | MIT OR Apache-2.0 | MIT | 확인됨 |
| pastey | 0.2.3 | 빌드·매크로용 | MIT OR Apache-2.0 | MIT | 확인됨 |
| peniko | 0.6.1 | 실행 시 | Apache-2.0 OR MIT | MIT | 확인됨 |
| pin-project-lite | 0.2.17 | 실행 시 | Apache-2.0 OR MIT | MIT | 확인됨 |
| pkg-config | 0.3.34 | 빌드·매크로용 | MIT OR Apache-2.0 | MIT | 확인됨 |
| png | 0.18.1 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| pollster | 1.0.1 | 실행 시 | Apache-2.0/MIT | MIT | 확인됨 |
| polycool | 0.4.0 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| proc-macro2-diagnostics | 0.10.1 | 빌드·매크로용 | MIT/Apache-2.0 | MIT | 확인됨 |
| proc-macro2 | 1.0.107 | 빌드·매크로용 | MIT OR Apache-2.0 | MIT | 확인됨 |
| profiling | 1.0.18 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| pxfm | 0.1.30 | 실행 시 | BSD-3-Clause OR Apache-2.0 | Apache-2.0 | 확인됨 |
| quick-error | 2.0.1 | 실행 시 | MIT/Apache-2.0 | MIT | 확인됨 |
| quote | 1.0.47 | 빌드·매크로용 | MIT OR Apache-2.0 | MIT | 확인됨 |
| rand | 0.10.3 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| rand_core | 0.10.1 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| raw-window-handle | 0.6.2 | 실행 시 | MIT OR Apache-2.0 OR Zlib | MIT | 확인됨 |
| raw-window-metal | 1.1.0 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| rayon-core | 1.13.0 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| rayon | 1.12.0 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| read-fonts | 0.41.0 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| ref-cast-impl | 1.0.27 | 빌드·매크로용 | MIT OR Apache-2.0 | MIT | 확인됨 |
| ref-cast | 1.0.27 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| rfd | 0.17.2 | 실행 시 | MIT | MIT | 확인됨 |
| rmcp | 3.5.1 | 실행 시 | Apache-2.0 | Apache-2.0 AND MIT | 확인됨 |
| rusqlite | 0.40.2 | 실행 시 | MIT | MIT | 확인됨 |
| rustc-hash | 1.1.0 | 실행 시 | Apache-2.0/MIT | MIT | 확인됨 |
| rustc-hash | 2.1.3 | 실행 시 | Apache-2.0 OR MIT | MIT | 확인됨 |
| rustc_version | 0.4.1 | 빌드·매크로용 | MIT OR Apache-2.0 | MIT | 확인됨 |
| schemars | 1.2.2 | 실행 시 | MIT | MIT | 확인됨 |
| schemars_derive | 1.2.2 | 빌드·매크로용 | MIT | MIT | 확인됨 |
| scopeguard | 1.2.0 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| self_cell | 1.3.0 | 실행 시 | Apache-2.0 OR GPL-2.0-only | Apache-2.0 | 확인됨 |
| semver | 1.0.28 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| serde | 1.0.229 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| serde_core | 1.0.229 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| serde_derive | 1.0.229 | 빌드·매크로용 | MIT OR Apache-2.0 | MIT | 확인됨 |
| serde_derive_internals | 0.30.0 | 빌드·매크로용 | MIT OR Apache-2.0 | MIT | 확인됨 |
| serde_json | 1.0.151 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| serde_spanned | 1.1.1 | 빌드·매크로용 | MIT OR Apache-2.0 | MIT | 확인됨 |
| sha2 | 0.10.9 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| shlex | 2.0.1 | 빌드·매크로용 | MIT OR Apache-2.0 | MIT | 확인됨 |
| signature | 2.2.0 | 실행 시 | Apache-2.0 OR MIT | MIT | 확인됨 |
| simd-adler32 | 0.3.10 | 실행 시 | MIT | MIT | 확인됨 |
| skrifa | 0.44.0 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| slab | 0.4.12 | 실행 시 | MIT | MIT | 확인됨 |
| slotmap | 1.1.1 | 실행 시 | Zlib | Zlib | 확인됨 |
| smallvec | 1.16.2 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| smol_str | 0.2.2 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| socket2 | 0.6.5 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| sse-stream | 0.2.6 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| static_assertions | 1.1.0 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| subtle | 2.6.1 | 실행 시 | BSD-3-Clause | BSD-3-Clause | 확인됨 |
| syn | 2.0.119 | 빌드·매크로용 | MIT OR Apache-2.0 | MIT | 확인됨 |
| syn | 3.0.6 | 빌드·매크로용 | MIT OR Apache-2.0 | MIT | 확인됨 |
| thiserror-impl | 2.0.21 | 빌드·매크로용 | MIT OR Apache-2.0 | MIT | 확인됨 |
| thiserror | 2.0.21 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| tiff | 0.11.3 | 실행 시 | MIT | MIT | 확인됨 |
| tinystr | 0.8.4 | 실행 시 | Unicode-3.0 | Unicode-3.0 | 확인됨 |
| tinyvec | 1.13.3 | 실행 시 | Zlib OR Apache-2.0 OR MIT | MIT | 확인됨 |
| tokio-macros | 2.7.2 | 빌드·매크로용 | MIT | MIT | 확인됨 |
| tokio-stream | 0.1.19 | 실행 시 | MIT | MIT | 확인됨 |
| tokio-util | 0.7.19 | 실행 시 | MIT | MIT | 확인됨 |
| tokio | 1.53.2 | 실행 시 | MIT | MIT | 확인됨 |
| toml | 1.1.6+spec-1.1.0 | 빌드·매크로용 | MIT OR Apache-2.0 | MIT | 확인됨 |
| toml_datetime | 1.1.1+spec-1.1.0 | 빌드·매크로용 | MIT OR Apache-2.0 | MIT | 확인됨 |
| toml_parser | 1.1.3+spec-1.1.0 | 빌드·매크로용 | MIT OR Apache-2.0 | MIT | 확인됨 |
| tower-service | 0.3.3 | 실행 시 | MIT | MIT | 확인됨 |
| tracing-attributes | 0.1.31 | 빌드·매크로용 | MIT | MIT | 확인됨 |
| tracing-core | 0.1.36 | 실행 시 | MIT | MIT | 확인됨 |
| tracing | 0.1.44 | 실행 시 | MIT | MIT | 확인됨 |
| try-lock | 0.2.5 | 실행 시 | MIT | MIT | 확인됨 |
| type-map | 0.5.1 | 실행 시 | MIT/Apache-2.0 | MIT | 확인됨 |
| typenum | 1.20.1 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| ufbx | 0.11.5 | 실행 시 | MIT OR Unlicense | MIT | 확인됨 |
| unic-langid-impl | 0.9.6 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| unic-langid | 0.9.6 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| unicode-general-category | 1.1.0 | 실행 시 | Apache-2.0 | Apache-2.0 | 확인됨 |
| unicode-ident | 1.0.26 | 실행 시 | (MIT OR Apache-2.0) AND Unicode-3.0 | MIT AND Unicode-3.0 | 확인됨 |
| unicode-segmentation | 1.13.3 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| unicode-width | 0.2.2 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| uuid | 1.27.0 | 실행 시 | Apache-2.0 OR MIT | MIT | 확인됨 |
| vcpkg | 0.2.15 | 빌드·매크로용 | MIT/Apache-2.0 | MIT | 확인됨 |
| vello_common | 0.1.0 | 실행 시 | Apache-2.0 OR MIT | MIT | 확인됨 |
| vello_cpu | 0.1.0 | 실행 시 | Apache-2.0 OR MIT | MIT | 확인됨 |
| version_check | 0.9.5 | 빌드·매크로용 | MIT/Apache-2.0 | MIT | 확인됨 |
| want | 0.3.2 | 실행 시 | MIT | MIT | 확인됨 |
| web-time | 1.1.0 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| weezl | 0.1.12 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| wgpu-core-deps-apple | 30.0.1 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| wgpu-core | 30.0.1 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| wgpu-hal | 30.0.1 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| wgpu-naga-bridge | 30.0.1 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| wgpu-types | 30.0.1 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| wgpu | 30.0.1 | 실행 시 | MIT OR Apache-2.0 | MIT | 확인됨 |
| winit | 0.30.13 | 실행 시 | Apache-2.0 | Apache-2.0 | 확인됨 |
| winnow | 1.0.4 | 빌드·매크로용 | MIT | MIT | 확인됨 |
| winresource | 0.1.31 | 빌드·매크로용 | MIT | MIT | 확인됨 |
| zerocopy-derive | 0.8.59 | 빌드·매크로용 | BSD-2-Clause OR Apache-2.0 OR MIT | MIT | 확인됨 |
| zerocopy | 0.8.59 | 실행 시 | BSD-2-Clause OR Apache-2.0 OR MIT | MIT | 확인됨 |
| zeroize | 1.9.0 | 실행 시 | Apache-2.0 OR MIT | MIT | 확인됨 |
| zmij | 1.0.23 | 실행 시 | MIT | MIT | 확인됨 |
| zune-core | 0.5.3 | 실행 시 | MIT OR Apache-2.0 OR Zlib | MIT | 확인됨 |
| zune-jpeg | 0.5.15 | 실행 시 | MIT OR Apache-2.0 OR Zlib | MIT | 확인됨 |

<a id="開発用-xtask-の依存一覧"></a>

## 개발용 xtask 의존성 목록

`cargo xtask` 자체는 제품에 포함하지 않습니다. 일반 의존성과 빌드 의존성을 포함하고 테스트용 의존성을 제외한 집합입니다.
각 대상에서 `python3 tools/third-party.py --target <target> --package xtask`를 실행하여 대조했습니다.
`yolu-update` 단독도 같은 지정의 `--package yolu-update`로 각각 31개를 대조했습니다(전체가 위 app 집합에 포함됨).
“○”는 대상에 포함되어 원문 대조에 성공한 항목, “—”는 대상 외 항목입니다.

| 크레이트 | 버전 | 선택할 라이선스 | Windows MSVC | Windows GNU | Linux GNU |
|---|---|---|---|---|---|
| adler2 | 2.0.1 | MIT | ○ | ○ | ○ |
| bitflags | 2.13.2 | MIT | — | — | ○ |
| block-buffer | 0.10.4 | MIT | ○ | ○ | ○ |
| bumpalo | 3.20.3 | MIT | ○ | ○ | ○ |
| cfg-if | 1.0.5 | MIT | ○ | ○ | ○ |
| cpufeatures | 0.2.17 | MIT | ○ | ○ | ○ |
| crc32fast | 1.5.2 | MIT | ○ | ○ | ○ |
| crypto-common | 0.1.7 | MIT | ○ | ○ | ○ |
| curve25519-dalek-derive | 0.1.1 | MIT | ○ | ○ | ○ |
| curve25519-dalek | 4.1.3 | BSD-3-Clause | ○ | ○ | ○ |
| digest | 0.10.7 | MIT | ○ | ○ | ○ |
| ed25519-dalek | 2.2.0 | BSD-3-Clause | ○ | ○ | ○ |
| ed25519 | 2.2.3 | MIT | ○ | ○ | ○ |
| equivalent | 1.0.2 | MIT | ○ | ○ | ○ |
| filetime | 0.2.29 | MIT | ○ | ○ | ○ |
| flate2 | 1.1.10 | MIT | ○ | ○ | ○ |
| generic-array | 0.14.7 | MIT | ○ | ○ | ○ |
| getrandom | 0.3.4 | MIT | ○ | ○ | ○ |
| hashbrown | 0.17.1 | MIT | ○ | ○ | ○ |
| hex | 0.4.3 | MIT | ○ | ○ | ○ |
| indexmap | 2.14.2 | MIT | ○ | ○ | ○ |
| itoa | 1.0.18 | MIT | ○ | ○ | ○ |
| libc | 0.2.190 | MIT | — | — | ○ |
| linux-raw-sys | 0.12.1 | MIT | — | — | ○ |
| log | 0.4.34 | MIT | ○ | ○ | ○ |
| memchr | 2.8.3 | MIT | ○ | ○ | ○ |
| miniz_oxide | 0.9.1 | MIT | ○ | ○ | ○ |
| proc-macro2 | 1.0.107 | MIT | ○ | ○ | ○ |
| quote | 1.0.47 | MIT | ○ | ○ | ○ |
| rustc_version | 0.4.1 | MIT | ○ | ○ | ○ |
| rustix | 1.1.5 | MIT | — | — | ○ |
| semver | 1.0.28 | MIT | ○ | ○ | ○ |
| serde | 1.0.229 | MIT | ○ | ○ | ○ |
| serde_core | 1.0.229 | MIT | ○ | ○ | ○ |
| serde_derive | 1.0.229 | MIT | ○ | ○ | ○ |
| serde_json | 1.0.151 | MIT | ○ | ○ | ○ |
| sha2 | 0.10.9 | MIT | ○ | ○ | ○ |
| signature | 2.2.0 | MIT | ○ | ○ | ○ |
| simd-adler32 | 0.3.10 | MIT | ○ | ○ | ○ |
| subtle | 2.6.1 | BSD-3-Clause | ○ | ○ | ○ |
| syn | 2.0.119 | MIT | ○ | ○ | ○ |
| syn | 3.0.6 | MIT | ○ | ○ | ○ |
| tar | 0.4.46 | MIT | ○ | ○ | ○ |
| typed-path | 0.12.3 | MIT | ○ | ○ | ○ |
| typenum | 1.20.1 | MIT | ○ | ○ | ○ |
| unicode-ident | 1.0.26 | MIT AND Unicode-3.0 | ○ | ○ | ○ |
| version_check | 0.9.5 | MIT | ○ | ○ | ○ |
| xattr | 1.6.1 | MIT | — | — | ○ |
| zeroize | 1.9.0 | MIT | ○ | ○ | ○ |
| zip | 9.0.4 | MIT | ○ | ○ | ○ |
| zlib-rs | 0.6.8 | Zlib | ○ | ○ | ○ |
| zmij | 1.0.23 | MIT | ○ | ○ | ○ |
| zopfli | 0.8.3 | Apache-2.0 | ○ | ○ | ○ |

<a id="韓国語-ui-フォントの追加"></a>

## 한국어 UI 글꼴 추가

한국어 UI용으로 **YoluPainter Hangul**(Regular / Bold)을 포함합니다. Noto Sans CJK KR 2.004의 한국어 페이스에서 한글 음절과 자모를 추출하고 글꼴 이름을 바꾼 파생 글꼴이며, 라이선스는 **SIL Open Font License 1.1**을 유지합니다. 각 파일에 현대 한글 완성형 11,172자가 포함됩니다.

원본은 Noto Sans CJK Regular / Bold TTC의 한국어 페이스(인덱스 1)입니다. 출처는 https://github.com/notofonts/noto-cjk 이며, 생성 스크립트는 `tools/make-ui-font-hangul.py`입니다. 저작권 및 라이선스 전문은 `crates/yolu-app/assets/fonts/OFL-Noto-CJK.txt`, 변경 내용은 같은 폴더의 `THIRD-PARTY-NOTICES-Hangul.md`에 기록했습니다. 파일과 고지의 SHA-256을 `tools/licenses-reviewed.json`에 등록했으며 배포용 라이선스 전문에도 포함합니다.
