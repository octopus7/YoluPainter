[한국어](README.ko.md) · [日本語](README.md)

<a id="互換性の正解データ"></a>

# 호환성 기준 데이터

모두 인공적으로 만든 테스트 데이터입니다. 사용자 작품이나 모델은 포함하지 않습니다.

C# 라이터·리더의 기준 데이터는 Unity 브리지 태그 `0.4.0` 이전의 `Runtime/Core`로 만든 것입니다(0.5.0 Unity 브리지는 .ylp를 처리하지 않으며 `Runtime/Core`와 `Tests/`를 제거함). C#으로 다시 만드는 경로는 남겨두지 않았습니다(Unity 브리지 태그 `0.4.0` 이전을 가져와 만든 것을 그대로 이어받음).

| 파일 | 내용·비교 대상 |
|---|---|
| `format1.ylp` ~ `format6.ylp` | 0.4.x까지의 Unity 브리지(태그 `0.4.0`까지)의 `Tests/Editor/Persistence/Fixtures~/`. 이전 형식 마이그레이션, 이미지·스마트 리소스·브러시, ZIP 전체 항목과 manifest의 바이트 일치 |
| `format5-shared-materials.ylp` | 같은 0.4.x까지의 Unity 브리지 테스트. 3개 세트와 공유 스마트 머티리얼 유지 |
| `native-v1.utpaint` ~ `native-v21.utpaint` | 버전별 기본 배치와 0.4.x까지의 Unity 브리지 `DocumentBinary.Read` 호환성. 픽셀·투명 RGB·일본어 이름·버전 2 이후 마스크·버전 7 이후 노멀 설정 포함 |
| `native-rich-v21.utpaint` | C# `DocumentBinary.Write`로 작성한 정본. 모든 채널, 마스크, 모든 필터 종류, 모든 생성기 종류, 고정 키·ID 색, 도형·램프, 이미지·데칼, 조정, 2D/3D 머티리얼 경로, 그룹, 잠금, Anchor, 수동 ID 색. 읽고 다시 저장한 바이트 배열을 비교 |
| `m1-mode-00.ylp` ~ `m1-mode-25.ylp`와 같은 이름의 PNG | 형식 7·정본 21 라이터와 CPU 합성. 26가지 합성 모드, 연속 클리핑, 맨 아래 레이어 클리핑 표시, 숨긴 레이어, 반투명, 투명 RGB, 일본어 이름, 17×11픽셀·8타일의 외곽. 00은 모든 material 종류와 첫 번째가 아닌 현재 세트도 포함. |
| `m1-pattern.ylp`와 같은 이름의 PNG | 65×33픽셀의 줄무늬와 그라데이션. 압축이 잘 되는 deflate 스트림에서도 C# 출력과 모든 바이트가 일치하는지 검증. |
| `selection-v1.bin` | C# `SelectionBinary.Write`로 작성한 선택 영역 |
| `effects-filters`·`effects-generators`·`effects-anchors`·`effects-fills`·`effects-paths`의 `.utpaint`·`.composite`·`.layers` 및 `effects-inputs.bin` | C# 실제 라이터(정본 21)가 만든 효과(필터 스택·생성기·Anchor·채우기 이미지와 투영·그라데이션·경로)가 들어 있는 정본. `.composite`는 모든 채널의 합성(합성 수식이 f32로 바뀐 뒤 core로 다시 생성), `.layers`는 레이어·마스크별 평가 출력, `effects-inputs.bin`은 평가에 전달하는 인공 입력(메시 맵·모델 루트·이미지). `tests/ylp/effects_bridge.rs`에서 사용 |
| `effects-legacy-v9` ~ `effects-legacy-v20`의 `.utpaint`·`.v21`·`.composite` | 효과가 있는 이전 버전(9~20) 배치의 정본을 C# 리더로 읽고 `DocumentBinary.Write`로 다시 작성한 버전 21(`.v21`)과 합성. 버전별 읽기와 마이그레이션 확인 |
| `rust-written-effects-v21.utpaint`·`.unity.txt`·`.layers`·`.composite` | 편집 API로 만든 효과 포함 버전 21을 Rust `from_core`로 작성한 파일(고정 ID. 테스트에서 같은 바이트 배열을 만드는지 확인)과 Unity 0.2.0 리더로 읽은 기록·평가 출력·합성 |
| `procedural-v23.utpaint`·`.unity.txt` | Rust `from_core`가 작성한 버전 23(노이즈·그런지)과 Unity 0.2.0 리더로 읽은 기록(“Unsupported archive version”으로 거부). `tests/ylp/procedural_bridge.rs` |
| `smart/` | `.ylsmart`와 인덱스의 인공 데이터([smart/README.md](smart/README.ko.md)) |
| `m2-groups.utpaint`·`m2-masks.utpaint`·`m2-channels.utpaint`·`m2-clipping.utpaint`·`m2-tiny.utpaint`와 같은 이름의 `.composite` | Unity 0.2.0 Core(정본 21)의 실제 라이터. 그룹(통과·분리·중첩·빈 그룹·숨김·클리핑), 래스터 마스크(활성화·반전·농도·픽셀 없음·그룹 마스크), 채널별 활성화와 합성, 채우기·조정(반전·레벨·색상/채도/명도), Normal 설정, 클리핑, 1픽셀. `.composite`는 C#의 모든 채널 합성(번호순, 아래 행부터)에 이어 Normal 파일 출력이 들어 있음. 단, m2-groups·m2-masks·m2-channels의 `.composite`는 합성 수식이 f32로 바뀐 뒤 core 출력으로 다시 만든 것(`YOLU_GOLDEN_UPDATE=1 cargo test -p yolu-io --test ylp m2_bridge`. C# 출력과 4~23바이트가 다르며 차이는 최대 1~2단계). m2-clipping·m2-tiny는 C# 출력 그대로 |
| `locks-v21.utpaint`와 같은 이름의 `.composite` | Unity 0.2.0 Core(정본 21)의 실제 라이터. 레이어 잠금(버전 12 속성 표시의 비트 1과 바로 뒤 int): 개별 4종·중첩·전체·그룹·채우기·조정·클리핑 및 채널별 합성과의 공존·잠금 없는 레이어. 잠금은 합성을 바꾸지 않음(`.composite`는 잠금을 풀어도 같음) |
| `rust-written-locks-v21.utpaint`·`.unity.txt`·`.composite` | `locks-v21`을 편집하여 잠금을 적용·해제하고 복제한 버전 21을 Rust `from_core`로 작성한 파일(고정 ID이며 테스트에서 같은 바이트 배열을 만드는지 확인), 이를 Unity 0.2.0 `DocumentBinary`로 읽은 기록(읽기 가능, 다시 쓰면 같은 바이트 배열, 레이어별 자체 잠금과 적용 잠금), C# 모든 채널 합성 |
| `user-channels-v22.utpaint` | Rust `from_core`가 작성한 버전 22(사용자 채널 3개, 번호 6·8·9). C#에 라이터가 없어 이 라이터의 바이트 배열을 기준으로 고정(테스트에서 같은 바이트 배열을 만드는지 확인) |
| `user-channels-v22.unity.txt` | 위 버전 22를 Unity 0.2.0의 `DocumentBinary.Read` / `ReadId` / `YlpFormat.Open`(`Runtime/Core`를 그대로 컴파일)으로 읽은 결과. `TexturePaintWindow.ReadTextureSets` 행은 창이 정본 읽기 실패에 붙이는 문구를 리더 예외에서 같은 형태로 만든 것이며 행 이름에도 그렇게 명시 |
| `adjust-v24.utpaint` | Rust `from_core`가 작성한 버전 24(색조 보정 6종: 그라데이션 맵·톤 곡선·색상 균형·밝기/대비·이진화·포스터화를 조정 레이어 6개, 채우기 레이어 필터 단계 6개·마스크 단계 1개에 배치). C#에 라이터가 없어 이 라이터의 바이트 배열을 기준으로 고정(테스트에서 같은 바이트 배열을 만드는지 확인). 재생성은 `YOLU_UPDATE_FIXTURES=1 cargo test -p yolu-io --test ylp adjust_bridge::version_24` |
| `adjust-v24.unity.txt` | 위 버전 24를 Unity 0.2.0(태그 `0.2.0`의 `Runtime/Core`를 그대로 컴파일)의 `DocumentBinary.Read` / `ReadId` / `YlpFormat.Open`으로 읽은 결과. 버전 22·23과 마찬가지로 “Unsupported archive version”으로 거부하는 기록 |
| `rust-written-v21.utpaint` | Rust `from_core`가 작성한 버전 21(`m2-groups`를 열어 채우기·조정·마스크·복제한 그룹·채널별 합성을 편집한 것). 사용자 채널이 없는 문서를 Unity 0.2.0이 읽을 수 있음을 확인하는 기준 데이터 |
| `rust-written-v21.unity.txt`·`rust-written-v21.composite` | 위 버전 21을 Unity 0.2.0의 `DocumentBinary.ReadId` / `Read`로 읽고 `Write`로 다시 쓴 바이트 배열이 원래와 같은지 및 레이어 수를 기록한 것과 C# 모든 채널 합성(`m2-*.composite`와 같은 순서) |
| `unity-generation/` | Unity 0.2.0의 `GenerationStore.Commit`(`Runtime/Core`를 그대로 컴파일)이 작성한 복구 저장소. 확정 2회(정본·선택 영역·resources·`recovery.json`. 바뀌지 않은 내용은 공유). Rust `GenerationStore`로 읽고 목록에 표시하며 계속 확정할 수 있음을 확인하는 기준 데이터 |
| `rust-generation.unity.txt` | 연 `format6.ylp`의 항목을 그대로 세대로 만든 저장소(`as-opened`)와 `sets/<ID>/` 아래의 중첩 이름을 제거한 것(`flat-names`)을 Unity 0.2.0의 `GenerationStore.Load`로 읽은 결과. 전자는 “Unsafe generation filename”으로 거부되고 후자는 읽을 수 있음(Rust 세대 이름의 허용 범위가 0.4.x까지의 Unity 브리지보다 넓다는 기록) |
| `selection/selection-*.bin`과 `.amounts` | 70×50픽셀·타일 16 문서에서 만든 선택 영역(사각형·타원·다각형 조합, 흐림, 모두 선택, 반전, 아무것도 선택하지 않음)의 C# `SelectionBinary.Write` 출력과 캔버스 선택량의 원시 배열(아래 행부터). Rust로 같은 선택 영역을 만들어 작성한 모든 바이트가 일치하며 읽으면 같은 양으로 복원되는지 확인 |
| `brushes/cases.txt`·`brushes/fuzz.txt`·`brushes/source.txt` | 브러시 형식 가져오기(`tests/brushes/brush_golden.rs`). Rust 테스트가 만든 입력(수동으로 만든 88사례, 함께 제공되는 Krita GIMP 형식 실제 파일 37개[`.gbr`·`.gih`], 수동 사례를 손상시킨 약 1.1만 입력. `tests/brush_files/corpus.rs`)을 0.4.x까지의 Unity 브리지 `GimpBrushReader`·`PhotoshopBrushReader`·`PhotoshopPatternReader`(`Runtime/Core`를 원문 그대로 빌드하여 실행)에 통과시킨 결과. `cases.txt`는 사례별 가져오기 성공·거부 여부와 설정 지문, `fuzz.txt`는 손상시킨 입력별 결과와 축약 지문, `source.txt`는 C# 원문 지문. 0.4.x까지의 Unity 브리지 출력을 이어받은 것으로 다시 만들지 않음 |
| `brushes/png-tips.txt` | 함께 제공되는 Krita PNG 브러시 끝 39개의 커버리지(어두울수록 칠하며 흰색과 투명은 칠하지 않음) 지문. 다른 디코더(Pillow)로 구한 것. `tests/brushes/brush_bundled.rs`에서 Rust 리더와 비교 |

