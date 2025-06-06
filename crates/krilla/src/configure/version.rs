use once_cell::sync::Lazy;
use pdf_writer::Pdf;
use xmp_writer::XmpWriter;

use crate::graphics::icc::{ICCMetadata, ICCProfile};
#[cfg(feature = "raster-images")]
use crate::image::BitsPerComponent;

/// The version of a PDF document.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum PdfVersion {
    /// PDF 1.4.
    Pdf14,
    /// PDF 1.5.
    Pdf15,
    /// PDF 1.6.
    Pdf16,
    /// PDF 1.7.
    Pdf17,
    /// PDF 2.0.
    Pdf20,
}

impl PdfVersion {
    pub(crate) fn write_xmp(&self, xmp: &mut XmpWriter) {
        match self {
            PdfVersion::Pdf14 => xmp.pdf_version("1.4"),
            PdfVersion::Pdf15 => xmp.pdf_version("1.5"),
            PdfVersion::Pdf16 => xmp.pdf_version("1.6"),
            PdfVersion::Pdf17 => xmp.pdf_version("1.7"),
            PdfVersion::Pdf20 => xmp.pdf_version("2.0"),
        };
    }

    /// Get a string representation of the PDF version.
    pub fn as_str(&self) -> &str {
        match self {
            PdfVersion::Pdf14 => "PDF 1.4",
            PdfVersion::Pdf15 => "PDF 1.5",
            PdfVersion::Pdf16 => "PDF 1.6",
            PdfVersion::Pdf17 => "PDF 1.7",
            PdfVersion::Pdf20 => "PDF 2.0",
        }
    }

    pub(crate) fn rgb_icc(&self) -> ICCProfile<3> {
        match self {
            PdfVersion::Pdf14 => SRGB_V2_ICC.clone(),
            PdfVersion::Pdf15 => SRGB_V2_ICC.clone(),
            PdfVersion::Pdf16 => SRGB_V2_ICC.clone(),
            PdfVersion::Pdf17 => SRGB_V4_ICC.clone(),
            PdfVersion::Pdf20 => SRGB_V4_ICC.clone(),
        }
    }

    pub(crate) fn grey_icc(&self) -> ICCProfile<1> {
        match self {
            PdfVersion::Pdf14 => GREY_V2_ICC.clone(),
            PdfVersion::Pdf15 => GREY_V2_ICC.clone(),
            PdfVersion::Pdf16 => GREY_V2_ICC.clone(),
            PdfVersion::Pdf17 => GREY_V4_ICC.clone(),
            PdfVersion::Pdf20 => GREY_V4_ICC.clone(),
        }
    }

    pub(crate) fn supports_icc(&self, metadata: &ICCMetadata) -> bool {
        match self {
            PdfVersion::Pdf14 => metadata.major <= 2 && metadata.minor <= 2,
            PdfVersion::Pdf15 => metadata.major <= 4,
            PdfVersion::Pdf16 => metadata.major <= 4 && metadata.minor <= 1,
            PdfVersion::Pdf17 => metadata.major <= 4 && metadata.minor <= 2,
            PdfVersion::Pdf20 => metadata.major <= 4 && metadata.minor <= 2,
        }
    }

    #[cfg(feature = "raster-images")]
    pub(crate) fn supports_bit_depth(&self, bits_per_component: BitsPerComponent) -> bool {
        match bits_per_component {
            BitsPerComponent::Eight => true,
            BitsPerComponent::Sixteen => *self >= PdfVersion::Pdf15,
        }
    }

    pub(crate) fn set_version(&self, pdf: &mut Pdf) {
        match self {
            PdfVersion::Pdf14 => pdf.set_version(1, 4),
            PdfVersion::Pdf15 => pdf.set_version(1, 5),
            PdfVersion::Pdf16 => pdf.set_version(1, 6),
            PdfVersion::Pdf17 => pdf.set_version(1, 7),
            PdfVersion::Pdf20 => pdf.set_version(2, 0),
        };
    }

    pub(crate) fn deprecates_proc_sets(&self) -> bool {
        *self >= PdfVersion::Pdf20
    }

    pub(crate) fn deprecates_cid_set(&self) -> bool {
        *self >= PdfVersion::Pdf20
    }
}

/// The ICC v4 profile for the SRGB color space.
static SRGB_V4_ICC: Lazy<ICCProfile<3>> =
    Lazy::new(|| ICCProfile::new(include_bytes!("../../icc/sRGB-v4.icc")).unwrap());
/// The ICC v2 profile for the SRGB color space.
static SRGB_V2_ICC: Lazy<ICCProfile<3>> =
    Lazy::new(|| ICCProfile::new(include_bytes!("../../icc/sRGB-v2-magic.icc")).unwrap());
/// The ICC v4 profile for the sgray color space.
static GREY_V4_ICC: Lazy<ICCProfile<1>> =
    Lazy::new(|| ICCProfile::new(include_bytes!("../../icc/sGrey-v4.icc")).unwrap());
/// The ICC v2 profile for the sgray color space.
static GREY_V2_ICC: Lazy<ICCProfile<1>> =
    Lazy::new(|| ICCProfile::new(include_bytes!("../../icc/sGrey-v2-magic.icc")).unwrap());
