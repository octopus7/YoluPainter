[한국어](README.ko.md) · [日本語](README.md)

# yolu-cli

`yolupainter-cli`는 명령([yolu-ops](../yolu-ops/README.ko.md))을 명령줄에서 `.ylp`(화면 없음) 또는 실행 중인 앱에 적용합니다. `yolupainter-cli mcp`는
표준 입출력 MCP 클라이언트를 실행 중인 앱의 MCP 엔드포인트([yolu-mcp](../yolu-mcp/README.ko.md))에 연결하는 중계입니다.
사용자용 문서는 [docs/CLI.md](../../docs/ko/CLI.md)와 [docs/MCP.md](../../docs/ko/MCP.md)입니다. 이 문서는 내부 구조를 설명합니다.

- 실행 파일은 `yolupainter-cli`입니다(Windows에서는 콘솔 프로그램 `yolupainter-cli.exe`). 앱과 같은 배포물에 같은 버전으로 제공하며 Claude Desktop용 `.mcpb`에도 포함됩니다(`cargo xtask mcpb`).
- `args`: 인수를 읽습니다. 명령의 JSON Schema(yolu-ops의 타입에서 생성)에 맞춰 `--名前 値`의 타입을 정합니다(문자열 필드의 `123`은 문자열, 숫자 필드는 숫자, 불리언은 `--flag`, `.`으로 중첩).
  `--file`·`--live`·`--save`·`--pretty`·`--lang`·`--timeout`·`--out`·`--port`·`--cwd`는 CLI 예약 이름이며, 테스트가 명령의 필드 이름과 겹치지 않는지 확인합니다.
- `cli`: 실행과 출력을 담당합니다. 성공하면 표준 출력에 JSON을 쓰고, 실패하면 표준 출력에 `{"error": ...}`, 표준 오류에 1줄을 씁니다. 종료 코드는 0 성공·1 명령이 거부함·2 인수 오류·3 앱에 연결할 수 없음·4 확인 필요입니다.
  `--file`은 `FileHost`로 열어 명령을 적용합니다(`--save`가 있고 성공했을 때만 저장). `batch`는 하나의 `FileHost`에 순서대로 적용하며 중간에 실패하면 저장하지 않습니다.
- `live`: 실행 중인 앱으로 연결합니다. 앱의 엔드포인트(`http://127.0.0.1:<番号>/mcp`, 번호는 `--port`, 기본값은 17347)에 MCP `tools/call`을 한 번 보내고 결과를 명령 응답으로 되돌립니다.
  (`yolu_mcp::client::reply_of`. 미리보기는 이미지 content를 PNG로 되돌립니다.) 엔드포인트는 상태를 유지하지 않으므로 `initialize`를 보내지 않습니다. 호출할 때마다 다시 연결합니다.
  연결할 수 없는 이유는 `data.live == "unreachable"` 오류로 만듭니다(종료 코드 3).
- `relay`(`mcp`): 한 줄당 메시지 하나인 JSON-RPC를 읽고 그대로 `POST /mcp`로 보내며 응답을 한 줄씩 씁니다. 도구 목록·내용·자료·버전 협상은 앱이 응답하므로
  중계에는 보관하지 않습니다(앱만 업데이트하면 최신 내용이 반영됨). `initialize`에서 결정한 버전을 이후 요청 헤더(`MCP-Protocol-Version`)에 넣으며, 2026-07-28 흐름에는 `Mcp-Method`·`Mcp-Name`도 넣습니다.
  요청은 최대 4개까지 병렬로 보냅니다. 앱에 연결할 수 없으면 `tools/call`에는 도구 실패(앱 오류와 같은 형식)를, 다른 요청에는 JSON-RPC 오류를 해결 방법과 함께 반환합니다.
- 표준 입력은 `lock()`을 계속 유지하지 않습니다(`LazyStdin`). tokio의 표준 입력 읽기가 같은 잠금을 사용하므로 계속 유지하면 중계의 읽기가 멈춥니다.

<a id="テスト"></a>

## 테스트

```sh
cargo test -p yolu-cli
cargo clippy -p yolu-cli --all-targets -- -D warnings
```

- `tests/cli.rs`: 각 명령을 `.ylp`에 적용하기·저장·확인·일괄 적용·미리보기 이미지·`schema`와 타입의 일치·종료 코드와 일본어/영어 메시지·실제 실행 파일을 검사합니다.
- `tests/live.rs`: 실행 중인 앱 대신 실제 MCP 엔드포인트(`yolu_mcp::http`)를 열고 실제 문서에 명령을 적용하는 서버(`tests/common`의 `FakeApp`)를 대상으로 왕복 통신·
  거부·미리보기 이미지·HTTP 단계에서 잘못 동작하는 상대(연결 닫힘·MCP가 아님·한도·응답 없음)를 검사합니다.
- `tests/mcp.rs`: 실제 `yolupainter-cli mcp`와 표준 입출력으로 통신하여 2025-11-25의 `initialize` 흐름과 2026-07-28의 `server/discover` 흐름이 앱까지 전달되고 돌아오는지,
  앱에 연결할 수 없을 때의 응답, 나중에 앱이 수신을 시작하면 연결되는지, 입력이 닫히면 종료되는지 검사합니다.
