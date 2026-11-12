[한국어](README.ko.md) · [English](README.md)

<a id="krita-4-default-brush-tips"></a>

# Krita 4 기본 브러시 끝

`brushes/`의 브러시 끝 이미지는 Krita의
`Krita_4_Default_Resources.bundle`에서 변경 없이 가져왔습니다.

- 출처: https://invent.kde.org/graphics/krita/-/blob/master/krita/data/bundles/Krita_4_Default_Resources.bundle
  (번들 SHA-256 `4180f474052305e9de6eaac1d832c1ddeb0654142bcde2eb2460629cf23796d0`, 2026-10-02에 내려받음)
- 라이선스: 번들의 `meta.xml`에 명시된 **CC0 1.0** (퍼블릭 도메인 헌정)
  (`<meta:meta-userdefined meta:name="license" meta:value="CC-0"/>`). 원래 `meta.xml`을 여기에 그대로 보관합니다.
- 제작자: Krita 프로젝트 (https://krita.org)의 David Revoy (Deevad). Ramon Miranda, Razvanc, Radian, Wolthera, Storm,
  Scottyp 등의 브러시에서 파생된 작품을 포함합니다. CC0는 출처 표시를 요구하지 않지만 예의상 표시하며,
  이 번들의 타사 미러에 CC BY 3.0이라는 표시가 붙은 사례가 있어 이를 명확히 합니다.

포함 내용: 래스터 브러시 끝 76개 (`.png`, `.gih`, `.gbr`). SVG 브러시 끝 3개는 제외했습니다 (이 도구는 SVG를 렌더링하지 않음).
번들의 Krita 페인트 작업 프리셋 (`.kpp`, Krita 브러시 엔진 설정)과 패턴도 제외했습니다. YoluPainter는
각 브러시 끝에 자체 기본 설정을 부여하며, 이는 Krita 프리셋이 아닙니다.

`SHA256SUMS`에 포함된 모든 파일을 나열했으므로 복사본을 번들과 비교할 수 있습니다.

브러시 끝을 읽는 방법: `.gbr` / `.gih`는 GIMP 형식을 따릅니다 (255 = 완전히 칠함). `.png` 브러시 끝은 Krita의 규칙을 따릅니다.
어두운 부분은 칠하고 흰색이나 투명한 부분은 칠하지 않으므로 커버리지 = (1 − 휘도) × 알파입니다.
