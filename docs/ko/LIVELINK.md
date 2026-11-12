[한국어](LIVELINK.md) · [日本語](../LIVELINK.md) · [English](../en/LIVELINK.md)

<a id="live-linkファイルの受け渡し"></a>

# Live Link(파일 전달)

Unity 에디터의 Unity 브리지(VPM 패키지 `net.yozolab.yolupainter`)와 독립 실행형 YoluPainter가 같은 PC의 폴더에 JSON 파일을 놓아 정보를 주고받는 구조의 사양입니다.
Unity는 선택한 대상(씬 오브젝트)의 **FBX 경로·본 값·머티리얼 값과 이미지 경로**를 「요청」에 기록하여 두고, 독립 실행형이 이를 가져와 엽니다.
독립 실행형에서 내보내면 작성한 PNG를 「응답」에 기록하여 두고 Unity가 가져옵니다. 메시나 이미지 픽셀은 전송하지 않으며 양쪽이 직접 파일을 읽습니다.

사용 방법은 [UNITY.md](UNITY.md#live-link)에 있습니다. 이 문서는 양쪽 구현을 만드는 개발자와 전달 내용을 살펴보려는 사람을 위한 문서입니다.

<a id="フォルダ"></a>

## 폴더

양쪽이 같은 경로를 각각 결정합니다.

| OS | 폴더 |
|---|---|
| Windows | `%LOCALAPPDATA%\YoluPainter\LiveLink\` |
| macOS | `~/Library/Application Support/YoluPainter/LiveLink/` |
| Linux 등 | `${XDG_DATA_HOME:-~/.local/share}/YoluPainter/LiveLink/`(`XDG_DATA_HOME`이 절대 경로가 아니면 기본값) |

- 환경 변수 `YOLUPAINTER_LIVELINK_DIR`(절대 경로)이 있으면 양쪽 모두 해당 폴더를 사용합니다(테스트·앱 두 개를 함께 실행할 때).
- 내부에 `inbox/`(Unity → 독립 실행형)·`claimed/`(독립 실행형이 가져간 요청과 처리 중 표시 `<頼みのファイルの名前>.lock`, Unity는 읽거나 쓰지 않음)·`outbox/`(독립 실행형 → Unity)와 실행 중 표시 `presence.json`을 둡니다.
- 폴더에는 해당 사용자만 접근할 수 있어야 합니다. Unix에서는 권한 0700과 본인 소유여야 하며(Unity는 생성 시 `chmod 0700`), Windows에서는 해당 사용자만 허용하는
  DACL을 사용합니다. 독립 실행형은 다른 사용자도 접근할 수 있거나 소유자가 다른 폴더를 사용하지 않습니다(진입점 아이콘에 이유 표시).
- 파일은 반드시 `<名前>.tmp`에 쓰고 닫은 뒤 최종 이름으로 교체합니다. 읽는 쪽은 `.tmp`로 끝나는 이름을 무시합니다(작성 중인 파일을 읽지 않음).

<a id="起きている印-presencejson"></a>

## 실행 중 표시 `presence.json`

독립 실행형이 요청을 받는 동안 2초마다 다시 씁니다. 수신을 중단하거나 종료할 때 삭제합니다.

```json
{ "format": 1, "app": "YoluPainter", "version": "0.7.0", "pid": 1234, "updated": "2026-10-07T12:00:00Z" }
```

Unity는 `updated`(UTC)가 6초 이내이면 독립 실행형이 실행 중이라고 판단합니다. 오래되었거나 없으면 `--livelink`를 붙여 독립 실행형을 시작한 뒤 요청을 놓습니다.
(시작이 끝날 때까지 기다리지 않아도 됩니다. 독립 실행형이 시작된 뒤 가져갑니다.)

<a id="頼み-inboxjsonunity--スタンドアロン"></a>

## 요청 `inbox/<id>.json`(Unity → 독립 실행형)

```json
{
  "format": 1,
  "kind": "open",
  "id": "8f0c2a4e-0000-4000-8000-000000000001",
  "bridge": { "version": "0.7.0", "unity": "2022.3.22f1" },
  "project": { "root": "C:/Work/MyProject", "name": "MyProject" },
  "target": {
    "key": "GlobalObjectId_V1-2-…",
    "name": "Avatar",
    "export_dir": "C:/Work/MyProject/Assets/YoluPainter/Avatar"
  },
  "root": { "world": [1,0,0,0, 0,1,0,0, 0,0,1,0, 0,0,0,1] },
  "models": [
    { "id": 0, "fbx": "C:/Work/MyProject/Assets/Avatar/Body.fbx", "guid": "0123456789abcdef0123456789abcdef",
      "import": { "global_scale": 1.0, "use_file_scale": true, "bake_axis_conversion": false,
                  "import_blend_shapes": true, "preserve_hierarchy": false } }
  ],
  "renderers": [
    { "path": "Body", "model": 0, "node": "Body", "enabled": true, "skinned": true,
      "blend_shapes": { "eye_close": 0.0, "smile": 35.0 }, "materials": [0, 1] }
  ],
  "bones": [
    { "model": 0, "node": "Armature/Hips", "local": { "t": [0.0, 0.9, 0.0], "r": [0.0, 0.0, 0.0, 1.0], "s": [1.0, 1.0, 1.0] } }
  ],
  "materials": [
    { "key": "guid:0123456789abcdef0123456789abcdef/fileid:2100000", "name": "Body",
      "shader": { "name": "Hidden/lilToonCutout", "guid": "…", "package": "jp.lilxyzw.liltoon", "version": "2.3.4", "keywords": [], "render_queue": 2450 },
      "values": { "floats": { "_Cutoff": 0.5 }, "colors": { "_Color": [1, 1, 1, 1] }, "vectors": {}, "ints": { "_lilToonVersion": 45 } },
      "textures": [
        { "property": "_MainTex", "path": "C:/Work/MyProject/Assets/Avatar/body.png", "guid": "…",
          "srgb": true, "normal_map": false, "scale": [1, 1], "offset": [0, 0] },
        { "property": "_MatCapTex", "path": null }
      ] }
  ],
  "refused": [ { "path": "Accessory", "reason": "mesh_not_from_fbx" } ]
}
```

- 문자는 UTF-8 JSON입니다. 경로는 모두 절대 경로이며 구분자는 `/`입니다. 숫자는 JSON 숫자입니다(NaN·무한대는 기록하지 않으며 f32에 들어가지 않는 크기도 유한하지 않은 것으로 보고 거부).
- `format`은 1, `kind`는 `"open"`(열기, 같은 `target.key` 문서가 열려 있으면 재전송)만 지원합니다. 알 수 없는 키는 건너뜁니다.
- `id`: 요청마다 새 ID입니다(1~64자의 영숫자와 `-`·`_`). 응답 이름에 사용합니다.
- `target.key`: 대상 씬 오브젝트의 식별 정보(`GlobalObjectId`)입니다. 같은 대상의 재전송을 구별합니다. `name`은 표시용, `export_dir`은 기본 내보내기 위치입니다.
- `root.world`(선택): Unity 월드에서 대상 루트로 변환하는 4×4 행렬입니다(열 우선, 대상 루트의 `worldToLocalMatrix`).
- `models[]`: 대상이 사용하는 FBX입니다. `id`는 `renderers[].model`·`bones[].model`이 참조하는 번호입니다. 같은 FBX를 두 인스턴스에서 사용하면 항목도 두 개로 만듭니다(독립 실행형은 한 번 읽고
  본과 메시를 두 벌 배치). `import`는 Unity ModelImporter 설정입니다(없는 필드는 Unity 기본값).
- `renderers[]`: 렌더러입니다. `path`는 대상 루트부터의 경로(표시와 사유용), `node`는 FBX 내부 메시 노드의 경로(아래 「경로」), `enabled`는 표시 여부,
  `blend_shapes`는 BlendShape 이름 → 가중치(Unity와 같은 0~100), `materials`는 서브메시 순서의 `materials[]` 인덱스입니다.
- `bones[]`: 본 값입니다. `node`는 FBX 내부 본 노드의 경로(빈 값은 해당 FBX 루트에 해당하는 노드), `local`은 FBX 부모 본에 대한 로컬 변환(`t`·`r`(x, y, z, w)·`s`)입니다.
  요청에 없는 본은 FBX 값을 그대로 사용합니다. FBX 루트에 해당하는 Transform이 대상 자체 또는 대상의 조상일 때 Unity는 해당 루트 노드(`node`가 빈 값)를 보내지 않습니다.
- `materials[]`: Unity 머티리얼입니다. `key`는 머티리얼 식별 정보(텍스처 세트를 연결하는 키, 아래 표), `shader`는 셰이더 이름·에셋 GUID·
  `package`(셰이더 에셋이 속한 UPM 패키지 이름, `Assets` 내부와 내장 셰이더는 빈 값 또는 생략)·버전·키워드·렌더링 순서입니다.
  `values`는 프로퍼티 값(`floats`·`colors`·`vectors`·`ints`), `textures`는 텍스처 프로퍼티별 이미지 파일입니다.
  (`srgb`·`normal_map`은 TextureImporter 설정, `scale`·`offset`은 배율과 오프셋.) Unity 내부에만 있는 이미지(파일 없는 생성물·RenderTexture)는 `path`를 `null`로 설정합니다.
- `refused[]`: Unity가 보내지 못한 렌더러입니다(사유는 아래 표).

| `materials[].key` | 머티리얼 |
|---|---|
| `guid:<32 桁の 16 進>/fileid:<数>` | 머티리얼 에셋 |
| `object:<GlobalObjectId>` | 씬 내부 머티리얼(에셋 아님) |
| `instance:<InstanceID>` | GlobalObjectId가 비어 있는 머티리얼(해당 Unity 세션 안에서만 유효한 식별 정보) |
| `none` | 머티리얼이 없는 서브메시(독립 실행형은 머티리얼 없는 그룹으로 처리) |

독립 실행형은 재전송하거나 `.ylp`를 다시 열 때 에셋 키는 식별자로, 그 외 키는 머티리얼 이름으로 세트에 다시 연결합니다.
- 한도(초과하면 `too_large`로 거부): 파일 16 MiB, 렌더러·모델·전송하지 못한 렌더러 1,024개, 본 16,384개, 머티리얼 1,024개, 머티리얼 하나의 텍스처 256개,
  렌더러 하나의 BlendShape·머티리얼 하나의 값 종류별 개수 4,096개, 문자열 8 KiB입니다.

<a id="パスfbx-の中のノード"></a>

### 경로(FBX 내부 노드)

- 경로는 Unity가 가져온 모델의 루트부터 이름을 `/`로 연결한 것입니다.
- Unity는 가져오기 설정 `preserveHierarchy`가 꺼져 있고(기본값) FBX 루트의 자식이 하나뿐이면 그 자식을 프리팹 루트로 사용합니다(해당 자식 이름은 경로에서 빠지고
  변환은 프리팹 루트로 옮겨짐). 독립 실행형은 `import.preserve_hierarchy`를 확인하여 같은 루트에서 경로를 따라갑니다.
- 같은 이름의 형제 노드가 있어 하나로 결정할 수 없는 경로는 `ambiguous_bone`, 찾을 수 없는 경로는 `bone_not_found`입니다.
- Unity는 가져올 때 이름이 같은 형제 노드를 `Twin`·`Twin 1`처럼 다시 이름 짓습니다. FBX에 없는 `<名前> <数>` 형태의 경로 부분이고 FBX에 해당 이름의 형제 노드가 둘 이상 있을 때도
  `ambiguous_bone`입니다(어떤 형제 노드에 새 이름을 붙였는지는 FBX만으로 결정할 수 없음).

<a id="ボーンの値の決め方unity-の側"></a>

### 본 값 결정 방법(Unity 측)

- 메시 에셋 경로(`AssetDatabase.GetAssetPath(mesh)`)가 `.fbx`인 렌더러만 보내고 그 외는 `refused`에 `mesh_not_from_fbx`로 넣습니다.
- 본의 FBX 내부 경로는 `PrefabUtility.GetCorrespondingObjectFromOriginalSource(transform)`으로 FBX 모델 에셋의 Transform을 가져온 뒤 루트부터의 이름 경로로 만듭니다.
  가져올 수 없으면(프리팹이 해제된 경우 등) FBX 루트에 해당하는 Transform부터의 이름 경로로 찾습니다. 하나로 결정할 수 없으면 해당 렌더러를 `refused`로 처리합니다(`bone_not_found`,
  같은 이름의 형제 노드가 있으면 `ambiguous_bone`).
- `local`은 FBX 부모 노드에 해당하는 Unity Transform에 대한 해당 본 Transform의 상대 변환입니다(`parent.worldToLocalMatrix * bone.localToWorldMatrix`를 T·R·S로 분해).
  Unity 계층에서 부모가 바뀌었어도 FBX 부모 기준 값입니다. FBX 루트에 해당하는 노드는 대상 루트 기준 값입니다.
- 좌표는 Unity 그대로입니다(왼손 좌표계·Y 위쪽·미터).

<a id="取り込みの設定"></a>

### 가져오기 설정

- 독립 실행형은 Unity와 같은 방향·크기로 FBX를 읽습니다. Unity 단위 = 미터 × `global_scale` × (`use_file_scale`이면 1, 꺼져 있으면 1 ÷ 파일 단위의 미터값.
  UnitScaleFactor가 1(센티미터)인 FBX라면 100).
- `bake_axis_conversion`이 true인 FBX는 일치시킬 수 없으므로 해당 렌더러를 `unsupported_import`로 처리하고 포함하지 않습니다.
- `import_blend_shapes`가 false인 FBX는 BlendShape를 포함하지 않습니다.
- 서브메시 순서는 Unity와 같습니다(FBX 노드의 머티리얼 슬롯 순서, 면이 없는 슬롯은 건너뜀).

<a id="スタンドアロンの受け方"></a>

## 독립 실행형의 수신 방식

- 설정 「Unity의 Live Link 수신」이 켜져 있거나(기본값 켜짐) `--livelink`로 시작하면 수신합니다. 수신 중에는 0.5초마다 `inbox/*.json`을 확인합니다.
- 요청을 가져갈 때 먼저 `claimed/<頼みのファイルの名前>.lock`(처리 중 표시)을 같은 이름이 있으면 반드시 실패하는 방식(배타적 생성)으로 만듭니다. 생성에 성공한 수신자만
  요청을 `inbox/`에서 `claimed/`로 옮겨 처리하므로 독립 실행형이 두 개 실행 중이어도 요청 하나를 받는 것은 표시를 만들 수 있었던 하나뿐입니다(이름 변경 성공 여부로 판단하지 않음).
  적용을 마치면 `claimed/`에서 요청을 삭제한 뒤 표시를 삭제합니다.
- `claimed/`에 1일 넘게 남은 요청과 표시(종료된 프로세스가 남긴 항목)는 수신을 시작할 때 정리합니다. 요청은 `inbox/`에 남아 있고 표시만 1일 넘게
  남아 있던 경우(표시를 만든 직후 종료)에도 정리 후 다음 수신자가 가져갈 수 있습니다.
- 그리는 중·저장 중·다른 모델을 불러오는 중에 온 요청은 거부하지 않고 `claimed/`에 둔 채 기다렸다가 작업이 끝나면 적용합니다.
- 같은 `target.key`의 문서가 열려 있으면 **재전송**입니다. 포즈·BlendShape·머티리얼 값·표시 여부를 적용합니다. FBX 경로와 `guid`가 같으면 다시 읽지 않습니다.
  (표시 여부·사용 메시·머티리얼 할당이 바뀌면 읽어 둔 FBX에서 다시 구성합니다.) 재전송한 포즈는 수동으로 움직인 내용도 덮어씁니다(실행 취소 한 단계).
- 다른 대상이며 저장하지 않은 변경 사항이 있으면 「열기」와 같은 확인을 합니다. 열기를 취소하면 `refused`(`declined`)를 반환하고 변경 사항을 버리면 새 프로젝트로 엽니다.
- 불러오는 중에 사용자가 문서를 바꾸면(새로 만들기·열기) 결과를 새 문서에 넣지 않고 `refused`(`declined`)를 반환합니다. 수신을 중단한 경우에도 아직 적용하지 않은 요청과
  불러오는 중인 요청을 취소하고 `declined`를 반환합니다(`.ylp` 다시 열기는 계속 진행).
- 여러 FBX는 하나의 모델로 합칩니다(FBX가 둘 이상이면 본 루트 이름 앞에 FBX 번호를 붙여 구분). 포즈 패널에서 본을 움직일 수 있습니다.
- 텍스처 세트는 머티리얼 `key`당 하나입니다(같은 Unity 머티리얼을 사용하는 렌더러는 같은 세트).
- 원본 이미지: Color가 연결될 슬롯(`_MainTex`)의 이미지 파일을 읽고(PNG·TGA·JPG·PSD), 새 세트와 아직 손대지 않은 초기 세트의
  맨 아래 「원본 이미지」 레이어에 넣습니다. 이미지가 없는 머티리얼과 Unity 내부에만 있는 이미지는 흰색으로, 읽을 수 없는 이미지는 흰색으로 시작하고 이유를 알립니다.
  원본 이미지를 넣는 세트에는 새 문서의 흰색 단색을 두지 않습니다(흰색 단색 위에 빈 레이어가 있는 초기 상태 세트에 넣는 경우 흰색 단색을 제거하고 원본 이미지를 맨 아래에 넣음).
  텍스처 세트 하나에 여러 머티리얼을 할당하면 원본 이미지와 수신한 외관을 세트의 첫 번째 머티리얼에서 읽습니다.
- 원본 이미지가 PSD(sRGB)이면 단일 「원본 이미지」가 아니라 PSD 레이어 그대로 세트에 넣습니다. 「파일 → 가져오기」의 PSD 가져오기와 같은 방식으로 복사하며
  ([PSD.md](PSD.md)), PSD 레이어가 아래에 놓이고 세트의 빈 레이어는 그 위에 남습니다. 세트 크기는 PSD 캔버스 크기 그대로입니다.
  (레이어를 확대·축소하지 않습니다.) 가져올 때 제외되거나 바뀌는 항목은 확인 창과 같은 이름으로 알림에 표시합니다(창을 띄우지 않고 가져옴). 원본 PSD는 읽기만 하며
  같은 파일에 PSD를 내보낼 때는 교체 전에 확인합니다. 한도 등의 이유로 레이어 상태로 가져올 수 없는 PSD는 이유를 알리고 단일 「원본 이미지」로 만듭니다.
  요청 하나에서 읽는 원본 이미지 픽셀(단일 이미지와 레이어 상태로 가져온 PSD 문서)은 합계 512 MiB까지이며 남은 한도를 초과하는 PSD는 병합합니다(병합해도 들어가지 않으면
  흰색으로 처리하고 이유를 알림). 같은 PSD를 사용하는 머티리얼이 여러 개면 머티리얼마다 별도 문서로 읽고 각각 집계합니다.
  선형 PSD(`srgb`가 false)와 Color 이외 슬롯의 PSD는 레이어를 합성한 이미지 한 장으로 읽습니다.
- 재전송 시 원본 이미지 파일(경로·수정 시각·크기·`srgb`)이 바뀌었으면 원본 이미지를 넣은 직후 그대로인 세트(그리거나 변경하지 않음)에는
  새 원본 이미지를 다시 넣습니다(이미지가 사라졌으면 흰색). 바뀐 파일을 읽지 못하면(손상·작성 중 등) 세트를 변경하지 않고(흰색으로 바꾸지 않음)
  「<세트 이름>: 원본 이미지 ‘body.psd’를 읽을 수 없어 세트를 그대로 유지합니다」라고 알린 뒤 다음 재전송에서 다시 읽습니다. 수정한 세트는 변경하지 않고 「<세트 이름>: 원본 이미지 ‘body.psd’가 변경되었습니다」라고 알립니다(같은 변경은 한 번만).
  막 넣은 상태인지는 앱이 열려 있는 동안만 기억하므로 `.ylp`에서 다시 연 세트는 수정한 세트로 취급합니다(다시 연 뒤의 변경을 알림).
- lilToon 머티리얼 값은 세트의 「수신한 외관」이 됩니다(3D 뷰의 lilToon 재현에 사용). `_MainTex`는 독립 실행형의 Color로 표시하며 다른 슬롯 이미지는
  파일에서 읽습니다(긴 변 최대 2048, 합계 최대 256 MiB). 재전송 시 경로·수정 시각·크기가 같은 파일은 다시 읽지 않습니다. lilToon이 아닌 머티리얼에서는 수신한 외관을 제거합니다.
- lilToon 여부는 셰이더 이름으로 판단하지 않습니다. `values`에 `_lilToonVersion`(lilToon 셰이더의 버전 값)이 있거나 `shader.package`가
  `jp.lilxyzw.liltoon`일 때만 lilToon으로 그립니다.
- `.ylp`에는 적용한 요청에 현재 포즈를 넣고 머티리얼 값(`_lilToonVersion` 제외)을 제거한 형태로 루트의 `livelink.json`에 남겨 Unity 없이 다시 열 수 있습니다([YLP_FORMAT.md](YLP_FORMAT.md#livelinkjson)).
  수신한 머티리얼 값은 설정 「Unity에서 받은 머티리얼 값 저장」이 켜져 있는 동안(기본값 켜짐) 세트의 `look.json`에 남습니다(텍스처 픽셀은 넣지 않으며 다시 열 때 파일에서 읽음).
  「배포용으로 저장」에서 「모델 참조」를 제외한 사본에는 `livelink.json`도 포함되지 않습니다([SAVE_FOR_DISTRIBUTION.md](SAVE_FOR_DISTRIBUTION.md)).

<a id="返事-outbox-jsonスタンドアロン--unity"></a>

## 응답 `outbox/<id>-<n>.json`(독립 실행형 → Unity)

```json
{ "format": 1, "request": "8f0c2a4e-0000-4000-8000-000000000001", "kind": "exported",
  "app": { "version": "0.7.0" },
  "problems": [ { "path": "Accessory", "reason": "bone_not_found" } ],
  "files": [ { "material": "guid:0123456789abcdef0123456789abcdef/fileid:2100000", "property": "_MainTex",
               "path": "C:/Work/MyProject/Assets/YoluPainter/Avatar/Avatar_Body_Main.png", "srgb": true, "normal_map": false } ] }
```

- `kind`: `opened`(수신하여 열었거나 재전송을 적용함, 일치하지 않은 항목은 `problems`), `refused`(수락하지 않음, 이유는 `problems`), `exported`(사용자가 내보냄)입니다.
- `<n>`은 요청별 0부터의 일련번호입니다. 읽을 수 없는 요청에 대한 응답은 `inbox/` 파일 이름(확장자 앞)을 `request`로 사용합니다.
- `exported`: Live Link 대상 문서를 내보낼 때 「텍스처 내보내기」 창의 기본 출력 위치는 `target.export_dir`입니다(사용자가 다시 선택하면 해당 위치). 작성한 PNG 중 lilToon
  템플릿 이미지(`Main` → `_MainTex`·`Normal` → `_BumpMap`·`Smoothness` → `_SmoothnessTex`·`Metallic` → `_MetallicGlossMap`·`Emission` → `_EmissionMap`)와
  lilToon 패킹 슬롯 이미지를 `files`에 넣습니다. 대응이 없는 이미지와 키가 `none`인 그룹의 이미지는 넣지 않습니다. 머티리얼이 여러 개인 텍스처 세트의 이미지는
  머티리얼마다 같은 `path`로 한 건씩 넣습니다.

<a id="unity-の受け方"></a>

## Unity의 수신 방식

- 요청을 보낸 뒤·Live Link 창이 열려 있는 동안 `outbox/`를 1초마다 확인하고 읽은 응답을 삭제합니다.
- `exported`의 `files`를 가져오고 `srgb`·`normal_map`을 TextureImporter에 적용합니다(`Assets` 밖의 경로는 가져오지 않고 이유 표시). 머티리얼 적용 여부는 변경할 조합
  (머티리얼·프로퍼티·이전 이미지 → 새 이미지) 목록으로 확인하며 적용 후에는 Unity의 실행 취소로 되돌릴 수 있습니다.

<a id="理由の言葉reason"></a>

## 사유 코드(`reason`)

| 코드 | 의미 |
|---|---|
| `mesh_not_from_fbx` | 메시의 원본이 FBX가 아님(.asset 등) |
| `bone_not_found` | FBX 내부에서 본·메시 노드를 찾을 수 없음 |
| `ambiguous_bone` | 같은 이름의 형제 노드가 있어 하나로 결정할 수 없음 |
| `unsupported_import` | 가져오기 설정(`bake_axis_conversion`)에 맞출 수 없음 |
| `fbx_unreadable` | FBX를 읽을 수 없음 |
| `texture_unreadable` | 이미지 파일을 읽을 수 없음 |
| `too_large` | 한도 초과 |
| `format_unknown` | 형식 버전·종류가 다름·JSON으로 읽을 수 없음·규칙에 맞지 않음 |
| `busy` | 그리는 중·저장 중(현재 독립 실행형은 이 응답을 반환하지 않고 작업 후 적용) |
| `declined` | 사용자가 열기를 취소함: 다른 대상의 요청에서 저장하지 않은 변경 사항을 버리지 않음·불러오는 중 문서 변경·수신 중단 |

알 수 없는 코드는 그대로 표시합니다.

<a id="04-までの-live-link-からの違い"></a>

## 0.4까지의 Live Link와의 차이

- Unity에서 포즈를 움직이면 즉시 반영되거나 그리는 동안 Unity 화면에 반영되는 기능은 없습니다(재전송으로 적용하고 Unity에는 내보낸 PNG를 반환).
- FBX에서 온 것이 아닌 메시(.asset 등)와 가져오기 시 축 변환을 베이크한 모델(`bake_axis_conversion`)은 열 수 없습니다.
- 연결을 계속 유지하는 통신(네이티브 브리지 라이브러리·공유 메모리)은 사용하지 않습니다.
