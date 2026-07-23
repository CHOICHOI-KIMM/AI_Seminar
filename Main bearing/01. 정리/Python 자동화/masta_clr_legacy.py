"""
CLR Legacy V2 활성화 정책 선기동 헬퍼
=====================================
mastapy(14.1.1, pythonnet3/clr_loader)를 '외부 파이썬'에서 초기화할 때,
HASP 라이선스 래퍼(haspdnert.dll)가 .NET 2.0(v2.0.50727) 혼합모드라서
    System.IO.FileLoadException: Mixed mode assembly ... 'v2.0.50727' ...
로 실패한다.

SMT 공식 해결책(python.exe.config 의 useLegacyV2RuntimeActivationPolicy)은
pythonnet 2.x 시절 방식이라 clr_loader 기반 mastapy 14.x 에서는 무효하다
(clr_loader 가 exe config 를 읽는 활성화 경로를 거치지 않음).

대신 여기서 mscoree 를 직접 호출해 CLR 을 로드되기 '전에'
ICLRRuntimeInfo::BindAsLegacyV2Runtime() 으로 레거시 V2 정책에 바인딩한다.
그러면 이후 clr_loader 가 CLR 을 기동할 때 혼합모드 v2 어셈블리가 로드된다.

사용법:  import mastapy 보다 '먼저' 이 모듈을 import 하면 됨.
    import masta_clr_legacy   # noqa: F401  (CLR 선기동)
    import mastapy
    mastapy.init(MASTA_DIR)
"""
import ctypes
from ctypes import c_void_p, POINTER, byref, c_wchar_p, c_long

_ole32 = ctypes.WinDLL("ole32.dll")


class _GUID(ctypes.Structure):
    _fields_ = [("Data1", ctypes.c_uint32),
                ("Data2", ctypes.c_uint16),
                ("Data3", ctypes.c_uint16),
                ("Data4", ctypes.c_ubyte * 8)]


def _guid(s):
    g = _GUID()
    hr = _ole32.CLSIDFromString(c_wchar_p(s), byref(g))
    if hr != 0:
        raise OSError(f"CLSIDFromString({s}) failed: 0x{hr & 0xffffffff:08x}")
    return g


_CLSID_CLRMetaHost   = _guid("{9280188D-0E8E-4867-B30C-7FA83884E8DE}")
_IID_ICLRMetaHost    = _guid("{D332DB9E-B9B3-4125-8207-A14884F53216}")
_IID_ICLRRuntimeInfo = _guid("{BD39D1D2-BA2F-486A-89B0-B4B0CB466891}")


def _vcall(p, index, *argtypes):
    """COM vtable 메서드 프로토 생성 (첫 인자 this = c_void_p)."""
    vtbl = ctypes.cast(p, POINTER(c_void_p))[0]
    fn = ctypes.cast(vtbl, POINTER(c_void_p))[index]
    return ctypes.WINFUNCTYPE(c_long, c_void_p, *argtypes)(fn)


_done = False


def bind_legacy_v2(runtime_version="v4.0.30319", verbose=False):
    """CLR 을 로드 전에 Legacy V2 활성화 정책으로 바인딩한다. (한 번만 수행)"""
    global _done
    if _done:
        return
    mscoree = ctypes.WinDLL("mscoree.dll")
    CLRCreateInstance = mscoree.CLRCreateInstance
    CLRCreateInstance.restype = c_long
    CLRCreateInstance.argtypes = [POINTER(_GUID), POINTER(_GUID), POINTER(c_void_p)]

    def _chk(name, hr):
        hr &= 0xffffffff
        if verbose:
            print(f"  [clr-legacy] {name}: 0x{hr:08x}")
        if hr not in (0x0, 0x1):
            raise OSError(f"{name} failed: 0x{hr:08x}")

    pMeta = c_void_p()
    _chk("CLRCreateInstance(MetaHost)",
         CLRCreateInstance(byref(_CLSID_CLRMetaHost),
                           byref(_IID_ICLRMetaHost), byref(pMeta)))

    # ICLRMetaHost::GetRuntime  (vtable idx 3)
    pInfo = c_void_p()
    _chk("MetaHost.GetRuntime",
         _vcall(pMeta, 3, c_wchar_p, POINTER(_GUID), POINTER(c_void_p))(
             pMeta, runtime_version, byref(_IID_ICLRRuntimeInfo), byref(pInfo)))

    # ICLRRuntimeInfo::BindAsLegacyV2Runtime  (vtable idx 13) ← 핵심
    _chk("BindAsLegacyV2Runtime", _vcall(pInfo, 13)(pInfo))
    _done = True


# import 시 자동 실행
bind_legacy_v2()
