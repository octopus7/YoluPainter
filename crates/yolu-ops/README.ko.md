[한국어](README.ko.md) · [日本語](README.md)

# yolu-ops

외부에서 문서를 조작하는 명령의 형식과 화면 없이 `.ylp`를 조작하는 호스트입니다.

- 명령(`Command`)·응답(`Reply`)·오류(`OpError`)는 serde JSON이며 명령 버전을 갖고 형식에서 JSON Schema를 생성할 수 있습니다.
- 문서 명령은 `yolu_core::Document` 위의 함수(`doc_ops`) 한 곳에 있으며 **명령 하나가 실행 취소 한 단계**입니다.
- 프로젝트 명령(세트 목록·문서 읽기/쓰기·내보내기·저장·미리 보기)은 `OpHost`를 대상으로 작성되어 있습니다. 호스트는 화면 없는
  `FileHost`(이 crate)와 실행 중인 앱 내부의 호스트입니다.
- 파괴적 작업(삭제·덮어쓰기 저장·교체)은 `confirm: true`가 없으면 거부합니다. 임의 코드를 실행하는 명령은 없습니다.

```rust,no_run
use yolu_ops::{execute, parse_command_str, FileHost, OpHost, PathPolicy};

fn main() -> Result<(), yolu_ops::OpError> {
    let mut host = FileHost::new(PathPolicy::current_dir().expect("今のフォルダ"));
    host.open("work/character.ylp".as_ref(), true)?;
    let command = parse_command_str(
        r#"{"command": "layer.set", "args": {"layer": "Base", "opacity": 0.5}}"#,
    )?;
    let reply = execute(&mut host, &command)?; // 取り消しの 1 段になる
    println!("{}", serde_json::to_string(&reply).unwrap());
    execute(&mut host, &parse_command_str(r#"{"command": "save", "args": {"confirm": true}}"#)?)?;
    Ok(())
}
```

<a id="命令の-json"></a>

## 명령 JSON

명령은 `{"v": 1, "command": "<名前>", "args": {...}}`입니다. `v`(명령 버전)와 `args`는 생략할 수 있습니다(현재 버전·빈 값). 알 수 없는 인수 필드는 거부합니다.
응답은 `{"reply": "<種類>", ...}`, 오류는 `{"code": "<種類>", "message": {"ja": "...", "en": "..."}, "data": {...}}`입니다.

대상 지정 방법:

- **세트**(`set`): ID 또는 이름. 생략하면 현재 세트입니다.
- **레이어**(`layer`, `above`, `parent`): 32자리 16진수 ID 또는 이름(같은 이름이 여러 개면 `ambiguous`이며 후보 ID를 반환).
- **채널**: `Color`·`Roughness`·`Metallic`·`Height`·`Normal`·`Emission` 또는 사용자 채널 이름(대소문자 구분 없음). 번호로도 지정할 수 있습니다.
- **효과**: 효과 ID(`effect.add` 응답과 `effect.get`에 표시).
- **색상**: `#rrggbb` 또는 `#rrggbbaa`.
- **상대 참조**(`refs`): 레이어 필드의 `$selected`는 호스트가 선택한 레이어입니다(`OpHost::selected_layer`. 화면 없는 `FileHost`는 .ylp에 선택 레이어가
  없으므로 거부). `$created:<n>`은 같은 실행 안에서 n번째로 만든 레이어 또는 효과(`layer.add`·`effect.add`의 통합 순번)이며 일괄 적용 실행(`execute_in`에
  `Created` 전달·`action::run`·`action.run`) 안에서만 사용할 수 있습니다. 단일 명령(`execute`)에서는 거부합니다. 실행 중인 앱에 하나씩 보내는 호출자는 보내기 전에 `substitute_created`로 치환합니다.

<a id="命令の一覧"></a>

## 명령 목록

“읽기”는 아무것도 바꾸지 않습니다. “편집”은 문서를 바꾸며 실행 취소 한 단계가 됩니다. “파괴적”은 `confirm: true`가 필요합니다. “교체”는 교체할 때만
`confirm: true`가 필요합니다(새 파일에는 필요 없음).

