[한국어](README.ko.md) · [日本語](README.md)

<a id="塗りつぶし投影の異方性の正解rust"></a>

# 채우기 투영의 이방성 기준 결과 (Rust)

`../fill-image/`와 같은 입력 (`tests/support/fill_cases.rs`의 투영 6개 × 입력 8개와 데칼 값)을 사용하여 이미지와 데칼 모양을 모두 이방성으로 읽어 (`FillInput::anisotropic`·`shape_anisotropic`을 켬) 렌더링한 결과입니다. 이름과 형식 (37×29, straight RGBA8, 왼쪽 아래부터 행 우선 순서)은 `../fill-image/`와 같습니다.

C# 평가식에는 이방성 읽기가 없으므로 Rust 출력을 기준 결과로 사용합니다. 다시 생성하려면 `YOLU_GOLDEN_UPDATE=1 cargo test -p yolu-core --test reference rust_anisotropic`을 실행합니다. 스레드 1의 출력으로 덮어쓰며, 스레드 2·4와 타일로 나눈 렌더링의 바이트가 같은지 확인하는 검사는 그대로 수행합니다. 다시 생성한 뒤에는 차이를 검토하여 의도한 변화만 있는지 확인하세요.

투영 범위가 원형인 픽셀 (UV 그대로 복사·확대)은 이방성을 켜도 껐을 때와 같은 바이트이므로 `0-0` 등 일부 파일은 `../fill-image/`와 내용이 같습니다.
