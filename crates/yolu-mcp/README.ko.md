[한국어](README.ko.md) · [日本語](README.md)

# yolu-mcp

YoluPainter MCP 서버의 구현과 로컬 HTTP 수신부·클라이언트입니다. 실행 중인 앱 (yolu-app의 `mcp_server`)이 수신부를 실행하며,
명령줄 (yolu-cli)은 클라이언트와 중계에 사용합니다. 사용자용 문서는 [docs/MCP.md](../../docs/ko/MCP.md)입니다.

- `tools`: 도구 정의를 yolu-ops의 명령 목록에서 생성합니다 (이름은 명령의 `.`을 `_`로 변환. 제목은 영어와 일본어, 설명은 영어, inputSchema는 명령 인자,
  outputSchema는 응답 내용에서 미리보기 PNG 필드를 제외한 것, 주석은 읽기 전용·파괴적 작업·반복 실행 가능·외부 세계와 상호작용하지 않음 여부). 수동으로 작성한 표는 두지 않습니다.
- `docs`: `docs/`의 사용자용 문서 (개발 절차 제외)를 실행 파일에 내장하고 `yolupainter://docs/<名前>` 리소스로 반환합니다 (`.ja`·`.en`으로 언어 구분).
  문서를 추가하면 여기에 추가합니다 (추가 누락은 테스트에서 거부합니다).
- `server`: rmcp의 `ServerHandler`입니다. 명령은 `Backend` (`fn run(&self, Command) -> 非同期の Result<Reply, OpError>`)로 전달합니다. 응답은 structuredContent와 같은 JSON의 text,
  미리보기는 image와 resource_link (최근 8장·64 MiB를 보관하고 `resources/read`로 반환), 실패는 `isError`의 오류 JSON입니다. 리소스는 문서·명령 목록·효과 유형입니다.
  버전은 rmcp가 2025-11-25 이전의 `initialize`와 2026-07-28의 `server/discover`를 모두 받습니다.
- `http`: 127.0.0.1에서만 대기하는 수신부입니다 (hyper의 HTTP/1.1. tokio는 `Running` 스레드 안에서만 사용). rmcp로 넘기기 전에 `Host` (`127.0.0.1:<番号>`·`localhost:<番号>`)와
  `Origin` (있다면 같은 2개의 `http://`)을 확인하며 맞지 않으면 403으로 거부합니다. 경로는 `/mcp`뿐입니다. rmcp는 상태를 유지하지 않는 방식입니다 (`Mcp-Session-Id`를 발급하지 않음·응답은 JSON·GET과 DELETE는 405).
  동시 연결은 최대 8개 (초과 시 503), 본문은 최대 64 MiB (413)이며, 요청 헤더는 최대 10초까지 기다립니다. 중지하면 새 연결을 받지 않고 실행 중인 요청의 응답 작성을 마친 뒤
  (최대 0.5초) 수신부를 닫습니다 (중지 후 같은 포트를 즉시 다시 열 수 있습니다).
- `client`: 수신부 클라이언트 (`post`·동기식 `post_blocking`), Streamable HTTP 클라이언트가 붙이는 헤더 (`mcp_headers`), 응답 본문 (JSON·SSE)에서 메시지를 꺼내는 `messages`,
  `tools/call` 결과를 명령 응답으로 되돌리는 `reply_of`입니다.
- `reach`: 실행 중인 앱에 연결할 수 없거나 응답이 없거나 수신을 중단했음을 나타내는 오류입니다 (`data.live == "unreachable"`. 명령줄 종료 코드 3).

기본 포트는 17347입니다 (IANA 등록에 할당되지 않았으며 Linux와 Windows의 임시 포트 범위 밖).

<a id="テスト"></a>

## 테스트

```sh
cargo test -p yolu-mcp
cargo clippy -p yolu-mcp --all-targets -- -D warnings
```

- `tests/mcp.rs`: 실행 중인 앱을 대신하는 화면 없는 호스트에 실제 수신부를 띄우고, 실제 클라이언트로 2개 버전의 흐름·도구 목록과 schema·호출·오류 형식·미리보기 이미지·리소스·
  structuredContent와 outputSchema의 일치를 확인합니다.
- `tests/http.rs`: Host·Origin 거부·경로·127.0.0.1 전용·동시 연결 상한·본문 상한·헤더 시간 초과·중지 시 연결 종료를 확인합니다.
- `tests/plugin.rs`: 배포 플러그인 (`plugin/`과 루트의 `.claude-plugin/marketplace.json`)이 이 수신부와 맞는지 확인합니다 (목록 버전이 앱과 같은지·MCP 설정이 기본 포트를 가리키는지·
  Claude Code와 Codex 목록이 같은 스킬과 설정을 가리키는지·스킬에 적힌 도구 이름·오류 유형·인자·리소스가 실제로 존재하는지).
