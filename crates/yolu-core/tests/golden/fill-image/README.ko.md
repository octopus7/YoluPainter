[한국어](README.ko.md) · [日本語](README.md)

<a id="塗りつぶし投影の-c-正解"></a>

# 채우기 투영의 C# 기준 결과

0.4.x까지의 Unity 브리지 C# Core (Unity 브리지 태그 `0.4.0`의 `Runtime/Core`) 출력을 이어받은 것입니다. 다시 만들 때는 Rust 출력으로 만듭니다. 모두 인공 데이터입니다.

이방성을 끈 경로 (`FillInput::anisotropic`·`shape_anisotropic`이 false)의 기준 결과입니다. 켠 경로의 기준 결과는 Rust로 생성한 [`../fill-image-anisotropic/`](../fill-image-anisotropic/README.ko.md)에 있습니다.

`<mode>-<variant>.rgba`는 37×29, straight RGBA8이며 왼쪽 아래부터 행 우선 순서입니다. mode는 0 UV, 1 삼면 투영, 2 평면, 3 구형, 4 원통형, 5 데칼입니다. variant 0〜7의 입력은 Rust의 `tests/support/fill_cases.rs`에 있습니다.

`decal-values-<variant>.rgba`는 데칼 (mode 5)의 `ApplyDecalToValue`입니다. 같은 variant 0〜7 설정에 좌표만으로 결정되는 인공 값 (`tests/support/fill_cases.rs`의 `decal_values`)을 전달한 결과이며 형식은 위와 같습니다.

`.rgba`는 줄바꿈을 변환하면 안 되는 바이트열이므로 저장소의 `.gitattributes`에서 `binary`, 이 폴더 전체를 `-text`로 설정했습니다.

`source.txt`는 참조 커밋과 평가식의 SHA-256, `contracts.txt`는 거부 설정과 예산 경계의 실행 결과입니다. C#의 병렬 처리 수 1과 4에서 모든 바이트가 일치함을 확인한 후 저장했습니다 (C#으로 다시 만드는 경로는 남겨 두지 않음).