| 명령 | 인수(`set`은 공통이며 생략 가능) | 응답 | 종류 |
|---|---|---|---|
| `doc.info` | 없음 | `doc` | 읽기 |
| `doc.open` | `path`, `confirm` | `doc` | 교체(열려 있던 문서의 저장하지 않은 변경을 버릴 때) |
| `set.info` | `set` | `set` | 읽기 |
| `layer.get` | `layer` | `layer` | 읽기 |
| `layer.add` | `kind`(paint·fill·group·adjustment·text), `name`, `above`, `fill`, `adjustment`, `channels`, `text` | `edited` | 편집 |
| `layer.delete` | `layer`, `confirm` | `edited` | 파괴적 |
| `layer.move` | `layer`, `parent`, `to_root`, `index` | `edited` | 편집 |
| `layer.set` | `layer`, `name`, `visible`, `opacity`, `blend_mode`, `clipping`, `locks`, `channels`, `fill`, `adjustment`, `points`, `text` | `edited` | 편집(전체가 한 단계) |
| `mask.add` | `layer` | `edited` | 편집 |
| `mask.delete` | `layer`, `confirm` | `edited` | 파괴적 |
| `mask.set` | `layer`, `enabled`, `inverted`, `density` | `edited` | 편집 |
| `effect.get` | `layer`, `effect` | `effects` | 읽기 |
| `effect.add` | `layer`, `target`(content·mask), `kind`, `values`, `channels`, `strength`, `enabled`, `index` | `edited` | 편집 |
| `effect.set` | `layer`, `effect`, `kind`, `values`, `channels`, `strength`, `enabled`, `index` | `edited` | 편집(전체가 한 단계) |
| `effect.delete` | `layer`, `effect`, `confirm` | `edited` | 파괴적 |
| `effect.list_kinds` | 없음 | `kinds` | 읽기 |
| `history.info` | `set` | `history` | 읽기 |
| `undo` / `redo` | `set`, `steps`(1~100) | `undone` | 편집 실행 취소·다시 실행 |
| `preview` | `set`, `channel`, `max_edge`(1~2048) | `preview` | 읽기 |
| `export.channels` | `set`, `channels`, `dir`, `name`, `confirm` | `exported` | 교체 |
| `export.textures` | `set`, `template`, `dir`, `name`, `confirm` | `exported` | 교체 |
| `export.psd` | `set`, `path`, `channel`, `mode`(bake·flat), `confirm` | `exported` | 교체 |
| `save` | `confirm` | `saved` | 파괴적(열린 `.ylp`를 덮어씀) |
| `save_as` | `path`, `confirm` | `saved` | 교체 |
| `action.run` | `commands`(명령 배열. [액션](#アクション)) | `action` | 편집(전체가 한 단계. 내부의 파괴적 명령은 각각 `confirm` 필요) |

도구 이름은 명령 이름의 `.`을 `_`로 바꾼 것입니다(`layer.set` → `layer_set`. `CommandSpec::tool_name`).
필드 형식·범위·설명은 `yolu_ops::commands()`(명령 목록. 이름·일본어/영어 설명·읽기 전용인지 파괴적인지·인수와 응답의 JSON Schema)와 `command_schema()`·
`reply_schema()`·`error_schema()`로 얻을 수 있습니다. 파괴적 작업 표시는 `Danger`(`Always`·`WhenReplacing`·`PerCommand`)이며, 확인 필드(`confirm`)가 있는 명령
(`Always`·`WhenReplacing`)과 없는 명령이 이 표시와 일치하는지 테스트로 확인합니다. `PerCommand`는 `action.run`으로, 배열 안의 파괴적 명령이 각각 `confirm`을 가집니다.

<a id="アクション"></a>

## 액션

`action`은 기록한 명령 배열 파일(`{"format": 1, "name": "...", "commands": [...]}`. 명령은 위의 JSON)과 해당 배열을 텍스처 세트 하나에
**실행 취소 한 단계**로 적용하는 `action::run`입니다. 명령 `action.run`(`{"commands": [...]}`. MCP 도구 `action_run`)도 같은 실행 방식이며 실행 중인 앱과 화면 없는
호스트 양쪽에 동일하게 적용됩니다. 응답은 `action`(`set`·명령별 `steps`[만들거나 바꾼 `layer`·`effect`, `unchanged`]·`unchanged`·`undo_count`·`can_undo`)입니다.

- 넣을 수 있는 것은 레이어·마스크·효과를 바꾸는 명령뿐입니다(`ACTION_COMMANDS`·`check_allowed`). 읽기·미리 보기·내보내기·저장·실행 취소·`action.run`은 적용 전에 거부합니다. 파괴적 명령의 `confirm`도 먼저 확인합니다.
- 전체를 `Document::batch` 한 번으로 적용합니다(`doc_ops`의 편집은 외부가 배치 안이면 해당 배치에 쌓음). 도중 명령이 거부되면 그때까지 적용한 내용도 되돌리고 오류의
  `data`에 `index`(0부터)·`command`·`completed`를 첨부합니다.
- 상한: 명령 1,000개(`MAX_COMMANDS`)·파일 4 MiB(`MAX_FILE_BYTES`)·이름 100문자(`MAX_NAME_CHARS`). `format`이 1이 아닌 파일은 `unsupported_version`입니다.

<a id="取り消し"></a>

## 실행 취소

편집 명령은 `Document::batch` 한 번으로 실행하며, 거부되면 쌓은 단계를 되돌려 문서를 원래 상태로 유지합니다. 실행 취소는 세션 안에서만 가능합니다(파일에 남지 않음).
같은 값을 전달한 편집은 아무것도 바꾸지 않고(`edited.unchanged`) 실행 취소 단계를 늘리지 않습니다.

<a id="効果の種類と値"></a>

## 효과 종류와 값

효과는 `kind`(종류 이름)와 `values`(필드 이름 → 수·참/거짓·선택지 문자열)로 추가하거나 변경합니다. 종류·필드 형식·범위·기본값은 `effect.list_kinds`가 반환하며,
범위 검사는 문서(core)와 같은 정의를 사용합니다(범위 밖은 잘라 맞추지 않고 거부). 전달하지 않은 필드는 추가할 때는 기본값, 변경할 때는 현재 값입니다.

- 필터: `blur`·`sharpen`·`noise`·`levels`·`invert`·`normalize`·`color_balance`·`brightness_contrast`·`threshold`·`posterize`·
  `histogram_scan`・`histogram_range`・`slope_blur`・`directional_blur`・`warp`・`morphology`・`edge_detect`・`high_pass`・`median`・`glow`
  (조정 레이어에는 `layer.add`·`layer.set`의 `adjustment`로 `levels`·`invert`·`hue_saturation` 등을 지정).
- 생성기: `edge_wear`·`dirt`·`position_gradient`·`thickness`·`direction`·`procedural_noise`·`grunge`·`pattern`·`light`·`mask_builder`·`uv_island_variation`.
- 목록·곡선·참조를 갖는 종류(`gradient_map`·`tone_curve`·`shape_gradient`·`id_color`·`anchor`·`image`)는 값만으로 추가할 수 없습니다(`addable: false`).
  기존 단계는 `effect.get`으로 읽을 수 있고 강도·활성화·채널은 바꿀 수 있으나 값을 전달하여 변경하는 것은 거부합니다(해당 부분을 알리지 않고 다시 만들지 않음).
- 독립 실행형 전용 종류(`rust_only: true`: 노이즈·그런지·이미지(`image`), 그라데이션 맵·톤 곡선·색상 균형·밝기/대비·이진화·포스터화,
  0.5.0 필터와 생성기[위 목록의 `histogram_scan`부터 `glow`까지 및 `pattern`·`light`·`mask_builder`·`uv_island_variation`])를 사용한 세트는 새 문서 버전으로 저장되며
  0.4.x까지의 Unity 브리지에서는 열 수 없습니다(이유와 함께 거부하며 내용은 사라지지 않음. 0.5.0 이후 Unity 브리지는 .ylp를 열지 않음). 저장 응답의 `notes`가 해당 세트를 알립니다.
- 효과 종류를 바꿀 때 적용 중인 채널에서 사용할 수 없는 설정은 어느 채널인지 밝히고 거부합니다(`channels`로 다시 선택).

**화면 없이 동작하지 않는 효과**: 베이크한 메시 맵·모델을 읽는 생성기(`needs_baked_maps: true`. 모델 UV 아일랜드를 읽는 `uv_island_variation` 포함)는 화면 없는 호스트에 맵과 모델이 없으므로 입력을 그대로 통과시킵니다.
설정은 문서에 남지만 미리 보기·내보내기·저장한 합성 PNG에는 반영되지 않습니다(응답의 `notes`·`inactive_effects`에 표시). 절차적 노이즈·그런지는 위치 맵이
없으면 UV로 평가하므로 적용됩니다. 채우기 이미지에는 프로젝트 이미지를 전달합니다.

<a id="見本の画像"></a>

## 미리 보기 이미지

`preview`는 세트·채널·최대 변 길이를 받아 PNG(위 행부터)를 반환합니다. 픽셀은 문서 기준 데이터의 합성(내보내기 채널 이미지와 같음)이며 축소할 때는 박스 평균을 사용합니다
(불투명도로 가중한 색 평균이므로 투명 가장자리가 검게 번지지 않음). 확대하지 않으며 축소하지 않는 크기에서는 합성 픽셀 그대로입니다.

<a id="書き出し"></a>

## 내보내기

검증한 임시 파일에서 한 장씩 교체하는 방식으로 쓰며 도중 실패해도 이미 있는 파일은 바뀌지 않습니다. 기존 파일은 `confirm: true`가 없으면 교체하지 않습니다
(쓰기 전에 교체할 파일 목록과 함께 거부).

- `export.channels`: 채널별 PNG. 이름은 `<名前>_<チャンネル>.png`(세트가 여러 개면 `<名前>_<セット>_<チャンネル>.png`). 기본 이름은 `.ylp` 이름입니다.
- `export.textures`: 템플릿(`unity-standard`·`unity-hdrp`·`liltoon`)의 이미지 중 읽을 대상이 있는 이미지만 내보냅니다. 색 확장(UV 바깥)과 베이크한 AO는 모델에서 만드는 것이며
  화면 없는 환경에서는 전달할 수 없어 사용하지 않습니다.
- `export.psd`: 채널 하나를 PSD로 내보냅니다(`bake`는 레이어를 유지하며 PSD에 대응 구조가 없는 것을 굽고 구운 것·반올림한 것·제외한 것을 `notes`에 표시. `flat`은 합성 결과 한 장).
  쓴 뒤 다시 읽어 확인합니다.

<a id="保存"></a>

## 저장

`FileHost` 저장은 yolu-io의 안전한 저장입니다. 검증한 임시 파일에서 한 번의 교체로 확정하며, 연 뒤 외부에서 수정된 파일은 덮어쓰지 않고(`conflict`)
이전 버전은 옆의 `<名前>-backups~/`에 남깁니다(지우지 않음).

- 편집한 세트만 문서에서 다시 만들며(합성 PNG 포함), 편집하지 않은 세트·읽기 전용 세트·선택 영역·외관 등 다른 항목은 연 파일의 바이트 배열 그대로 복사합니다.
- 아무것도 편집하지 않았으면 쓰지 않습니다(`saved.written: false`).
- 이전 형식의 파일은 형식 7로 올려 씁니다(`saved.upgraded_from`).
- `save_as`는 새 파일에는 확인이 필요 없으며, 기존 파일은 유효한 `.ylp`에 한해 `confirm: true`로 교체합니다(이전 버전은 백업). 이름은 `.ylp`로 끝나야 합니다.
- core에서 처리할 수 없는 내용이 있는 세트는 **읽기 전용**입니다(`read_only`. `set.info`는 크기와 이유만 반환). 저장해도 해당 세트의 바이트 배열은 바뀌지 않습니다.

<a id="ファイルの道"></a>

## 파일 경로

절대 경로는 그대로 사용합니다. 상대 경로는 작업 폴더(`PathPolicy`. 기본은 현재 폴더)를 기준으로 하며 `..`로 작업 폴더 밖으로 나가는 경로는 이유와 함께 거부합니다
(`path_refused`). 문자열만 검사하며 심볼릭 링크는 따라가지 않습니다. 빈 경로·NUL 포함·드라이브만 붙은 상대 경로(`C:foo`)도 거부합니다.

<a id="誤り"></a>

## 오류

| `code` | 의미 |
|---|---|
| `invalid_request` | JSON을 읽을 수 없음·인수 부족·알 수 없는 필드·형식 불일치 |
| `unknown_command` | 알 수 없는 명령 이름(`data.commands`에 목록) |
| `unsupported_version` | 명령 버전 불일치(`data.supported`) |
| `no_document` | 열린 문서 없음 |
| `not_found` | 세트·레이어·효과·채널·효과 종류·템플릿 없음 |
| `ambiguous` | 이름이 여러 대상과 일치(`data.candidates`. 레이어는 `{id, kind}`·세트는 `{id, name}`·채널은 `{index, name}`의 배열) |
| `invalid_value` | 범위 밖 값·선택지에 없음·거부된 조합 |
| `read_only` | 읽기 전용 세트 |
| `unsupported` | 해당 레이어·채널·종류에는 불가능하거나 이 버전에서 처리하지 않음 |
| `confirm_required` | 파괴적 작업에 `confirm: true` 없음(`data.files` 등에 대상 표시) |
| `path_refused` | 사용할 수 없는 경로(작업 폴더 밖·이름 형식·일반 파일이 아님) |
| `budget` | 예산·상한 초과 |
| `refused` | 문서가 거부한 작업(잠금·병합 조건 등) |
| `busy` | 지금은 수행 불가(스트로크 도중 등) |
| `conflict` | 저장 대상이 외부에서 변경됨·다른 저장이 진행 중 |
| `invalid_project` | `.ylp` 손상·읽을 수 없는 구조 |
| `io` | 파일 읽기·쓰기 실패 |
| `cancelled` | 취소됨 |
| `internal` | 예상하지 못한 실패 |

모든 오류는 일본어와 영어 메시지를 모두 가집니다(`message.ja`·`message.en`). 거부한 명령은 아무것도 바꾸지 않습니다.

<a id="バージョン"></a>

## 버전

명령 버전은 `COMMAND_VERSION`(현재 1)입니다. 인수 필드만 추가할 때는 올리지 않고 기존 명령을 읽을 수 없게 되는 변경을 할 때 올립니다. 버전이 다른 명령은 `unsupported_version`으로 거부합니다.
`.ylp` 형식은 바꾸지 않습니다(저장은 현재 형식 7 또는 이름을 붙여 저장한 선택 영역을 사용하는 파일의 형식 8 유지).

<a id="起動中のアプリへの通信"></a>

## 실행 중인 앱과의 통신

실행 중인 앱으로 보내는 명령은 앱의 MCP 엔드포인트(`http://127.0.0.1:<番号>/mcp`. yolu-mcp)의 도구(`tools/call`)로 전달합니다. 이 crate에는 통신 기능이 없습니다.
앱 호스트는 받은 명령을 UI 스레드에서 하나씩 실행합니다(저장은 백그라운드에서 실행하며 응답은 저장이 끝날 때까지 지연). 상대 경로는 앱이 연 프로젝트
폴더 기준이므로 호출자는 절대 경로로 전달하세요.

<a id="ホストを作る側へ"></a>

## 호스트 구현자를 위한 안내

`OpHost`를 구현하면 같은 명령이 동작합니다. 필요한 것은 문서 열기(`open`)·세트 문서 읽기(`read_set`)·수정(`write_set`)·저장(`save`)입니다. 내보내기(`export`)와
미리 보기(`preview`)는 기본적으로 `read_set`에서 `export::run`·`preview::render`를 거칩니다(실행 중인 앱의 호스트는 앱의 내보내기 설정이나 합성 경로를 사용하도록 대체 가능).
`execute`는 파괴적 작업 확인 → 분기를 수행합니다.

<a id="テスト"></a>

## 테스트

```sh
cargo test -p yolu-ops
cargo clippy -p yolu-ops --all-targets -- -D warnings
```

<a id="命令にしていないこと"></a>

## 명령으로 제공하지 않는 기능

그리기(스트로크·채우기·선택)·베이크·레이어 세트 조작(추가·이름·출력 할당. 여러 레이어 세트 문서를 열어 적용할 수 있으며 `layer.add`에서 `above`를 생략하면 첫 레이어 세트에 추가)·Live Link·모델 불러오기·여러 문서를 동시에 열기입니다. 화면 없는 호스트에서는 베이크한 메시 맵·모델을 읽는 생성기 평가도 하지 않습니다.