이전 정본 데이터는 버전별 기본 배치를 나타내는 인공 데이터이며 모든 버전과 모든 속성의 조합을 망라하지는 않습니다.

<a id="作り直し"></a>

## 재생성

C# 라이터가 만든 기준 데이터(`.utpaint`·`.composite`·`.layers`·`effects-inputs.bin`·`selection/`·`unity-generation/`·`brushes/` 등)와 0.4.x까지의 Unity 브리지 리더 기록은 0.4.x까지의 Unity 브리지 출력을 이어받은 것으로 다시 만들지 않습니다(호환성 기준이므로 Rust 출력으로 대체하지 않음). 새 테스트 데이터를 추가할 때도 원본 파일을 덮어써 호환성 기준을 대체하지 마세요. 원래 형식 1~6 ZIP도 다시 만들지 않습니다.

Rust가 작성하는 기준 정본(버전 22와 버전 21)은 다시 생성할 때만 다음을 실행합니다.

```sh
YOLU_UPDATE_FIXTURES=1 cargo test -p yolu-io --test ylp -- m2_bridge::version_22_fixture m2_bridge::version_21
```

레이어 잠금 정본 중 Rust가 작성하는 잠금 포함 버전 21은 `YOLU_UPDATE_FIXTURES=1 cargo test -p yolu-io --test ylp layer_locks::rust_written`으로 다시 만듭니다.

Rust가 작성하는 효과 포함 버전 21·절차적 버전 23·색조 보정 버전 24 정본은 각각 `YOLU_UPDATE_FIXTURES=1 cargo test -p yolu-io --test ylp effects_bridge::rust_written_effects_fixture_is`·`procedural_bridge::version_23_fixture`·`adjust_bridge::version_24`로 다시 만듭니다.

PSD 입력·호환성 판정·다시 쓰기 기준 데이터는 [psd/README.md](psd/README.ko.md)를 참조하세요.
