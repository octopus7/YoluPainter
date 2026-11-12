[한국어](MCP.md) · [日本語](../MCP.md) · [English](../en/MCP.md)

<a id="ai-のアシスタントから操作するmcp"></a>

# AI 도우미에서 조작하기(MCP)

YoluPainter는 설정에서 「외부 명령 수신」을 켜 둔 동안 MCP 서버로서 `http://127.0.0.1:17347/mcp`에서 대기합니다. AI 도우미(Claude Code·Codex·
Claude Desktop 등)는 여기에 연결하여 앱에서 현재 열린 문서의 레이어·마스크·효과를 읽고, 값을 수정하고, 미리보기 이미지를 확인하고, 내보내기·저장을 할 수 있습니다.
도구는 앱이 제공하므로 앱을 업데이트하면 새 도구를 사용할 수 있습니다. 이 PC 내부에서만 통신하며 외부 네트워크로 나가지 않습니다.
사용 가능한 도구는 [명령줄](CLI.md)의 명령과 같은 26개이며 이름의 `.`은 `_`가 됩니다. 그리기 작업(스트로크·채우기·선택)·베이킹·레이어 세트 작업·다른 문서 열기는 지원하지 않습니다.

<a id="yolupainter-の設定"></a>

## YoluPainter 설정

1. 앱의 「편집 → 설정…」에서 「Live Link 및 외부 명령」의 「외부 명령 수신」을 켭니다(기본값은 꺼짐). 켜져 있을 때만 대기하며 상태 표시줄 오른쪽 끝에 작은 원이 나타납니다. 원의 툴팁에 연결할 URL이 표시됩니다.
2. 켜져 있을 때 아래에 표시되는 「포트 번호」에서 번호를 바꿀 수 있습니다(기본값 17347, 1024~65535). 바꾸면 연결하는 쪽 설정의 번호도 같게 맞춥니다.
3. 대기할 수 없으면(다른 프로그램이 해당 번호를 사용하는 경우 등) 원이 「수신 불가」 색상으로 바뀌고 툴팁에 이유가 나옵니다. 번호를 바꾸거나 해당 프로그램을 닫은 뒤 설정을 껐다 켭니다.

수신한 명령은 앱의 실행 취소 한 단계가 되며 화면에 그대로 반영됩니다. 그리는 중·저장 중·읽기 전용 세트 등에서는 이유와 함께 거부합니다(`busy`·`read_only`).
끄면 수신을 중단합니다. 응답을 기다리던 요청에는 「수신을 중단했습니다」 오류를 반환합니다(이미 시작한 저장은 끝까지 계속됨).

<a id="つなぎ方"></a>

## 연결 방법

<a id="claude-codeプラグイン"></a>

### Claude Code(플러그인)

YoluPainter 플러그인에는 앱에 연결하는 MCP 설정과 도구 사용 방법(스킬)이 포함되어 있습니다.

```
/plugin marketplace add YozoraKurage/YoluPainter
/plugin install yolupainter@yolupainter
```

새 플러그인 버전을 자동으로 받으려면 `/plugin`의 Marketplaces에서 `yolupainter`를 선택하고 자동 업데이트를 켭니다(다른 사람이 만든 마켓플레이스는 기본적으로 자동 업데이트가 꺼져 있음).
셸에서는 `claude plugin marketplace add YozoraKurage/YoluPainter`·`claude plugin install yolupainter@yolupainter`를 사용합니다.

플러그인 없이 MCP 서버만 추가할 수도 있습니다.

```
claude mcp add --transport http yolupainter http://127.0.0.1:17347/mcp
```

플러그인 설정은 기본 포트를 가리킵니다. 앱에서 번호를 바꿨다면 위의 `claude mcp add`로 해당 번호의 서버를 추가하고 `/mcp`에서 플러그인의 `yolupainter`를 끕니다.

<a id="codexプラグイン"></a>

### Codex(플러그인)

```
codex plugin marketplace add YozoraKurage/YoluPainter
codex plugin add yolupainter@yolupainter
```

