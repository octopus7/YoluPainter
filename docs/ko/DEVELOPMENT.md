[한국어](DEVELOPMENT.md) · [日本語](../DEVELOPMENT.md)

<a id="開発用の手順"></a>

# 개발 절차

다음 명령은 저장소 루트에서 실행합니다. 일반 빌드와 사용법은 [README](../../README.md)를 참고하세요.

<a id="テスト"></a>

## 테스트

```sh
cargo test --workspace --locked
```

명령줄과 MCP 서버는 `cargo test -p yolu-cli` (`cargo clippy -p yolu-cli --all-targets -- -D warnings`)로 테스트합니다. 구조와 테스트 분류는 [crates/yolu-cli/README.md](../../crates/yolu-cli/README.ko.md)에 있습니다.

GPU·화면 테스트에는 작동하는 렌더링 백엔드가 필요합니다. 일부 GPU 테스트는 어댑터가 없으면 처리를 생략하므로 passed 결과만으로 렌더링을 확인했다고 판단하지 말고 표준 오류의 이유도 확인하세요. 환경 변수 `YOLUPAINTER_REQUIRE_GPU=1`을 지정하면 생략하지 않고 실패합니다 (CI 화면 테스트에서 사용). 자세한 내용은 [yolu-gpu](../../crates/yolu-gpu/README.ko.md#検証と計測)를 참고하세요.

레이어 합성의 기준은 Rust f32 수식이며 합성을 거치는 정답은 Rust에서 다시 생성했습니다. 정답을 다시 생성할 때는 `YOLU_GOLDEN_UPDATE=1 cargo test -p yolu-core -p yolu-io`로 달라진 정답만 현재 출력으로 갱신하고, diff를 확인해 의도한 변경만 있는지 검사합니다. Normal 채널 합성과 브러시 픽셀도 f32 수식이며 Rust에서 다시 생성했습니다. 필터·Generator 값 등 f64를 유지하는 수식은 현재도 0.4.x까지의 Unity 브리지 (C#) 정답과 대조합니다.

0.4.x까지의 Unity 브리지 (C#)와 대조할 때는 저장소에 포함된 인공 데이터를 사용합니다. 정답 (`crates/yolu-core/tests/golden/`·`crates/yolu-io/tests/fixtures/` 등)은 0.4.x까지의 Unity 브리지 (C#) 출력을 이어받은 것이며 C# 출력과 다시 대조하는 경로는 남기지 않았습니다. 재생성 시 Rust 출력에서 만듭니다 (`YOLU_GOLDEN_UPDATE=1` 지원 테스트는 해당 방식 사용). 효과·레이어 잠금·레이어 조작의 연결부 대조에서 불일치가 나면 `SEAM_DUMP_DIR=<フォルダ>`로 테스트를 실행해 Rust 원시 바이트열을 내보냅니다. I/O와 PSD 데이터 형식·재생성 방법은 [I/O 픽스처](../../crates/yolu-io/tests/fixtures/README.ko.md)와 [PSD 픽스처](../../crates/yolu-io/tests/fixtures/psd/README.ko.md)를 참고하세요.

<a id="aarch64neonのテストを-x86_64-の-linux-で回す"></a>

### x86_64 Linux에서 aarch64 (NEON) 테스트 실행

aarch64 경로 (NEON)는 Mac CI뿐 아니라 x86_64 Linux에서도 QEMU 사용자 모드로 테스트할 수 있습니다. Debian·Ubuntu에서는 `qemu-user-static`·`gcc-aarch64-linux-gnu`·`libc6-dev-arm64-cross`를 설치하고 Rust 타깃 `aarch64-unknown-linux-gnu`를 추가합니다 (`rustup target add aarch64-unknown-linux-gnu`). target은 평소 사용하는 것과 분리해 실행합니다.

```sh
CARGO_TARGET_DIR=target-arm CARGO_INCREMENTAL=0 \
CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER=aarch64-linux-gnu-gcc \
CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_RUNNER="qemu-aarch64-static -L /usr/aarch64-linux-gnu" \
CC_aarch64_unknown_linux_gnu=aarch64-linux-gnu-gcc \
cargo test -p yolu-core --lib --target aarch64-unknown-linux-gnu --locked
```

QEMU는 IEEE 규칙대로 부동소수점 연산을 재현하므로 NEON 결과 바이트가 스칼라와 일치하는지 확인할 수 있습니다 (`YOLU_SIMD=scalar`·`YOLU_SIMD=neon`으로 경로를 고정해 실행해도 됨). QEMU로 속도를 측정할 수는 없으므로 실제 Mac에서 측정합니다 (아래 “픽셀 계산 SIMD”). 타입만 확인하려면 `cargo clippy -p yolu-core --all-targets --target aarch64-unknown-linux-gnu -- -D warnings`를 QEMU 없이 실행할 수 있습니다.

<a id="yolu-app-の結合テストの置き方"></a>

### yolu-app 통합 테스트 배치

`crates/yolu-app/tests/`의 테스트는 성격별로 여러 실행 파일 (“묶음”)에 모아 둡니다. 파일 1개마다 실행 파일 1개를 만들면 테스트 수만큼 앱 전체 링크와 공통 구성 요소 재빌드가 늘고 `target/`의 테스트 실행 파일만으로 10 GB를 초과하기 때문입니다.

| 묶음 (`cargo test -p yolu-app --test <束>`) | 내용 |
| --- | --- |
| `headless` | 창도 GPU 장치도 만들지 않는 테스트 (문서·레이어 세트·저장 (백그라운드 저장·선택 영역·포즈 포함)·가져오기·Live Link 폴더 교환과 요청·외부 조작 (MCP 수신부)·디스크 캐시·도구 순서·소스와 UI 문구 (.ftl) 검사). 병렬 실행 |
| `gui_canvas` | 캔버스·도구·브러시·선택·색상·효과 UI와 기본 패널 배치 (`egui_kittest`) |
| `gui_shell` | 창 전체·메뉴·설정 창과 단축키 편집·패널 배치와 별도 창·문서 입출력 (PSD 드롭 포함)·종료 흐름과 저장 도중 종료·GPU 장치 손실·복구·업데이트·Live Link UI와 수신 상한·언어·에셋·라이브러리 |
| `gui_view3d` | 3D 보기 (앤티앨리어싱·블룸·시점 조작·여러 보기·3D 위 선택과 그리기 포함)·모드 (페인트/편집/포즈/숨기기)·머티리얼 외관 (lilToon 포함)·포즈·텍스처 세트·출력 |
| `threads`·`window_lease`·`windowpos` (바로 아래에 파일 1개당 실행 파일 1개) | 프로세스 전체 상태를 다루는 테스트: rayon 전역 풀·창 대여 수 계산·저장된 창 위치 |

묶음 내부 파일은 `tests/<束>/<名前>.rs`, 묶음 진입점은 `tests/<束>/main.rs`의 `mod` 배열입니다. 테스트 이름은 `<ファイル名>::<テスト名>`이므로 파일명이나 테스트 이름으로 필터링할 수 있습니다.

```sh
cargo test -p yolu-app --test gui_shell                      # 1 つの束
cargo test -p yolu-app --test gui_shell i18n::               # 束の中の 1 ファイル（以前の `--test i18n`）
cargo test -p yolu-app --test headless no_instruction_text:: # 以前の `--test no_instruction_text`
cargo test -p yolu-app --test gui_canvas layerops::undo      # ファイル名とテスト名の一部
cargo test -p yolu-app -- --list | grep layerops             # どの束にあるか（`Running tests/<束>/main.rs` の下）
```

- **새 테스트 추가**: UI를 만들지 않으면 `headless/`, 만든다면 (`common::app`·`common::gpu_thread::builder`) 내용과 가까운 `gui_*/`에 파일을 두고 묶음의 `main.rs`에 `mod 名前;`를 1행 추가합니다. 추가 누락은 `headless/bundle_layout.rs`가 실패해 알립니다 (파일만 두면 빌드되지 않아 실행하지 않았는데도 통과하기 때문). 파일 맨 위에 `use crate::common;`을 쓰면 공통 구성 요소 (`tests/common/`)를 `common::…`로 사용할 수 있습니다.
- 바로 아래 (`tests/<名前>.rs`)의 파일 1개당 실행 파일 1개인 테스트를 묶음으로 옮기려면 `git mv tests/<名前>.rs tests/<束>/<名前>.rs`를 실행하고, 파일 맨 위의 `mod common;`을 `use crate::common;`으로 바꾼 뒤 묶음의 `main.rs`에 `mod <名前>;`을 추가합니다. 주석과 문서의 `--test <名前>`도 `--test <束> <名前>::`로 수정합니다 (테스트 이름은 `<名前>::<元の名前>`이 됨).
- **바로 아래에 파일 1개당 실행 파일 1개로 두는 것**은 프로세스 전체 상태 (rayon 전역 풀·환경 변수·창 대여 수·저장된 창 위치)를 바꾸거나 계산하는 테스트뿐입니다. 묶음 안 테스트는 같은 프로세스에서 실행되므로 이런 테스트를 섞으면 순서에 따라 결과가 달라집니다.
- **창 (harness)은 반드시 `common::gpu_thread::builder()`로 만듭니다** (`Harness::builder` 등을 직접 쓰면 `window_lease`가 실패). 창이 있는 테스트는 대여를 통해 1개씩 실행합니다 (lavapipe에서 동시에 장치를 만들면 충돌한 사례가 있기 때문). 렌더링 설정은 `common::app` 또는 `.renderer(common::shared_gpu::renderer())`를 사용합니다 (`.wgpu()`는 창마다 장치와 3D 파이프라인을 다시 빌드하므로 사용하지 않음. 예외는 장치를 파기하거나 오류 수신부를 붙여 공유 장치를 망가뜨리는 `gpu_lost`, 제품과 같은 장치 설정이 필요한 `view3d_fx`로, 대여 범위 안에서 자체 장치를 생성). GPU 연결은 프로세스당 1개를 공유하고 `Renderer`·텍스처·3D 이미지는 창마다 다시 만듭니다. 창을 만들지 않아도 GPU 장치를 만드는 테스트 (제품 스레드에서 GPU 확인·베이크를 수행하는 테스트, `common::canvas_device::begin`)는 시작 시 `common::gpu_thread::lease()`를 얻습니다 (`canvas_device::begin`은 내부에서 획득).
- `crates/yolu-gpu/tests/`의 GPU 테스트는 장치 (`GpuPainter::new` 등)를 만들기 전에 `support::gpu_lease::lease()`를 호출합니다. 같은 실행 파일의 다른 테스트 스레드에서 장치를 동시에 만들어 사용하면 lavapipe 내부에서 프로세스 전체가 충돌한 사례가 있기 때문입니다 (테스트 1개가 장치를 여러 개 만들어도 대여 1회면 충분).
- **UI 테스트는 AppState 필드를 직접 조작하지 않고** 구동 인터페이스 (`tests/common/drive.rs`의 `Drive`, `h.doc()`·`h.act(…)` 등)를 사용합니다. 필드 위치가 바뀌어도 이 파일만 수정하면 됩니다. 화면 없는 테스트 (`AppState`)에도 제공됩니다.
- **임시 폴더**는 `common::tmp::test_dir(タグ)` (테스트 종료 시 삭제)를 사용하거나 직접 만든 위치에서 `common::tmp::clean_up_after_test(&dir)`를 호출합니다. Live Link 교환 폴더는 `common::livelink::Exchange::new(タグ)`가 `common::tmp::test_dir` 아래에 만들어 요청·응답과 함께 테스트 종료 시 삭제됩니다. 조사용으로 남기려면 `YOLUPAINTER_KEEP_TEST_FILES=1`을 지정합니다.
- “다시 쓰지 않음”을 수정 시각으로 검증할 때는 `common::tmp::backdate(&path)`로 수정 시각을 조금 이전으로 설정한 뒤 비교합니다 (시각 해상도보다 빠른 재기록을 놓치지 않고 `sleep`을 기다리지 않음).
- 테스트가 자신의 실행 파일을 자식 프로세스로 시작할 때 (`--exact`에 테스트 이름 전달), 묶음 안에서는 이름에 모듈 경로 (`livelink::child_unity`)가 붙습니다. `module_path!()`로 만들어 주세요.

같은 묶음·테스트를 연속 실행해 불안정성을 조사하려면 `tools/repeat-tests.py`를 사용합니다 (`--shuffle`은 매회 테스트 순서를 섞어 순서 의존성도 확인).

```sh
tools/repeat-tests.py --rounds 20 --out /tmp/repeat --test gui_canvas --test gui_shell --test gui_view3d
tools/repeat-tests.py --rounds 20 --out /tmp/repeat --shuffle --test headless
```

재빌드 시간의 대부분은 yolu-app 라이브러리 1개의 rustc 처리입니다 (리프 파일 1개 변경 후: `cargo check` 약 6초, `cargo build` 약 20초, `cargo test -p yolu-app --test headless --no-run` 약 25초. 링크는 실행 파일 1개당 약 1초). 환경 변수 `CARGO_INCREMENTAL=0`을 지정하지 않았다면 dev 빌드는 증분 방식이며 같은 변경 후 재빌드는 3〜5초입니다 (캐시는 `target/debug/incremental`에 약 1.4 GB. 디스크 여유가 부족할 때만 0으로 설정).

<a id="yolu-coreyolu-io-の結合テストの置き方"></a>

### yolu-core·yolu-io 통합 테스트 배치

yolu-app과 마찬가지로 `crates/yolu-core/tests/`·`crates/yolu-io/tests/` 테스트도 성격별 묶음 (`tests/<束>/main.rs`)에 모아 둡니다. 테스트 이름은 `<ファイル名>::<テスト名>`입니다.

| 크레이트 | 묶음 (`--test <束>`) | 내용 |
| --- | --- | --- |
| yolu-core | `edit` | 레이어·레이어 조작·선택 영역·복사와 붙여넣기·기록·채널·색상 보정·내보내기·2D 합성 |
| | `effects` | 필터·Generator·Anchor·경로·스마트 머티리얼 및 레이어 잠금·레이어 조작과의 연결부 |
| | `reference` | 실제 C# Core 정답·수록한 해시와 모든 바이트 대조 |
| | `surface` | 3D 표면 투영·표면 스트로크·대칭·메시 맵·브러시 참조 원본·스텐실·채널 페인팅 |
| | `brush`·`document`·`golden`·`mix`·`parallelism`·`paths`·`pressure` (바로 아래에 파일 1개당 실행 파일 1개) | 워커 임계값 (`yolu_core::brush::set_parallel_dab_pixels`, 프로세스당 1개)을 바꾸거나 해당 값에 의존해 브러시 자국 경로를 확인하는 테스트 |
| | `seam_memory` (바로 아래에 파일 1개당 실행 파일 1개) | 할당을 세는 `#[global_allocator]`를 두므로 실행 파일 1개로 분리. 이음새를 넘는 평가의 실제 할당이 추정치 안에 들어가는지 확인 |
| yolu-io | `brushes` | 브러시 가져오기 (동봉 팁·GIMP·Photoshop·CLIP STUDIO 형식·신뢰할 수 없는 파일) |
| | `psd_io` | PSD 내보내기·가져오기·베이크·보정 레이어·C# 정답 대조 |
| | `ylp` | .ylp 형식 읽기·쓰기·이전 버전 호환·거부 방식·C# 작성기와의 일치·형식 명세 문서 |
| | `projects` | 큰 .ylp·복구 세대·라이브러리·에셋·배포용 사본·동시 저장·새 프로젝트 |
| | `mesh` | 메시 맵 베이크·레이 탐색 (BVH) |

```sh
cargo test -p yolu-core --test reference                 # 1 つの束
cargo test -p yolu-core --test reference seam_golden::   # 束の中の 1 ファイル（以前の `--test seam_golden`）
cargo test -p yolu-io --test ylp format_doc::            # 以前の `--test format_doc`
```

- 추가 방법은 yolu-app과 같습니다 (내용과 가까운 묶음 폴더에 파일을 두고 `main.rs`에 `mod 名前;` 추가). 추가 누락과 이유 없이 바로 아래에 파일 1개당 실행 파일 1개 테스트가 늘어난 것은 `yolu-core/tests/edit/bundle_layout.rs`·`yolu-io/tests/ylp/bundle_layout.rs`가 실패해 알립니다.
- 공통 구성 요소 (`attach_support`·`golden_update`·`brush_files` 등)는 묶음의 `main.rs`에서 `#[path]`를 붙여 1회만 선언하고 파일에서는 `use crate::<部品>;`을 씁니다. `include_str!`·`include_bytes!` 경로는 파일 위치를 기준으로 계산합니다 (묶음 폴더보다 1단계 위가 `tests/`).
- 메모리 사용량 (VmHWM)을 측정하는 수동 테스트 (`#[ignore]`의 `bigdoc::measure`·`psd_stream::measure`)는 이름으로 필터링해 실행합니다 (같은 묶음의 다른 테스트와 같은 프로세스에서 동시에 돌리면 측정이 흐트러짐).

## CI

`.github/workflows/ci.yml`은 `pull_request`·`workflow_dispatch`로 시작합니다 (같은 브랜치의 이전 실행은 취소). `main` push에서는 실행하지 않습니다 (main은 CI를 통과한 PR로만 변경되며 push CI는 PR의 마지막 CI와 같은 내용을 다시 빌드할 뿐이기 때문). 대신 `.github/workflows/main-tested.yml`이 main의 내용 (파일 트리)이 PR의 마지막 CI를 통과한 내용과 같은지, 해당 CI가 성공했는지만 확인합니다 (README의 CI 표시는 이 결과). 다르면 (PR 브랜치가 main보다 오래된 상태로 통합됨) 실패하므로 `ci.yml`을 수동 실행합니다. main 대상 PR에서는 테스트 작업과 함께 배포물 빌드 (`dist-plan` → `dist`, `.github/workflows/dist-build.yml`)도 실행합니다. 배포는 그 산출물을 받습니다 ([RELEASING.md](RELEASING.md#配る物をビルドする場所と受け取る道)). 외부 Actions는 커밋 SHA로 고정하고 버전 이름은 주석에 적습니다. 업데이트할 때는 해당 버전 태그가 가리키는 커밋을 확인한 뒤 SHA를 바꿉니다.

- Linux 테스트와 정적 검사 (`ubuntu-24.04`): `cargo test --workspace --exclude yolu-app --exclude yolu-gpu --exclude yolu-mock --locked --no-fail-fast` (렌더링하지 않는 크레이트, 기본 병렬 테스트), 배포 도구 테스트, `cargo clippy --workspace --all-targets --locked -- -D warnings`, `cargo fmt --all -- --check`.
- Linux 화면 테스트 (`ubuntu-24.04`): Xvfb와 Mesa lavapipe (`WGPU_BACKEND=vulkan`) 소프트웨어 렌더링으로 yolu-gpu·yolu-app 테스트를 `tools/render-tests.py`가 실행합니다. 테스트 실행 파일별 별도 프로세스를 3개 레인에 나누고 (레인 안에서는 순서대로), 프로세스 내부는 `--test-threads=1`입니다 (같은 프로세스에서 창·GPU 장치를 동시에 만들면 lavapipe에서 충돌한 사례가 있기 때문. 프로세스끼리는 별도 장치). `YOLUPAINTER_REQUIRE_GPU=1`을 지정하므로 어댑터를 얻지 못하면 GPU 테스트를 건너뛰지 않고 실패합니다. 전체 실행을 20분 (`--time-limit`)으로 제한하고 초과한 테스트는 프로세스 그룹 전체를 종료해 실패 처리합니다. 멈춘 테스트 이름을 알 수 있도록 로그 끝부분을 즉시 출력합니다 (해당 레인의 나머지는 “실행하지 않음”으로 실패 목록에 포함). 단위 테스트는 `--lib`·`--bins`, 문서 테스트는 `--doc`로 실행하며, 어느 쪽에서도 실행되지 않는 테스트 target (example·bench의 `test = true`)이 있으면 분배 전에 중단합니다. 로컬에서 같은 방식으로 실행하려면 `xvfb-run -a tools/render-tests.py --log-dir <フォルダ>`를 사용합니다 (`--lanes 1`이면 1개씩). 이어 개발용 목업 (`yolu-mock`, 배포하지 않는 크레이트) 테스트를 `cargo test -p yolu-mock --locked -- --test-threads=1`로 실행합니다 (GPU로 화면을 캡처하므로 비렌더링 크레이트 단계에서 제외해 여기에 배치. 앞 단계 빌드를 사용하고 창은 열지 않으므로 xvfb 불필요). 수록된 정답이 glibc와 Mesa 버전에 연결되므로 runner의 Ubuntu 버전을 고정합니다.
- Linux 통신 감시 (`ubuntu-24.04`, 화면 테스트와 별도 작업): 실제 앱을 `strace` 아래에서 실행하고 `tools/network-watch.py`로 외부 통신 시도가 없음을 확인합니다 ([통신 감시](#通信の見張り)의 단계 2). 별도 작업인 이유는 업데이트 확인을 실행하려면 업데이트용 공개 키 (일회용)를 내장해 빌드해야 하고, 해당 환경 변수를 바꾸면 yolu-update와 yolu-app을 다시 빌드하기 때문입니다 (같은 작업에 두면 화면 테스트 산출물 캐시가 다음 화면 테스트 때 재빌드를 유발). 캐시는 화면 테스트 작업과 공유 키 (`shared-key: linux-render`)로 읽기만 하며 (`save-if: false`) 키를 내장한 산출물로 덮어쓰지 않습니다. xdotool이 작동하지 않아 장면을 건너뛰면 마지막에 `::warning::`를 표시합니다. 도구 테스트 (`tools/test-network-watch.py`)는 “Linux 테스트와 정적 검사”에서도 실행합니다.
- cargo-deny (`ubuntu-24.04`, `.github/workflows/cargo-deny.yml`): 주 1회 (월요일 03:23 UTC)와 수동 실행만 하며 매번 CI에 포함하지 않습니다. `deny.toml` 규칙으로 advisories (알려진 취약점·철회 버전)·bans·licenses·sources를 검사합니다.
  (로컬에서는 `cargo deny --locked check advisories bans licenses sources`). cargo-deny는 버전을 고정해 `cargo install`합니다 (`deny.toml` 형식이 버전에 의존하므로 업데이트 시 함께 확인).
  `paste` (RUSTSEC-2024-0436, 유지보수 종료)는 egui_dock이 빌드 시에만 쓰는 매크로이며 취약점이 아니므로 이유를 적어 무시합니다. `yolu-mock`은 배포하지 않는 크레이트이므로 `deny.toml`의 `exclude`로 의존성까지 검사에서 제외합니다 (배포물은 `cargo xtask preflight`가 확인). 발견한 취약점은 업데이트로 수정 가능한지, 실제 사용 방식에 해당하지 않는지 확인하고 후자만 이유와 함께 `ignore`에 추가합니다.
- Windows (`windows-latest`, MSVC): `cargo build -p yolu-app -p yolu-cli --locked`, core·io·brush-sets·protocol·ops·cli 테스트, app의 `--lib`와 묶음 내부 `headless_` 테스트 (`--test gui_shell -- update::headless_`·`--test headless -- brush_list::headless_ recovery::headless_ livelink_files::headless_ saved_selections::headless_ pose_saved::headless_`, 복구 OS 잠금과 교체·Live Link 교환 폴더와 파일 교체·.ylp 교체 포함). GPU·화면 통합 테스트는 대상이 아닙니다.
- macOS (`macos-14`, Apple Silicon): Windows 작업 전반부와 같은 절차로 `cargo build -p yolu-app -p yolu-cli --locked`, core·io·brush-sets·protocol·ops·cli 테스트, app의 `--lib`를 실행합니다. 이어 묶음의 `headless_` 테스트 중 `--test headless -- recovery::headless_ livelink_files::headless_` (복구 잠금, Live Link 교환 폴더와 파일 교체)만 실행합니다 (업데이트·브러시 목록·선택 영역·포즈 저장의 `headless_`는 이 작업에서 실행하지 않음. Linux 화면 테스트가 Unix에서 실행). macOS 태블릿 입력 (`pen::mac_tablet`) 빌드 가능 여부, 값 보정 (`pen::tablet`, OS 독립이므로 Linux에서도 실행), NSEvent 상수 일치 테스트 통과 여부를 확인합니다. GPU·화면 통합 테스트는 대상이 아닙니다. 배포물 (.app와 ad-hoc 서명)은 이 작업에서 만들지 않고 `dist` 작업 (`dist-build.yml`)이 macOS runner에서 만듭니다 ([배포 절차](RELEASING.md#macos-の試作)).
- 모든 OS에서 [Swatinem/rust-cache](https://github.com/Swatinem/rust-cache)를 사용하며 같은 브랜치의 이전 CI는 후속 실행이 취소합니다.

0.4.x까지의 Unity 브리지 (C#)를 실행하는 정답 재생성·대조에는 해당 구현 소스와 Unity에 포함된 .NET·Mono가 필요하므로 이 CI에서는 실행하지 않습니다. 수록된 인공 데이터를 사용하는 Rust 대조 테스트는 일반 `cargo test`에 포함됩니다.

CI 정의는 `actionlint .github/workflows/*.yml`로 실행 없이 검사할 수 있습니다. runner 버전을 올렸다면 Ubuntu Mesa 버전에 따른 화면 정답 차이를 확인하세요. 소프트웨어 렌더링 결과는 실제 Windows의 렌더링·펜 태블릿·Unity 연결 확인을 대신하지 않습니다.

<a id="画面の文字とパネルだけの確かめ"></a>

## UI 문구와 패널만 확인하기

UI 문구의 일본어·영어·한국어는 `crates/yolu-app/locales/{ja,en,ko}/<모듈>.ftl` (Fluent)에 두며 코드는 키로 조회합니다 (인자 없는 문장은 `Lang::t`, 인자 있는 문장은 `Lang::f`, core 거부 이유는 `Lang::reason`). 문구를 실행 파일에 내장하므로 배포물은 파일 1개를 유지합니다.
일본어·영어·한국어의 키 집합과 인자 이름이 같은지, 코드가 조회하는 키가 세 언어 모두에 있는지, 어디에서도 조회하지 않는 키가 없는지는 `cargo test -p yolu-app --test headless ftl_keys::`가 확인합니다. 기존 두 언어 값의 선택은 `Lang::pick(ja, en)`를 유지하며 한국어에서는 영어 값을 사용합니다. 새 UI 문구는 한국어 카탈로그에도 등록합니다. 한국어 회귀는 `korean_localization`과 `gui_shell::korean`, 글꼴 정렬 검사에서 확인합니다.

UI를 만들거나 수정할 때 외관 확인에는 개발용 크레이트 `yolu-mock` (배포하지 않음)을 사용합니다. 실제 구성 요소 (`yolu_app::ui`)와 더미 값으로 패널이나 초안을 열거나 PNG로 캡처하는 진입점입니다.

```sh
cargo run -p yolu-mock -- --list                                      # 開ける物（パネル・下書き・見本帳）
cargo run -p yolu-mock -- --panel layers --png /tmp/layers.png         # 本物のパネルを 1 つだけ撮る（--lang en・--size 360x720・--scale 2・--scene empty|layers）
cargo run -p yolu-mock -- --gallery --png /tmp/gallery.png             # 今ある部品の見本帳
```

설정·키 설정·복구·로그 파일은 읽거나 쓰지 않습니다. 에셋 라이브러리는 실행마다 임시 폴더에 만드는 빈 라이브러리입니다.

<a id="通信の見張り"></a>

## 통신 감시

앱이 외부와 통신하는 경우는 사용자가 선택한 업데이트 확인뿐입니다([README](../../README.md)·[INSTALL](INSTALL.md)에 명시한 약속). 외부 조작은 설정에서 켜 둔 동안에만 `127.0.0.1`에서 요청을 기다립니다.
새 코드나 의존성을 추가하면서 이 약속이 조용히 깨지지 않도록, 코드와 의존성을 검사하는 감시(1단계)를 매 CI에서 실행합니다.

통신 구성 요소 감시(`cargo xtask netguard`. 매 CI에서 실행)와 cargo-deny(주 1회)는 역할이 다릅니다. netguard는 “앱이 외부와 통신하는 구성 요소가 들어 있지 않은가”를 소스와 의존성 소스의 토큰을 통해 검사합니다
(라이선스나 취약점은 검사하지 않습니다). cargo-deny는 “포함된 의존성에 알려진 취약점·허용하지 않은 라이선스·알 수 없는 출처(registry 외부·git)가 없는가”를 검사합니다(통신 여부는 검사하지 않습니다).
어느 한쪽을 통과해도 다른 한쪽을 대신할 수는 없습니다.

<a id="段-1-コードと依存を読む"></a>

### 1단계: 코드와 의존성 검사

```sh
cargo xtask netguard         # 許す一覧の使われ方の表を出す。一覧の外の出口や依存の問題があれば終了コード 1
cargo test -p xtask netguard # 同じ確かめ（CI では `cargo test --workspace --exclude yolu-app --exclude yolu-gpu` の中で回る）と、見張りが効くことのテスト
```

구현은 `crates/xtask/src/netguard/`에 있습니다(`mod.rs`: 전체 흐름과 결과 표, `lex.rs`: 토큰 분석, `markers.rs`: 감지 표식과 목적지, `allowed.rs`: 허용 목록, `deps.rs`: 의존성, `uses.rs`: `use` 분석, `tests.rs`: 감시가 작동하는지 확인하는 테스트).

- 소스: `crates/*/src/**/*.rs`와 `crates/*/build.rs`(배포하지 않는 xtask 제외)에서 통신 및 외부 출구를 나타내는 표식(`MARKERS`)을 찾습니다. 소켓 타입(`TcpStream`·`TcpListener`·`UdpSocket`·`std::net`·`tokio::net`·`rustix::net` 등),
  HTTP·TLS·WebSocket 구성 요소(`hyper`·`reqwest`·`WinHttp*`·`WinInet`·`Urlmon` 등), 프로세스 실행과 “열기” 호출(`Command::new`·`tokio::process`·`ShellExecute*`·`xdg-open`·`open::that`·egui의 `open_url`/`hyperlink`),
  `http://`·`https://`가 포함된 문자열이 대상입니다. `Command::new`는 `use std::process::Command as Cmd;` 같은 별칭이나 `tokio::process`에서도 찾으며, 자체 정의한 `Command` 타입은 표식으로 취급하지 않습니다.
  주석과 문자열 안의 식별자에는 반응하지 않습니다. `#[cfg(test)]`가 붙은 항목, `#[cfg(test)] mod x;`(`#[path = "…"]`가 있으면 그 경로의 파일), 모듈 안의 `#![cfg(test)]`는 배포물에 들어가지 않으므로 제외합니다.
  `TcpListener::bind`·`TcpStream::connect`처럼 이름에 `Listener`·`Stream`·`Socket`이 들어가는 타입 및 그 별칭(`use … as L;`·`type S = …;`)의 `bind`·`connect`는 목적지가 127.0.0.1로 확정되는 형식이 아니면 실패합니다(허용 목록에 추가해도 통과하지 않습니다).
  허용하는 형식은 문자열(`"127.0.0.1:0"`·`"[::1]:0"`·`"localhost:80"`), `(host, port)` 쌍, `SocketAddr::from/new/V4` 등의 래퍼이며, 호스트는 문자열이나 `Ipv4Addr::LOCALHOST`·`Ipv6Addr::LOOPBACK` 같은 상수여야 합니다(`IpAddr::V4(…)` 래퍼도 허용).
  변수·`unwrap_or`·`if`·`match`·`UNSPECIFIED`·`"0.0.0.0"`은 목적지가 확정되지 않으므로, 목적지를 호출 인수에 직접 적습니다.
- 의존성: `Cargo.lock`에 HTTP 클라이언트·TLS·WebSocket/QUIC·이름 해석·텔레메트리·서버 프레임워크 이름(`reqwest`·`ureq`·`rustls`·`quinn`·`hickory-resolver`·`sentry` 등)이 들어가면 실패합니다.
  `cargo metadata`(`--locked`. 미리 받은 의존성이 부족하면 다운로드를 허용해 다시 읽음)로 `hyper`·`hyper-util`·`socket2`·`mio`를 직접 의존성으로 가지며 tokio의 `net`·`full`·`process` 기능을 사용하는 크레이트가 yolu-mcp뿐인지,
  배포 앱(yolu-app·yolu-cli)의 의존성에 `open`·`opener`·`webbrowser`가 들어 있지 않은지(현재 `open`은 테스트용 의존성에만 있음), `socket2`·`mio`·`hyper`·`hyper-util`에 의존하는 크레이트가 정해진 범위(`NET_PARTS`)보다 늘어나지 않았는지,
  Windows API 기능(`windows`·`windows-sys`)의 통신 기능(`Win32_Networking*`·`Win32_NetworkManagement*`·`Networking*`·`Web*`·`Win32_Web*`·`Win32_System_Com_Urlmon`·`Win32_Data_Xml_MsXml`)이 선언과 Cargo가 실제로 활성화한 기능
  (tokio·mio·socket2가 추가하는 WinSock·IpHelper 포함) 모두에서 정해진 범위(`WINDOWS_RESOLVED_NET`)보다 늘어나지 않았는지 확인합니다.
  배포 앱 의존성(프로덕션 및 빌드 스크립트의 전이 의존성 전체. 테스트 전용 의존성은 제외)의 소스도 같은 토큰 분석기로 읽습니다(현재 약 440개 크레이트). 소켓·이름 해석·HTTP 구성 요소의 표식을 사용하는 크레이트는 이유를 적은 목록
  (`SOCKET_CRATES`. 현재 25개 이름)에 등록하며, 목록에 없는 크레이트가 사용하기 시작하면 실패합니다. windows의 선언 모음과 문자열·프로세스 실행 표식은 검사하지 않습니다.

실패 메시지의 형식은 `<ファイル>:<行> の <印> が、許す一覧にありません`입니다.

허용 목록(`ALLOWED`)에는 파일과 표식별로 횟수(`Exactly`: 늘어나면 실패. `AtLeastOne`: 같은 종류의 식별자가 여러 개 나열되는 경우에만 사용)와 이유를 적습니다. 횟수는 표식의 출현 횟수이며, `use` 행의 식별자도 1건으로 셉니다
(`use std::net::TcpStream;` 1행에서 `std::net`과 `TcpStream`을 각각 1건으로 계산). 추가할 때의 규칙:

- 먼저 README·INSTALL의 약속(업데이트 확인 외에는 통신하지 않음. 외부 조작은 설정에서 켜 둔 동안에만 `127.0.0.1` 사용)을 어기지 않는지 확인합니다. 어긴다면 목록에 추가하지 말고 약속 자체를 다시 결정합니다.
- 이유에는 어떤 용도의 출구인지(업데이트 확인·눌렀을 때만 브라우저나 파일 탐색기로 열기·`127.0.0.1` 수신 지점·로컬 빌드 등)를 적습니다.
- 소스에서 출구가 사라지면 목록에서도 해당 행을 삭제합니다(사용하지 않는 행도 실패합니다. `SOCKET_CRATES`는 의존성 업데이트로 달라지므로 사용하지 않게 되어도 실패하지 않습니다).
- 부족한 표식은 `MARKERS`에 추가하고 `SNIPPETS`(테스트용 가상 소스)에도 1개 추가합니다(양쪽의 표식 이름이 일치하지 않으면 테스트가 실패합니다).

감시가 읽는 대상은 소스와 의존성 소스·선언뿐입니다. 실제 연결 목적지는 검사하지 않습니다(2단계에서 검사). 매크로가 생성하는 코드는 추적하지 않으며, 기능(feature)에 의해 비활성화된 코드도 셉니다.

<a id="段-2-実際の通信を見るlinux-だけ"></a>

### 2단계: 실제 통신 검사(Linux만)

```sh
# 更新の確認は、更新用の公開鍵を組み込んだビルドでだけ動く。使い捨ての公開鍵を渡して作る（鍵を変えると yolu-update と yolu-app を作り直す）
YOLUPAINTER_UPDATE_PUBLIC_KEY=6acc8faaa5f8beb6897bc7936ab6b930853dc4e563e4dc5f49a76f156ab16201 cargo build -p yolu-app -p yolu-cli --locked
xvfb-run -a -s "-screen 0 1920x1080x24" python3 tools/network-watch.py --log-dir /tmp/network-logs
python3 tools/test-network-watch.py   # ツール自身のテスト（アプリは起動しない）
```

`strace`·`xdotool`·`xvfb`·Mesa의 lavapipe(화면 테스트와 동일)가 필요합니다. `--only main,reopen,idle,update`로 실행 대상을 좁힐 수 있습니다.

`tools/network-watch.py`는 앱(`yolupainter`)을 xvfb 안에서 `strace -f` 아래에 실행하고, CLI(`yolupainter-cli`, 이것도 `strace` 아래에서 실행)로 조작하며, `connect`·`bind`·`listen`·`sendto`·`sendmsg`·`sendmmsg`·
`execve`를 읽습니다. 연결 성공이나 거부 여부가 아닌 시도 자체로 판정하므로, CI에서 GitHub에 접속할 수 없어도 같은 결과가 나옵니다. 설정 폴더·내보내기 대상·Live Link 폴더는 실행마다 만든 임시 폴더입니다.

- 실행은 4회: `main`(수신 기능을 켜고 시작·업데이트 질문에 Esc로 “아니요” 선택·레이어와 효과 편집·`xdotool`로 브러시 조작·저장·PNG와 템플릿 및 PSD 내보내기·Live Link로 3D 모델 열기·저장·Ctrl+Q로 종료),
  `reopen`(저장한 3D 모델 포함 문서를 실행 인수로 다시 열기·샘플), `idle`(수신 기능을 켜지 않는 기본 설정. 업데이트 질문은 열린 채로 둠), `update`(업데이트 확인을 “사용”으로 설정해 실행).
  그리기 조작은 CLI·MCP 명령에 없으므로 브러시만 `xdotool`로 움직입니다(`xdotool`이 없거나 브러시 입력이 전달되지 않으면 해당 장면을 건너뛰고 표에 기록합니다).
- 실패 조건: PC 내부(AF_UNIX의 화면 및 D-Bus·AF_NETLINK)와 127.0.0.1·::1 이외의 대상으로 연결·전송(`sendto`·`sendmsg`·`sendmmsg`의 목적지는 스레드가 많으면 결과 행에만 나오므로 그곳에서도 읽음)·수신 대기.
  이름 해석(목적지 포트가 53·853·5353. 127.0.0.53 같은 루프백 중계와 systemd-resolved 및 avahi의 UNIX 소켓도 포함. glibc가 passwd 등에도 여는 nscd 소켓은 세지 않음).
  수신 기능을 끈 실행에서 수신 대기하는 경우, 켠 실행에서 127.0.0.1의 해당 포트 이외에서 대기하는 경우, 대기 호출이 1개도 발견되지 않는 경우. 업데이트 확인 장면 이외에서 `curl`을 실행하는 경우, 앱 충돌 대화 상자(zenity 등)를 실행하는 경우.
- 허용하는 통신: 업데이트 확인 장면(`update` 실행)에서 `curl`(하위 스레드 포함)의 외부 통신 시도. 단, curl 인수는 `update/http.rs`의 `curl_args`(단위 테스트 `curl_arguments_are_exactly_these`로 전체를 고정)와 같은 조합만 허용하며
  (`--data`·`-H`·`-b`·`-T`·`-K`·`--url` 등이 추가되면 실패), 가져올 URL은 `--` 뒤의 1개이며 `https://github.com/YozoraKurage/YoluPainter/` 아래에 있고 `?`·`#`·`@`가 없는 형식만 허용합니다.
  curl의 접속 대상 주소(GitHub 배포 대상 IP나 이름 해석)는 고정하지 않습니다. 다른 프로세스의 외부 통신 시도는 실패로 처리합니다.
- 먼저 외부로 connect·sendto·sendmmsg, 실제 이름 해석(`getaddrinfo`), 루프백 통신을 시도하는 작은 프로그램을 `strace`로 읽어 감시가 감지하는지 확인합니다(`strace`를 사용할 수 없는 환경이나 파싱 오류 때문에 앱 검사 결과가 아무것도 감지하지 못한 채 통과하지 않도록 합니다).
- 마지막으로 장면별 연결 횟수와 목적지 표를 출력합니다(실패하면 어느 실행·장면·프로세스·호출인지 제시합니다. 전체 로그는 `--log-dir`에 저장).

보장 범위: Linux의 `strace`로 볼 수 있는 시도(시스템 호출) 중 실행한 장면에 한정됩니다. Windows는 1단계(소스와 의존성 감시)만 있으며 2단계에 해당하는 테스트는 없습니다. 화면을 닫는 조작은 Ctrl+Q 키로 수행하며,
실행 중 외부 조작 수신 기능을 켰다가 끄는 조작은 수행하지 않습니다(켠 실행과 끈 실행을 따로 검사합니다. 실행 중 켜고 끄는 동작은 `crates/yolu-app/tests/headless/mcp_server.rs`의 테스트가 127.0.0.1에서 확인합니다).

<a id="windows-のインストーラーの画面なしテストwine"></a>

## Windows 설치 프로그램의 헤드리스 테스트(Wine)

Windows 설치 프로그램(NSIS)을 화면 없이 실행하는 테스트는 `python3 tools/test-installer.py`입니다(Wine·MinGW-w64·`makensis` 필요. 자세한 내용은 [RELEASING](RELEASING.md#windows-のインストーラー) 참조).

<a id="配布用の許諾全文"></a>

## 배포용 라이선스 전문

Python 3.10 이상과 Cargo를 사용합니다.

```sh
python3 tools/third-party.py --bundle --offline
```

처음 실행하는 등 의존성이나 원문을 미리 받지 않은 경우에는 `--offline`을 빼고 실행합니다. 대조에 성공하면 `target/third-party/<クレート>/THIRD_PARTY_LICENSES.txt`가 생성되므로 해당 배포물에 포함합니다. 추가 Python 패키지는 필요하지 않습니다.

`--target x86_64-pc-windows-msvc`·`--target x86_64-unknown-linux-gnu`·`--target aarch64-unknown-linux-gnu`(Linux ARM. 시험 버전)·`--target universal-apple-darwin`(macOS 배포물. `aarch64-apple-darwin`과 `x86_64-apple-darwin` 의존성의 합집합. 이 2개를 개별 지정할 수도 있음) 중 하나를 지정하면 대상별로 대조하여 `target/third-party/<target>/<クレート>/`에 출력합니다. 생략하면 기존 Windows GNU를 대상으로 합니다. `--package yolu-update`로 업데이트 크레이트도 확인할 수 있습니다. 배포 세부 사항은 [RELEASING](RELEASING.md)을 참고하세요. `tools/licenses-reviewed.json`에 원문과 SHA-256을 기록하며, 원문 누락·변경이나 확인하지 않은 버전·라이선스가 있으면 생성에 실패합니다. 제품별 범위와 크레이트 이외의 고지는 [THIRD_PARTY.md](../../THIRD_PARTY.ko.md)에 있습니다.

<a id="画素の計算の-simd"></a>

## 픽셀 계산의 SIMD

x86_64에서는 합성(Normal 채널 포함)·조정 레이어·필터의 픽셀 계산에 AVX2(및 FMA)·SSE4.1을 사용하며, 실행 시 CPU가 지원하는 가장 넓은 경로를 선택합니다
(Windows 배포물도 동일). aarch64(Apple Silicon Mac 등)에서는 NEON(f64 2레인·f32 4레인)을 사용합니다. NEON은 모든 aarch64 CPU에 있으므로 실행 시 판정하지 않습니다.
그 밖의 CPU는 기존 픽셀별 계산을 사용합니다. 어느 경로든 결과 바이트는 같으며, 테스트에서 각 경로를 픽셀별 수식과 비교합니다.
환경 변수 `YOLU_SIMD`(x86_64는 `scalar`·`sse41`·`avx2`, aarch64는 `scalar`·`neon`)로 더 좁은 경로로 낮출 수 있습니다(CPU가 지원하지 않는 넓은 경로로 올리지는 않음. 다른 CPU의 경로 이름은 알 수 없는 값으로 간주해 무시).
2D 합성(사각형·타일 묶음·간격을 둔 저해상도 합성·그룹 출력)은 이 행 단위 커널로 중첩합니다. 기준 구현인 `composite_pixel`(픽셀별 수식)과 바이트가 같으며, 테스트에서 모든 모드·마스크·클리핑·그룹·조정 레이어·Normal 채널을 경로별로 비교합니다.
표시에 기여하는 레이어·여러 레이어·그룹의 병합도 타일 합성으로 굽습니다(아래 레이어와 병합하는 `merge_down`은 아래 레이어의 픽셀을 바탕으로 삼는 방식이 있으므로 픽셀별 수식을 유지).

`cargo run --release -p yolu-core --example simd_bench [blend|adjust|filter|kernel|all] [回数]`로 혼합 모드·조정 종류·필터별 시간
(1타일과 4096². `kernel`은 행 단위 커널만의 ns/픽셀)을 측정합니다. 스레드는 `SIMD_THREADS`(기본 1), 이름 필터는 `SIMD_FILTER`(쉼표로 구분)로 지정합니다.
Mac(Apple Silicon)에서 NEON의 효과는 같은 명령을 `YOLU_SIMD=scalar`와 기본값(NEON)으로 실행해 비교합니다. 출력 맨 앞의 경로 이름을 통해 어떤 경로로 측정했는지 알 수 있습니다.
간격을 둔 저해상도 합성(드래그 중 임시 이미지)은 `cargo run --release -p yolu-core --example coarse_bench [回数]`로 변 길이 × 레이어 수 × 간격 × 스레드 수 조합별 1회 시간을 측정합니다
(조합은 `COARSE_SCENE`·`COARSE_SIZE`·`COARSE_LAYERS`·`COARSE_STRIDE`, 묶음의 타일 수는 `COARSE_CHUNK`).

2D 브러시 자국의 픽셀(원형·브러시 끝 이미지·종이 질감·듀얼·스머지·흐림·복제·색 혼합·자국별 색상)도 같은 경로에서 행별로 레인을 사용해 그립니다.
기준은 픽셀별 수식(`YOLU_SIMD=scalar`)이며, 테스트에서 무작위로 정한 브러시·레이어·타일 크기·점열을 경로별로 그려 레이어의 전체 바이트와 브러시 자국 수를 비교합니다.
선택 영역·투명 영역 잠금은 색을 칠하거나 지우기만 하는 브러시에서는 행 단위 커널을, 픽셀별 색상·효과 브러시에서는 픽셀별 수식을 사용합니다. 스텐실·곱하기 이외의 종이 질감·3D 표면의 브러시 자국도 픽셀별 수식을 유지합니다. 브러시 자국을 타일별 워커로 그릴지는
경계 상자의 크기에 픽셀별 예상 시간(브러시 종류에 따라 결정)을 곱해 결정합니다. 빠른 브러시의 큰 자국은 워커를 깨우는 비용이 더 크므로 직렬로 그립니다.

`cargo run --release -p yolu-core --example stroke_bench -- --threads 1`로 브러시 종류 × 크기 × 간격별로 4096² 문서에 1스트로크를 그리는 시간을 측정합니다
(`--threads 1`은 CPU 시간, 2 이상은 실제 경과 시간. `--features stroke-profile`을 추가하면 단계별 시간도 출력). 펜 입력이 화면에 나타날 때까지의 지연은
`cargo run --release -p yolu-app --example stroke_latency`, 가져온 브러시는 `cargo run --release -p yolu-io --example stroke_bench_imported -- --bundled 6`으로 측정합니다.

3D 뷰의 브러시 자국은 `cargo run --release -p yolu-core --features stroke-profile --example surface_dab_bench -- --threads 8`로 합성 구(3,888개 및 202,800개 삼각형)·문서 2048² 및 4096²·브러시 크기 64·128·256 각각에 대해 빠른 스트로크의
자국 1개 시간(중앙값·최댓값)과 세부 시간(투영 페인트·구역 재생성·커버리지 수집·문서에 페인트), 같은 문서·크기의 2D 브러시 자국과의 비교, 1프레임 페인트 시간 제한(`--budget`, 밀리초)으로 그렸을 때의 프레임 시간과 펜을 놓은 뒤 남은 작업을 마칠 때까지의 시간을 측정합니다.

<a id="機能ごとのテストと実装の置き場"></a>

## 기능별 테스트와 구현 위치

사용자용 문서에 있던 테스트 명령·확인 범위·구현 위치를 이곳에 모았습니다.

<a id="復旧docsrecoverymdのテスト"></a>

### 복구(docs/RECOVERY.md) 테스트

```text
cargo test -p yolu-io --test projects generation::   世代の置き場（量の数え方・書く前の空きの確かめ）
cargo test -p yolu-app --lib recovery::        上限の消し方・空きの守り・設定
cargo test -p yolu-app --test headless recovery::   書き置きと上限・空きの守り・ウィンドウの数え直し
cargo test -p yolu-app --test gui_shell recovery_ui::   ウィンドウ（使う量・詳しく・使用中）
```

여유 공간 보호는 테스트에서 여유 공간을 가짜로 지정해 확인합니다. 실제 디스크를 가득 채운 테스트는 하지 않았습니다. Windows의 `GetDiskFreeSpaceExW` 호출은 `x86_64-pc-windows-gnu` 대상으로 컴파일되는 것까지만 확인했으며, 실제 기기에서는 확인하지 않았습니다.

<a id="ブラシの取り込みの確かめた範囲docsbrush_importmd"></a>

### 브러시 가져오기에서 확인한 범위(docs/BRUSH_IMPORT.md)

- 합성 `.sut`로 확인한 사항: 설정 복사, 이미지(미리보기·평면 PNG·내장 PNG) 추출과 커버리지, 여러 브러시 끝의 순서, 이름으로 정할 수 없을 때의 나열 순서,
  질감, 영향 원인(필압의 최솟값과 곡선, 기울기·속도·무작위, 읽을 수 없거나 너무 큰 BLOB), 플래그 안내, 테이블 누락, `VariantID` 열 누락, 노드·소재 수 상한,
  브러시 끝 일부가 누락되었을 때의 알림, 순서로 대응시키는 조건(소재와 참조 수가 정확히 같음)과 추정 알림, 브러시 끝 256개에서 중단, 명령 수와 시간 상한
  (상한을 낮춰 도달시키는 테스트와 실제 상한에서 거부되는 스캔이 많은 파일), SQL의 `load_extension()` 거부, 픽셀 예산,
  뷰·가상 테이블 거부, 읽지 않는 생성 열을 평가하지 않음, WAL 표식, 약 32만 가지 손상 방식(모든 바이트 덮어쓰기·잘라내기·극단적인 값) 및 손상 방식으로는 만들 수 없는
  형식이 올바른 극단적인 파일(참조 수가 상한에 가득 찬 경우·소재 수가 상한에 도달한 경우 등).
  Windows 대상(GNU) 빌드를 Wine에서 실행한 테스트도 통과했습니다(실제 Windows 기기에서는 확인하지 않음).
- 합성 `.sut`로 확인한 사항(시작·끝 테이퍼·기울기·브러시 끝 방향·손떨림 보정. `tests/brushes/brush_import_sut.rs`): 테이퍼 길이가 픽셀 단위로 선 보조 기능에 복사됨·한쪽만 적용·길이 0·상한으로 제한·
  영향 대상(종류·대상 플래그·BLOB 형식)이나 단위 또는 길이가 맞지 않으면 복사하지 않고 알림, 기울기가 크기·불투명도·흐름의 전환으로 복사됨(직선은 안내 없음·곡선 형상은
  근사 안내·상승 곡선과 굵기 및 속도의 병용은 복사하지 않음·곡선 슬롯이 없거나 길이가 맞지 않는 헤더는 복사하지 않음), 슬롯으로 필압 곡선 선택(기울기 곡선을 필압으로 잘못 해석하지 않음),
  브러시 끝 각도의 순환 처리·각도 무작위 플래그와 강도·방향의 필압/기울기/속도 안내, 손떨림 보정 단계의 복사와 범위, 정수 열은 항상 백분율임(경도 1은 1%), 복사한 항목의 순서와 중복 없음.
- 합성 `.sut`로 확인한 사항(브러시 끝·질감·영향 원인·보정 이외의 설정. `tests/brushes/brush_import_sut.rs`·`brush_import_sut_refer.rs`): 브러시 끝 반복 방식 6가지와 반전 4가지·브러시 끝끼리 어둡게 비교·질감 회전과 밝기·대비 베이크,
  연속 분사와 리본(연속 분사와 후처리 보정이 동시에 켜졌으면 후처리 보정을 끈 상태로 복사하고 알림), 영향 원인의 모든 입력 최솟값·기울기 최댓값, 테이퍼 영향 대상(복사 가능한 대상의 최솟값, 복사할 수 없는 대상·백분율 지정·페이드 알림),
  보정(속도에 따른 손떨림 보정 조정·후처리 보정·끝 처리), 색상 변화(부호 있는 최솟값 포함), 산포 효과, 듀얼 브러시(혼합 모드 13종·2번째 브러시 끝·질감·산포·방향·영향 원인), 색 혼합 3방식(혼색 모드·휘도 보정·번짐 강도),
  수채화 경계, 영역 이탈 방지(색상 오차·영역 확대/축소). 복사할 수 없거나 값이 범위를 벗어난 설정은 복사하지 않고 일본어와 영어로 알림.
- CLIP STUDIO PAINT가 내보낸 이미지와의 차이를 수치로 출력하는 테스트(`csp_line_compare.rs`·`csp_edge_compare.rs`·`csp_refer_compare.rs`·`csp_stab_compare.rs`): 이미지와 `.sut`는 저장소에 넣지 않으며, 환경 변수 `YOLU_CSP_DIFF_DIR`(색상 변화는 `YOLU_CSP_COLOR_DIR`,
  차이 이미지를 쓸 위치는 `YOLU_CSP_DIFF_OUT`)이 없으면 아무 작업 없이 통과합니다. `cargo test -p yolu-io --test brushes csp_line_compare:: -- --ignored --nocapture`처럼 실행하면 비교 표를 출력합니다(가져온 선의 수식을 맞출 때 사용하는 외부 기준).
- “CLIP STUDIO에서” 창 테스트(`yolu-app/tests/headless/brush_import.rs`·`yolu-app/tests/gui_canvas/brush_import_ui.rs`·`yolu-io/tests/brushes/brush_clipstudio.rs`): 목록(이름·브러시 끝 샘플·읽지 못한 행 선택 불가), 선택한 항목만 가져오기,
  찾을 수 없는 이유(일본어·영어), 수동으로 선택한 폴더와 다시 검색, 검색 중 취소, 가져오는 중에 다시 가져오지 않음, CLIP STUDIO 폴더의 “수정 시각·크기·내용·항목 수가 변하지 않음”,
  테이퍼·손떨림 보정이 있는 브러시의 덮어쓰기·복원·복제·추가·저장 및 다시 읽기(버전 4).
- 합성 C2F로 확인한 사항(`tests/brushes/brush_import_sut_layer.rs`): 원래 크기의 브러시 끝·질감(커버리지·방향·가장자리가 잘린 타일·픽셀이 없는 타일·긴 오버플로·UTF-8과 UTF-16LE·`Parameter`
  끝의 단어 수 차이), 원본 이미지와 렌더링용 이미지의 우선순위, 미리보기보다 C2F 우선, 청크 손상(표식·CRC·잘림·알 수 없는 청크·청크 수), 읽을 수 없는 쪽의 페이지 수 불일치, 내부 페이지 순환·존재하지 않는 자식,
  폴더가 아닌 레이어 수, 색상 평면·너무 큰 이미지·빈 평면, zlib 손상, 청크의 CRC를 다시 계산한 뒤 약 3400가지 손상과 잘라내기(파일 전체는 반드시 가져올 수 있어야 함),
  상한의 안팎(B 트리 깊이 8의 리프는 읽고 9는 거부·1행 60 MiB는 읽고 64 MiB라고 주장하는 행은 거부〔작은 파일이 큰 행이라고 주장하는 형식〕·행 합계가 256 MiB 초과·
  소재 256개를 나열해도 가져오기 전체 작업 상한에서 중단·2048×2048 소재를 나열해도 전체 픽셀 수 상한에서 중단), 이름이 같고 루트가 다른 테이블(같은 루트의 사본은 읽을 수 있음),
  UTF-8과 UTF-16LE 혼용, 타일 청크 손상(번호 중복·수량 부족과 초과·범위 초과·크기 필드·길이 필드·zlib 압축 해제 길이·끝 표식·청크 전체 길이) 및 각 거부 이유,
  소재 종류별 대응 방식(듀얼 브러시를 포함한 수가 맞는 경우·맞지 않는 경우·종류를 알 수 없는 소재가 섞인 경우). 손상된 C2F는 원형 브러시 끝으로 돌아가며, 미리보기가 있으면 그것을 사용합니다.
- 실제 `.sut` 7개로 확인(일반 테스트 외부. 파일의 배포 허가 여부를 알 수 없어 저장소에 포함하지 않음. 환경 변수 `YOLU_REAL_SUT_DIR`의 폴더를 읽는 `#[ignore]` 테스트
  `tests/brushes/brush_import_sut_real.rs`. 브러시 이름·파일명은 출력하지 않고 복사한 값과 안내 종류만 출력): 열 스키마(정수 열은 백분율), 영향 원인 BLOB 176개의 형식(44바이트 헤더와
  곡선 슬롯), 복사한 설정(기울기(크기) 4개·손떨림 보정 1개〔단계 6〕·브러시 끝 각도 2개〔180°·220°〕·각도 무작위 3개〔강도 30·80·100〕·경도 1% 1개. 테이퍼는 길이가 있는 것도 영향 대상이 모두 꺼져 있어 복사하지 않음), 폴더로 목록을 만들고 살펴봐도 수정 시각·크기가 변하지 않음(7개 목록 생성 및 확인에 약 100 ms 소요).
- 실제 `.sut` 1개로 확인(일반 테스트 외부. 파일의 배포 허가 여부를 알 수 없어 저장소에 포함하지 않음. 환경 변수 `YOLU_REAL_SUT` 경로를 읽는 `#[ignore]` 테스트):
  소재 4개의 이미지를 원래 크기로 읽을 수 있음(브러시 끝 2개·질감 2개. 1변 500~1000픽셀), 브러시 1개의 브러시 끝과 질감이 (종류별 순서 추정으로) 올바르게 대응되고 “독자 형식이라 읽을 수 없음”으로 보고되지 않음.
  픽셀의 의미는 같은 환경 변수를 사용하는 `#[ignore]` 테스트 `the_pixels_are_the_black_ink_opacity_the_material_thumbnail_shows`(`src/brushes/sut/c2f.rs`)가 소재 안에 있는 CLIP STUDIO 샘플의 불투명도와의
  상관관계·방향(상하·좌우)·값의 방향·평균·잉크 색으로 확인합니다(샘플이 있는 소재 2개. 수치는 위의 “픽셀의 의미” 참조).
- 확인하지 않은 사항: **실제 Windows 기기**에서 CLIP STUDIO가 설치된 PC의 폴더를 찾을 수 있는지·CLIP STUDIO가 열린 상태에서 읽을 수 있는지(위의 “서브 도구 폴더에서 선택”).
  테이퍼 길이의 단위가 픽셀인지·영향 대상 열이 없는 버전에서 테이퍼가 적용되는 방식·비율의 의미, 기울기 곡선의 축과 방향, 손떨림 보정 단계의 단위, 브러시 끝 각도 방향의 부호와 각도 무작위 강도의 의미,
  방향 플래그의 하위 2비트, 속도·무작위 영향 강도의 위치(실물 7개는 모두 기본값에 가까워 다른 값을 보지 못함).
- 확인하지 않은 사항: 실제 `.sut`(CLIP STUDIO 버전별)의 열 이름·단위·곡선 의미, `.sut`를 CLIP STUDIO에서 다시 읽을 수 있는지(내보내기는 하지 않음). C2F는 다른 버전·
  컬러 소재(5개 평면)·테이블 루트가 읽을 수 없는 쪽에 있는 소재로 확인하지 않았습니다(읽을 수 없으면 거부). 소재 종류별 대응은 파일 1개의 소재 4개만을 바탕으로 한 추정입니다. 브러시 끝 픽셀의 최대
  농도(확인한 브러시 끝에서는 95 / 255)가 CLIP STUDIO의 그리기에 그대로 사용되는지는 확인하지 않았습니다. 질감의 “흰 부분에 칠해지는” 방향(질감 픽셀을 휘도 255 − v로 바꾸어 곱하는 방향)은
  샘플이 보여 주는 범위가 픽셀의 의미(검정의 불투명도)까지이므로, CLIP STUDIO가 질감을 페인트에 어떻게 사용하는지는 확인하지 않았습니다(화면 안내는 “질감 무늬”의 추정 알림뿐이며, 이 방향을 확인하지 않았다는 점은 알리지 않음). 샘플이 없는 소재(`CanvasPreview`를 가진 소재)의 픽셀 의미도 확인하지 않았습니다.

<a id="ツールの登録とキーdocssubtoolsmd"></a>

### 도구 등록과 키(docs/SUBTOOLS.md)

도구 이름·키·서브 도구 종류·도구 속성·옵션 바·캔버스와 3D 뷰 입력은 `crates/yolu-app/src/tools/`의 도구 테이블(`TOOLS`)에서 관리합니다.
키보드 할당·마우스와 보조 키의 조합·뷰 안의 키는 `crates/yolu-app/src/keymap.rs`의 테이블에서 관리하며, 키 처리와 설정 창의 “단축키” 분류가 같은 테이블을 읽습니다.
사용자가 설정 창에서 바꾼 부분만 `crates/yolu-app/src/keyconfig.rs`가 설정 폴더의 `keymap.json`에 보관하며, `keymap::install`로 기본 테이블에 덮어씁니다.
