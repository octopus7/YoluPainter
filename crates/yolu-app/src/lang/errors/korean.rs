//! Korean text for structured diagnostics that predate the Fluent catalog.
//! Keep the original error values and technical identifiers in the message.
use super::*;

pub(super) fn project_note(note: &yolu_io::Note) -> String {
    use yolu_io::Note;
    match note {
        Note::ViewSlotUnreadable(_) => {
            "view.json의 머티리얼 슬롯을 읽을 수 없어 0을 사용합니다".into()
        }
        Note::Migrated { format } => {
            format!("메모리에서 형식 {format}을 형식 7의 구조로 변환했습니다")
        }
        Note::MaterialRefsMigrated { format } => {
            format!("메모리에서 형식 {format}의 머티리얼 참조를 형식 7로 변환했습니다")
        }
        Note::UnknownEntryKept(name) => format!("지원하지 않는 항목을 그대로 보존했습니다: {name}"),
        Note::SetNotConvertible { set, issue } => format!(
            "텍스처 세트 “{set}”을 변환할 수 없습니다: {}",
            issue_key(issue)
        ),
        Note::SmartResourceKept(name) => {
            format!("스마트 리소스 “{name}”을 파일로 보존했습니다 (미리보기로 펼치지 않음)")
        }
        Note::BrushKept => {
            "브러시 설정과 이미지를 그대로 보존했습니다 (검증하거나 실행하지 않음)".into()
        }
    }
}

pub(super) fn inactive_reason(reason: &InactiveReason) -> String {
    use generator::Inactive as I;
    match reason {
        InactiveReason::Generator(I::MissingMap(k)) => format!("{k:?} 맵이 없습니다"),
        InactiveReason::Generator(I::StaleMap(k)) => {
            format!("{k:?} 맵이 다른 설정으로 베이크되었습니다")
        }
        InactiveReason::Generator(I::UnverifiedMap(k)) => format!("{k:?} 맵을 검증할 수 없습니다"),
        InactiveReason::Generator(I::MapSize(k)) => {
            format!("{k:?} 맵의 크기가 텍스처 세트와 다릅니다")
        }
        InactiveReason::Generator(I::PinMismatch(k)) => {
            format!("{k:?} 맵이 고정한 베이크와 다릅니다")
        }
        InactiveReason::Generator(I::MissingFrame) => "모델의 루트 위치를 알 수 없습니다".into(),
        InactiveReason::Generator(I::EmptyBounds) => "위치 경계의 크기가 0입니다".into(),
        InactiveReason::Generator(I::NoIdColors) => "선택한 ID 색상이 없습니다".into(),
        InactiveReason::Generator(I::Anchor(anchor::Issue::NotChosen)) => {
            "선택한 앵커가 없습니다".into()
        }
        InactiveReason::Generator(I::Anchor(anchor::Issue::Missing)) => {
            "읽을 앵커가 사라졌습니다".into()
        }
        InactiveReason::Generator(I::Anchor(anchor::Issue::NotBelow)) => {
            "앵커가 해당 레이어 아래에 있지 않습니다".into()
        }
        InactiveReason::Generator(I::NoImage) => "선택한 이미지가 없습니다".into(),
        InactiveReason::Generator(I::MissingImage) => {
            "이미지가 프로젝트에 없거나 읽을 수 없습니다".into()
        }
        InactiveReason::Generator(I::NoModel) => "모델이 없습니다".into(),
        InactiveReason::Generator(I::IslandMap) => {
            "UV 아일랜드 맵이 메모리 한도를 초과합니다".into()
        }
        InactiveReason::Rejected(why) if why.is_ascii() => {
            format!("설정을 사용할 수 없습니다 ({why})")
        }
        InactiveReason::Rejected(_) => "설정을 사용할 수 없습니다".into(),
    }
}

pub(super) fn clipboard_refusal(reason: yolu_core::ClipboardRefusal) -> &'static str {
    use yolu_core::ClipboardRefusal::*;
    match reason {
        NoPixels => "이 레이어에는 픽셀이 없습니다",
        NothingToCopy { selection: true } => "선택 영역 안에 복사할 내용이 없습니다",
        NothingToCopy { selection: false } => "레이어가 비어 있습니다",
        TooLarge { .. } => "복사할 영역이 너무 큽니다",
        OperationBudget { .. } => "붙여넣을 레이어가 작업 한도를 초과합니다",
        NotPaintLayer => "채우기 레이어는 잘라낼 수 없습니다",
    }
}

