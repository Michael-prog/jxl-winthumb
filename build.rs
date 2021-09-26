fn main() {
    windows::build! {
      Windows::Win32::UI::Shell::{IThumbnailProvider, SHChangeNotify},
      Windows::Win32::Graphics::Imaging::{IWICBitmapDecoder, WICBitmapDecoderCapabilities, IWICImagingFactory, IWICComponentInfo, CLSID_WICImagingFactory, WICRect, IWICBitmapFrameDecode, GUID_WICPixelFormat32bppRGBA},
      Windows::Win32::Storage::StructuredStorage::IStream,
      Windows::Win32::System::Com::{IClassFactory, CoCreateInstance},
      Windows::Win32::System::LibraryLoader::GetModuleFileNameW,
      Windows::Win32::System::SystemServices::DLL_PROCESS_ATTACH,
      Windows::Win32::Foundation::{
        WINCODEC_ERR_BADIMAGE, WINCODEC_ERR_WRONGSTATE, WINCODEC_ERR_CODECNOTHUMBNAIL, WINCODEC_ERR_PALETTEUNAVAILABLE, WINCODEC_ERR_UNSUPPORTEDOPERATION, WINCODEC_ERR_NOTINITIALIZED,
        S_OK, E_FAIL, E_UNEXPECTED, E_INVALIDARG,
        CLASS_E_CLASSNOTAVAILABLE, E_NOTIMPL, CLASS_E_NOAGGREGATION, E_NOINTERFACE,
        HINSTANCE,
        PWSTR,
      },
    };
}
