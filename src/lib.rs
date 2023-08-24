use std::io::Write;

#[cfg(not(feature = "dicom-encoding"))]
mod encoding {
    /// An encoder with its type erased.
    pub type DynEncoder<'w, W> = Box<dyn EncodeTo<W> + 'w>;

    /// Type trait for a data element encoder to a single known writer type `W`.
    pub trait EncodeTo<W: ?Sized> {}
}

/// `E` is the encoder
#[derive(Debug)]
pub struct StatefulEncoder<E>(E);

// the type alias below breaks the compiler

#[cfg(not(feature = "dicom-encoding"))]
pub type DynStatefulEncoder<'w> = StatefulEncoder<self::encoding::DynEncoder<'w, dyn Write>>;

#[cfg(feature = "dicom-encoding")]
pub type DynStatefulEncoder<'w> = StatefulEncoder<dicom_encoding::transfer_syntax::DynEncoder<'w, dyn Write>>;
