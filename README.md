[한국어](README.md) · [日本語](README.jp.md) · [English](README.en.md)

# YoluPainter

[![CI](https://github.com/YozoraKurage/YoluPainter/actions/workflows/main-tested.yml/badge.svg?branch=main)](https://github.com/YozoraKurage/YoluPainter/actions/workflows/main-tested.yml)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

YoluPainter는 2D 캔버스와 3D 모델에 텍스처를 그리는 페인팅 앱입니다.
Unity와 연결하면 씬의 모델과 머티리얼 값을 가져와 열고, 그린 이미지를 Unity 머티리얼에 적용할 수 있습니다.

Windows를 주 대상으로 개발합니다. Mac과 Linux 버전은 시험용입니다.

## 한국어 지원

이 소스에는 한국어 UI와 한국어 문서가 포함되어 있습니다. **편집 → 설정 → 언어 → 한국어**에서 선택할 수 있습니다. 처음 실행할 때 운영체제 표시 언어가 한국어이면 한국어를 선택하며, 이후에는 저장한 언어 설정을 우선합니다. `YOLUPAINTER_LANG=ko` 환경 변수로 초기 언어를 지정할 수도 있습니다.

한글 기본 글꼴과 굵은 글꼴을 실행 파일에 포함하므로 앱의 UI 표시를 위해 별도 한글 글꼴을 설치할 필요가 없습니다. 기존 일본어와 영어도 선택할 수 있습니다. CLI/MCP의 기존 `ja`/`en` 구조 메시지는 한국어 선택 시 영어를 사용하며, 앱의 화면 문구는 한국어 카탈로그를 사용합니다. 일부 외부 디코더의 상세 진단과 기존 프리셋 이름은 영어로 표시될 수 있습니다.

**아래 원 프로젝트의 Releases에서 내려받는 실행 파일에는 이 포크의 변경 사항이 포함되지 않습니다.** 한국어판은 이 소스를 빌드하거나, 이 포크에서 별도로 제공하는 한국어판 실행 파일을 사용하세요.

## 주요 기능

- 2D 및 3D 페인팅. 필압과 기울기에 반응하는 브러시, 색상 혼합, 흐림·문지르기·복제, 대칭, 자, 스텐실
- 페인팅·편집·포즈 모드. 3D 뷰에서 투영 상자, 데칼, 포인트, 3D 경로와 포즈의 본을 선택하고 이동
- 설정 창의 단축키 항목에서 키보드·마우스 조합 및 원형 메뉴 할당 변경
- 레이어, 그룹, 마스크, 클리핑, 26가지 혼합 모드, 조정 레이어, 텍스트 레이어, 잠금, 채널별 출력을 선택하는 레이어 세트
- Color·Roughness·Metallic·Height·Normal·Emission 및 사용자 채널을 한 번의 스트로크로 함께 그리는 머티리얼 페인팅
- 필터, 베이크한 메시 맵(AO/앰비언트 오클루전·곡률·두께·ID 등)을 사용하는 생성기, 노이즈와 그런지, 스마트 머티리얼
- 3D 뷰의 lilToon 외관. 여러 3D 뷰를 추가하고 나란히 배치
- Unity Live Link. 씬의 모델을 한 번의 작업으로 열고 내보낸 이미지를 Unity로 반환. 머티리얼 적용은 Unity에서 확인한 경우에만 실행
- 명령줄(`yolupainter-cli`) 및 MCP로 연결한 AI 도우미를 통한 레이어·마스크·효과 조작
- PSD 레이어·그룹·마스크·조정 읽기와 쓰기, ABR 및 CLIP STUDIO 브러시(`.sut`) 가져오기
- 채널별 PNG와 Unity Standard·URP·HDRP·lilToon용 템플릿 내보내기
- 비정상 종료 후 자동 복구

## 다운로드

원 프로젝트의 Windows 64비트 설치 프로그램과 ZIP은 [Releases](https://github.com/YozoraKurage/YoluPainter/releases)에 있습니다.
설치 방식과 업데이트는 [설치 안내](docs/ko/INSTALL.md)를 참고하세요. 이 포크의 한국어 변경 사항을 사용하려면 아래 빌드 절차를 따르세요.

Linux에는 x86_64 tar.gz(`yolupainter-<버전>-x86_64-unknown-linux-gnu.tar.gz`)와 시험용 ARM/aarch64 tar.gz(`yolupainter-<버전>-aarch64-unknown-linux-gnu-experimental.tar.gz`)도 제공됩니다.
창은 X11로 열리므로 Wayland 데스크톱에서는 XWayland 위에서 실행됩니다. 앱 자체 업데이트는 지원하지 않으며, 새 버전이 나오면 도움말 메뉴에 릴리스 페이지를 여는 항목이 표시됩니다.
원 프로젝트는 ARM 실기기에서 실행을 검증하지 않았습니다. 실행에 필요한 구성 요소는 [빌드 안내의 Linux 절](docs/ko/BUILDING.md#linux試用)을 참고하세요.

Mac에는 시험용 ZIP(`yolupainter-<버전>-macos-universal-experimental.zip`)도 제공됩니다. Apple Silicon과 Intel용 코드를 모두 포함하는 Universal `YoluPainter.app`이 들어 있습니다.
Apple 개발자 서명과 공증은 없으며 ad-hoc 서명만 적용됩니다. 앱 자체 업데이트는 지원하지 않으며, 새 버전이 나오면 도움말 메뉴의 릴리스 페이지 링크로 알립니다.
원 프로젝트는 Apple Silicon Mac(macOS 27)에서 0.6.0을 열어 창 표시와 그리기를 확인했으며, Intel Mac에서는 실행하지 않았다고 안내합니다.

정식 서명이 없으므로 ZIP을 푼 `YoluPainter.app`을 처음 열 때 macOS가 차단할 수 있습니다. 원문의 실행 절차는 다음과 같습니다([Apple 안내](https://support.apple.com/ja-jp/guide/mac-help/mh40616/mac)).

1. `YoluPainter.app`을 더블 클릭하고 차단 안내를 닫습니다.
2. Apple 메뉴 → 시스템 설정 → 개인정보 보호 및 보안을 엽니다.
3. 보안 항목에서 열기를 누릅니다.
4. 그대로 열기를 누릅니다. 원문 안내에 따르면 앱 실행을 시도한 뒤 약 1시간 동안 사용할 수 있습니다.
5. 로그인 암호를 입력하고 확인합니다. 이후에는 더블 클릭으로 실행할 수 있습니다.

## 소스에서 빌드

Rust stable과 C/C++ 빌드 환경이 필요합니다.

```sh
cargo build --release -p yolu-app --locked
```

Windows 실행 파일은 `target/release/yolupainter.exe`에 생성됩니다. Linux와 macOS에서는 `target/release/yolupainter`입니다.
운영체제별 준비는 [빌드 안내](docs/ko/BUILDING.md)를 참고하세요. 소스 패키지의 `docs/ko/DEVELOPMENT.md`에는 테스트와 개발 절차가 있습니다.

## 문서

- [사용 설명서](docs/ko/GUIDE.md)
- [Unity 연동](docs/ko/UNITY.md)(Live Link)
- [명령줄과 AI 조작](docs/ko/CLI.md)(`yolupainter-cli`·[MCP 연결](docs/ko/MCP.md))
- [PSD](docs/ko/PSD.md), [브러시](docs/ko/BRUSH.md), [브러시 가져오기](docs/ko/BRUSH_IMPORT.md), [서브 도구](docs/ko/SUBTOOLS.md), [그라데이션 맵](docs/ko/GRADIENT_MAP.md), [3D 뷰](docs/ko/PREVIEW.md), [복구](docs/ko/RECOVERY.md), [배포용 저장](docs/ko/SAVE_FOR_DISTRIBUTION.md), [.ylp 형식](docs/ko/YLP_FORMAT.md)
- [변경 기록](CHANGELOG.ko.md)
- [한국어 지원 변경 및 검증 안내](KOREAN_SUPPORT.md)

각 문서 상단의 언어 링크로 한국어판과 원문을 전환할 수 있습니다. 일본어 원문 문서는 기존 경로에 보존하며, 루트 일본어 README만 [README.jp.md](README.jp.md)로 옮겼습니다.

## 문의

원 프로젝트 문의, 개발 질문, 기능 요청은 [Discord](https://discord.gg/c8NNfhJ94J)를 이용할 수 있습니다. 이 포크에만 적용된 한국어 번역이나 변경 사항은 포크 리포지터리에서 관리하세요.

## 개인정보 보호

원 프로젝트의 동작 기준으로, 업데이트 확인을 선택하면 GitHub에 최신 버전을 조회합니다. 그 외에는 네트워크로 정보를 보내지 않습니다. Live Link는 같은 PC의 폴더를 통해 파일을 교환합니다. 외부 조작 기능은 설정에서 켜 둔 동안에만 `127.0.0.1`(현재 PC)에서 연결을 기다립니다.

## 라이선스

[MIT License](LICENSE). 사용한 라이브러리·글꼴·아이콘의 라이선스는 [한국어 제3자 안내](THIRD_PARTY.ko.md)와 [원문](THIRD_PARTY.md)에 있으며, 배포물에는 라이선스 전문이 포함됩니다. 추가한 한글 글꼴은 Noto Sans CJK KR에서 파생한 SIL OFL 1.1 글꼴입니다.
