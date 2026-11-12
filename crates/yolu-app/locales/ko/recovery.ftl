# 화면 텍스트 (recovery). 툴팁 키는 -tip으로 끝납니다.

recovery-cannot-save-the-recovery-settings = 복구 설정을 저장할 수 없습니다.
recovery-cannot-open-the-recovery = 복구본을 열 수 없습니다
recovery-automatic = 자동
recovery-low = 낮음
recovery-standard = 표준
recovery-high = 높음
recovery-custom = 사용자 지정
recovery-untitled-recovered = 제목 없음(복구됨)
recovery-internal-error-while-writing = 저장 중 내부 오류가 발생했습니다
recovery-not-a-generation-in-the = 복구 폴더에 있는 복구본이 아닙니다
recovery-low-disk-space = 디스크 공간이 부족합니다
recovery-recovery-checkpoint-skipped = 복구 지점 생성을 건너뛰었습니다
recovery-recovery-checkpoint-failed = 복구 지점 생성에 실패했습니다
recovery-recovery-checkpoints-are-working-again = 복구 지점 생성이 다시 정상 작동합니다
recovery-recovery-setting-is-out-of = 복구 설정 { $key }={ $value }이(가) 범위({ $range }–{ $range_2 })를 벗어나 기본값을 사용합니다
recovery-recovery-folder-is-not-absolute = 복구 폴더 { $dir }이(가) 절대 경로가 아니어서 기본값을 사용합니다
recovery-generations-are-not-trimmed-because = 복구 설정을 읽을 수 없어 오래된 복구본을 정리하지 않습니다({ $reason })
recovery-cannot-settle-the-previous-recovery = 이전 복구 표시를 정리할 수 없습니다
recovery-recovery-is-unavailable = 복구를 사용할 수 없습니다
recovery-in-use = { $used } 사용 중
recovery-disk-space-used-by-recovery-tip = 복구에 사용 중인 디스크 공간입니다
recovery-this-session-tip = 현재 세션
recovery-crashed-sessions-tip = 비정상 종료된 세션
recovery-closed-sessions-tip = 종료된 세션
recovery-other-windows-tip = 다른 창
recovery-limit-tip = 한도
recovery-free-on-disk-tip = 디스크 여유 공간
recovery-just-now = 방금
recovery-s-ago = { $s }초 전
recovery-min-ago = { $s }분 전
recovery-h-ago = { $s }시간 전
recovery-d-ago = { $s }일 전
recovery-time-from-a-change-to-tip = 변경 후 복구 지점을 생성하기까지의 시간입니다
recovery-how-much-disk-space-the-tip = 복구본이 사용할 수 있는 디스크 공간입니다. 초과하면 오래된 복구본부터 삭제합니다(현재 세션과 각 비정상 종료 세션의 최신 복구본은 유지). 자동은 2 GB와 복구에 사용할 수 있는 디스크 공간의 10% 중 작은 값을 사용합니다. 디스크가 거의 가득 차면 한도와 관계없이 복구 지점 생성을 건너뜁니다
recovery-number-of-generations-to-keep-tip = 유지할 복구본 수입니다(오래된 복구본은 정리됩니다)
recovery-unreadable = 읽을 수 없음
recovery-untitled = 제목 없음
recovery-recovery = 복구
recovery-close-window = 창 닫기
recovery-min = { $n }분
recovery-s = { $n }초
recovery-checkpoint-interval = 복구 지점 생성 간격
recovery-generations-kept = 유지할 복구본 수
recovery-disk-space = 디스크 공간
recovery-details = 상세 정보
recovery-the-disk-space-the-recovery-tip = 복구본이 사용할 수 있는 디스크 공간입니다(1 GB = 1024 MB). 슬라이더를 놓으면 사용자 지정 용량으로 전환되며, 한도를 초과하면 오래된 복구본부터 삭제합니다
recovery-limit = 한도
recovery-crash-report = 충돌 보고서
recovery-open = 열기
recovery-opens-the-selected-generation-as = 선택한 복구본을 “제목 없음(복구됨)”으로 엽니다. 원본 파일에는 저장하지 않습니다
recovery-discard = 삭제
recovery-deletes-the-selected-generation-saved = 선택한 복구본을 삭제합니다. 저장된 .ylp 파일은 유지됩니다
recovery-close = 닫기
recovery-discard-generation = 복구본 삭제
recovery-cancel = 취소
recovery-deletes-this-generation-permanently-saved-tip = 이 복구본을 영구적으로 삭제합니다. 저장된 .ylp 파일은 유지됩니다
recovery-sets-count = { $count } { $count ->
        [one] 세트
       *[other] 세트
    }
recovery-damaged-generation-cannot-be-read = { $reason ->
       *[other] 손상된 복구본(읽을 수 없음)
    }
