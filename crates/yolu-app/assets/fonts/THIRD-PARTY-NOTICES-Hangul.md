[한국어 README](../../../../README.md) · [한국어 지원 안내](../../../../KOREAN_SUPPORT.md)

# 한글 UI 글꼴의 제3자 고지

`YoluPainterHangul-Regular.otf`와 `YoluPainterHangul-Bold.otf`는 Noto Sans CJK KR 2.004에서 한글 음절과 자모를 추출한 파생 글꼴입니다. 원본의 저작권 및 SIL Open Font License 1.1을 유지하며, 파생 글꼴의 패밀리 이름은 `YoluPainter Hangul`로 변경했습니다.

- 원 프로젝트: https://github.com/notofonts/noto-cjk
- 원본: `NotoSansCJK-Regular.ttc`, `NotoSansCJK-Bold.ttc`의 한국어 페이스(인덱스 1), 버전 2.004
- 추출 범위: U+1100–11FF, U+3130–318F, U+A960–A97F, U+AC00–D7A3, U+D7B0–D7FF, U+FFA0–FFDC 중 원본이 지원하는 글리프
- 포함: 현대 한글 완성형 11,172자 및 지원되는 자모, Regular/Bold
- 변경: 한글 범위 서브셋, 패밀리/스타일/PostScript 이름 변경. 원본의 글리프 윤곽과 저작권·라이선스 메타데이터 유지
- 생성 스크립트: `tools/make-ui-font-hangul.py`
- 생성에 사용한 fontTools: 4.61.1

실행 예시:

```sh
python tools/make-ui-font-hangul.py /usr/share/fonts/opentype/noto
```

라이선스 원문은 같은 폴더의 `OFL-Noto-CJK.txt`에 있으며 아래에도 그대로 수록합니다. 파일 해시는 `tools/licenses-reviewed.json`에서 확인합니다.

```text
Copyright 2010-2012 Google Corporation
Copyright 2014-2021 Adobe (http://www.adobe.com/)

-----------------------------------------------------------
SIL OPEN FONT LICENSE Version 1.1 - 26 February 2007
-----------------------------------------------------------

PREAMBLE

The goals of the Open Font License (OFL) are to stimulate worldwide
development of collaborative font projects, to support the font
creation efforts of academic and linguistic communities, and to provide
a free and open framework in which fonts may be shared and improved in
partnership with others.

The OFL allows the licensed fonts to be used, studied, modified and
redistributed freely as long as they are not sold by themselves. The
fonts, including any derivative works, can be bundled, embedded,
redistributed and/or sold with any software provided that any reserved
names are not used by derivative works. The fonts and derivatives,
however, cannot be released under any other type of license. The
requirement for fonts to remain under this license does not apply to
any document created using the fonts or their derivatives.

DEFINITIONS

"Font Software" refers to the set of files released by the Copyright
Holder(s) under this license and clearly marked as such. This may
include source files, build scripts and documentation.

"Reserved Font Name" refers to any names specified as such after the
copyright statement(s).

"Original Version" refers to the collection of Font Software components
as distributed by the Copyright Holder(s).

"Modified Version" refers to any derivative made by adding to,
deleting, or substituting -- in part or in whole -- any of the
components of the Original Version, by changing formats or by porting
the Font Software to a new environment.

"Author" refers to any designer, engineer, programmer, technical writer
or other person who contributed to the Font Software.

PERMISSION & CONDITIONS

Permission is hereby granted, free of charge, to any person obtaining a
copy of the Font Software, to use, study, copy, merge, embed, modify,
redistribute, and sell modified and unmodified copies of the Font
Software, subject to the following conditions:

1) Neither the Font Software nor any of its individual components, in
Original or Modified Versions, may be sold by itself.

2) Original or Modified Versions of the Font Software may be bundled,
redistributed and/or sold with any software, provided that each copy
contains the above copyright notice and this license. These can be
included either as stand-alone text files, human-readable headers or
in the appropriate machine-readable metadata fields within text or
binary files as long as those fields can be easily viewed by the user.

3) No Modified Version of the Font Software may use the Reserved Font
Name(s) unless explicit written permission is granted by the
corresponding Copyright Holder. This restriction only applies to the
primary font name as presented to the users.

4) The name(s) of the Copyright Holder(s) or the Author(s) of the Font
Software shall not be used to promote, endorse or advertise any
Modified Version, except to acknowledge the contribution(s) of the
Copyright Holder(s) and the Author(s) or with their explicit written
permission.

5) The Font Software, modified or unmodified, in part or in whole, must
be distributed entirely under this license, and must not be distributed
under any other license. The requirement for fonts to remain under this
license does not apply to any document created using the Font Software.

TERMINATION

This license becomes null and void if any of the above conditions are
not met.

DISCLAIMER

THE FONT SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO ANY WARRANTIES OF
MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT
OF COPYRIGHT, PATENT, TRADEMARK, OR OTHER RIGHT. IN NO EVENT SHALL THE
COPYRIGHT HOLDER BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY,
INCLUDING ANY GENERAL, SPECIAL, INDIRECT, INCIDENTAL, OR CONSEQUENTIAL
DAMAGES, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
FROM, OUT OF THE USE OR INABILITY TO USE THE FONT SOFTWARE OR FROM
OTHER DEALINGS IN THE FONT SOFTWARE.
```