Codex는 시작할 때마다 추가한 마켓플레이스를 새로고침하고 그곳에서 설치한 플러그인도 다시 설치합니다(수동 업데이트는 `codex plugin marketplace upgrade`).

플러그인을 사용하지 않으려면 `~/.codex/config.toml`에 다음을 추가합니다(`codex mcp add yolupainter --url http://127.0.0.1:17347/mcp`도 같음).

```toml
[mcp_servers.yolupainter]
url = "http://127.0.0.1:17347/mcp"
```

<a id="claude-desktop拡張"></a>

### Claude Desktop(확장)

Claude Desktop 채팅은 표준 입출력 확장(`.mcpb`)으로 연결합니다. [Releases](https://github.com/YozoraKurage/YoluPainter/releases)의
`yolupainter-<バージョン>-x86_64-pc-windows-msvc.mcpb`를 다운로드하고 더블 클릭하거나 Claude Desktop 설정의 확장 기능 화면으로 드래그한 뒤 설치를 확인합니다.
확장에는 표준 입출력을 앱의 엔드포인트로 연결하는 중계(`yolupainter-cli mcp`)만 있으며 도구 정보는 앱이 응답합니다. 앱에서 번호를 바꿨다면 확장 설정의 「Port」도 같게 맞춥니다.
확장은 Claude Desktop이 시작할 때 앱에 요청합니다. 그때 앱이 수신 중이 아니면 연결에 실패하므로 앱을 실행하고 설정을 켠 뒤 확장을 껐다 켭니다.

<a id="そのほかのクライアント"></a>

### 그 밖의 클라이언트

Streamable HTTP MCP를 지원하는 클라이언트는 `http://127.0.0.1:17347/mcp`를 지정하면 연결할 수 있습니다. 표준 입출력만 지원하는 클라이언트는
`yolupainter-cli mcp`(설치 프로그램으로 설치한 경우 `%LOCALAPPDATA%\Programs\YoluPainter\yolupainter-cli.exe`, Mac 시험판 zip에서는 `YoluPainter.app/Contents/MacOS/yolupainter-cli`, 포트를 바꿨다면 `--port 番号`)를 실행하도록 설정합니다.

<a id="ツールと資料"></a>

## 도구와 자료

도구 목록과 각 명령의 인수·종류는 [명령줄 문서](CLI.md#命令の一覧)에 있습니다. 각 도구에는 이름·제목·설명·입출력 JSON Schema와
읽기 전용인지·파괴적인지·같은 인수로 반복해도 되는지에 관한 주석이 붙습니다. 응답은 structuredContent(출력 JSON Schema와 일치)와 같은 내용의 text입니다.
실패 시 `isError` 응답으로 `code`·일본어와 영어 `message`·`data`를 가진 JSON을 반환합니다.

- 미리보기(`preview`)는 PNG를 image로 반환하고 같은 PNG의 resource_link(`yolupainter://preview/<番号>.png`, 최근 8장까지 보관)도 제공합니다.
- 자료(resources): `yolupainter://docs/<名前>`는 설치된 버전의 문서입니다(사용 설명서 시작 페이지 `guide`와 하위 페이지 `guide-start`·`guide-paint`·`guide-select`·`guide-layers`·`guide-fill`·`guide-paths`·`guide-3d`·`guide-files`·`guide-settings`·`guide-keys`, `cli`·`mcp`·`install`·`psd`·`brush`·.ylp 형식 사양 `ylp-format` 등. 영어판이 있는 문서는 영어가 기본이며 `<名前>.ja`·`<名前>.en`·`<名前>.ko`로 언어를 선택할 수 있음. 예를 들어 `yolupainter://docs/guide.ko`는 한국어 사용 설명서).
  `yolupainter://ops/commands`는 명령 목록과 JSON Schema, `yolupainter://ops/effect-kinds`는 효과 종류와 값 범위입니다(`effect_list_kinds`와 같음).
- `doc_open`은 앱에서 열린 파일을 지정한 경우에만 그 문서를 반환합니다. 다른 파일은 `unsupported`로 거부합니다(앱은 문서를 전환하지 않음).
- 레이어 필드(`layer`·`above`·`parent`)에 `$selected`를 쓰면 앱의 현재 텍스처 세트에서 선택한 레이어를 가리킵니다. `$created:<n>`은 일괄 실행
  (명령줄 batch·액션) 안에서만 사용할 수 있으며 도구를 하나만 호출할 때는 거부합니다([상대 참조](CLI.md#相対の指し方)).
- `action_run`은 레이어·마스크·효과를 바꾸는 명령 배열(`{"commands": [{"command": "layer.add", "args": {...}}, ...]}`)을 앱의 실행 취소 한 단계로 적용합니다.
  중간 명령이 거부되면 모두 되돌리고 몇 번째인지(`data.index`, 0부터)를 반환합니다. 배열 안에서는 `$created:<n>`도 사용할 수 있습니다.
- 프로토콜 버전 2025-11-25와 2026-07-28에 모두 응답합니다(`initialize`가 있는 흐름과 `server/discover` 및 요청별 `_meta`를 사용하는 흐름). 엔드포인트는 상태를 유지하지 않으며
  `Mcp-Session-Id`를 사용하지 않습니다.

<a id="安全"></a>

## 안전

- 파괴적 작업(레이어·마스크·효과 삭제, 덮어쓰기 저장, 기존 파일 교체)은 인수 `confirm: true`가 없으면 아무것도 변경하지 않고 거부하며
  도구에 「파괴적」 주석이 붙습니다(`action_run` 안의 파괴적 명령은 각각 `confirm: true`가 필요하며 도구에도 「파괴적」 주석이 붙음). AI는 사용자에게 확인한 뒤 `confirm: true`를 지정합니다. 저장 시 이전 버전은 옆의 `<ファイル名>-backups~`에 남깁니다.
- 임의 코드를 실행하는 도구는 없습니다. 처리하는 파일은 내보내기·저장 도구에서 지정한 경로뿐입니다. 상대 경로는 열린 프로젝트의 폴더를 기준으로 하며 `..`로 밖으로 나갈 수 없습니다.
- `127.0.0.1`(이 PC 내부)에서만 대기합니다. 요청의 `Host`가 `127.0.0.1:<番号>`·`localhost:<番号>`가 아니거나 `Origin`이 있으면서 같은 위치가 아닌 요청
  (다른 이름으로 127.0.0.1을 가리키는 웹 페이지 또는 다른 사이트 페이지에서 보낸 요청)은 거부합니다. 동시 연결은 최대 8개입니다.
- 비밀번호는 없습니다. 설정이 켜져 있는 동안 이 PC의 다른 계정에서 실행한 프로그램도 연결할 수 있습니다. 사용하지 않을 때는 꺼 둡니다.
- 도구 주석을 신뢰할지는 MCP 클라이언트가 결정합니다. 도우미 설정에서 파괴적 도구는 호출할 때마다 확인하도록 설정하면 안전합니다.

<a id="つながらないとき"></a>

## 연결되지 않을 때

- 「실행 중인 YoluPainter에 연결할 수 없습니다」: 앱이 실행되지 않았거나 「외부 명령 수신」이 꺼져 있거나 포트가 다릅니다. 앱 상태 표시줄의 원 툴팁에서 수신 중인 URL을 확인합니다.
- 원이 「수신 불가」 색상인 경우: 툴팁의 이유를 확인합니다. 「다른 프로그램이 사용 중입니다」라면 앱의 포트를 바꾸고 연결하는 쪽의 포트도 같게 맞춥니다.
- 「연결 수가 한도에 도달했습니다」(`busy`): 동시 연결 8개가 다른 클라이언트 등으로 가득 찼습니다. 잠시 기다렸다가 다시 요청합니다.
- 「응답이 없습니다」: 앱에서 그리는 중인 경우 등 명령을 처리하지 못한 상태입니다. 작업이 완료되었는지는 알 수 없으므로 `doc_info`나 `history_info`로 확인한 뒤 다시 요청합니다.