pub(super) fn view_error(error: &ViewError) -> String {
    let lang = Lang::Ko;
    match error {
        ViewError::Stroking => super::super::refusals::during_stroke(lang).into(),
        ViewError::NoPoseModel => "포즈를 편집할 모델이 없습니다".into(),
        ViewError::NoPoseEdit => "포즈 편집을 시작하지 않았습니다".into(),
        ViewError::NoLinkModel => "모델보다 먼저 포즈를 받았습니다".into(),
        ViewError::NoPoseBase => "포즈를 적용할 모델이 없습니다".into(),
        ViewError::BadMeshIndex => "메시 인덱스가 정점 수를 초과합니다".into(),
        ViewError::NoTriangles => "삼각형이 없습니다".into(),
        ViewError::Cancelled => "취소됨".into(),
        ViewError::LoadStopped => "불러오기가 중단되었습니다".into(),
        ViewError::PoseGeneration {
            pose,
            model: Some(model),
        } => format!("포즈 세대 {pose}가 모델 세대 {model}과 다릅니다"),
        ViewError::PoseGeneration { pose, model: None } => {
            format!("포즈 세대 {pose}에 해당하는 모델이 없습니다")
        }
        ViewError::PoseMesh => "포즈의 메시 인덱스가 범위를 벗어났습니다".into(),
        ViewError::PoseVertices => "포즈의 정점 수가 메시와 다릅니다".into(),
        ViewError::Rig(e) => lang.rig_error(e),
        ViewError::Geometry(e) => lang.geometry_error(*e).into(),
        ViewError::Model(e) => lang.model_error(e),
    }
}

pub(super) fn rig_error(error: &RigError) -> String {
    match error {
        RigError::TooLarge { what, value, limit } => format!(
            "{} 수가 너무 많습니다 ({value}, 최대 {limit})",
            rig_what(what)
        ),
        RigError::BadParent { bone } => format!("본 {bone}의 부모가 올바르지 않습니다"),
        RigError::BadMesh { mesh } => {
            format!("메시 {mesh}의 인덱스 또는 정점 수가 올바르지 않습니다")
        }
        RigError::BadSkin { mesh } => format!("메시 {mesh}의 스킨이 올바르지 않습니다"),
        RigError::BadBlendShape { mesh, shape } => {
            format!("메시 {mesh}의 BlendShape {shape}가 올바르지 않습니다")
        }
        RigError::NonFinite { what } => format!("{}에 유한하지 않은 값이 있습니다", rig_what(what)),
        RigError::PoseMismatch => "포즈가 모델과 일치하지 않습니다".into(),
    }
}

fn rig_what(what: &str) -> &str {
    match what {
        "ボーン" => "본",
        "メッシュ" => "메시",
        "頂点" => "정점",
        "三角形" => "삼각형",
        "ウェイト" => "가중치",
        "BlendShape" => "BlendShape",
        "BlendShape の差分" => "BlendShape 변위",
        "1 つの頂点のウェイト" => "정점당 가중치",
        "ボーンの変換" => "본 변환",
        "メッシュの位置・法線・UV" => "메시의 위치·노멀·UV",
        "ポーズ" => "포즈",
        "スキンの行列・ウェイト（負のウェイトを含む）" => {
            "스킨 행렬과 가중치"
        }
        _ if what.is_ascii() => what,
        _ => "값",
    }
}

pub(super) fn model_error(error: &ModelError) -> String {
    let lang = Lang::Ko;
    match error {
        ModelError::Io(e) => lang.with_reason(lang.t("lang-cannot-read-the-file"), e),
        ModelError::Parse(e) => lang.with_reason(lang.t("lang-invalid-fbx"), e),
        ModelError::FileTooLarge { bytes, limit } => format!(
            "파일이 너무 큽니다 ({:.1} MiB, 최대 {:.0} MiB)",
            *bytes as f64 / 1048576.0,
            *limit as f64 / 1048576.0
        ),
        ModelError::NoMesh => "삼각형 메시가 없습니다".into(),
        ModelError::Cancelled => "취소됨".into(),
        ModelError::Rig(e) => lang.rig_error(e),
        ModelError::Changed => "불러온 이후 파일이 변경되었습니다".into(),
        ModelError::NoTake => "파일에 해당 테이크가 없습니다".into(),
    }
}
