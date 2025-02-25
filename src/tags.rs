// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright © 2021 Adrian <adrian.eddy at gmail>

declare_groups! {
    Default,
    Exposure,
    Autofocus,
    Colors,
    Lens,
    GPS,
    Gyroscope,
    Accelerometer,
    Magnetometer,
    Quaternion,
    Position3D,
    IBIS,
    LensOSS,
    GravityVector,
    CameraOrientation,
    ImageOrientation,
}

declare_ids! {
    Metadata,
    IrisFStop,
    IrisTStop,
    FocusDistance,
    MacroEnabled,
    LensZoom35mm,
    LensZoomNative,
    OpticalZoomPercent,
    LensAttributes,
    IrisRingPosition,
    FocusRingPosition,
    ZoomRingPosition,
    ColorPrimaries,
    CodingEquation,
    CaptureGammaEquation,
    AutoExposureMode,
    AutoFocusMode,
    ColorCorrectionSetting,
    NDFilterSetting,
    SensorWidth,
    SensorHeight,
    FrameRate,
    SensorReadoutMode,
    ShutterAngle,
    ShutterSpeed,
    ISOValue,
    AutoWBMode,
    WhiteBalance,
    MasterBlackLevel,
    KneePoint,
    KneeSlope,
    LuminanceDynamicRange,
    CameraAttributes,
    ColorMatrix,
    GroupIdentifier,
    StabilizationEnabled,
    CaptureTimestamp,

    Name,
    Enabled,
    Data,
    Data2,
    Unit,
    Matrix,
    Temperature,
    Scale,
    Frequency,
    TimestampMs,
    TimestampUs,
    Offset,
    Count,
    Orientation,
    OrientationIn,
    OrientationOut,
}

declare_types! {
     u8: u8,   i8: i8,
    u16: u16, i16: i16,
    u32: u32, i32: i32,
    u64: u64, i64: i64,
    f32: f32, f64: f64,
    String: String,
    bool:   bool,
    u32x2:  (u32, u32),
    Uuid:   (u32, u32, u32, u32),
    f64x3:  (f64, f64, f64),

    GpsData: GpsData,

    Vec_Quaternioni16: Vec<Quaternion<i16>>,
    Vec_TimeScalar_f64: Vec<TimeScalar<f64>>,
    Vec_TimeScalar_i64: Vec<TimeScalar<i64>>,
    Vec_GpsData: Vec<GpsData>,

    Json:                serde_json::Value,
    Vec_Json:            Vec<serde_json::Value>,
    Vec_TimeScalar_Json: Vec<TimeScalar<serde_json::Value>>,

    Vector3_i8: Vector3<i8>,
    Vector3_i16: Vector3<i16>,
    Vector3_i32: Vector3<i32>,

    Vec_String: Vec<String>,

    Vec_i8:  Vec<i8>,  Vec_Vec_i8:  Vec<Vec<i8>>,  Vec_Vector3_i8:  Vec<Vector3<i8>>,  Vec_TimeVector3_i8:  Vec<TimeVector3<i8>>,
    Vec_u8:  Vec<u8>,  Vec_Vec_u8:  Vec<Vec<u8>>,  Vec_Vector3_u8:  Vec<Vector3<u8>>,  Vec_TimeVector3_u8:  Vec<TimeVector3<u8>>,
    Vec_i16: Vec<i16>, Vec_Vec_i16: Vec<Vec<i16>>, Vec_Vector3_i16: Vec<Vector3<i16>>, Vec_TimeVector3_i16: Vec<TimeVector3<i16>>,
    Vec_u16: Vec<u16>, Vec_Vec_u16: Vec<Vec<u16>>, Vec_Vector3_u16: Vec<Vector3<u16>>, Vec_TimeVector3_u16: Vec<TimeVector3<u16>>,
    Vec_i32: Vec<i32>, Vec_Vec_i32: Vec<Vec<i32>>, Vec_Vector3_i32: Vec<Vector3<i32>>, Vec_TimeVector3_i32: Vec<TimeVector3<i32>>,
    Vec_u32: Vec<u32>, Vec_Vec_u32: Vec<Vec<u32>>, Vec_Vector3_u32: Vec<Vector3<u32>>, Vec_TimeVector3_u32: Vec<TimeVector3<u32>>,
    Vec_f32: Vec<f32>, Vec_Vec_f32: Vec<Vec<f32>>, Vec_Vector3_f32: Vec<Vector3<f32>>, Vec_TimeVector3_f32: Vec<TimeVector3<f32>>,
    Vec_f64: Vec<f64>, Vec_Vec_f64: Vec<Vec<f64>>, Vec_Vector3_f64: Vec<Vector3<f64>>, Vec_TimeVector3_f64: Vec<TimeVector3<f64>>,
    Vec_i64: Vec<i64>, Vec_Vec_i64: Vec<Vec<i64>>, Vec_Vector3_i64: Vec<Vector3<i64>>, Vec_TimeVector3_i64: Vec<TimeVector3<i64>>,
    Vec_u64: Vec<u64>, Vec_Vec_u64: Vec<Vec<u64>>, Vec_Vector3_u64: Vec<Vector3<u64>>, Vec_TimeVector3_u64: Vec<TimeVector3<u64>>,

    Vec_TimeVector3_i64f64: Vec<TimeVector3<i64, f64>>,
    Vec_TimeQuaternion_f64: Vec<TimeQuaternion<f64>>,

    Vec_TimeArray8_f64: Vec<TimeArray8<f64>>,
    Vec_TimeArray4_f64: Vec<TimeArray4<f64>>,
    Vec_TimeArray2_f64: Vec<TimeArray2<f64>>,
}
