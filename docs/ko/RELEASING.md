[한국어](RELEASING.md) · [日本語](../RELEASING.md)

<a id="配布の手順"></a>

# 배포 절차

Windows x86_64 MSVC의 zip과 설치 프로그램(NSIS setup.exe), Linux x86_64의 tar.gz(시험 사용. `release.yml` 입력 `linux`는 기본값 켬), Linux ARM(aarch64)의 tar.gz(시제품. 실제 장치에서는 확인하지 않았습니다. 입력 `linux_arm`은 기본값 켬. 빌드가 실패해도 배포는 멈추지 않으며 해당 파일은 올리지 않음)를 배포합니다. macOS는 시제품으로 서명 없는(ad-hoc 서명만 적용한) `YoluPainter.app` zip을 배포합니다.
([macOS 시제품](#macos-の試作). 입력 `macos`는 기본값 켬이며 끄면 올리지 않습니다. 시제품이므로 빌드가 실패해도 배포는 멈추지 않고 Windows만 배포합니다. [macOS(시제품) 실패 시](#macos試作が落ちたとき).) AppImage는 대상이 아닙니다.
실행 파일·설치 프로그램의 코드 서명은 [SignPath Foundation 신청](#コード署名signpath-foundation)이 승인되기 전까지 붙이지 않습니다(서명 없이 배포).
빌드에는 Rust stable, Python 3.10 이상, 각 OS의 C/C++ 빌드 환경이 필요합니다. Windows 설치 프로그램에는 NSIS 3도 필요합니다. macOS .app은 macOS에서만 만들 수 있습니다(Xcode Command Line Tools의 `lipo`·`codesign` 등). Linux 실행 환경은 [BUILDING.md의 「Linux(시험 사용)」](BUILDING.md#linux試用)을 참조하세요.
Ubuntu 22.04에서 만들므로 그보다 오래된 glibc 환경의 동작은 보장하지 않습니다.

Linux의 기존 의존성 `wayland-protocols-plasma`와 `wayland-protocols-misc`의 protocol XML은 LGPL-2.1-or-later이며, 실행 파일에는
그로부터 생성한 바인딩이 들어갑니다. `tools/licenses-reviewed.json`에서 두 항목 모두 `LGPL-2.1-or-later`를 선택하고 전문(`COPYING.LIB`)과
`server-decoration.xml`의 표기를 대조하여 `THIRD_PARTY_LICENSES.txt`에 넣습니다([THIRD_PARTY.md](../../THIRD_PARTY.ko.md)).

의존성을 추가·갱신하여 `licenses-reviewed.json`에 승인을 추가할 때는 크레이트의 선언뿐 아니라 동봉하는 XML·소스 전체에
GPL/LGPL 표기가 있는지 확인합니다. 발견하면 선택하지 않는 이중 라이선스 쪽인지(예: `self_cell`의 GPL) 확인하고, 그렇지 않으면 `blocked`에 이유를 기록합니다.

```sh
grep -rIlE 'SPDX-License-Identifier:.*GPL|GNU (Lesser|Library) General Public' ~/.cargo/registry/src/*/<クレート>-<バージョン>
```

라이선스 원문은 크레이트에 동봉된 것(`path`) 또는 해당 크레이트 발행 시점 커밋의 업스트림 것(`url`. 커밋 고정)을 SHA-256으로 고정합니다. 크레이트와 업스트림 모두 라이선스 본문이 없으면
업스트림 표기(`Cargo.toml`의 `license`와 `authors`)를 바탕으로 구성한 문서를 `tools/license-texts/`에 두고 고정합니다(`repo`. 이 아래의 파일만 가능). 문서 앞부분에 업스트림 원문이 아니라
어떤 표기로부터 구성했는지 기록합니다. 연도는 업스트림 표기로 알 수 있는 것만 씁니다(현재 문서는 모두 연도를 쓰지 않음). 업스트림의 LICENSE.md 등 본문은 아니지만 라이선스에 관해 쓰인 자료는 문서 뒤에 나란히 붙여 고정합니다.

<a id="出す流れ"></a>

## 배포 흐름

1. 버전을 올리는 릴리스(`x.y.0`)에서는 [배포 전 문서 확인](#出す前の文書の確かめ)을 마치고 수정한 문서를 작업 브랜치에 넣습니다.
2. 버전을 올리고([버전과 배포물](#バージョンと配布物)) main 대상 PR을 만듭니다. PR의 CI가 테스트를 실행하고 배포물도 빌드합니다([배포물을 빌드하는 위치와 받는 경로](#配る物をビルドする場所と受け取る道)).
3. Actions의 「배포물 만들기」를 작업 브랜치에서 dry-run=true(릴리스 초안을 만들지 않는 시험 실행)로 실행하고 배포물을 확인합니다.
4. PR의 CI가 모두 성공하면 PR을 main에 병합합니다.
5. main에서 「배포물 만들기」를 dry-run=false로 실행합니다. environment 승인 후 릴리스 초안이 만들어집니다.
6. 초안을 확인하고 본문을 작성하여 공개합니다.

각 단계의 자세한 절차는 [GitHub 설정과 실행](#github-の設定と起動)에 있습니다.

<a id="出す前の文書の確かめ"></a>

## 배포 전 문서 확인

버전을 올리는 릴리스(`0.6.0`처럼 마지막이 0인 버전)에서는 main 대상 PR을 만들기 전(배포 워크플로를 시험 실행하기 전)에
공개한 문서를 모두 해당 버전의 변경 기록·화면 문구·코드와 대조하고 오래된 절차와 누락을 수정하여 같은 PR에 넣습니다.
사용자 문서는 배포물(zip·설치 프로그램)에도 그대로 들어가므로([버전과 배포물](#バージョンと配布物)) 배포한 버전의 배포물 안에 있는 문서는 나중에 수정할 수 없습니다.

확인할 문서:

- 독립 실행형 앱(이 저장소): `README.md`·`README.en.md`, `docs/`의 일본어 문서와 `docs/en/`, `CHANGELOG.md`, 코드 안의 README(`crates/`와 `tools/` 아래의 `README.md`·`smart.README.md`),
  `plugin/`의 `SKILL.md`, 라이선스 표기(`THIRD_PARTY.md`·`crates/` 아래의 `THIRD-PARTY-NOTICES.md`)
- Unity 브리지(YoluPainter-UnityBridge): `README.md`·`Editor/LiveLink/README.md`·`licence.md`·`Documentation~/THIRD_PARTY.md`. 독립 실행형 앱과 같은 버전으로 배포하므로 함께 확인합니다.

확인 방법(문서마다 처음부터):

1. 확인할 수 있는 주장에 표시합니다. 파일·타입·모듈의 이름과 위치, 명령과 인수, 워크플로 입력과 작업 이름, 수치·기본값·버전, 화면 이름과 조작 순서입니다.
2. 하나씩 실물과 대조합니다. 화면 문구(`crates/yolu-app/locales/{ja,en}/*.ftl`·메뉴·툴팁), 코드와 테스트, `.github/workflows/`·`crates/xtask`·`tools/`,
   해당 버전의 `CHANGELOG.md`, 커밋 본문(`git log <前のバージョンのタグ>..HEAD -- <道>`)을 확인합니다.
3. 오래된 부분·오류·누락(해당 버전에서 추가한 기능의 설명이 없는 부분)을 수정합니다. 실물로 확인하지 못한 것은 쓰지 않습니다.
4. 용어를 화면에 맞춥니다(레이어·도구·창·아일랜드·폰트·캔버스. 빌드는 「빌드」라고 씁니다).
5. 테스트로 감시할 수 있는 부분은 실행합니다. `cargo test -p xtask`(배포 문서 목록과 포함하는 문서의 상대 링크가 배포물 안에서 끊어지지 않는지),
   `cargo test -p yolu-io --test ylp format_doc`(`docs/YLP_FORMAT.md`의 버전과 엔트리가 구현과 맞는지), 라이선스를 바꿨다면 `cargo xtask preflight --only licenses`를 실행합니다.

<a id="バージョンと配布物"></a>

## 버전과 배포물

`Cargo.toml`의 workspace.package.version을 갱신하고 `Cargo.lock`을 갱신·커밋한 뒤 해당 커밋을 배포 대상으로 삼습니다.
`Cargo.lock`이 바뀌면(버전만 갱신해도) 해시가 바뀌므로 `python3 tools/third-party.py`로 제품별 목록을 다시 만들고 `THIRD_PARTY.md`의 「Cargo.lock SHA-256」
(제품별 목록마다 및 Cargo.lock 전체 절)을 출력에 맞춥니다. 크레이트 수가 바뀌면 개수 표도 수정합니다.
AI 어시스턴트용 플러그인(`plugin/.claude-plugin/plugin.json`·`plugin/.codex-plugin/plugin.json`)의 `version`도 같은 버전으로 맞춥니다(`yolu-mcp`의 테스트 `tests/plugin.rs`가 불일치를 거부).
플러그인은 Release에 올리지 않고 `main`의 `plugin/`과 루트의 `.claude-plugin/marketplace.json`에서 배포합니다(Claude Code는 이 `version`이 바뀌면 새 버전을 설치).
버전은 SemVer입니다. 베타 버전(kind=prerelease)은 `0.1.0-rc.1`처럼 사전 릴리스 식별자를 `alpha.N`·`beta.N`·`rc.N`(N은 정수) 중 하나로 설정합니다.
이 형식이면 `rc.2` < `rc.10` < 정식 버전 순으로 정렬되고(`rc10`처럼 붙이면 사전순으로 `rc10` < `rc2`가 되므로 허용하지 않음), 앱의 「베타 버전 사용」 설정이 찾을 수 있습니다.
([베타 버전 배포 위치](#ベータ版の置き場).) stable은 식별자가 포함된 버전을 사용할 수 없습니다. `cargo xtask preflight --kind prerelease`가 버전 형식을 확인합니다.

```sh
cargo xtask build --target x86_64-pc-windows-msvc --release
cargo xtask bundle --target x86_64-pc-windows-msvc
cargo xtask installer --target x86_64-pc-windows-msvc
cargo xtask symbols --target x86_64-pc-windows-msvc   # Windows だけ。PDB の付属物（下の「PDB の付属物」）
cargo xtask mcpb --target x86_64-pc-windows-msvc      # Windows だけ。Claude Desktop に入れる拡張（下の「Claude Desktop の拡張（.mcpb）」）
# Linux では target を x86_64-unknown-linux-gnu に替える（installer は Windows だけ）。Linux の ARM（試作）は aarch64-unknown-linux-gnu（ARM の Linux の上で。クロスコンパイルはしません）
# macOS（macOS の上で）は target を universal-apple-darwin にする: build が Apple Silicon と Intel の 2 つをビルドして lipo で 1 つにし、bundle が .app を作って zip にする
cargo xtask updater-json --version 0.1.0-rc.1 --assets target/dist
```

`bundle`과 `installer`는 직전에 같은 커밋에서 빌드한 release 실행 파일을 사용합니다. 오래된 빌드는 사용하지 마세요.
`bundle`과 `installer`는 `target/dist`에 과거 버전을 남기므로 로컬에서 배포물을 만들 때는 `target/dist`를 비운 뒤 만듭니다.
다른 버전이나 불필요한 파일이 남아 있으면 `updater-json`이 거부합니다(PDB 부속물 `yolupainter-<バージョン>-x86_64-pc-windows-msvc-pdb.zip` 이름만 예외. CI는 매번 빈 상태).
출력은 `target/dist/yolupainter-<バージョン>-<target>.zip`(또는 `.tar.gz`. Linux ARM은 `yolupainter-<バージョン>-aarch64-unknown-linux-gnu-experimental.tar.gz`. macOS는 `yolupainter-<バージョン>-macos-universal-experimental.zip`)과 Windows의 `yolupainter-<バージョン>-x86_64-pc-windows-msvc-setup.exe`, `symbols`를 실행했을 때의 PDB 부속물, `mcpb`를 실행했을 때의 `.mcpb`입니다.
실행 파일(앱 `yolupainter`, 명령줄 및 MCP 서버 `yolupainter-cli`. `build`가 같은 명령으로 빌드), LICENSE, 조작·동작 환경을 포함한 README(일본어·영어), THIRD_PARTY.md, 대상별 DEPENDENCIES.md와 라이선스 전문, 사용자용 `docs/`(`docs/en/` 포함)를
동봉합니다(설치 프로그램도 같은 내용을 넣음). 문서는 배포물 안에도 폴더가 있는 `docs/GUIDE.md`·`docs/en/GUIDE.md` 이름으로 들어가므로 README의 상대 링크가 그대로 동작합니다.
개발 절차(`docs/DEVELOPMENT.md`·`docs/RELEASING.md`)는 넣지 않습니다. 포함할 항목 목록은 `crates/xtask/src/main.rs`의 `BUNDLED_DOCS`와 `LEFT_OUT_DOCS` 한 곳에 있으며,
`docs/`에 추가한 파일은 반드시 둘 중 하나에 올립니다(빠뜨리면 `bundle`·`installer`가 멈추고 `cargo test -p xtask`도 실패).
포함하는 것은 `.md` 문서뿐이며 이미지 등 `.md`가 아닌 항목은 `LEFT_OUT_DOCS`에 올리고 넣지 않습니다(업데이트 시 이전 버전에만 있던 문서를 지우는 설치 프로그램의 정리가 `.md`만 대상으로 하기 때문. 포함할 필요가 생기면 `installer/yolupainter.nsi`의 `RemoveOldDocs`도 수정해야 합니다. 테스트가 거부합니다).
포함 문서의 상대 링크가 배포물 안에서 끊어지지 않는지도 테스트에서 확인하므로 포함하지 않는 항목(`docs/DEVELOPMENT.md`·`CHANGELOG.md`·`crates/`의 README 등)은 GitHub URL로 연결합니다.
설치 프로그램 스크립트 `installer/yolupainter.nsi`의 `DocFiles`에도 같은 목록이 있으며 테스트에서 대조합니다(문서를 추가하면 양쪽 모두에 추가).
`tools/third-party.py`가 대상마다 라이선스를 대조하며 확인하지 않은 의존성이나 원문 불일치가 있으면 묶지 않습니다.
업데이트 크레이트(`yolu-update`)는 앱에 포함되고 명령줄(`yolu-cli`)은 같은 배포물에 들어가므로 해당 의존성의 라이선스 전문도 포함합니다(`--include-update --include-cli`).
`xtask` 자체는 배포하지 않습니다. 별도의 `CARGO_TARGET_DIR`을 사용하지 말고 출력을 `target/`으로 맞추세요.

`updater-json`의 입력은 해당 버전의 배포물만 놓은 전용 폴더입니다. 다른 버전이나 불필요한 파일은 거부합니다.
(PDB 부속물은 이름이 완전히 일치하는 하나만 허용하며 업데이트 정보에는 올리지 않음.)
Windows zip은 있는데 설치 프로그램이 없는 버전도 거부합니다(설치 프로그램으로 설치한 앱은 업데이트에도 설치 프로그램을 사용하므로 함께 배포).
업데이트 정보(schema 1)에는 zip·tar.gz를 대상 트리플(`x86_64-pc-windows-msvc` 등)로, 설치 프로그램을 별도 키 `x86_64-pc-windows-msvc-setup`으로, Linux ARM tar.gz를 키 `aarch64-unknown-linux-gnu`로, macOS zip을 키 `universal-apple-darwin`으로 올립니다.
파일 이름은 `updater-v1.json`입니다(schema 번호 포함. [업데이트 정보 호환성](#更新情報の互換) 참조).
URL은 `https://github.com/YozoraKurage/YoluPainter/releases/download/v<バージョン>/<配布物名>`으로 고정됩니다. 앱이 업데이트 정보를 가져오는 위치는
`https://github.com/YozoraKurage/YoluPainter/releases/latest/download/updater-v1.json`(GitHub의 「최신 Release」. 초안·사전 릴리스 제외)입니다.
「베타 버전 사용」 설정을 켠 앱은 여기에 더해 고정 태그 위치(`releases/download/updater-beta/updater-v1.json`)도 확인합니다.
포크에서 배포하려면 업데이트 크레이트의 `RELEASE_BASE`·`UPDATER_URL`도 변경하여 빌드합니다.
서명 없는 JSON은 검사 전용이며 업데이트 크레이트는 수락하지 않습니다.

<a id="macos-の試作"></a>

### macOS 시제품

`cargo xtask build --target universal-apple-darwin --release`는 먼저 이전 출력(두 타깃과 universal 실행 파일)을 지운 뒤 `aarch64-apple-darwin`과 `x86_64-apple-darwin`으로
같은 명령(`-p yolu-app -p yolu-cli`)을 빌드하고 `lipo`로 하나로 합칩니다(`target/universal-apple-darwin/release/`. 도중에 실패해도 이전 파일이 남지 않으므로 `bundle`이 오래된 실행 파일을 새 버전 zip으로 만들지 않음.
양쪽 슬라이스의 최소 macOS는 `MACOSX_DEPLOYMENT_TARGET=11.0`으로 맞춤). `cargo xtask bundle --target universal-apple-darwin`은
`YoluPainter.app/Contents/{Info.plist,MacOS,Resources}`를 만들고 `codesign --force --deep --sign - YoluPainter.app`(ad-hoc)으로 서명한 뒤 `ditto -c -k --keepParent --norsrc --noextattr --noqtn`으로 zip을 만듭니다.
(`crates/xtask/src/macos.rs`. 리소스 포크·확장 속성·격리 표시는 zip에 넣지 않음.) `--identifier`는 붙이지 않습니다. 번들 식별자는 `Info.plist`의 `net.yozolab.yolupainter`,
번들 안의 명령줄에는 파일 이름에 내용 해시를 더한 `yolupainter-cli-…`가 붙습니다(`--identifier`를 붙이면 `--deep`에 의해 양쪽 식별자가 같아짐).

- 배포물은 `yolupainter-<バージョン>-macos-universal-experimental.zip`입니다. 최상위 폴더 `yolupainter-<バージョン>-macos-universal-experimental/`에 `YoluPainter.app`과 Windows·Linux 배포물과 같은 문서 및 라이선스 표기
  (`LICENSE`·README·`THIRD_PARTY.md`·`DEPENDENCIES.md`·`THIRD_PARTY_LICENSES.txt`·`docs/`. 목록은 `BUNDLED_DOCS`·`ROOT_FILES` 한 곳)이 들어갑니다.
  명령줄 `yolupainter-cli`는 `.app`의 `Contents/MacOS/` 안에 있습니다(`--deep` 서명 적용).
- `Info.plist`의 버전은 워크스페이스 버전입니다. 표시 버전 `CFBundleShortVersionString`은 `x.y.z` 정수만 사용합니다(베타 버전 식별자는 넣지 않음). 빌드 번호 `CFBundleVersion`은 정식 버전은 `x.y.z`,
  베타 버전은 식별자 번호를 더한 `x.y.z.N`(`0.6.1-rc.2` → `0.6.1.2`)입니다. 베타끼리 및 정식 버전과 구별하기 위한 형식이며 신구 순서를 나타내지는 않습니다(버전 전후는 업데이트 정보의 SemVer로 결정). 번들 식별자는 `net.yozolab.yolupainter`입니다.
  아이콘은 기존 로고(`crates/yolu-app/assets/logo/yolupainter-1024.png`)에서 `sips`·`iconutil`로 만듭니다(새 이미지를 만들지 않음). `.ylp` 연결(`CFBundleDocumentTypes`)은 넣지 않습니다.
  (macOS가 열기 요청을 전달하는 방식은 Apple Event이며 앱은 인수만 읽기 때문.)
- 라이선스는 `tools/third-party.py --target universal-apple-darwin`(두 Rust 타깃 트리의 합집합)으로 확인합니다. `cargo xtask preflight --target aarch64-apple-darwin --only licenses,attributes`처럼
  어느 타깃이든 단독으로도 대조할 수 있습니다.
- Apple Developer 서명·공증·.dmg·Homebrew는 아직 없습니다. 다운로드한 zip의 `.app`은 Gatekeeper에 차단되므로 여는 방법을 README와 Release 본문에 씁니다.
  앱은 업데이트를 다운로드하거나 교체하지 않으며 새 버전을 알리고 릴리스 페이지를 열기만 합니다([검증과 현재 범위](#検証と現在の範囲)).
- 확인하는 것은 zip을 다른 위치에 풀어 `codesign -dv`·`codesign --verify --deep --strict`·`lipo -info`·실행 파일 실행(몇 초간 계속 동작하며 설정이나 로그 파일을 쓰는지), 그리고 격리 표시(`com.apple.quarantine`)가 붙은 상태에서의 동작입니다.
  SSH에서는 화면이 보이지 않으므로 버전마다 한 번씩 실제 Mac에서 열어 창이 나타나고 그릴 수 있는지 확인하세요(확인 전까지 README·`docs/INSTALL.md`·CHANGELOG·릴리스 본문은 「실행·쓰기·서명·universal 형식까지. 창 표시와 그리기는 미확인」 범위로 작성).
- 시제품이므로 macOS 빌드가 실패해도 배포는 멈추지 않습니다([macOS(시제품) 실패 시](#macos試作が落ちたとき)).

<a id="macos試作が落ちたとき"></a>

### macOS(시제품) 실패 시

macOS 대상은 `tools/dist-targets.json`에서 `experimental: true`입니다. 실패해도 배포를 멈추지 않습니다. 다른 대상(Windows)은 기존처럼 필수이므로 하나라도 빠지면 멈춥니다.
(`cargo xtask preflight`의 `targets`가 macOS는 시제품, Windows는 시제품이 아닌지 확인.)

- PR의 CI: `dist-build.yml`의 build 작업은 `continue-on-error: ${{ matrix.experimental || false }}`이므로 macOS 작업이 실패해도 CI 결론은 실패가 되지 않습니다(해당 작업은 빨간색으로 표시됨).
  시간 상한은 대상의 `timeout_minutes`입니다(macOS는 90분, 나머지는 60분).
- `main-tested.yml`: `python3 tools/dist.py ci-ok`는 시제품 대상 작업(이름에 `（universal-apple-darwin）` 포함)만 실패한 CI를 성공으로 간주합니다. 다른 작업이 하나라도 실패하면 기존처럼 실패입니다.
- 배포물 수신: `judge_run`은 시제품 대상 작업의 실패를 세지 않으며, `find`는 macOS 산출물이 없어도(실패·만료·목록 불일치) Windows 산출물을 받습니다. macOS는 올리지 않습니다.
- 배포 안의 빌드: macOS만 실패해도 `metadata`는 계속 진행합니다. `install`은 시제품이 아닌 대상의 산출물이 빠지면 멈추고, 시제품 대상의 산출물이 없으면 올리지 않으며 실행 Summary에
  「universal-apple-darwin(시제품)이 포함되지 않음」과 이유를 표시합니다. 초안은 Windows만으로 만듭니다.
- 포함되지 않은 버전은 업데이트 정보에 macOS 항목이 없으며(macOS 앱은 「이 환경용 배포물이 없습니다」라고 응답), 릴리스 본문에도 macOS 절을 붙이지 않습니다.
- macOS를 제외하려면: `tools/dist-targets.json`의 macOS 대상에서 `default`를 제거하는 커밋을 만듭니다(PR CI에서 빌드하지 않게 됨). 해당 배포에서만 제외하려면 입력 `macos`를 끕니다.

확인할 사항(GitHub 설정으로, 저장소 밖이므로 여기서는 확인할 수 없음): main ruleset의 필수 상태 검사에 macOS 작업(이름에 `universal-apple-darwin`이 포함된 「배포물 빌드」)이 들어 있지 않아야 합니다.
들어 있으면 macOS가 실패한 PR은 병합할 수 없습니다. 포함하지 않는 것이 원칙입니다.

<a id="pdb-の付属物"></a>

### PDB 부속물

충돌 기록은 각 프레임의 주소와 실행 파일의 기준 주소(`Image base`)를 기록하므로 배포물의 PDB가 있으면 같은 버전의 함수 이름·행을 찾을 수 있습니다.
`.github/workflows/dist-build.yml`의 Windows 빌드는 `cargo xtask build` 전에 환경에 `CARGO_PROFILE_RELEASE_DEBUG=line-tables-only`·`CARGO_PROFILE_RELEASE_STRIP=none`을 설정하고
(명령은 바꾸지 않고 행 번호가 있는 PDB를 실행 파일과 별도로 생성), `cargo xtask installer` 뒤에 `cargo xtask symbols`로 `yolupainter.pdb`만 들어 있는
`yolupainter-<バージョン>-x86_64-pc-windows-msvc-pdb.zip`을 `target/dist`에 만듭니다. Release 부속물로는 올라가지만 zip·설치 프로그램에는 들어가지 않으며
업데이트 대상도 아닙니다(서명된 업데이트 정보에 넣지 않고 앱은 받으러 가지 않음). `verify`는 일반 파일이며 비어 있지 않고 크기 상한 이내이며 내용이 `yolupainter.pdb`뿐인지 확인합니다.
`symbols`는 PDB를 검사하기 전에 이전 부속물을 지우므로 다시 빌드하는 데 실패한 뒤 오래된 PDB가 남지 않습니다.

<a id="claude-desktop-の拡張mcpb"></a>

### Claude Desktop 확장(.mcpb)

`cargo xtask mcpb --target x86_64-pc-windows-msvc`는 `target/dist/yolupainter-<バージョン>-x86_64-pc-windows-msvc.mcpb`를 만듭니다(`build` 이후 실행. `yolupainter-cli.exe` 사용).
내용은 zip으로, `manifest.json`(mcpb manifest_version 0.3. `server.type`은 `binary`, 실행 파일은 `server/yolupainter-cli.exe`, 인수는 `mcp --port ${user_config.port}`(확장 설정 「Port」. 기본값 17347),
대상은 `win32`만. 실행 파일은 표준 입출력을 실행 중인 앱의 연결 지점으로 이어 주는 중계이며, 도구 목록은 쓰지 않고 `tools_generated`로 설정하여 앱이 `tools/list`로 반환하는 것을 정본으로 사용)·아이콘(앱 로고 PNG)·`LICENSE`·명령줄 라이선스 전문(`DEPENDENCIES.md`·`THIRD_PARTY_LICENSES.txt`.
`tools/third-party.py --package yolu-cli --built-with yolu-app --bundle`. 실행 파일은 앱과 한 번에 빌드하여 기능이 합쳐지므로 해당 빌드의 `yolu-cli` 하위 트리로 집계)·실행 파일 하나입니다. 로고를 바꾸려면 `crates/xtask/src/main.rs`의 `MCPB_LOGO`를 바꿉니다.

PDB 부속물과 마찬가지로 Release에 올릴 뿐이며 업데이트 대상은 아닙니다. 서명된 업데이트 정보에는 올리지 않고(앱은 받으러 가지 않으며 설치한 확장은 새 `.mcpb`를 다시 설치해 교체),
`updater-json`은 이 이름(버전과 대상이 현재 버전과 일치하는 하나)만 알고 건너뛰며, `verify`는 내용 형식(목록에 있는 파일만 있으며 manifest가 현재 버전·실행 파일을 가리키는지)을 확인합니다.
`dist-build.yml`의 Windows 빌드가 `xtask mcpb`를 실행하며 `target/dist/*`와 함께 산출물·Release 부속물이 됩니다.
manifest는 공식 검증(`npx @anthropic-ai/mcpb validate manifest.json`)을 통과하는지 확인했습니다. Windows 실제 장치에서는 앱 설정의 「외부 조작 허용」을 켠 뒤 Claude Desktop에 설치하여 도구가 나열되고 앱 문서를 조작할 수 있는지 버전마다 한 번 확인하세요.

<a id="windows-のインストーラー"></a>

### Windows 설치 프로그램

NSIS 스크립트는 `installer/yolupainter.nsi`입니다. 사용자별 설치(`%LOCALAPPDATA%\Programs\YoluPainter`, 관리자 권한 없음, 64비트)로
시작 메뉴 바로 가기와 제거 프로그램(「설정 → 앱」에 등록)을 배치합니다. `.ylp` 연결은 화면에서 선택할 수 있습니다.

| 인수 | 동작 |
|---|---|
| `/S` | 화면을 표시하지 않음 |
| `/ASSOC=1` / `/ASSOC=0` | `.ylp` 연결을 설정·해제(무인 실행 시. 생략하면 현재 상태 유지, 처음이면 설정하지 않음) |
| `/RUN` | 설치를 마치면 앱 실행(앱 업데이트가 사용). 대기 상한에 도달해 아무것도 바꾸지 않고 끝날 때도 현재 설치된 앱을 다시 실행 |
| `/D=<パス>` | 설치 위치(마지막에 지정. 생략하면 이전 설치 위치) |
| 제거 프로그램의 `/S` `/DELETEDATA` | 무인 실행. 앱이 다시 만들 수 있는 데이터(설정·창 배치·복구·충돌 기록·썸네일 캐시. `%APPDATA%\YoluPainter`와 `%LOCALAPPDATA%\YoluPainter`의 명시된 항목)는 화면에서 삭제 여부를 묻고, 무인 실행에서는 `/DELETEDATA`일 때만 삭제합니다. 개인 라이브러리·브러시·보조 도구·그라데이션·컬러 세트·표시 프리셋 등 사용자가 만든 항목과 알 수 없는 파일은 어느 쪽이든 남깁니다(표는 `docs/INSTALL.md`) |

실행 중인 앱을 종료시키지 않습니다. 실행 파일(앱과 `yolupainter-cli.exe`. MCP 클라이언트가 계속 실행하고 있을 수 있음)을 사용하는 동안에는 기다리며(무인 실행은 최대 60초. 넘으면 아무것도 바꾸지 않고 종료 코드 5. `/RUN`이 있으면 현재 설치된 앱 실행 파일을 다시 실행),
제거 프로그램은 자신이 설치한 파일만 지웁니다(설치 위치에 사용자 파일이 있으면 설치 폴더는 남음).

문서는 설치 위치의 `docs\`·`docs\en\`에 들어갑니다. 설치한 문서 이름은 `docs\.installed`에 기록하며, 업데이트(덮어쓰기)할 때 이전 버전 기록에 있는 문서를 먼저 지운 뒤 현재 버전 문서를 넣으므로
이전 버전에만 있던 문서(이름이 바뀌거나 제외된 문서)가 남지 않습니다. 지우는 것은 기록에 있는 `docs\` 아래의 `.md`뿐이며 `..`가 포함된 이름은 건너뜁니다.
(사용자가 `docs\`에 둔 파일과 설치 위치 밖은 지우지 않음.) 제거할 때는 목록의 문서와 기록을 지우고 `docs\en`·`docs\`는 비어 있을 때만 지웁니다(`RMDir /r` 사용하지 않음).

NSIS는 3.x가 필요합니다. 워크플로는 windows-latest 이미지에 `makensis`가 없으면 Chocolatey(`choco install nsis --version=3.11.0`)로 설치하며,
기존 것이든 새로 설치한 것이든 `makensis /VERSION`이 3.11이 아니면 멈춥니다(배포물을 만드는 방식을 바꾸지 않기 위함). 확인한 `makensis`는
`MAKENSIS`로 `cargo xtask installer`에 전달합니다. 로컬에서는 Windows는 `winget install NSIS.NSIS`, Debian·Ubuntu는
`sudo apt install nsis`로 설치합니다(`MAKENSIS`에 `makensis` 위치를 지정할 수도 있음).
`cargo test -p xtask`는 `makensis`가 있으면 실제로 스크립트를 컴파일합니다(CI Linux는 `nsis`를 설치하여 실행).

화면 없는 흐름(신규·업데이트·`/RUN`·연결 유지·제거. 문서 설치 방식 및 업데이트 시 이전 버전에만 있던 문서가 지워지는지·사용자 파일은 지워지지 않는지 포함)과 대기 상한(쓰기용으로 열 수 없는 실행 파일이 남아 있는 동안 대기,
상한을 넘으면 아무것도 바꾸지 않고 종료 코드 5. `/RUN` 유무에 따라 현재 실행 파일을 다시 실행하는지도 확인. 테스트를 위해 상한을 3초로 줄인 설치 프로그램 `-DWAIT_STEPS=6` 사용)은 Wine으로 통과시킬 수 있습니다.

```sh
python3 tools/test-installer.py
```

실제로 실행 중인 앱을 기다리는 동작은 Wine이 실행 중인 실행 파일의 덮어쓰기를 거부하는 버전에서만 확인할 수 있습니다(덮어쓸 수 있는 버전에서는 주의를 표시하고 통과).
페이지 순서·체크·확인 창 등의 화면은 Wine에서 확인하지 않습니다. Windows 실제 장치에서 설치·업데이트·실행 중인 앱이 있을 때의 업데이트·제거를 한 번씩 진행하세요.

<a id="exe-のバージョン情報"></a>

### exe 버전 정보

`crates/yolu-app/build.rs`가 제품명 `YoluPainter`·버전(workspace 버전. 사전 릴리스 식별자 포함)·작성자 이름·아이콘
(`crates/yolu-app/assets/logo/yolupainter.ico`)을 실행 파일에 넣습니다. 설치 프로그램도 같은 제품명·버전을 가집니다.
(코드 서명 조건의 「제품명과 버전 메타데이터」.)

<a id="配るビルドの-profile"></a>

### 배포 빌드의 profile

배포 빌드는 기본 release(`cargo build --release`)입니다. `lto = "fat"`·`codegen-units = 1`(패닉은 unwind 유지) profile도 측정했으나 채택하지 않았습니다.
재측정은 `python3 tools/bench-profiles.py --app --cold`로 수행합니다(해당 profile 정의는 도구가 `--config`로 전달. Cargo.toml에는 넣지 않음).

측정 범위(2026-10-05. AMD Ryzen 9 7950X3D의 Linux 컨테이너, rustc 1.99.0, cargo 병렬 8·sccache 없음·증분 빌드 없음, 부하 평균 2~7.
작업 시간은 CPU 하나에 고정·rayon 스레드 하나로, 5회 중앙값을 3번(profile을 번갈아) 실행한 중앙값):

| | release | lto fat·codegen-units 1 |
|---|---:|---:|
| 캐시가 없는 상태에서 yolu-app 빌드 | 106 s | 244 s(2.30배) |
| exe 크기(Linux) | 68.9 MB | 56.1 MB(0.81배) |
| 무거운 작업의 속도 비(release ÷ 오른쪽. 1보다 클수록 오른쪽이 빠름) | | 합성 0.993·브러시 0.997·내보내기 0.999·저장과 열기 1.019·PSD 1.022(51행의 기하평균 0.999. 행별로 0.93~1.08) |

무거운 작업은 5%도 빨라지지 않았고 빌드 시간은 2.3배였습니다. GitHub Windows runner에서는 2026-10-05의 0.3.0 배포에서 「빌드와 배포물 만들기」 단계가 9분 25초였으므로
2.3배라면 20분을 넘을 것으로 예상되어 배포 빌드 시간이 지나치게 길어집니다. 이점은 exe가 약 19% 작아진다는 것뿐이었습니다.
측정한 것은 Linux·CPU 하나·yolu-core의 합성·브러시·내보내기(`bench`·`export_bench`), 저장·PSD(`tools/profile-bench/io_bench.rs`)이며 GPU·화면·여러 스레드·Windows 속도는 측정하지 않았습니다.

<a id="事前確認"></a>

## 사전 확인

배포물을 만드는 단계(라이선스 대조·패키징·설치 프로그램)가 실제 배포에서 처음 실행되어 멈추는 종류의 실패를 빌드 없이 나열합니다. 로컬과 CI 빌드 시작 시 모두 같은 명령을 사용합니다.

```sh
cargo xtask preflight                       # 既定の対象（tools/dist-targets.json の、入力が要らない対象）
cargo xtask preflight --kind stable         # 出す種類も確かめる（stable にプレリリースのバージョンは使えない）
cargo xtask preflight --kind prerelease     # ベータ版は alpha.N・beta.N・rc.N のバージョンだけ
cargo xtask preflight --target x86_64-unknown-linux-gnu --offline
cargo xtask preflight --target aarch64-apple-darwin --only licenses,attributes   # macOS の Rust のターゲット単独の許諾の照合（universal-apple-darwin はその和）
cargo xtask preflight --installer           # 加えて、Wine でインストーラーを通す（tools/test-installer.py。wine32 が要る）
cargo xtask preflight --only version --kind prerelease
```

| 확인 | 확인 내용 |
|---|---|
| `licenses` | 대상별 라이선스 대조(`tools/third-party.py --target T --bundle`과 같음. 미확인 의존성·원문 불일치·`blocked`가 있으면 실패. macOS 대상은 objc2 계열 등 macOS 전용 의존성도 집계) |
| `attributes` | SHA-256으로 대조하는 표기 파일(`tools/licenses-reviewed.json`의 `bundled`와 저장소에 둔 라이선스 문서 `tools/license-texts/`)이 `.gitattributes`에서 `eol=lf` 또는 `-text`인지(Windows checkout에서 CRLF가 되면 대조 실패) |
| `nsis` | `installer/yolupainter.nsi`의 `Target`이 공식 Windows용 NSIS가 가진 stub(`x86-unicode`·`x86-ansi`)인지(amd64 stub은 없음) |
| `mcpb` | Windows 대상에서 `.mcpb` manifest를 만들 수 있고 형식이 맞는지, 확장에 넣을 로고 PNG가 있는지, 명령줄 라이선스 묶음(`tools/third-party.py --package yolu-cli --built-with yolu-app --bundle`)을 만들 수 있는지 |
| `macos` | macOS .app을 만드는 대상(`universal-apple-darwin`)에서 `Info.plist`를 구성할 수 있고 버전이 맞는지, 아이콘 원본 로고가 1024 x 1024 PNG인지, 동봉 문서가 목록에 있는지. macOS에서는 추가로 `lipo`·`codesign`·`iconutil`·`sips`·`ditto`·`plutil` 및 두 Rust 타깃이 설치되어 있는지 |
| `version` | yolu-app 버전 태그 `v<バージョン>`이 origin에 아직 없는지(`git ls-remote --tags origin`. 조회할 수 없으면 「건너뜀」 표시), `--kind stable`이면 사전 릴리스 버전이 아닌지, `--kind prerelease`이면 `alpha.N`·`beta.N`·`rc.N` 버전인지 |
| `targets` | `tools/dist-targets.json`의 대상이 xtask에서 배포할 수 있는 대상이며 Windows에만 설치 프로그램이 있는지. macOS 대상이 macOS runner를 사용하고 `rust_targets`가 xtask의 두 타깃과 같은지 |
| `workflows` | 워크플로 YAML을 읽을 수 있는지, release.yml 입력(기본 켬·끔 포함)과 대상 배열이 일치하는지, 외부 Action이 SHA로 고정되어 있는지, 배포물 절차에서 캐시를 사용하지 않는지(`tools/check-workflows.py`. PyYAML이 없으면 「건너뜀」) |
| `installer` | Wine으로 설치 프로그램 실행(`--installer` 또는 `--only installer`일 때만) |

실패는 이유와 함께 한 줄씩 나열하며 모두 실행한 뒤 종료 코드 1을 반환합니다(첫 번째 실패에서 멈추지 않음). 「건너뜀」은 실패로 처리하지 않습니다.

<a id="配る物をビルドする場所と受け取る道"></a>

## 배포물을 빌드하는 위치와 받는 경로

배포물(zip·설치 프로그램)은 **main 대상 PR의 CI**가 빌드합니다. 배포(`release.yml`)는 같은 트리의 산출물이 있으면 **다시 빌드하지 않고 받습니다**.
같은 내용을 PR CI와 배포에서 두 번 빌드하면 낭비이며 테스트를 통과한 것과 배포하는 것이 서로 다른 빌드가 되기 때문입니다.

- `ci.yml`은 main에 push할 때 실행하지 않습니다(main은 CI를 통과한 PR로만 변경). 대신 `main-tested.yml`이 main의 트리가 병합한 PR의 선두 트리와 같고
  해당 PR의 CI가 성공했는지만 확인합니다(다시 빌드하지 않음. README의 CI 배지는 이 결과).
  PR CI(`ci.yml`)는 같은 저장소 브랜치에서 main으로 보내는 PR일 때만 테스트 작업과 나란히 「배포물 빌드」
  (`dist-plan` → `dist`)를 실행합니다(대기 시간이 늘지 않음). 포크 PR에서는 빌드하지 않습니다.
- 빌드 절차는 `.github/workflows/dist-build.yml` 하나이며 `ci.yml`과 `release.yml` 양쪽에서 호출합니다. 대상 배열은 `tools/dist-targets.json` 한 곳에 있습니다.
  (Windows는 항상 포함. Linux의 `x86_64-unknown-linux-gnu`·Linux ARM의 `aarch64-unknown-linux-gnu`(시제품. 실행 환경은 `ubuntu-22.04-arm`)·macOS의 `universal-apple-darwin`은 입력 `linux`·`linux_arm`·`macos`가 기본값 켬이므로 PR CI에서도 빌드합니다.
  배포에서 끌 수 있는 것은 입력뿐입니다. 끈 배포는 해당 대상을 제외하고 CI 산출물을 받거나 빌드합니다. 시제품은 ARM Linux와 macOS이며 실패해도 배포는 멈추지 않습니다. 대상을 추가하면 이 목록과 필요 시 입력을 수정합니다. `cargo xtask preflight`의 `targets`·`workflows`가 불일치를 거부합니다.)
  macOS 대상은 macOS runner(`macos-14`)에서 빌드하며 목록의 `rust_targets`(Apple Silicon과 Intel 2개)를 runner에 설치합니다. 절차는 진단용 ID(`tools/dist.py revision`) → `cargo xtask preflight --target <対象>` → `cargo xtask build --release`(PR CI와 dry-run=false 배포는 `--require-update-key`도 붙이며 공개키가 비어 있으면 멈춤)
  → `bundle`(macOS는 .app 구성·서명·zip) → `installer`(Windows. 먼저 NSIS 3.11 확인) → `mcpb`·`symbols`(Windows) → 목록(`tools/dist.py catalog`) → 산출물 업로드입니다.
- 산출물 이름은 `dist-<木>-<対象>`입니다(14일 보관). 트리는 `git rev-parse HEAD^{tree}`이며 PR CI에서는 merge commit의 트리입니다. 안에는 `catalog.json`
  (트리·commit·앱에 넣은 ID의 원본 commit·버전·대상·포함한 업데이트용 공개키·각 파일의 SHA-256과 크기·rustc·runner)이 들어갑니다.
- 앱 버전 표시(「0.4.0 · a1b2c3d」)와 충돌 보고서 `Git:`의 ID는 PR CI에서는 **PR 선두 commit**입니다. PR CI가 빌드하는 것은 merge commit
  (`refs/pull/N/merge`)으로, main이나 태그에 들어가지 않으며 PR을 닫은 뒤 참조할 수 있다는 보장이 없기 때문입니다. 승격 조건에 따라 PR 선두 commit의 트리는 산출물 트리(＝배포할 참조의 트리)와 같으므로
  ID는 배포물과 같은 트리의 commit을 가리킵니다. 목록의 `commit`은 빌드한 merge commit, `revision`은 ID의 원본 commit입니다. 배포에서 빌드하면 dispatch한 commit입니다.
- `release.yml`은 먼저 `plan`(대상과 dispatch한 참조의 트리), `find`(검색)를 실행합니다. `find`는 다음 **모든 조건**을 만족하는 실행에서 해당 트리의 산출물을 찾습니다.
  - `.github/workflows/ci.yml`의 `pull_request` 실행이며 완료·성공 상태(모든 작업이 `success`. `skipped`도 받지 않음. 단, 시제품 대상(Linux ARM·macOS) 작업의 실패는 세지 않음)이고 같은 저장소 브랜치의 실행
  - 해당 실행의 `head_sha`(PR 선두 commit) 트리를 GitHub API로 확인하여 산출물 이름의 트리와 일치
  - 산출물이 만료되지 않았으며 GitHub가 기록한 digest가 있고 다운로드한 zip과 같으며(digest 기록이 없는 산출물은 확인할 방법이 없어 받지 않음), 목록의 트리·대상·포함한 공개키(현재 저장소 변수와 같음)·각 파일의 SHA-256이 일치
  - 모든 대상이 하나의 동일한 실행에 갖춰져 있음(시제품 대상(Linux ARM·macOS)은 해당 실행에 산출물이 있고 목록이 일치할 때만 포함하며, 없으면 제외하고 수신)
- 갖춰져 있으면 `build`(빌드)를 건너뛰고 `metadata`가 목록의 SHA-256을 다시 확인하여 `target/dist`에 모읍니다(가져올 산출물은 `plan` 출력 `pattern`으로 이번 대상 목록에 있는 것만 제한. 입력 `macos`를 끈 배포는 CI에 macOS 산출물이 있어도 가져오지 않음). 그 이후(미서명 업데이트 정보 확인·릴리스 초안의 서명과 생성)는 빌드한 경우와 같습니다.
  갖춰지지 않았으면(트리 불일치·만료·성공하지 않음·Linux 추가·키 변경·API 실패) 일반적으로 같은 절차로 빌드합니다. 이유는 실행 Summary에 한 줄씩 표시합니다.
  입력 `rebuild`를 켜면 검색하지 않고 반드시 빌드합니다.
- 배포 워크플로(`release.yml`·`dist-build.yml`)는 캐시를 사용하지 않습니다(Actions 캐시·rust-cache·sccache). 오염된 캐시가 배포물에 들어가는 경로를 만들지 않기 위함이며
  `cargo xtask preflight`의 `workflows`가 확인합니다. PR CI의 테스트 작업은 기존처럼 캐시를 사용합니다.
- `metadata`는 받는 경로와 빌드하는 경로 모두에서 현재 배포할 태그가 아직 없는지, 종류·버전이 맞는지를 `cargo xtask preflight --only version`으로 다시 확인합니다(PR 시점의 확인과 별도).

<a id="安全の筋と保証しないこと"></a>

### 안전성의 근거와 보장하지 않는 사항

근거: 산출물 이름의 트리, 해당 실행의 PR 선두 트리, 지금 배포하려는 참조의 트리가 같으면 워크플로 스크립트(`.github/workflows/` 내용)와 소스도 같습니다.
즉, 산출물은 main에 들어갈 것과 같은 스크립트·같은 소스를 GitHub runner가 실행하여 빌드한 것입니다. PR 브랜치가 워크플로를 바꿔 이름만 main 트리라고 주장하는 산출물을 만들어도
해당 실행의 선두 트리는 수정된 내용의 트리이므로 받지 않습니다. PR 선두 트리와 merge 트리가 어긋나면(PR 브랜치가 main을 따라잡지 못함) 이름의 트리와 선두 트리가 맞지 않아 다시 빌드합니다.
(받을 기회를 포기하고 확인을 지키는 쪽을 택함.)

보장하지 않는 사항:
- 승격한 산출물에 포함된 ID는 Release 대상 commit 자체가 아닙니다. PR 선두 commit으로 트리는 같지만 main이 PR을 squash하여 가져오면 main 기록에는 들어가지 않습니다.
  (GitHub의 `refs/pull/N/head`에서 찾을 수 있지만 보관은 GitHub에 맡김.) 트리가 같은지는 산출물 이름과 목록으로 확인합니다.
- 의존성 크레이트의 다운로드는 `Cargo.lock`의 고정(체크섬)에 의존합니다. PR 시점과 배포 시점에 별도로 다운로드해도 내용이 같은지까지 확인하지는 않습니다.
- Rust 툴체인은 `stable`이며 runner 이미지도 갱신됩니다. PR 시점과 현재의 버전이 다를 수 있습니다(목록의 `rustc`·runner에 기록). 같은 트리에서 같은 바이트열이 만들어진다고 보장하지 않습니다(재현 가능한 빌드가 아님).
- 업데이트용 공개키는 앱에 포함됩니다. PR 이후 저장소 변수를 바꾸면 목록의 키와 맞지 않아 다시 빌드합니다.
- PR CI 테스트 통과는 PR 시점의 runner·툴체인에서 얻은 결과입니다. Windows 실제 장치의 확인을 대신하지 않습니다.
- 보관 기간은 14일입니다. PR을 낸 뒤 14일 이상 지나 배포하면 다시 빌드합니다.

로컬에서 수신 조건을 확인하려면 `python3 tools/test-dist.py`(GitHub API를 가짜 엔드포인트로 교체한 테스트)를 실행합니다.

<a id="ベータ版の置き場"></a>

## 베타 버전 배포 위치

베타 버전(kind=prerelease Release)은 `releases/latest`에 나타나지 않습니다. 「베타 버전 사용」 설정을 켠 앱이 찾을 수 있도록
공개한 베타 버전의 서명된 업데이트 정보를 **고정 태그 `updater-beta`의 Release**에 원본 그대로 덮어씁니다.

- 앱(`yolu-update`의 `UpdateClient::check_channels`)은 설정이 켜져 있으면 stable 위치(`releases/latest/download/updater-v1.json`)와
  베타 버전 위치(`releases/download/updater-beta/updater-v1.json`)를 모두 가져와 **현재 버전보다 새 버전인 쪽**을 권합니다. 둘 다 같은 키·같은 검증을 사용합니다.
  (서명·형식·버전·대상·크기·SHA-256.) 같은 버전이 양쪽에 있으면 stable을 권하며 버전을 낮추는 업데이트는 권하지 않습니다.
- 끄면 stable 위치만 봅니다. stable 업데이트 정보에 베타 버전이 있어도 받지 않습니다. 베타 버전을 설치한 사람이 설정을 꺼도
  다음 stable이 현재 버전보다 새로워질 때까지 아무것도 권하지 않습니다(베타에서 정식 버전으로 돌아가는 것은 그때).
- stable이 나오면 `releases/latest`가 전환됩니다. 베타 배포 위치는 건드리지 않아도 되며 베타 사용자에게도 베타보다 새로운 stable이 보입니다.
- 확인 중 설정을 끄면 해당 확인은 베타 결과만 버리고 stable 결과로 응답합니다(현재보다 새 stable이 있으면 권함. 「최신」이라고 하는 것은 stable도 새롭지 않을 때만).
  다운로드 중인 베타는 취소하고 전송 완료 후 확인·쓰기 중에 꺼도 받지 않습니다. 이미 다운로드한 베타는 저장 위치의 설치 프로그램도 지웁니다.
- 실패 이유(통신 불가·검증 실패)는 stable 수신 결과로 정합니다. 베타 위치를 조회할 수 없는 것(아직 하나도 배포하지 않은 동안에는 항상 해당)은 이유에 섞지 않습니다.
- 베타 위치를 조회할 수 없거나(아직 하나도 배포하지 않은 경우 포함) 서명이 맞지 않거나 대상 배포물이 없으면 stable 결과만으로 응답합니다.
  반대로 stable을 조회할 수 없으면 새 베타를 찾았을 때는 그것을 권하고, 찾지 못했을 때는 「최신」이라고 답하지 않고 실패를 알립니다.

<a id="置き場の決め方の比べ"></a>

### 배포 위치 선택 비교

| 후보 | 인증 없이 조회 가능 | 횟수 제한 | 캐시 | 견고성 | 서명과의 관계 | 공개 순서·운영 |
|---|---|---|---|---|---|---|
| **고정 태그 Release의 자산**(채택) | 가능(stable 경로와 같은 종류의 URL) | 없음(웹 배포 경로로 API 횟수 제한 밖) | 최초 응답은 `no-cache`(stable 경로에서 확인). 교체는 자산 삭제와 재등록이므로 오래된 복사본이 반환될 가능성은 낮음(교체 반영 지연은 측정하지 않음) | 앱이 정한 업데이트 정보 형식을 그대로 반환. GitHub API의 JSON 형식에 의존하지 않음 | 서명된 업데이트 정보 원본. stable과 같은 키·같은 형식으로 앱은 추가 장치 없이 검증 가능 | 공개(published) 시 한 번만 덮어쓰기. 릴리스 초안인 동안에는 위치가 변하지 않음. 배포 위치용 Release 하나 유지 |
| GitHub API로 최신 prerelease 조회(`releases`) | 가능(인증 없음) | 인증 없으면 IP당 시간당 60회. 공유 회선 사용자가 한꺼번에 차단됨 | API 응답에 의존 | Release 본문 등을 포함하는 큰 JSON 형식·페이지 이동·순서(생성일순이며 stable과 섞임)에 의존 | API 응답은 서명 밖. 결국 업데이트 정보를 받는 두 번째 통신 필요 | 초안은 인증 없이는 보이지 않음. 배포 위치를 갱신하는 수고 없음 |
| `releases.atom`(피드) 읽기 | 가능 | 없음 | 피드 캐시에 의존 | XML 분석과 의존성 추가. 항목에서 prerelease 여부를 확실하게 구분할 수 있다는 보장 없음 | 서명 밖의 안내이며 두 번째 통신 필요 | 배포 위치를 갱신하는 수고 없음 |
| 저장소 파일(`main` raw)에 배치 | 가능 | 없음 | 몇 분의 캐시 | 파일 이름 고정 | 서명된 업데이트 정보를 둘 수 있음 | 워크플로가 보호된 `main`에 push하게 되어 PR 필수 운영과 맞지 않음 |

채택 이유는 인증과 API 횟수에 의존하지 않고 서명된 업데이트 정보를 원본 그대로(재서명 없이·개인키에 접근하지 않고) 둘 수 있으며 앱의 검증이 stable과 하나로 통합되기 때문입니다.
대가는 배포 위치용 Release가 하나 늘어나는 점(Releases 목록에 prerelease로 표시)과 교체 중 몇 초간 수신이 실패할 수 있는 점입니다.
(그때의 확인은 stable 결과로 응답하므로 업데이트가 멈추지는 않음.)

<a id="ベータ版を出す手順"></a>

### 베타 버전 배포 절차

1. 작업 브랜치에서 `Cargo.toml`의 workspace.package.version을 `0.4.0-rc.1`처럼 `alpha.N`·`beta.N`·`rc.N` 버전으로 설정하고 `Cargo.lock`을 갱신해 커밋한 뒤 main 대상 PR을 냅니다.
   (CI가 배포물을 빌드합니다. `cargo xtask preflight --kind prerelease`로 버전 형식도 확인 가능.)
2. Actions의 「배포물 만들기」를 kind=prerelease로 설정하고 먼저 작업 브랜치에서 dry-run=true로 시험 실행합니다. PR을 main에 병합한 뒤 main에서 dry-run=false(environment 승인. `main`에서만 실행 가능)로 실행하면
   prerelease 표시가 붙은 릴리스 초안이 나옵니다.
3. 실제 장치에서 확인한 뒤 초안을 공개합니다. **공개하면 `beta-channel.yml`이 실행되어** 공개한 Release의 배포물을 모두 받고 공개키만으로 서명·버전·크기·SHA-256·패키지 내용을 다시 확인한 뒤
   (`cargo xtask beta-channel`) `updater-beta` Release의 `updater-v1.json`을 덮어씁니다(처음이면 Release 생성).
   배포 위치용 Release는 매번 prerelease·「최신 Release」로 설정하지 않는 상태로 다시 맞추므로 stable의 `releases/latest`는 변하지 않습니다.
4. 「베타 버전 사용」을 켠 앱은 다음 업데이트 확인에서 새 베타 버전을 찾습니다. 켜지 않은 사람에게는 변화가 없습니다.

`xtask beta-channel` 규칙:
- 버전은 `alpha.N`·`beta.N`·`rc.N` 형식만 허용합니다(정식 버전이나 다른 식별자의 버전은 배포 위치에 올리지 않음).
- 배포 위치의 현재 업데이트 정보 버전이 이번 버전 이상이면 교체하지 않습니다(오래된 베타를 나중에 공개해도 배포 위치가 되돌아가지 않음). 현재 배포 위치를 읽을 수 없으면 복구 목적으로 올립니다.
  이 비교는 되돌림 방지 기준이며 서명 확인은 아닙니다(앱이 서명을 확인).
- 워크플로는 배포 위치용 Release가 아직 없을 때(`release not found`)만 새로 만듭니다. 수신 실패 등 다른 실패에서는 멈춥니다(배포 위치의 버전을 보지 않고 덮어쓰지 않기 위함).
- 실패하면 Actions의 「Re-run」으로 같은 공개에 대해 재실행할 수 있습니다. 배포 위치용 Release를 직접 지우거나 prerelease 표시를 해제하지 마세요.
  (해제하면 `releases/latest`가 이 위치를 가리킵니다. 다음 베타 공개 때 다시 설정됩니다.)

보장하지 않는 사항: 배포 위치의 업데이트 정보는 서명된 과거 정보의 재전송을 막지 못합니다(stable과 같음). 베타 위치에 반영되기까지는 공개 후 워크플로가 끝날 때까지(몇 분) 걸립니다.
반영 지연(자산 교체부터 받는 쪽에 새 내용이 반환되기까지)은 측정하지 않았습니다.

<a id="以前のバージョンとの違いと互換"></a>

### 이전 버전과의 차이 및 호환성

- 없어진 기능: 이전에는 베타 버전으로 실행 중인 앱이 stable 업데이트 정보에 올라온 베타 버전도 자동으로 받았습니다(현재 버전이 사전 릴리스이면 받는다는 판정).
  지금은 「베타 버전 사용」 설정이 켜져 있을 때만 받습니다. stable 업데이트 정보는 preflight가 베타 버전을 거부하므로 공식 절차로 배포한 것에서는 발생하지 않는 상황이지만,
  로컬에서 만들어 stable 위치에 올린 베타는 설정을 끈 앱에 더 이상 전달되지 않습니다.
- 설정 파일: 베타를 켜면 `update.conf`에 `use_beta=on` 행이 추가됩니다(끄면 이전 버전과 같은 한 줄뿐이며 파일이 없던 사용자에게는 만들지 않음).
  0.3.x는 이 파일을 「한 줄만 있는 형식」으로 읽으므로 행이 늘어난 파일은 읽을 수 없는 선택으로 처리하여 최초 질문(시작 시 확인할지)을 다시 표시합니다.
  베타 설정 자체는 0.3.x에 없으므로 되돌린 앱은 stable만 확인합니다. 끈 뒤 되돌리면 이전의 한 줄 형식을 유지합니다.
- 배포 워크플로: kind 기본값은 prerelease 그대로이지만 prerelease는 베타 형식의 버전만 받게 되었습니다. 정식 형식의 버전을 배포할 때는 kind=stable을 선택합니다.

<a id="更新情報の互換"></a>

## 업데이트 정보 호환성

업데이트 크레이트는 서명을 통과한 업데이트 정보 중 알 수 없는 target의 배포물은 건너뛰고 자신의 target 배포물만 엄격하게 확인합니다.
macOS 등 대상을 추가한 버전의 업데이트 정보를 이미 배포한 버전이 거부하지는 않습니다. 설치 프로그램도 알 수 없는 키의 배포물로서 이전 클라이언트가 건너뜁니다. 배포물 개수 상한은 고정값(`MAX_ASSETS`)입니다.

다음 변경은 schema를 올려야 합니다. 기존 클라이언트는 schema가 다른 업데이트 정보를 거부하며 이전 버전에 그대로 남습니다.

- 본문 항목의 추가·삭제·의미 변경
- 알려진 target 배포물 항목의 추가·삭제·의미 변경

schema를 올릴 때는 이전 schema를 읽는 클라이언트가 가져가는 업데이트 정보를 이전 형식으로 남깁니다.
앱이 가져가는 URL(`UPDATER_URL`)은 파일 이름에 schema 번호를 포함합니다. 새 schema는 새 이름(`updater-v2.json`)에 두고
이전 `updater-v1.json`도 같은 Release에 계속 두세요(`UPDATER_FILE`·`UPDATER_URL`·`UPDATER_SCHEMA`를 함께 변경).

<a id="更新署名の鍵初回だけ"></a>

## 업데이트 서명 키(처음 한 번만)

배포 관리자가 안전한 로컬 환경에서 실행하세요. 저장소 밖에 접근을 제한한 보관 위치를 준비합니다.

```sh
cargo xtask keygen --output /secure/location/update-private-key.hex
```

개인키는 32바이트 seed를 hex로 기록한 파일입니다. 기존 파일은 덮어쓰지 않습니다.
Unix에서는 생성 권한을 0600으로 설정합니다. Windows에서는 보관 폴더의 ACL을 관리자 본인으로 제한하세요.
표준 출력에는 공개키만 표시됩니다. 개인키는 Git·로그·배포물에 넣지 않으며 분실에 대비해 안전하게 백업합니다.
공개키는 16진수 64문자로 배포 빌드가 앱에 포함합니다. 기록을 잃었으면 개인키 파일에서 추출할 수 있습니다.

```sh
cargo xtask pubkey --key-file /secure/location/update-private-key.hex
```

앱은 빌드 시 환경 변수 `YOLUPAINTER_UPDATE_PUBLIC_KEY`에 넣은 공개키만 신뢰하며(`yolu_update::embedded_public_key`),
공개키를 포함하지 않은 빌드(소스에서 로컬로 빌드한 경우 등)는 도움말 메뉴의 업데이트 항목과 최초 질문을 표시하지 않고 통신도 하지 않습니다.
`cargo xtask build`는 이 환경 변수를 검사하며 빈 값(GitHub의 미설정 변수)은 미설정으로 취급하고, 값이 들어 있지만 키로 사용할 수 없는 경우
(길이·형식이 다름, 약한 키)는 빌드를 멈춥니다. 릴리스 초안을 만드는 배포는 `--require-update-key`를 붙여 미설정도 차단합니다.
(업데이트할 수 없는 앱을 배포하지 않기 위함.) 업데이트 서버에서 공개키를 받는 설계로 만들지 마세요.
키를 바꾸려면 기존 앱이 신뢰하는 공개키도 바꿔야 하며 자동 키 갱신은 지원하지 않습니다.

로컬에서 서명하려면 다음과 같이 실행합니다. 개인키 자체를 인수에 쓰지 마세요.

```sh
cargo xtask updater-json --version 0.1.0-rc.1 --assets target/dist --sign --key-file /secure/location/update-private-key.hex
```

`--key-file`을 생략하면 환경 변수 `YOLUPAINTER_UPDATE_PRIVATE_KEY`를 읽습니다.
서명한 뒤에는 공개키만으로 앱과 같은 검증(서명·버전·크기·SHA-256)을 통과하는지 확인합니다.
다른 키로 서명한 업데이트 정보, 배포물과의 불일치, 목록에는 있지만 실제로 없는 배포물은 여기서 실패합니다.
zip·tar.gz는 내용도 열어 패키지 목록(위의 문서 포함)과 대조합니다. 누락 파일·목록에 없는 파일·같은 이름의 중복은 모두 이름을 나열하고 거부합니다.
(설치 프로그램 내부는 열 수 없으므로 목록대로의 단계에서 만드는 것과 스크립트 테스트로 확인.)

```sh
cargo xtask verify --version 0.1.0-rc.1 --assets target/dist --public-key <公開鍵の hex>
```

서명은 Ed25519이며 대상은 JSON envelope의 `payload` 문자열 UTF-8 바이트입니다.
`signature`는 64바이트 서명의 hex입니다. payload를 분석·재정렬한 뒤 검증하지 마세요.
본문에는 schema=1, version, target/name/url/sha256/size를 가진 assets가 들어갑니다.

<a id="github-の設定と起動"></a>

## GitHub 설정과 실행

1. 저장소에 environment `release`를 만들고 승인자를 정하며 사용할 수 있는 브랜치를 `main`, 태그를 `v*`로 제한합니다.
2. **environment secret** `YOLUPAINTER_UPDATE_PRIVATE_KEY`에 개인키 파일의 내용을 저장합니다. 저장소 전체 secret에는 두지 않습니다.
3. 저장소 변수(Variables) `YOLUPAINTER_UPDATE_PUBLIC_KEY`에 공개키 hex를 저장합니다. 공개키는 비밀 정보가 아닙니다.
   배포물 빌드는 이 값을 앱에 포함합니다(main 대상 PR CI 빌드와 dry-run=false 배포 빌드에서는 비어 있으면 멈춤. 산출물 목록에도 기록되며 변수를 바꾸면 PR CI 산출물을 받지 않고 다시 빌드).
   draft 작업은 서명 직후 이 공개키로 `verify`를 실행하며 통과하지 않으면 릴리스 초안을 만들지 않습니다.
4. Actions의 「배포물 만들기」를 열고 kind를 prerelease 또는 stable로 설정합니다.
   버전이 `0.4.0-rc.1` 같은 베타이면 prerelease, `0.4.0`처럼 사전 릴리스 식별자가 없는 정식 형식이면 **stable**을 선택합니다.
   kind 기본값은 prerelease이므로 정식 형식의 버전을 기본값 그대로 실행하면 preflight의 version에서 멈춥니다(릴리스 초안을 만들기 전에 멈춤).
   실행할 브랜치는 dry-run=true 시험 실행이면 배포 대상 브랜치를 선택할 수 있습니다. **dry-run=false는 `main`에서만** 가능합니다(릴리스 초안 작업은 environment `release`를 사용하므로 1의 제한에 맞지 않는 브랜치는 environment가 거부).
   따라서 실제 배포는 PR을 main에 병합하고 CI 확인이 끝난 뒤 실행합니다.
5. 처음에는 **dry-run=true(기본값. 릴리스 초안을 만들지 않는 시험 실행)**로 실행합니다. main 대상 PR CI에 같은 트리의 산출물이 있으면 받고([배포물을 빌드하는 위치와 받는 경로](#配る物をビルドする場所と受け取る道)), 없으면 Windows와 macOS(`linux`를 켰으면 Linux도. `macos`를 끄면 macOS 제외)를 빌드해 미서명 JSON을 포함한 `release-preview` artifact를 만듭니다. secret에 접근하지 않고 Release도 만들지 않습니다.
6. artifact를 받아 빌드한 OS(Windows와 macOS. 입력 `linux`를 켰으면 Linux도)에서 압축 해제·실행·동봉 문서·라이선스 전문을 확인합니다. Linux 실행 파일에는 실행 권한이 있습니다.
   Windows는 설치 프로그램으로 설치·실행·업데이트(이전 버전 설치 프로그램으로 설치한 위에 설치)·제거를 진행합니다. macOS는 zip을 풀어 `codesign -dv`·`lipo -info`·실행을 확인하고, README 절차(「그래도 열기」)로 열 수 있는지, 창이 표시되고 그릴 수 있는지를 실제 Mac에서 한 번 확인합니다.
7. PR을 main에 병합한 뒤 main에서 kind를 맞춰 dry-run=false로 실행하고 environment 승인을 합니다. 서명된 JSON과 아카이브를 **릴리스 초안(Draft Release)**에 업로드합니다.
   병합 후 main의 트리(파일 내용)가 PR 선두 트리와 같으면(`main-tested.yml`에서 확인) PR CI 산출물을 받을 수 있습니다. 다르면 일반적으로 다시 빌드합니다([배포물을 빌드하는 위치와 받는 경로](#配る物をビルドする場所と受け取る道)).
8. 서명과 각 배포물의 SHA-256·크기는 워크플로가 공개키로 확인한 상태입니다. 초안의 태그와 대상 커밋, 버전, prerelease 상태를 확인합니다.
   본문은 비어 있는 상태로 만들어지므로 실제 장치에서 확인한 뒤 관리자가 본문을 작성하고 직접 공개합니다. macOS 시제품이 포함되는 버전은 서명이 없다는 점·여는 방법·동작을 확인한 환경·자동 업데이트가 없다는 점도 본문에 씁니다.
   (코드 서명을 붙이게 되면 README의 「Code signing policy」 절 링크도 본문에 넣습니다. 절 준비는 [코드 서명](#コード署名signpath-foundation) 참조.)
   **공개한 시점에 `releases/latest`가 전환되어 앱의 업데이트 확인이 해당 버전을 찾기 시작합니다**(stable만. prerelease는 `latest`에 나타나지 않음).
   prerelease를 공개하면 `.github/workflows/beta-channel.yml`이 실행되어 베타 버전 배포 위치를 갱신합니다([베타 버전 배포 위치](#ベータ版の置き場)).

동시 실행은 하나입니다. 실행 중인 처리는 자동 취소하지 않습니다. 같은 태그의 기존 Release는 덮어쓰지 않습니다.
실패한 뒤에는 릴리스 초안과 태그 상태를 확인한 다음 재실행하세요. 비공개 저장소에서는 인증과 Actions 사용 한도에도 주의하세요.
버전 형식과 kind는 짝을 이룹니다. kind=prerelease는 `alpha.N`·`beta.N`·`rc.N` 버전만, kind=stable은 사전 릴리스 식별자가 없는 버전만 받습니다.
(`cargo xtask preflight`의 version이 확인하며 맞지 않으면 릴리스 초안을 만들기 전에 멈춤.)

<a id="コード署名signpath-foundation"></a>

## 코드 서명(SignPath Foundation)

0.x는 서명 없이 배포하고 사용되기 시작하면 [SignPath Foundation 무료 코드 서명](https://signpath.org/terms.html)을 신청합니다.
저장소에서 준비한 조건과 신청할 때 관리자가 할 일입니다.

| 조건 | 상태 |
|---|---|
| OSI 라이선스·자체 작성 코드만 | [MIT](../../LICENSE). 의존성 라이선스는 `THIRD_PARTY.md`와 라이선스 전문으로 추적 |
| 제품명과 버전 메타데이터 | 실행 파일(`build.rs`)과 설치 프로그램(NSIS)이 제품명 `YoluPainter`와 같은 버전을 가짐 |
| 묻지 않고 통신하지 않음 | 업데이트 확인은 최초 질문에서 「예」를 선택했을 때와 직접 눌렀을 때만 수행. 개인정보 보호는 README 「개인정보 보호」 절에 한 문장, 전문은 [INSTALL.md](INSTALL.md) |
| 시스템 변경 고지·제거 | `.ylp` 연결은 선택 항목이며 무인 실행에서는 설정하지 않음(현재 상태 유지). 제거 프로그램은 설치한 파일만 지우고 사용자 데이터는 확인 |
| Code signing policy 표기 | README에 현재 이 절은 없음. 신청 전에 서명 역할(작성자·검토·서명 승인)과 개인정보 보호를 적은 절을 추가. 정형 문구는 신청 승인 후 추가 |
| 검증 가능한 빌드·릴리스별 수동 승인 | `配布物の作成` 워크플로(수동 실행·dry-run 기본값·릴리스 초안 생성에는 environment 승인 필요) |

관리자가 할 일:

1. 첫 릴리스(서명 없음)를 배포하고 사용되기 시작할 때까지 기다립니다(신청 조건은 「서명할 형태로 이미 릴리스되어 있음」).
2. README에 「Code signing policy」 절(서명 역할과 개인정보 보호)을 추가합니다.
3. GitHub와 SignPath의 모든 구성원에게 다중 인증을 활성화하고 SignPath Foundation에 신청합니다.
4. 승인되면 README의 「Code signing policy」 절 앞에 다음 정형 문구를 추가하고 Release 본문에서도 이 절로 연결합니다.

   ```
   Free code signing provided by [SignPath.io](https://about.signpath.io/), certificate by [SignPath Foundation](https://signpath.org/)
   ```

5. SignPath에서 서명할 실행 파일의 제품명을 `YoluPainter`로, 제품 버전을 빌드 안에서 동일한 값으로 강제하는 설정(artifact configuration)을 만들고
   워크플로에 서명 단계를 추가합니다. 서명 대상은 `yolupainter.exe`(zip과 설치 프로그램에 넣기 전)와 설치 프로그램 자체입니다.
   서명한 실행 파일이 들어 있는 zip·설치 프로그램을 다시 만들고, 업데이트 정보(`updater-json`)의 SHA-256은 서명 후 배포물에서 생성합니다.
   설치 프로그램이 출력하는 제거 프로그램의 서명은 NSIS `!uninstfinalize`를 사용하는 절차가 되므로 이 단계에서 설계합니다.
   이 서명 단계는 아직 워크플로에 들어 있지 않습니다.

<a id="検証と現在の範囲"></a>

## 검증과 현재 범위

```sh
cargo test -p yolu-update -p xtask --locked
python3 tools/test-release-tools.py
python3 tools/test-dist.py
cargo xtask preflight
actionlint .github/workflows/*.yml
cargo test --workspace --locked
cargo clippy --workspace --all-targets --locked -- -D warnings
```

업데이트 크레이트는 HTTP 연결부를 교체하는 방식으로 서명 확인·버전 비교·명시적 승인 후 다운로드·크기와 SHA-256 확인까지 제공합니다.
앱의 HTTP 구현(`crates/yolu-app/src/update/http.rs`)은 Windows에서는 WinHTTP(OS 인증서·프록시·TLS), Linux와 macOS에서는 `curl`을 사용하며
https만 허용·시간 초과·읽는 중 크기 상한·취소를 지키고 끝나지 않는 리디렉션 연쇄는 실패로 처리합니다. 리디렉션 횟수는 curl은 최대 5회(대상도 https만),
WinHTTP는 5회로 제한하는 설정을 시도하며 설정할 수 없는 환경에서는 OS 기본값(10회)까지입니다. https에서 http로 이동은 WinHTTP 기본값이 거부합니다.
현재 API는 동기식이며 받은 데이터를 메모리에 보관합니다. 업데이트 정보는 1 MiB, 배포물은 2 GiB가 상한입니다.
베타 버전은 「베타 버전 사용」 설정이 켜져 있을 때만 제안합니다(stable 업데이트 정보에 베타 버전이 있어도 받지 않음). 같은 버전·이전 버전으로의 업데이트는 제안하지 않습니다.
서명된 과거 정보를 재전송해 「새 버전을 보이지 않게 하는」 공격에 대한 최신성 보장은 없습니다.
다운로드는 사용자가 「업데이트」를 누른 뒤에만 시작합니다. 다운로드한 설치 프로그램은 서명된 업데이트 정보의 SHA-256·크기로 확인한 것을
사용자별 위치(Windows는 `%LOCALAPPDATA%\YoluPainter\updates`)에 두고 실행 직전에 다시 확인합니다.
저장 위치의 설치 프로그램과 쓰다 남은 파일은 다음 다운로드 및 앱 시작 시(현재 버전 이하인 것만. 실행 중인 것은 다음 시작 시) 정리합니다.
앱 자체는 실행 파일을 교체하지 않습니다. 설치된 Windows 앱은 설치 프로그램을 무인 실행한 뒤 종료하며(설치 프로그램이 종료를 기다려 설치하고 `/RUN`으로 재실행),
zip·Linux·macOS는 해당 버전의 릴리스 페이지를 엽니다. 실행 파일 옆에 `uninstall.exe`가 있는지로 설치 프로그램을 통해 설치한 것인지 구분합니다.
macOS는 Intel과 Apple Silicon 모두 같은 키 `universal-apple-darwin`의 배포물을 찾으며, 다운로드하지 않고 저장 위치(`updates`)도 만들지 않으며 교체도 하지 않습니다(`.app` 서명이 ad-hoc뿐이라 자기 자신을 교체하는 것은 안전하지 않기 때문).
업데이트 정보에 이 배포물이 없는 버전(`macos`를 끄고 배포한 버전)에서는 서명이 올바른 상태에서 「이 환경용 배포물이 없습니다」라고 응답합니다.
