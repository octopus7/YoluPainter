[한국어](INSTALL.md) · [日本語](../INSTALL.md) · [English](../en/INSTALL.md)

<a id="ダウンロードと更新"></a>

# 다운로드와 업데이트

> 이 포크의 한국어 UI는 이 소스에서 빌드한 실행 파일에 포함됩니다. 원 프로젝트의 Releases 실행 파일과는 구분해 사용하세요. [한국어 지원 안내](../../KOREAN_SUPPORT.md)를 참고하세요.


[Releases](https://github.com/YozoraKurage/YoluPainter/releases)에서 받을 수 있습니다. Windows (64비트)는 설치 프로그램과 zip의 2가지 형식이 있습니다. Claude Desktop용 확장 (`.mcpb`)도 제공됩니다. Linux는 tar.gz (x86_64와 실험적인 ARM)가 있습니다 ([아래](#linux)). Mac은 실험적인 zip이 있습니다 ([아래](#mac試作)).

- `yolupainter-<バージョン>-x86_64-pc-windows-msvc-setup.exe` (설치 프로그램): 사용자별로 설치합니다. 관리자 권한은 필요하지 않습니다. 설치 위치는 `%LOCALAPPDATA%\Programs\YoluPainter` (변경 가능)이며, 시작 메뉴에 등록합니다. `.ylp`를 이 앱으로 열도록 연결할지는 선택할 수 있습니다.
  - 제거는 “설정 → 앱”에서 진행하며, 설정과 복구 데이터도 삭제할지 묻습니다 (묻지 않으면 유지됩니다. 직접 만든 항목은 어느 쪽이든 삭제하지 않습니다. [아래 표](#アンインストールとデータ)).
  - 화면을 표시하지 않고 설치하려면 `/S`, 파일 연결은 `/ASSOC=1` (연결하지 않으려면 `/ASSOC=0`), 설치 후 앱을 실행하려면 `/RUN`을 붙입니다.
  - 화면을 표시하지 않고 제거하려면 `/S`, 설정과 복구 데이터도 삭제하려면 `/DELETEDATA`를 붙입니다.
- `yolupainter-<バージョン>-x86_64-pc-windows-msvc.zip`: 압축을 풀고 `yolupainter.exe`를 실행합니다 (설치 불필요).
- `yolupainter-<バージョン>-x86_64-pc-windows-msvc.mcpb`: Claude Desktop에 설치하는 확장입니다 ([AI 어시스턴트에서 조작](MCP.md)). 더블 클릭해 설치합니다. 내용은 실행 중인 앱으로의 중계 기능뿐이며 도구는 앱이 제공합니다 (앱 업데이트 시 갱신). Claude Code와 Codex는 플러그인으로 설치합니다.

두 배포물 모두 명령줄 프로그램 `yolupainter-cli.exe` ([사용법](CLI.md)), `README.md`, 이 문서를 포함한 `docs` 폴더 (영어는 `docs/en`, 한국어는 `docs/ko`)가 들어 있어 네트워크 없이도 읽을 수 있습니다.

## Linux

- `yolupainter-<バージョン>-x86_64-unknown-linux-gnu.tar.gz`: x86_64 Linux용입니다.
- `yolupainter-<バージョン>-aarch64-unknown-linux-gnu-experimental.tar.gz`: ARM (aarch64) Linux용 실험적 버전입니다. 실제 ARM 장치에서 실행해 확인하지 않았습니다.

둘 다 압축을 풀고 `yolupainter`를 실행합니다 (설치 불필요). 압축을 푼 위치에는 명령줄 프로그램 `yolupainter-cli` ([사용법](CLI.md)), `README.md`, 라이선스 전문, 이 문서를 포함한 `docs` 폴더 (영어는 `docs/en`, 한국어는 `docs/ko`)가 들어 있습니다.

앱 메뉴에 표시하려면 압축을 푼 폴더에서 `./install-desktop-entry.sh`를 실행합니다 (현재 사용자에게만 등록하며 관리자 권한 불필요). 메뉴 항목 (`~/.local/share/applications/yolupainter.desktop`)과 아이콘을 배치하고 압축을 푼 폴더의 `yolupainter`를 실행하도록 설정합니다. 폴더를 이동했다면 다시 실행합니다. `./install-desktop-entry.sh --remove`로 등록을 해제합니다.

창은 X11로 열기 때문에 Wayland 데스크톱에서는 XWayland에서 동작합니다 (XWayland가 없는 Wayland 전용 환경에서는 실행 불가). GPU 드라이버가 필요합니다. 파일 선택에는 D-Bus 세션, `xdg-desktop-portal`, 데스크톱에 맞는 포털 백엔드가 필요하며, 확인 창 (저장하지 않은 변경 사항 폐기 등)에는 `zenity`가 필요합니다 (없으면 해당 확인이 필요한 작업은 취소됨). 자세한 내용은 [BUILDING의 Linux 섹션](BUILDING.md#linux試用)을 참고하세요.

Linux 앱은 스스로 업데이트하지 않습니다 (다운로드와 교체 모두 수행하지 않음). 첫 질문에서 “예”를 선택하면 시작할 때마다 GitHub에 최신 버전을 문의합니다 (“도움말 → 업데이트 확인…”에서는 수동 확인). 새 버전이 있으면 “도움말” 제목에 표시가 붙으며, “YoluPainter x.y.z 릴리스 열기”로 해당 버전의 릴리스 페이지를 엽니다. 새 tar.gz의 압축을 다시 풀어 주세요. 다른 폴더에 풀었다면 해당 폴더에서 `./install-desktop-entry.sh`를 다시 실행하면 메뉴에서 새 버전을 실행합니다. 설정과 프로젝트는 유지됩니다.

<a id="mac試作"></a>

<a id="mac-experimental"></a>

## Mac (실험적)

- `yolupainter-<バージョン>-macos-universal-experimental.zip`: 압축을 풀면 `YoluPainter.app` (Apple Silicon용과 Intel용 코드를 모두 포함한 universal), `README.md`, 라이선스 전문, 이 문서를 포함한 `docs` 폴더 (영어는 `docs/en`, 한국어는 `docs/ko`)가 들어 있습니다. `YoluPainter.app`을 “응용 프로그램” 등으로 옮겨 사용합니다. 명령줄 프로그램 ([사용법](CLI.md))은 `YoluPainter.app/Contents/MacOS/yolupainter-cli`입니다.

실험적 버전이므로 Apple 개발자 서명과 공증은 없습니다 (ad-hoc 서명만 있음). 실행 파일이 요구하는 최소 macOS는 11입니다. Apple Silicon Mac (macOS 27)에서 0.6.0을 열어 창이 표시되고 그릴 수 있음을 확인했습니다. Intel Mac에서는 실행하지 않았습니다. 서명이 없어 압축을 푼 `YoluPainter.app`을 처음 열면 macOS가 차단합니다. 다음 순서로 엽니다 ([Apple 안내](https://support.apple.com/ja-jp/guide/mac-help/mh40616/mac)).

1. `YoluPainter.app`을 더블 클릭합니다. 차단되면 표시된 창을 닫습니다.
2. Apple 메뉴 → “시스템 설정”을 열고 사이드바의 “개인정보 보호 및 보안”을 누릅니다.
3. “보안” 항목으로 이동해 “열기”를 누릅니다.
4. “확인 없이 열기”를 누릅니다 (앱 열기를 시도한 뒤 약 1시간 동안만 사용 가능).
5. 로그인 암호를 입력하고 “확인”을 누릅니다. 이후에는 더블 클릭으로 엽니다.

Mac 앱은 스스로 업데이트하지 않습니다 (다운로드와 교체 모두 수행하지 않음). 첫 질문에서 “예”를 선택하면 시작할 때마다 GitHub에 최신 버전을 문의합니다 (“도움말 → 업데이트 확인…”에서는 수동 확인). 새 버전이 있으면 “도움말” 제목에 표시가 붙으며, “YoluPainter x.y.z 릴리스 열기”로 해당 버전의 릴리스 페이지를 엽니다. 새 zip의 압축을 풀고 `YoluPainter.app`을 교체해 주세요. 교체해도 설정과 프로젝트는 유지됩니다.

제거하려면 `YoluPainter.app`을 휴지통에 넣습니다. 그 밖의 데이터는 자동으로 삭제되지 않으므로 필요 없다면 직접 삭제합니다.

| 위치 | 내용 |
| --- | --- |
| `~/Library/Application Support/YoluPainter/` | 설정·복구용 세대 (`recovery/`)·충돌 기록 (`logs/`)·Live Link 교환 폴더 (`LiveLink/`), 라이브러리·브러시 등 직접 만든 항목 |
| `~/Library/Caches/YoluPainter/thumbnails/` | 썸네일 캐시 |

<a id="更新"></a>

## 업데이트

처음 실행하면 “시작할 때 업데이트를 확인할까요?”라고 묻습니다. “예”를 선택했을 때만 시작할 때마다 GitHub에 최신 버전을 문의합니다. 이 선택은 “편집 → 설정…”의 “업데이트” 구분에 있는 “시작할 때 업데이트 확인”에서 언제든 바꿀 수 있으며, “도움말 → 업데이트 확인…”으로 수동 확인할 수도 있습니다.

“편집 → 설정…”의 “업데이트” 구분에서 “베타 버전 사용”을 켜면 정식 버전 이전의 베타 버전 (버전이 `0.4.0-rc.1`처럼 `alpha`·`beta`·`rc` 식별자를 가진 버전)도 업데이트 후보에 포함하고 정식과 베타 중 더 새로운 버전을 권장합니다. 기본값은 꺼짐이며, 꺼져 있을 때는 정식 버전만 확인합니다. 베타 버전 사용 중에는 상태 표시줄 버전 왼쪽에 “베타 버전”이 나타납니다. 버전을 낮추는 업데이트는 하지 않으므로, 베타 버전을 설치한 사용자가 설정을 꺼도 다음 정식 버전이 현재 버전보다 새로워질 때까지는 아무것도 권장하지 않습니다.

새 버전이 있으면 “도움말” 제목에 표시가 붙습니다. 설치 프로그램으로 설치한 Windows에서는 “YoluPainter x.y.z로 업데이트”로 다운로드·검증·업데이트를 진행합니다 (저장하지 않은 변경 사항이 있으면 먼저 저장할지 확인). 다운로드에는 서명된 업데이트 정보의 서명·크기·SHA-256을 확인한 파일만 사용합니다. zip으로 압축을 푼 Windows와 Linux, Mac에서는 해당 버전의 릴리스 페이지를 엽니다. 소스에서 빌드한 앱에는 업데이트 항목 (도움말의 “업데이트 확인…”과 설정의 “업데이트”)이 없습니다 (배포용 빌드에만 업데이트 공개 키를 내장).

설치된 YoluPainter의 다른 창이 열려 있으면 “다른 YoluPainter가 열려 있습니다”를 표시하고 업데이트를 시작하지 않습니다 (설치 프로그램이 사용 중인 실행 파일을 변경할 수 없기 때문). 다운로드한 설치 프로그램은 유지되므로 다른 창을 닫고 “업데이트 후 다시 시작”을 다시 누르면 바로 설치합니다. 무인 실행 시 설치 프로그램은 실행 파일이 사용 중인 동안 최대 60초 기다리고, 여전히 사용 중이면 아무것도 변경하지 않고 종료합니다. 앱 업데이트는 `/RUN`을 붙여 실행하므로 이 경우 현재 설치된 앱을 다시 시작합니다 (`/RUN` 없는 무인 실행에서는 시작하지 않음).

<a id="アンインストールとデータ"></a>

## 제거와 데이터

제거 시 설치된 파일 (앱 본체·문서·바로 가기·파일 연결)과 업데이트용으로 다운로드한 설치 프로그램 (`%LOCALAPPDATA%\YoluPainter\updates`)을 삭제합니다. 그 밖의 데이터는 “설정과 복구 데이터도 삭제할까요?”에 “예”라고 답했을 때 (무인 실행에서는 `/DELETEDATA`) 아래 표의 “삭제됨” 항목만 삭제합니다. 직접 만든 항목은 어떤 답을 선택해도 삭제하지 않습니다.

| 위치 | 내용 | “삭제” 선택 시 |
| --- | --- | --- |
| `%APPDATA%\YoluPainter\settings.conf` | 설정 | 삭제됨 |
| `%APPDATA%\YoluPainter\recovery.conf`·`update.conf`·`layout.json` | 복구 설정·업데이트 확인 및 베타 버전 선택·패널과 창 배치 | 삭제됨 |
| `%APPDATA%\YoluPainter\places.conf` | 파일 선택 창의 시작 위치 (종류별로 이전에 선택한 폴더) | 삭제됨 |
| `%APPDATA%\YoluPainter\recovery\` | 복구용 세대 | 삭제됨 |
| `%APPDATA%\YoluPainter\logs\` | 충돌 기록 | 삭제됨 |
| `%LOCALAPPDATA%\YoluPainter\thumbnails\` | 썸네일 캐시 | 삭제됨 |
| `%LOCALAPPDATA%\YoluPainter\LiveLink\` | Live Link 교환 폴더 (Unity 요청과 응답·실행 중 표시) | 삭제됨 |
| `%APPDATA%\YoluPainter\Library\` (기본 라이브러리 위치) | 개인 라이브러리 | 유지됨 |
| `%APPDATA%\YoluPainter\brushes\` | 사용자 브러시·지우개 | 유지됨 |
| `%APPDATA%\YoluPainter\subtools\` | 사용자 보조 도구 | 유지됨 |
| `%APPDATA%\YoluPainter\gradients\` | 그라데이션 세트 | 유지됨 |
| `%APPDATA%\YoluPainter\colorsets\` | 색상 세트 | 유지됨 |
| `%APPDATA%\YoluPainter\hide_presets\` | 면 숨기기 프리셋 | 유지됨 |
| `%APPDATA%\YoluPainter\pose_presets\` | 포즈 프리셋 | 유지됨 |
| `%APPDATA%\YoluPainter\actions\` | 액션 | 유지됨 |
| `%APPDATA%\YoluPainter\keymap.json` | 단축키 (키·마우스 조합·원형 메뉴) 할당. 읽지 못한 파일은 다음에 기록할 때 `keymap.broken.json`으로 옮겨 보관합니다. 같은 이름이 있으면 `keymap.broken-2.json`처럼 번호를 붙입니다 | 유지됨 |
| `%APPDATA%\YoluPainter\tools.json` | 도구 모음과 브러시 그룹의 순서 (읽지 못한 파일은 `tools.broken.json`으로 옮겨 보관) | 유지됨 |

유지할 항목이나 이 앱이 만들지 않은 파일이 있으면 `%APPDATA%\YoluPainter` 폴더 전체가 남습니다. 설정에서 라이브러리 위치나 복구 저장 위치를 다른 폴더로 지정했다면 그 폴더는 변경하지 않습니다. `.ylp` 등의 문서는 어느 경우에도 삭제하지 않습니다.

<a id="起動時の警告smartscreen"></a>

## 실행 시 경고 (SmartScreen)

현재 배포물에는 코드 서명이 없습니다. Windows SmartScreen이 “Windows의 PC 보호”를 표시하면 “추가 정보” → “실행”으로 시작할 수 있습니다.

<a id="プライバシー"></a>

## 개인정보 보호

업데이트 확인을 선택했을 때 (“시작할 때 업데이트 확인”에 “예”를 선택했거나 “업데이트 확인…”을 눌렀을 때) GitHub에 최신 버전을 문의하는 것 외에는 네트워크로 정보를 보내지 않습니다. 문의는 업데이트 정보 파일을 받는 일반 HTTPS 요청뿐입니다 (“베타 버전 사용”을 켜면 베타 버전의 업데이트 정보 파일도 받음). 앱 이름과 버전 (User-Agent) 외에 사용자를 식별하는 정보를 추가하지 않습니다. Live Link는 같은 PC의 폴더에서 파일만 주고받으며, 외부 조작은 설정에서 켜 둔 동안에만 `127.0.0.1` (이 PC 내부)에서 대기합니다.
