---
name: yolupainter
description: MCP 도구를 통해 YoluPainter 텍스처 페인팅 앱에서 열린 프로젝트를 작업합니다. 텍스처 세트와 레이어를 읽고, 레이어·마스크·필터·생성기를 추가하거나 편집하고, 채널 미리보기를 확인하고, 텍스처를 내보내고 저장합니다. 사용자가 자신의 YoluPainter 프로젝트를 살펴보거나 변경해 달라고 요청할 때 사용합니다.
---

[한국어](SKILL.ko.md) · [English](SKILL.md)

# YoluPainter

`yolupainter` 도구는 이 PC에서 실행 중인 YoluPainter 앱(`http://127.0.0.1:17347/mcp`)과 통신합니다. 앱에서 열린 프로젝트에 작용하며 사용자는 모든 변경 사항을 앱에서 볼 수 있습니다.

<a id="before-the-first-call"></a>

## 첫 호출 전

- 도구가 `io` 코드와 "Cannot reach a running YoluPainter"로 실패하면 사용자에게 앱을 시작하고 설정 > 외부 명령 수신을 켜 달라고 요청합니다. 앱에서 포트를 바꿨다면 도우미 설정의 연결에도 같은 포트를 사용해야 합니다.

<a id="reading"></a>

## 읽기

- `doc_info`: 파일, 텍스처 세트, 현재 세트, 저장하지 않은 변경 사항 유무를 확인합니다.
- `set_info`: 크기, 채널, 위에서 아래 순서의 레이어 스택을 확인합니다. 두 레이어의 이름이 같으면 id로 지정합니다(`ambiguous`가 후보를 나열함).
- `layer_get`, `effect_get`, `history_info`.
- `preview`: 채널의 PNG입니다(`max_edge`로 크기를 줄일 수 있음). 외관이 바뀌는 편집 후 확인합니다.

<a id="editing"></a>

## 편집

- `layer_add`, `layer_set`, `layer_move`, `mask_add`, `mask_set`, `effect_add`, `effect_set`.
- 각 편집 도구는 앱의 실행 취소 기록 한 단계가 됩니다. `undo`와 `redo`로 되돌리거나 다시 실행합니다.
- `action_run`은 여러 명령을 한 번에 적용합니다(`commands`: `{"command": "layer.add", "args": {...}}` 목록). 실행 취소 한 단계로 처리되며 하나라도 거부되면 모두 롤백합니다. 목록 안의 `$created:1`은 그 목록에서 처음 만든 레이어 또는 효과를 가리킵니다.
- `effect_add` 또는 `effect_set`을 호출하기 전에 `yolupainter://ops/effect-kinds`를 읽거나 `effect_list_kinds`를 호출하여 종류, 매개변수, 값 범위를 확인합니다.

<a id="destructive-tools-need-consent"></a>

## 파괴적 도구에는 동의 필요

- `layer_delete`, `mask_delete`, `effect_delete`, `save`와 파일을 교체하는 `save_as`, `export_channels`, `export_textures`, `export_psd`에는 `confirm: true`가 필요합니다.
- `action_run` 안의 파괴적 명령에는 각각 `confirm: true`가 필요합니다.
- `confirm: true`를 전달하기 전에 사용자에게 확인합니다. 이 값이 없으면 도구는 아무것도 변경하지 않고 `confirm_required`를 반환합니다.
- `save`는 파일 옆의 백업 폴더에 이전 버전을 보관합니다.
- `save_as`와 내보내기 도구에는 절대 경로를 사용합니다.

<a id="when-a-tool-is-refused"></a>

## 도구가 거부되었을 때

- `busy`: 사용자가 그리는 중이거나 저장 중이거나 앱의 모든 연결이 사용 중입니다. 잠시 기다린 뒤 다시 호출합니다.
- `read_only`: 텍스처 세트를 편집할 수 없습니다. 메시지에 이유가 나옵니다.
- `doc_open`의 `unsupported`: 앱은 문서를 전환하지 않습니다. 사용자에게 앱에서 파일을 열어 달라고 요청합니다.

<a id="not-available"></a>

## 사용할 수 없는 기능

스트로크 그리기, 채우기, 선택 영역, 베이킹, 레이어 세트 작업, 다른 문서 열기는 사용할 수 없습니다. 이 작업은 앱에서 하도록 사용자에게 안내합니다. 텍스처 세트에는 여러 레이어 세트가 있을 수 있습니다. `set_info`는 레이어가 어느 레이어 세트에 속하는지 구분하지 않고 함께 나열하며, `above` 없는 `layer_add`는 첫 번째 레이어 세트에 추가합니다.

<a id="more"></a>

## 추가 자료

- `yolupainter://docs/mcp`: 도구와 연결에 관한 문서입니다.
- `yolupainter://docs/guide`: 사용 설명서의 시작 페이지로, 각 페이지를 나열합니다. 페이지마다 별도의 리소스가 있습니다.
  `yolupainter://docs/guide-layers`(레이어, 마스크, 채널), `yolupainter://docs/guide-fill`(채우기 레이어, 필터와 생성기, 에셋, 액션),
  `yolupainter://docs/guide-files`(저장, 내보내기, PSD), `yolupainter://docs/guide-3d`(3D 뷰, 모델, 베이킹),
  `yolupainter://docs/guide-paths`, `yolupainter://docs/guide-settings`, `yolupainter://docs/guide-start`, `yolupainter://docs/guide-paint`,
  `yolupainter://docs/guide-select`, `yolupainter://docs/guide-keys`.
