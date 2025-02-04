/* automatically generated by rust-bindgen 0.59.2 */

pub const AVIF_VERSION_MAJOR: u32 = 0;
pub const AVIF_VERSION_MINOR: u32 = 10;
pub const AVIF_VERSION_PATCH: u32 = 0;
pub const AVIF_VERSION_DEVEL: u32 = 0;
pub const AVIF_VERSION: u32 = 100000;
pub const AVIF_TRUE: u32 = 1;
pub const AVIF_FALSE: u32 = 0;
pub const AVIF_DIAGNOSTICS_ERROR_BUFFER_SIZE: u32 = 256;
pub const AVIF_DEFAULT_IMAGE_SIZE_LIMIT: u32 = 268435456;
pub const AVIF_DEFAULT_IMAGE_COUNT_LIMIT: u32 = 2592000;
pub const AVIF_QUANTIZER_LOSSLESS: u32 = 0;
pub const AVIF_QUANTIZER_BEST_QUALITY: u32 = 0;
pub const AVIF_QUANTIZER_WORST_QUALITY: u32 = 63;
pub const AVIF_PLANE_COUNT_YUV: u32 = 3;
pub const AVIF_SPEED_DEFAULT: i32 = -1;
pub const AVIF_SPEED_SLOWEST: u32 = 0;
pub const AVIF_SPEED_FASTEST: u32 = 10;
pub type __uint8_t = ::std::os::raw::c_uchar;
pub type __uint16_t = ::std::os::raw::c_ushort;
pub type __uint32_t = ::std::os::raw::c_uint;
pub type __uint64_t = ::std::os::raw::c_ulong;
pub type avifBool = ::std::os::raw::c_int;
pub const AVIF_PLANES_YUV: avifPlanesFlag = 1;
pub const AVIF_PLANES_A: avifPlanesFlag = 2;
pub const AVIF_PLANES_ALL: avifPlanesFlag = 255;
pub type avifPlanesFlag = ::std::os::raw::c_uint;
pub type avifPlanesFlags = u32;
pub const AVIF_CHAN_R: avifChannelIndex = 0;
pub const AVIF_CHAN_G: avifChannelIndex = 1;
pub const AVIF_CHAN_B: avifChannelIndex = 2;
pub const AVIF_CHAN_Y: avifChannelIndex = 0;
pub const AVIF_CHAN_U: avifChannelIndex = 1;
pub const AVIF_CHAN_V: avifChannelIndex = 2;
pub type avifChannelIndex = ::std::os::raw::c_uint;
extern "C" {
    pub fn avifVersion() -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn avifCodecVersions(outBuffer: *mut ::std::os::raw::c_char);
}
extern "C" {
    pub fn avifLibYUVVersion() -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn avifAlloc(size: usize) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn avifFree(p: *mut ::std::os::raw::c_void);
}
pub const AVIF_RESULT_OK: avifResult = 0;
pub const AVIF_RESULT_UNKNOWN_ERROR: avifResult = 1;
pub const AVIF_RESULT_INVALID_FTYP: avifResult = 2;
pub const AVIF_RESULT_NO_CONTENT: avifResult = 3;
pub const AVIF_RESULT_NO_YUV_FORMAT_SELECTED: avifResult = 4;
pub const AVIF_RESULT_REFORMAT_FAILED: avifResult = 5;
pub const AVIF_RESULT_UNSUPPORTED_DEPTH: avifResult = 6;
pub const AVIF_RESULT_ENCODE_COLOR_FAILED: avifResult = 7;
pub const AVIF_RESULT_ENCODE_ALPHA_FAILED: avifResult = 8;
pub const AVIF_RESULT_BMFF_PARSE_FAILED: avifResult = 9;
pub const AVIF_RESULT_NO_AV1_ITEMS_FOUND: avifResult = 10;
pub const AVIF_RESULT_DECODE_COLOR_FAILED: avifResult = 11;
pub const AVIF_RESULT_DECODE_ALPHA_FAILED: avifResult = 12;
pub const AVIF_RESULT_COLOR_ALPHA_SIZE_MISMATCH: avifResult = 13;
pub const AVIF_RESULT_ISPE_SIZE_MISMATCH: avifResult = 14;
pub const AVIF_RESULT_NO_CODEC_AVAILABLE: avifResult = 15;
pub const AVIF_RESULT_NO_IMAGES_REMAINING: avifResult = 16;
pub const AVIF_RESULT_INVALID_EXIF_PAYLOAD: avifResult = 17;
pub const AVIF_RESULT_INVALID_IMAGE_GRID: avifResult = 18;
pub const AVIF_RESULT_INVALID_CODEC_SPECIFIC_OPTION: avifResult = 19;
pub const AVIF_RESULT_TRUNCATED_DATA: avifResult = 20;
pub const AVIF_RESULT_IO_NOT_SET: avifResult = 21;
pub const AVIF_RESULT_IO_ERROR: avifResult = 22;
pub const AVIF_RESULT_WAITING_ON_IO: avifResult = 23;
pub const AVIF_RESULT_INVALID_ARGUMENT: avifResult = 24;
pub const AVIF_RESULT_NOT_IMPLEMENTED: avifResult = 25;
pub const AVIF_RESULT_OUT_OF_MEMORY: avifResult = 26;
pub type avifResult = ::std::os::raw::c_uint;
extern "C" {
    pub fn avifResultToString(result: avifResult) -> *const ::std::os::raw::c_char;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct avifROData {
    pub data: *const u8,
    pub size: usize,
}
impl Default for avifROData {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct avifRWData {
    pub data: *mut u8,
    pub size: usize,
}
impl Default for avifRWData {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
extern "C" {
    pub fn avifRWDataRealloc(raw: *mut avifRWData, newSize: usize);
}
extern "C" {
    pub fn avifRWDataSet(raw: *mut avifRWData, data: *const u8, len: usize);
}
extern "C" {
    pub fn avifRWDataFree(raw: *mut avifRWData);
}
pub const AVIF_PIXEL_FORMAT_NONE: avifPixelFormat = 0;
pub const AVIF_PIXEL_FORMAT_YUV444: avifPixelFormat = 1;
pub const AVIF_PIXEL_FORMAT_YUV422: avifPixelFormat = 2;
pub const AVIF_PIXEL_FORMAT_YUV420: avifPixelFormat = 3;
pub const AVIF_PIXEL_FORMAT_YUV400: avifPixelFormat = 4;
pub type avifPixelFormat = ::std::os::raw::c_uint;
extern "C" {
    pub fn avifPixelFormatToString(format: avifPixelFormat) -> *const ::std::os::raw::c_char;
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct avifPixelFormatInfo {
    pub monochrome: avifBool,
    pub chromaShiftX: ::std::os::raw::c_int,
    pub chromaShiftY: ::std::os::raw::c_int,
}
extern "C" {
    pub fn avifGetPixelFormatInfo(format: avifPixelFormat, info: *mut avifPixelFormatInfo);
}
pub const AVIF_CHROMA_SAMPLE_POSITION_UNKNOWN: avifChromaSamplePosition = 0;
pub const AVIF_CHROMA_SAMPLE_POSITION_VERTICAL: avifChromaSamplePosition = 1;
pub const AVIF_CHROMA_SAMPLE_POSITION_COLOCATED: avifChromaSamplePosition = 2;
pub type avifChromaSamplePosition = ::std::os::raw::c_uint;
pub const AVIF_RANGE_LIMITED: avifRange = 0;
pub const AVIF_RANGE_FULL: avifRange = 1;
pub type avifRange = ::std::os::raw::c_uint;
pub const AVIF_COLOR_PRIMARIES_UNKNOWN: ::std::os::raw::c_uint = 0;
pub const AVIF_COLOR_PRIMARIES_BT709: ::std::os::raw::c_uint = 1;
pub const AVIF_COLOR_PRIMARIES_IEC61966_2_4: ::std::os::raw::c_uint = 1;
pub const AVIF_COLOR_PRIMARIES_UNSPECIFIED: ::std::os::raw::c_uint = 2;
pub const AVIF_COLOR_PRIMARIES_BT470M: ::std::os::raw::c_uint = 4;
pub const AVIF_COLOR_PRIMARIES_BT470BG: ::std::os::raw::c_uint = 5;
pub const AVIF_COLOR_PRIMARIES_BT601: ::std::os::raw::c_uint = 6;
pub const AVIF_COLOR_PRIMARIES_SMPTE240: ::std::os::raw::c_uint = 7;
pub const AVIF_COLOR_PRIMARIES_GENERIC_FILM: ::std::os::raw::c_uint = 8;
pub const AVIF_COLOR_PRIMARIES_BT2020: ::std::os::raw::c_uint = 9;
pub const AVIF_COLOR_PRIMARIES_XYZ: ::std::os::raw::c_uint = 10;
pub const AVIF_COLOR_PRIMARIES_SMPTE431: ::std::os::raw::c_uint = 11;
pub const AVIF_COLOR_PRIMARIES_SMPTE432: ::std::os::raw::c_uint = 12;
pub const AVIF_COLOR_PRIMARIES_EBU3213: ::std::os::raw::c_uint = 22;
pub type _bindgen_ty_1 = ::std::os::raw::c_uint;
pub type avifColorPrimaries = u16;
extern "C" {
    pub fn avifColorPrimariesGetValues(acp: avifColorPrimaries, outPrimaries: *mut f32);
}
extern "C" {
    pub fn avifColorPrimariesFind(
        inPrimaries: *const f32,
        outName: *mut *const ::std::os::raw::c_char,
    ) -> avifColorPrimaries;
}
pub const AVIF_TRANSFER_CHARACTERISTICS_UNKNOWN: ::std::os::raw::c_uint = 0;
pub const AVIF_TRANSFER_CHARACTERISTICS_BT709: ::std::os::raw::c_uint = 1;
pub const AVIF_TRANSFER_CHARACTERISTICS_UNSPECIFIED: ::std::os::raw::c_uint = 2;
pub const AVIF_TRANSFER_CHARACTERISTICS_BT470M: ::std::os::raw::c_uint = 4;
pub const AVIF_TRANSFER_CHARACTERISTICS_BT470BG: ::std::os::raw::c_uint = 5;
pub const AVIF_TRANSFER_CHARACTERISTICS_BT601: ::std::os::raw::c_uint = 6;
pub const AVIF_TRANSFER_CHARACTERISTICS_SMPTE240: ::std::os::raw::c_uint = 7;
pub const AVIF_TRANSFER_CHARACTERISTICS_LINEAR: ::std::os::raw::c_uint = 8;
pub const AVIF_TRANSFER_CHARACTERISTICS_LOG100: ::std::os::raw::c_uint = 9;
pub const AVIF_TRANSFER_CHARACTERISTICS_LOG100_SQRT10: ::std::os::raw::c_uint = 10;
pub const AVIF_TRANSFER_CHARACTERISTICS_IEC61966: ::std::os::raw::c_uint = 11;
pub const AVIF_TRANSFER_CHARACTERISTICS_BT1361: ::std::os::raw::c_uint = 12;
pub const AVIF_TRANSFER_CHARACTERISTICS_SRGB: ::std::os::raw::c_uint = 13;
pub const AVIF_TRANSFER_CHARACTERISTICS_BT2020_10BIT: ::std::os::raw::c_uint = 14;
pub const AVIF_TRANSFER_CHARACTERISTICS_BT2020_12BIT: ::std::os::raw::c_uint = 15;
pub const AVIF_TRANSFER_CHARACTERISTICS_SMPTE2084: ::std::os::raw::c_uint = 16;
pub const AVIF_TRANSFER_CHARACTERISTICS_SMPTE428: ::std::os::raw::c_uint = 17;
pub const AVIF_TRANSFER_CHARACTERISTICS_HLG: ::std::os::raw::c_uint = 18;
pub type _bindgen_ty_2 = ::std::os::raw::c_uint;
pub type avifTransferCharacteristics = u16;
pub const AVIF_MATRIX_COEFFICIENTS_IDENTITY: ::std::os::raw::c_uint = 0;
pub const AVIF_MATRIX_COEFFICIENTS_BT709: ::std::os::raw::c_uint = 1;
pub const AVIF_MATRIX_COEFFICIENTS_UNSPECIFIED: ::std::os::raw::c_uint = 2;
pub const AVIF_MATRIX_COEFFICIENTS_FCC: ::std::os::raw::c_uint = 4;
pub const AVIF_MATRIX_COEFFICIENTS_BT470BG: ::std::os::raw::c_uint = 5;
pub const AVIF_MATRIX_COEFFICIENTS_BT601: ::std::os::raw::c_uint = 6;
pub const AVIF_MATRIX_COEFFICIENTS_SMPTE240: ::std::os::raw::c_uint = 7;
pub const AVIF_MATRIX_COEFFICIENTS_YCGCO: ::std::os::raw::c_uint = 8;
pub const AVIF_MATRIX_COEFFICIENTS_BT2020_NCL: ::std::os::raw::c_uint = 9;
pub const AVIF_MATRIX_COEFFICIENTS_BT2020_CL: ::std::os::raw::c_uint = 10;
pub const AVIF_MATRIX_COEFFICIENTS_SMPTE2085: ::std::os::raw::c_uint = 11;
pub const AVIF_MATRIX_COEFFICIENTS_CHROMA_DERIVED_NCL: ::std::os::raw::c_uint = 12;
pub const AVIF_MATRIX_COEFFICIENTS_CHROMA_DERIVED_CL: ::std::os::raw::c_uint = 13;
pub const AVIF_MATRIX_COEFFICIENTS_ICTCP: ::std::os::raw::c_uint = 14;
pub type _bindgen_ty_3 = ::std::os::raw::c_uint;
pub type avifMatrixCoefficients = u16;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct avifDiagnostics {
    pub error: [::std::os::raw::c_char; 256usize],
}
impl Default for avifDiagnostics {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
extern "C" {
    pub fn avifDiagnosticsClearError(diag: *mut avifDiagnostics);
}
pub const AVIF_TRANSFORM_NONE: avifTransformFlag = 0;
pub const AVIF_TRANSFORM_PASP: avifTransformFlag = 1;
pub const AVIF_TRANSFORM_CLAP: avifTransformFlag = 2;
pub const AVIF_TRANSFORM_IROT: avifTransformFlag = 4;
pub const AVIF_TRANSFORM_IMIR: avifTransformFlag = 8;
pub type avifTransformFlag = ::std::os::raw::c_uint;
pub type avifTransformFlags = u32;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct avifPixelAspectRatioBox {
    pub hSpacing: u32,
    pub vSpacing: u32,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct avifCleanApertureBox {
    pub widthN: u32,
    pub widthD: u32,
    pub heightN: u32,
    pub heightD: u32,
    pub horizOffN: u32,
    pub horizOffD: u32,
    pub vertOffN: u32,
    pub vertOffD: u32,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct avifImageRotation {
    pub angle: u8,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct avifImageMirror {
    pub mode: u8,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct avifCropRect {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}
extern "C" {
    pub fn avifCropRectConvertCleanApertureBox(
        cropRect: *mut avifCropRect,
        clap: *const avifCleanApertureBox,
        imageW: u32,
        imageH: u32,
        yuvFormat: avifPixelFormat,
        diag: *mut avifDiagnostics,
    ) -> avifBool;
}
extern "C" {
    pub fn avifCleanApertureBoxConvertCropRect(
        clap: *mut avifCleanApertureBox,
        cropRect: *const avifCropRect,
        imageW: u32,
        imageH: u32,
        yuvFormat: avifPixelFormat,
        diag: *mut avifDiagnostics,
    ) -> avifBool;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct avifImage {
    pub width: u32,
    pub height: u32,
    pub depth: u32,
    pub yuvFormat: avifPixelFormat,
    pub yuvRange: avifRange,
    pub yuvChromaSamplePosition: avifChromaSamplePosition,
    pub yuvPlanes: [*mut u8; 3usize],
    pub yuvRowBytes: [u32; 3usize],
    pub imageOwnsYUVPlanes: avifBool,
    pub alphaRange: avifRange,
    pub alphaPlane: *mut u8,
    pub alphaRowBytes: u32,
    pub imageOwnsAlphaPlane: avifBool,
    pub alphaPremultiplied: avifBool,
    pub icc: avifRWData,
    pub colorPrimaries: avifColorPrimaries,
    pub transferCharacteristics: avifTransferCharacteristics,
    pub matrixCoefficients: avifMatrixCoefficients,
    pub transformFlags: avifTransformFlags,
    pub pasp: avifPixelAspectRatioBox,
    pub clap: avifCleanApertureBox,
    pub irot: avifImageRotation,
    pub imir: avifImageMirror,
    pub exif: avifRWData,
    pub xmp: avifRWData,
}
impl Default for avifImage {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
extern "C" {
    pub fn avifImageCreate(
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        depth: ::std::os::raw::c_int,
        yuvFormat: avifPixelFormat,
    ) -> *mut avifImage;
}
extern "C" {
    pub fn avifImageCreateEmpty() -> *mut avifImage;
}
extern "C" {
    pub fn avifImageCopy(
        dstImage: *mut avifImage,
        srcImage: *const avifImage,
        planes: avifPlanesFlags,
    );
}
extern "C" {
    pub fn avifImageSetViewRect(
        dstImage: *mut avifImage,
        srcImage: *const avifImage,
        rect: *const avifCropRect,
    ) -> avifResult;
}
extern "C" {
    pub fn avifImageDestroy(image: *mut avifImage);
}
extern "C" {
    pub fn avifImageSetProfileICC(image: *mut avifImage, icc: *const u8, iccSize: usize);
}
extern "C" {
    pub fn avifImageSetMetadataExif(image: *mut avifImage, exif: *const u8, exifSize: usize);
}
extern "C" {
    pub fn avifImageSetMetadataXMP(image: *mut avifImage, xmp: *const u8, xmpSize: usize);
}
extern "C" {
    pub fn avifImageAllocatePlanes(image: *mut avifImage, planes: avifPlanesFlags);
}
extern "C" {
    pub fn avifImageFreePlanes(image: *mut avifImage, planes: avifPlanesFlags);
}
extern "C" {
    pub fn avifImageStealPlanes(
        dstImage: *mut avifImage,
        srcImage: *mut avifImage,
        planes: avifPlanesFlags,
    );
}
pub const AVIF_RGB_FORMAT_RGB: avifRGBFormat = 0;
pub const AVIF_RGB_FORMAT_RGBA: avifRGBFormat = 1;
pub const AVIF_RGB_FORMAT_ARGB: avifRGBFormat = 2;
pub const AVIF_RGB_FORMAT_BGR: avifRGBFormat = 3;
pub const AVIF_RGB_FORMAT_BGRA: avifRGBFormat = 4;
pub const AVIF_RGB_FORMAT_ABGR: avifRGBFormat = 5;
pub type avifRGBFormat = ::std::os::raw::c_uint;
extern "C" {
    pub fn avifRGBFormatChannelCount(format: avifRGBFormat) -> u32;
}
extern "C" {
    pub fn avifRGBFormatHasAlpha(format: avifRGBFormat) -> avifBool;
}
pub const AVIF_CHROMA_UPSAMPLING_AUTOMATIC: avifChromaUpsampling = 0;
pub const AVIF_CHROMA_UPSAMPLING_FASTEST: avifChromaUpsampling = 1;
pub const AVIF_CHROMA_UPSAMPLING_BEST_QUALITY: avifChromaUpsampling = 2;
pub const AVIF_CHROMA_UPSAMPLING_NEAREST: avifChromaUpsampling = 3;
pub const AVIF_CHROMA_UPSAMPLING_BILINEAR: avifChromaUpsampling = 4;
pub type avifChromaUpsampling = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct avifRGBImage {
    pub width: u32,
    pub height: u32,
    pub depth: u32,
    pub format: avifRGBFormat,
    pub chromaUpsampling: avifChromaUpsampling,
    pub ignoreAlpha: avifBool,
    pub alphaPremultiplied: avifBool,
    pub isFloat: avifBool,
    pub pixels: *mut u8,
    pub rowBytes: u32,
}
impl Default for avifRGBImage {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
extern "C" {
    pub fn avifRGBImageSetDefaults(rgb: *mut avifRGBImage, image: *const avifImage);
}
extern "C" {
    pub fn avifRGBImagePixelSize(rgb: *const avifRGBImage) -> u32;
}
extern "C" {
    pub fn avifRGBImageAllocatePixels(rgb: *mut avifRGBImage);
}
extern "C" {
    pub fn avifRGBImageFreePixels(rgb: *mut avifRGBImage);
}
extern "C" {
    pub fn avifImageRGBToYUV(image: *mut avifImage, rgb: *const avifRGBImage) -> avifResult;
}
extern "C" {
    pub fn avifImageYUVToRGB(image: *const avifImage, rgb: *mut avifRGBImage) -> avifResult;
}
extern "C" {
    pub fn avifRGBImagePremultiplyAlpha(rgb: *mut avifRGBImage) -> avifResult;
}
extern "C" {
    pub fn avifRGBImageUnpremultiplyAlpha(rgb: *mut avifRGBImage) -> avifResult;
}
extern "C" {
    pub fn avifFullToLimitedY(
        depth: ::std::os::raw::c_int,
        v: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn avifFullToLimitedUV(
        depth: ::std::os::raw::c_int,
        v: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn avifLimitedToFullY(
        depth: ::std::os::raw::c_int,
        v: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn avifLimitedToFullUV(
        depth: ::std::os::raw::c_int,
        v: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
pub const AVIF_CODEC_CHOICE_AUTO: avifCodecChoice = 0;
pub const AVIF_CODEC_CHOICE_AOM: avifCodecChoice = 1;
pub const AVIF_CODEC_CHOICE_DAV1D: avifCodecChoice = 2;
pub const AVIF_CODEC_CHOICE_LIBGAV1: avifCodecChoice = 3;
pub const AVIF_CODEC_CHOICE_RAV1E: avifCodecChoice = 4;
pub const AVIF_CODEC_CHOICE_SVT: avifCodecChoice = 5;
pub type avifCodecChoice = ::std::os::raw::c_uint;
pub const AVIF_CODEC_FLAG_CAN_DECODE: avifCodecFlag = 1;
pub const AVIF_CODEC_FLAG_CAN_ENCODE: avifCodecFlag = 2;
pub type avifCodecFlag = ::std::os::raw::c_uint;
pub type avifCodecFlags = u32;
extern "C" {
    pub fn avifCodecName(
        choice: avifCodecChoice,
        requiredFlags: avifCodecFlags,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn avifCodecChoiceFromName(name: *const ::std::os::raw::c_char) -> avifCodecChoice;
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct avifCodecConfigurationBox {
    pub seqProfile: u8,
    pub seqLevelIdx0: u8,
    pub seqTier0: u8,
    pub highBitdepth: u8,
    pub twelveBit: u8,
    pub monochrome: u8,
    pub chromaSubsamplingX: u8,
    pub chromaSubsamplingY: u8,
    pub chromaSamplePosition: u8,
}
pub type avifIODestroyFunc = ::std::option::Option<unsafe extern "C" fn(io: *mut avifIO)>;
pub type avifIOReadFunc = ::std::option::Option<
    unsafe extern "C" fn(
        io: *mut avifIO,
        readFlags: u32,
        offset: u64,
        size: usize,
        out: *mut avifROData,
    ) -> avifResult,
>;
pub type avifIOWriteFunc = ::std::option::Option<
    unsafe extern "C" fn(
        io: *mut avifIO,
        writeFlags: u32,
        offset: u64,
        data: *const u8,
        size: usize,
    ) -> avifResult,
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct avifIO {
    pub destroy: avifIODestroyFunc,
    pub read: avifIOReadFunc,
    pub write: avifIOWriteFunc,
    pub sizeHint: u64,
    pub persistent: avifBool,
    pub data: *mut ::std::os::raw::c_void,
}
impl Default for avifIO {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
extern "C" {
    pub fn avifIOCreateMemoryReader(data: *const u8, size: usize) -> *mut avifIO;
}
extern "C" {
    pub fn avifIOCreateFileReader(filename: *const ::std::os::raw::c_char) -> *mut avifIO;
}
extern "C" {
    pub fn avifIODestroy(io: *mut avifIO);
}
pub const AVIF_STRICT_DISABLED: avifStrictFlag = 0;
pub const AVIF_STRICT_PIXI_REQUIRED: avifStrictFlag = 1;
pub const AVIF_STRICT_CLAP_VALID: avifStrictFlag = 2;
pub const AVIF_STRICT_ALPHA_ISPE_REQUIRED: avifStrictFlag = 4;
pub const AVIF_STRICT_ENABLED: avifStrictFlag = 7;
pub type avifStrictFlag = ::std::os::raw::c_uint;
pub type avifStrictFlags = u32;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct avifIOStats {
    pub colorOBUSize: usize,
    pub alphaOBUSize: usize,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct avifDecoderData {
    _unused: [u8; 0],
}
pub const AVIF_DECODER_SOURCE_AUTO: avifDecoderSource = 0;
pub const AVIF_DECODER_SOURCE_PRIMARY_ITEM: avifDecoderSource = 1;
pub const AVIF_DECODER_SOURCE_TRACKS: avifDecoderSource = 2;
pub type avifDecoderSource = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct avifImageTiming {
    pub timescale: u64,
    pub pts: f64,
    pub ptsInTimescales: u64,
    pub duration: f64,
    pub durationInTimescales: u64,
}
pub const AVIF_PROGRESSIVE_STATE_UNAVAILABLE: avifProgressiveState = 0;
pub const AVIF_PROGRESSIVE_STATE_AVAILABLE: avifProgressiveState = 1;
pub const AVIF_PROGRESSIVE_STATE_ACTIVE: avifProgressiveState = 2;
pub type avifProgressiveState = ::std::os::raw::c_uint;
extern "C" {
    pub fn avifProgressiveStateToString(
        progressiveState: avifProgressiveState,
    ) -> *const ::std::os::raw::c_char;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct avifDecoder {
    pub codecChoice: avifCodecChoice,
    pub maxThreads: ::std::os::raw::c_int,
    pub requestedSource: avifDecoderSource,
    pub allowProgressive: avifBool,
    pub allowIncremental: avifBool,
    pub ignoreExif: avifBool,
    pub ignoreXMP: avifBool,
    pub imageSizeLimit: u32,
    pub imageCountLimit: u32,
    pub strictFlags: avifStrictFlags,
    pub image: *mut avifImage,
    pub imageIndex: ::std::os::raw::c_int,
    pub imageCount: ::std::os::raw::c_int,
    pub progressiveState: avifProgressiveState,
    pub imageTiming: avifImageTiming,
    pub timescale: u64,
    pub duration: f64,
    pub durationInTimescales: u64,
    pub alphaPresent: avifBool,
    pub ioStats: avifIOStats,
    pub diag: avifDiagnostics,
    pub io: *mut avifIO,
    pub data: *mut avifDecoderData,
}
impl Default for avifDecoder {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
extern "C" {
    pub fn avifDecoderCreate() -> *mut avifDecoder;
}
extern "C" {
    pub fn avifDecoderDestroy(decoder: *mut avifDecoder);
}
extern "C" {
    pub fn avifDecoderRead(decoder: *mut avifDecoder, image: *mut avifImage) -> avifResult;
}
extern "C" {
    pub fn avifDecoderReadMemory(
        decoder: *mut avifDecoder,
        image: *mut avifImage,
        data: *const u8,
        size: usize,
    ) -> avifResult;
}
extern "C" {
    pub fn avifDecoderReadFile(
        decoder: *mut avifDecoder,
        image: *mut avifImage,
        filename: *const ::std::os::raw::c_char,
    ) -> avifResult;
}
extern "C" {
    pub fn avifDecoderSetSource(decoder: *mut avifDecoder, source: avifDecoderSource)
        -> avifResult;
}
extern "C" {
    pub fn avifDecoderSetIO(decoder: *mut avifDecoder, io: *mut avifIO);
}
extern "C" {
    pub fn avifDecoderSetIOMemory(
        decoder: *mut avifDecoder,
        data: *const u8,
        size: usize,
    ) -> avifResult;
}
extern "C" {
    pub fn avifDecoderSetIOFile(
        decoder: *mut avifDecoder,
        filename: *const ::std::os::raw::c_char,
    ) -> avifResult;
}
extern "C" {
    pub fn avifDecoderParse(decoder: *mut avifDecoder) -> avifResult;
}
extern "C" {
    pub fn avifDecoderNextImage(decoder: *mut avifDecoder) -> avifResult;
}
extern "C" {
    pub fn avifDecoderNthImage(decoder: *mut avifDecoder, frameIndex: u32) -> avifResult;
}
extern "C" {
    pub fn avifDecoderReset(decoder: *mut avifDecoder) -> avifResult;
}
extern "C" {
    pub fn avifDecoderIsKeyframe(decoder: *const avifDecoder, frameIndex: u32) -> avifBool;
}
extern "C" {
    pub fn avifDecoderNearestKeyframe(decoder: *const avifDecoder, frameIndex: u32) -> u32;
}
extern "C" {
    pub fn avifDecoderNthImageTiming(
        decoder: *const avifDecoder,
        frameIndex: u32,
        outTiming: *mut avifImageTiming,
    ) -> avifResult;
}
extern "C" {
    pub fn avifDecoderDecodedRowCount(decoder: *const avifDecoder) -> u32;
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct avifExtent {
    pub offset: u64,
    pub size: usize,
}
extern "C" {
    pub fn avifDecoderNthImageMaxExtent(
        decoder: *const avifDecoder,
        frameIndex: u32,
        outExtent: *mut avifExtent,
    ) -> avifResult;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct avifEncoderData {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct avifCodecSpecificOptions {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct avifEncoder {
    pub codecChoice: avifCodecChoice,
    pub maxThreads: ::std::os::raw::c_int,
    pub minQuantizer: ::std::os::raw::c_int,
    pub maxQuantizer: ::std::os::raw::c_int,
    pub minQuantizerAlpha: ::std::os::raw::c_int,
    pub maxQuantizerAlpha: ::std::os::raw::c_int,
    pub tileRowsLog2: ::std::os::raw::c_int,
    pub tileColsLog2: ::std::os::raw::c_int,
    pub speed: ::std::os::raw::c_int,
    pub keyframeInterval: ::std::os::raw::c_int,
    pub timescale: u64,
    pub ioStats: avifIOStats,
    pub diag: avifDiagnostics,
    pub data: *mut avifEncoderData,
    pub csOptions: *mut avifCodecSpecificOptions,
}
impl Default for avifEncoder {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
extern "C" {
    pub fn avifEncoderCreate() -> *mut avifEncoder;
}
extern "C" {
    pub fn avifEncoderWrite(
        encoder: *mut avifEncoder,
        image: *const avifImage,
        output: *mut avifRWData,
    ) -> avifResult;
}
extern "C" {
    pub fn avifEncoderDestroy(encoder: *mut avifEncoder);
}
pub const AVIF_ADD_IMAGE_FLAG_NONE: avifAddImageFlag = 0;
pub const AVIF_ADD_IMAGE_FLAG_FORCE_KEYFRAME: avifAddImageFlag = 1;
pub const AVIF_ADD_IMAGE_FLAG_SINGLE: avifAddImageFlag = 2;
pub type avifAddImageFlag = ::std::os::raw::c_uint;
pub type avifAddImageFlags = u32;
extern "C" {
    pub fn avifEncoderAddImage(
        encoder: *mut avifEncoder,
        image: *const avifImage,
        durationInTimescales: u64,
        addImageFlags: avifAddImageFlags,
    ) -> avifResult;
}
extern "C" {
    pub fn avifEncoderAddImageGrid(
        encoder: *mut avifEncoder,
        gridCols: u32,
        gridRows: u32,
        cellImages: *const *const avifImage,
        addImageFlags: avifAddImageFlags,
    ) -> avifResult;
}
extern "C" {
    pub fn avifEncoderFinish(encoder: *mut avifEncoder, output: *mut avifRWData) -> avifResult;
}
extern "C" {
    pub fn avifEncoderSetCodecSpecificOption(
        encoder: *mut avifEncoder,
        key: *const ::std::os::raw::c_char,
        value: *const ::std::os::raw::c_char,
    );
}
extern "C" {
    pub fn avifImageUsesU16(image: *const avifImage) -> avifBool;
}
extern "C" {
    pub fn avifPeekCompatibleFileType(input: *const avifROData) -> avifBool;
}
