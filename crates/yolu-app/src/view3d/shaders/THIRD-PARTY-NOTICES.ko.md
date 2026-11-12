[한국어](THIRD-PARTY-NOTICES.ko.md) · [日本語](THIRD-PARTY-NOTICES.md)

<a id="3d-ビューの-liltoon-の再現の第三者表記"></a>

# 3D 보기의 lilToon 재현에 관한 제3자 고지

`liltoon/`의 구성 요소 (`*.wgsl`. 연결 방식은 `crates/yolu-app/src/view3d/liltoon_wgsl.rs`)와 `crates/yolu-app/src/look/liltoon.rs`의 속성 기본값·이름은 lilToon 2.3.4의
셰이더 (`Shader/Includes`) 및 인스펙터에서 수식과 값을 WGSL·Rust로 옮긴 것입니다. lilToon의 파일·텍스처·
이미지는 동봉하지 않습니다. 배포처: [lilToon](https://github.com/lilxyzw/lilToon) (버전 2.3.4,
`https://github.com/lilxyzw/lilToon/releases/download/2.3.4/jp.lilxyzw.liltoon-2.3.4.zip`).

빌트인 렌더 파이프라인의 조명 수식은 lilToon에 포함된 OpenLit Library 1.0.2 (`openlit_core.hlsl`)에서 옮겼습니다.
OpenLit의 라이선스는 [CC0 1.0 Universal](https://creativecommons.org/publicdomain/zero/1.0/)입니다 (표기 의무는 없지만 출처로 기재합니다).

반짝이의 난수 (`hash_rgb`)는 lilToon의 `lil_common_functions_thirdparty.hlsl`에 있는 `lilHashRGB4`에서 옮겼습니다. lilToon은 출처를
Shadertoy의 “Simplest Fastest 2D Hash” (James_Harnett, 2018년, https://www.shadertoy.com/view/MdcfDj)로 명시합니다 (lilToon의
`Third Party Notices.md`에서 참고 문서 항목). 원문 첫머리는 `// LICENSE: http://unlicense.org/`이며 라이선스는
[The Unlicense](https://unlicense.org/)입니다 (퍼블릭 도메인 기증. 표기 의무는 없지만 출처로 기재합니다. 2026-10-05에 원문 확인).
3번째 상수 (2912667907)와 4점을 한 번에 생성하는 방식은 lilToon의 것입니다 (MIT).

lilToon 라이선스 원문 (패키지의 `LICENSE`, SHA-256 `7ca2e241979e77241d90ce5c122dc2f5128881ed5af3538eb9aa1e61f35d2660`):

```
MIT License

Copyright (c) 2020-present lilxyzw

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```
