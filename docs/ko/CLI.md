[한국어](CLI.md) · [日本語](../CLI.md) · [English](../en/CLI.md)

<a id="コマンドラインyolupainter-cli"></a>

# 명령줄(yolupainter-cli)

`yolupainter-cli`는 YoluPainter의 `.ylp`를 화면 없이 또는 현재 실행 중인 앱을 대상으로 명령으로 조작하는 작은 콘솔 프로그램입니다.
스크립트나 AI에서 레이어·마스크·효과 읽기와 쓰기, 미리보기 이미지, 내보내기, 저장을 호출할 수 있습니다. AI 도우미는 실행 중인 앱에 MCP로 직접 연결합니다
([MCP 문서](MCP.md)). 표준 입출력 MCP만 지원하는 클라이언트를 위한 중계도 같은 프로그램(`yolupainter-cli mcp`)에 있습니다.

설치 프로그램으로 설치하면 앱과 같은 폴더(기본값 `%LOCALAPPDATA%\Programs\YoluPainter\yolupainter-cli.exe`)에 들어갑니다. zip과 tar.gz에서도 앱 옆에 있습니다. Mac 시험판 zip에서는 `YoluPainter.app` 내부의 `Contents/MacOS/yolupainter-cli`입니다([다운로드와 업데이트의 「Mac(시험판)」](INSTALL.md#mac試作)).
설치 프로그램은 PATH를 바꾸지 않으므로 명령 프롬프트나 PowerShell에서는 해당 폴더로 이동하거나 전체 경로로 호출합니다.
앱과 같은 버전이 설치되며 업데이트할 때도 함께 교체됩니다.

<a id="使い方"></a>

## 사용 방법

```
yolupainter-cli <命令> [--名前 値 ...] [--file project.ylp [--save]] [--pretty]
yolupainter-cli batch [ファイル|-] --file project.ylp [--save]
yolupainter-cli run-action <アクション.json> [--file project.ylp [--save]]
yolupainter-cli commands            命令の一覧
yolupainter-cli schema [命令]       命令の JSON Schema（--tools は MCP のツールの定義）
yolupainter-cli mcp [--port 番号]   標準入出力の MCP を起動中のアプリへ中継する
```

<a id="相手を選ぶ"></a>

### 대상 선택

- `--file project.ylp`: 앱을 사용하지 않고 해당 `.ylp`를 열어 명령 하나를 적용합니다. 저장하지 않으면 파일은 바뀌지 않습니다. `--save`를 붙이면 명령이 성공한 뒤 해당 `.ylp`에 덮어씁니다.
  (이전 버전은 옆의 `<ファイル名>-backups~` 폴더에 남습니다.)
- `--file`을 생략하면 현재 실행 중인 YoluPainter를 대상으로 합니다. 앱 설정의 「외부 명령 수신」을 켜 둡니다(기본값은 꺼짐).
  명령은 앱 안에서 실행되고 명령 하나가 화면의 실행 취소 한 단계가 됩니다. 앱이 수신하지 않으면 해결 방법을 안내하는 오류를 반환합니다(종료 코드 3).
  - 앱과는 이 PC 내부의 HTTP(`http://127.0.0.1:17347/mcp`)로만 연결합니다. 앱 설정에서 포트를 바꿨다면 `--port 番号`로 같은 번호를 지정합니다.
  - 응답은 최대 60초 기다립니다(`--timeout 秒`로 변경 가능). 시간 안에 응답하지 않으면 작업이 완료되었는지 알 수 없으므로 `doc.info`나 `history.info`로 확인한 뒤 다시 요청합니다.

<a id="引数の渡し方"></a>

### 인수 전달 방법

명령 이름은 `layer.set` 형식입니다(MCP 도구 이름인 `layer_set`도 허용). 인수는 `--名前 値`로 전달하며 값의 타입은 명령의 JSON Schema에 맞춰 읽습니다.
(문자열 필드에 `123`을 써도 문자열이며 숫자 필드는 숫자입니다.)

```
yolupainter-cli layer.set --file work.ylp --layer Base --opacity 0.5 --blend-mode Multiply --save
yolupainter-cli effect.add --file work.ylp --layer Base --kind blur --values.radius 4
yolupainter-cli layer.add --file work.ylp --kind fill --name Wash --fill.Color "#336699"
yolupainter-cli layer.delete --file work.ylp --layer Tint --confirm
yolupainter-cli export.channels --file work.ylp --dir out --channels Color,Normal
```

- 불리언 필드는 `--confirm`(참)·`--visible false`·`--no-confirm`으로 전달합니다. 배열 필드는 반복 지정(`--channels Color --channels Normal`), 쉼표 구분 또는 JSON 배열로 전달합니다.
- 중첩 필드는 `.`으로 접근하거나(`--values.radius 4`) JSON으로 전달합니다(`--values '{"radius":4}'`).
- 전체를 JSON으로 전달하려면 명령 이름 뒤에 `'{"layer":"Base","opacity":0.5}'`·`@args.json`·`-`(표준 입력) 중 하나를 둡니다. 뒤에 쓴 `--名前`가 덮어씁니다.
- `--file`의 상대 경로와 출력 대상(`--dir`·`--path`)의 상대 경로는 현재 폴더(`--cwd`로 변경 가능)를 기준으로 합니다. `..`로 작업 폴더 밖으로 나가는 상대 경로는 거부합니다.
  (그 위치에 저장하려면 절대 경로를 지정합니다.)

<a id="まとめて当てるbatch"></a>

### 일괄 적용(batch)

`.ylp`를 반복해서 열고 닫지 않고 한 번 실행하여 여러 명령을 적용합니다. 입력은 JSON 배열 또는 한 줄당 명령 하나입니다(빈 줄과 `#`로 시작하는 줄은 건너뜀).

```
yolupainter-cli batch commands.jsonl --file work.ylp --save
```

```json
{"command": "layer.add", "args": {"kind": "fill", "name": "Wash", "fill": {"Color": "#336699"}}}
{"command": "layer.set", "args": {"layer": "Wash", "opacity": 0.5}}
{"command": "preview", "args": {"max_edge": 512}}
```

중간에 실패하면 그 지점에서 중단하고 아무것도 저장하지 않습니다(오류의 `data.index`가 몇 번째인지, `data.completed`가 몇 개 완료했는지 알려 줌).
응답은 `{"replies": [...], "saved": {...}}`입니다(`saved`는 `--save`가 있을 때).
batch 안에서는 앞선 명령으로 만든 레이어·효과를 `$created:<n>`으로 지정할 수 있습니다([상대 참조](#相対の指し方)).

<a id="アクションを当てるrun-action"></a>

### 액션 적용(run-action)

앱의 「액션」 패널에서 기록한 액션은 설정 폴더의 `actions/<名前>.json`에 저장됩니다(Windows는
`%APPDATA%\YoluPainter\actions`). 이 파일을 화면 없는 `.ylp`에도, 실행 중인 앱에도(`--file`이 없을 때) 적용할 수 있습니다.

```
yolupainter-cli run-action "アクション 1.json" --file work.ylp --save
```

```json
{
  "format": 1,
  "name": "アクション 1",
  "commands": [
    {"command": "layer.add", "args": {"kind": "fill", "name": "Wash", "fill": {"Color": "#336699ff"}, "above": "$selected"}},
    {"command": "effect.add", "args": {"layer": "$created:1", "kind": "blur", "values": {"radius": 4.0}}}
  ]
}
```

- 명령 배열을 하나의 텍스처 세트에 **실행 취소 한 단계**로 적용합니다. 중간 명령이 거부되면 그때까지 적용한 내용도 되돌린 뒤 중단하고 아무것도 저장하지 않습니다.
  (오류의 `data.index`·`data.command`·`data.completed`는 batch와 같습니다.)
- 넣을 수 있는 명령은 레이어·마스크·효과를 바꾸는 명령뿐입니다(`layer.add`·`layer.delete`·`layer.move`·`layer.set`·`mask.add`·`mask.delete`·`mask.set`·
  `effect.add`·`effect.set`·`effect.delete`). 읽기·미리보기·내보내기·저장·실행 취소 명령이 있으면 적용 전에 거부합니다. `set`은 생략하거나 모두 같게 맞춥니다.
- 한도: 액션 하나당 명령 1,000개, 파일 4 MiB, 이름 100자까지입니다. `format`이 1이 아닌 파일은 읽지 않습니다.
- 내보내기·저장은 액션에 포함되지 않으며 앱의 기록에도 남지 않습니다(액션은 레이어·마스크·효과 편집만 지원).
- 응답은 `{"action": "<名前>", "reply": "action", "set": "<セットの ID>", "steps": [{"layer": "..."}, ...], "undo_count": 1, "can_undo": true, "saved": {...}}`입니다.
  `steps`는 각 명령이 만들거나 변경한 레이어(`layer`)와 효과(`effect`), 아무것도 바꾸지 않았는지(`unchanged`)를 담습니다.
- 실행 중인 앱에는 `action.run` 명령 한 번으로 보내며 앱에서 실행 취소 한 번으로 모두 되돌립니다. `$selected`는 앱의 현재 텍스처 세트에서 선택한 레이어입니다.
  `.ylp`에는 선택했던 레이어가 저장되지 않으므로 `$selected`를 사용하는 액션은 화면 없이 실행할 때 거부합니다.
- 같은 실행은 `action.run` 명령(`{"commands": [...]}`, MCP 도구 `action_run`)으로도 호출할 수 있습니다.

<a id="相対の指し方"></a>

### 상대 참조

기록한 액션을 다른 문서에서도 사용할 수 있도록 레이어·효과 필드(`layer`·`above`·`parent`·`effect`)에는 ID와 이름 외에 다음 참조를 쓸 수 있습니다.

| 표기 | 참조 대상 | 사용 가능한 곳 |
|---|---|---|
| `$selected` | 선택한 레이어(레이어 필드에서만) | 실행 중인 앱(현재 텍스처 세트에서 선택한 레이어). 액션과 batch에서는 시작 시점의 하나를 사용합니다. 화면 없는 `.ylp`에서는 거부합니다 |
| `$created:<n>` | 같은 실행 안에서 n번째(1부터)로 만든 레이어 또는 효과(`layer.add`·`effect.add` 순서의 통합 번호) | 액션과 batch 내부에서만 가능합니다. 단일 명령에서는 거부합니다 |

- 레이어 필드에 효과를, 효과 필드에 레이어를 지정하면 거부합니다. 번호가 범위를 벗어나도 거부합니다(만든 개수를 `data.created`로 반환).
- `$`로 시작하는 다른 문자열은 이름으로 검색합니다(`$created:`로 시작하지만 번호를 읽을 수 없으면 거부).

<a id="返事と終了コード"></a>

### 응답과 종료 코드

응답은 표준 출력의 JSON입니다(`--pretty`로 서식 지정). 미리보기(`preview`)의 PNG는 base64로 들어갑니다. `--out 見本.png`를 붙이면 PNG를 해당 파일에 쓰고 JSON에는 `png_file` 경로를 표시합니다.

실패하면 표준 출력에 `{"error": {"code": "...", "message": {"ja": "...", "en": "..."}, "data": {...}}}`, 표준 오류에 메시지 1줄을 출력합니다.
기존 메시지의 `ja`/`en` JSON 구조는 유지하며 `ko`는 영어 메시지를 사용합니다. 메시지 언어는 `--lang ja|en|ko`(환경 변수 `YOLUPAINTER_LANG`·`LANG`도 사용 가능)으로 선택하며 결정하지 못하면 일본어와 영어를 함께 표시합니다.

| 종료 코드 | 의미 |
|---:|---|
| 0 | 성공 |
| 1 | 명령이 거부됨(찾을 수 없음·값이 범위 밖·읽기 전용 세트·파일 실패 등, `error.code`로 구분) |
| 2 | 인수 오류(알 수 없는 명령·필드, 타입 불일치, JSON을 읽을 수 없음) |
| 3 | 실행 중인 앱에 연결할 수 없음(앱이 실행되지 않음·설정이 꺼짐·포트가 다름) |
| 4 | 파괴적 작업에 확인(`--confirm`)이 없음 |

오류 `code` 목록은 [명령 사양](https://github.com/YozoraKurage/YoluPainter/blob/main/crates/yolu-ops/README.md)에 있습니다.

<a id="命令の一覧"></a>

## 명령 목록

「읽기」는 아무것도 바꾸지 않습니다. 「편집」은 문서를 바꾸고 실행 취소 한 단계가 됩니다. 「파괴적」은 `--confirm`이 필요합니다. 「교체」는 기존 파일을 교체할 때만 `--confirm`이 필요합니다.
`set`은 텍스처 세트 ID 또는 이름이며 생략하면 현재 세트입니다. 레이어는 32자리 16진수 ID 또는 이름으로 지정합니다(같은 이름이 여러 개면 후보 ID를 반환하고 거부).
[상대 참조](#相対の指し方)(`$selected`·`$created:<n>`)도 사용할 수 있습니다.

| 명령 | 인수 | 종류 |
|---|---|---|
| `doc.info` | | 읽기 |
| `doc.open` | `path`·`confirm` | 교체(열린 문서의 저장하지 않은 변경 사항을 버릴 때) |
| `set.info` | `set` | 읽기 |
| `layer.get` | `layer` | 읽기 |
| `layer.add` | `kind`(paint·fill·group·adjustment·text)·`name`·`above`·`fill`·`adjustment`·`channels`·`text` | 편집 |
| `layer.delete` | `layer`·`confirm` | 파괴적 |
| `layer.move` | `layer`·`parent`·`to_root`·`index` | 편집 |
| `layer.set` | `layer`·`name`·`visible`·`opacity`·`blend_mode`·`clipping`·`locks`·`channels`·`fill`·`adjustment`·`points`(채우기의 포인트 그라데이션, 채널 → `space`·`spread`·`points`, `null`로 제거)·`text` | 편집 |
| `mask.add` | `layer` | 편집 |
| `mask.delete` | `layer`·`confirm` | 파괴적 |
| `mask.set` | `layer`·`enabled`·`inverted`·`density` | 편집 |
| `effect.get` | `layer`·`effect` | 읽기 |
| `effect.add` | `layer`·`target`·`kind`·`values`·`channels`·`strength`·`enabled`·`index` | 편집 |
| `effect.set` | `layer`·`effect`·`kind`·`values`·`channels`·`strength`·`enabled`·`index` | 편집 |
| `effect.delete` | `layer`·`effect`·`confirm` | 파괴적 |
| `effect.list_kinds` | | 읽기 |
| `history.info` | `set` | 읽기 |
| `undo`·`redo` | `set`·`steps` | 편집 |
| `preview` | `set`·`channel`·`max_edge` | 읽기 |
| `export.channels` | `set`·`channels`·`dir`·`name`·`confirm` | 교체 |
| `export.textures` | `set`·`template`·`dir`·`name`·`confirm` | 교체 |
| `export.psd` | `set`·`path`·`channel`·`mode`·`confirm` | 교체 |
| `save` | `confirm` | 파괴적(열린 `.ylp`를 덮어씀) |
| `save_as` | `path`·`confirm` | 교체 |
| `action.run` | `commands`([액션](#アクションを当てるrun-action)의 명령 배열) | 편집(전체가 한 단계이며 내부 파괴적 명령마다 `confirm` 필요) |

필드의 타입·범위·설명은 `yolupainter-cli schema <命令>`으로 출력할 수 있습니다. 효과 종류와 값 범위는 `effect.list_kinds`가 반환합니다.
그리기 작업(스트로크·채우기·선택)은 아직 명령으로 제공하지 않습니다.
레이어 세트(텍스처 세트 내부의 이름 있는 레이어 묶음)를 추가하거나 이름을 바꾸거나 출력 할당을 바꾸는 명령도 아직 없습니다. 레이어 세트가 여러 개 있는 문서도 열어 명령을 적용할 수 있지만 `set.info`는 모든 레이어 세트의 레이어를 함께 나열하며 어느 레이어 세트에 속하는지는 표시하지 않습니다. `layer.add`에서 `above`를 생략하면 첫 번째 레이어 세트에 들어갑니다. 레이어 세트를 가로지르는 순서 변경과 다른 레이어 세트의 그룹으로 넣거나 빼는 작업은 거부합니다. 미리보기와 내보내기는 채널별 출력 할당이 가리키는 최종 결과입니다.

<a id="安全のこと"></a>

## 안전에 관하여

- 파괴적 작업(삭제·덮어쓰기 저장·기존 파일 교체)은 `--confirm`이 없으면 아무것도 변경하지 않고 거부합니다. `--save`를 지정하면 덮어쓰기를 요청한 것으로 취급합니다.
- 저장은 검증한 임시 파일을 한 번 교체하여 확정합니다. 연 뒤에 외부에서 파일을 바꿨다면 덮어쓰지 않으며(`conflict`), 이전 버전은 옆의 백업 폴더에 남깁니다.
- 임의 코드를 실행하는 명령은 없습니다. 처리하는 파일은 `--file`의 `.ylp`와 내보내기·저장 명령에서 지정한 경로뿐입니다.
- 앱 명령은 이 PC 내부(`127.0.0.1`)에서만 받습니다. 비밀번호가 없으므로 설정이 켜져 있는 동안 이 PC의 다른 계정에서 실행한 프로그램도 연결할 수 있습니다([MCP 문서](MCP.md)의 「안전」).
- 화면 없이 실행할 때는 베이크한 메시 맵이나 모델을 읽는 Generator가 작동하지 않습니다(설정은 남지만 미리보기·내보내기·저장한 합성 PNG에는 반영되지 않으며, 응답의 `notes`·`inactive_effects`에 표시됨).
  독립 실행형 전용 기능(노이즈·그런지·그라데이션 맵 등)을 사용한 세트는 새로운 형식 버전으로 저장되며 0.4.x까지의 Unity 브리지에서는 열 수 없습니다([.ylp 형식](YLP_FORMAT.md)의 「읽기 구현별 지원 범위」). 저장 응답의 `notes`가 해당 세트를 알려 줍니다.
