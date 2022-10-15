/* Copyright 2020 Google LLC
Modifications copyright (C) 2022 ISP RAS

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

     http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
*/
//! Module contains constants for parsing stack traces.

pub const STACK_FRAME_FUNCION_IGNORE_REGEXES: &'static [&'static str] = &[
    // Function names (exact match).
    r"^abort$",
    r"^exit$",
    r"^pthread_create$",
    r"^pthread_kill$",
    r"^raise$",
    r"^tgkill$",
    r"^__chk_fail$",
    r"^__fortify_fail$",
    // Function names (startswith).
    r"^(|__)aeabi_",
    r"^(|__)memcmp",
    r"^(|__)memcpy",
    r"^(|__)memmove",
    r"^(|__)memset",
    r"^(|__)strcmp",
    r"^(|__)strcpy",
    r"^(|__)strdup",
    r"^(|__)strlen",
    r"^(|__)strncpy",
    r"^<null>",
    r"^Abort\(",
    r"^CFCrash",
    r"^ExitCallback",
    r"^IsSandboxedProcess",
    r"^MSanAtExitWrapper",
    r"^New",
    r"^RaiseException",
    r"^SbSystemBreakIntoDebugger",
    r"^SignalAction",
    r"^SignalHandler",
    r"^TestOneProtoInput",
    r"^WTF::",
    r"^WTFCrash",
    r"^X11Error",
    r"^_L_unlock_",
    r"^_\$LT\$",
    r"^__GI_",
    r"^__asan::",
    r"^__asan_",
    r"^__assert_",
    r"^__cxa_atexit",
    r"^__cxa_rethrow",
    r"^__cxa_throw",
    r"^__dump_stack",
    r"^__hwasan::",
    r"^__hwasan_",
    r"^__interceptor_",
    r"^__kasan_",
    r"^__libc_",
    r"^__lsan::",
    r"^__lsan_",
    r"^__msan::",
    r"^__msan_",
    r"^__pthread_kill",
    r"^__run_exit_handlers",
    r"^__rust_try",
    r"^__sanitizer::",
    r"^__sanitizer_",
    r"^__tsan::",
    r"^__tsan_",
    r"^__ubsan::",
    r"^__ubsan_",
    r"^_asan_",
    r"^_hwasan_",
    r"^_lsan_",
    r"^_msan_",
    r"^_objc_terminate",
    r"^_sanitizer_",
    r"^_start",
    r"^_tsan_",
    r"^_ubsan_",
    r"^abort",
    r"^alloc::",
    r"^android\.app\.ActivityManagerProxy\.",
    r"^android\.os\.Parcel\.",
    r"^art::Thread::CreateNativeThread",
    r"^asan_",
    r"^asan\.module_ctor",
    r"^asan\.module_dtor",
    r"^calloc",
    r"^check_memory_region",
    r"^common_exit",
    r"^core::fmt::write",
    r"^core::panicking",
    r"^core::result",
    r"^delete",
    r"^demangling_terminate_handler",
    r"^dump_backtrace",
    r"^dump_stack",
    r"^exit_or_terminate_process",
    r"^fpehandler\(",
    r"^free",
    r"^fuzzer::",
    r"^g_log",
    r"^generic_cpp_",
    r"^gsignal",
    r"^kasan_",
    r"^libfuzzer_sys::initialize",
    //r"^main",
    r"^malloc",
    r"^mozalloc_",
    r"^new",
    r"^object_err",
    r"^operator",
    r"^panic_abort::",
    r"^print_trailer",
    r"^realloc",
    r"^rust_begin_unwind",
    r"^rust_fuzzer_test_input",
    r"^rust_oom",
    r"^rust_panic",
    r"^scanf",
    r"^show_stack",
    r"^std::__terminate",
    r"^std::io::Write::write_fmt",
    r"^std::panic",
    r"^std::process::abort",
    r"^std::sys::unix::abort",
    r"^std::sys_common::backtrace",
    r"^__rust_start_panic",
    r"^__scrt_common_main_seh",
    // Functions names (contains).
    r".*ASAN_OnSIGSEGV",
    r".*BaseThreadInitThunk",
    r".*DebugBreak",
    r".*DefaultDcheckHandler",
    r".*ForceCrashOnSigAbort",
    r".*MemoryProtection::CMemoryProtector",
    r".*PartitionAlloc",
    r".*RtlFreeHeap",
    r".*RtlInitializeExceptionChain",
    r".*RtlReportCriticalFailure",
    r".*RtlUserThreadStart",
    r".*RtlpHeapHandleError",
    r".*RtlpLogHeapFailure",
    r".*SkDebugf",
    r".*StackDumpSignalHandler",
    r".*__android_log_assert",
    r".*__tmainCRTStartup",
    r".*_asan_rtl_",
    r".*agent::asan::",
    r".*allocator_shim",
    r".*asan_Heap",
    r".*asan_check_access",
    r".*asan_osx_dynamic\.dylib",
    r".*assert",
    r".*base::FuzzedDataProvider",
    r".*base::allocator",
    r".*base::android::CheckException",
    r".*base::debug::BreakDebugger",
    r".*base::debug::CollectStackTrace",
    r".*base::debug::StackTrace::StackTrace",
    r".*ieee754\-",
    r".*libpthread",
    r".*logger",
    r".*logging::CheckError",
    r".*logging::ErrnoLogMessage",
    r".*logging::LogMessage",
    r".*stdext::exception::what",
    r".*v8::base::OS::Abort",
];

pub const STACK_FRAME_FILEPATH_IGNORE_REGEXES: &'static [&'static str] = &[
    // File paths.
    r".*/usr/include/c\+\+/",
    r".*\-gnu/c\+\+/",
    r".*\-gnu/bits/",
    r".*/clang/",
    r".*base/callback",
    r".*/rust(|c)/",
    r".*/AOSP\-toolchain/",
    r".*/bindings/ToV8\.h",
    r".*/crosstool/",
    r".*/gcc/",
    r".*/glibc\-",
    r".*/jemalloc/",
    r".*/libc\+\+",
    r".*/libc/",
    r".*/compiler\-rt/lib/fuzzer/",
    r".*/llvm\-build/",
    r".*/minkernel/crts/",
    r".*/sanitizer_common/",
    r".*/tcmalloc/",
    r".*/vc/include/",
    r".*/vctools/crt/",
    r".*/win_toolchain/",
    r".*libc\+\+/",
    // Others (uncategorized).
    r".*\+Unknown",
    r".*<unknown module>",
    r".*Inline Function @",
    r"^<unknown>$",
    r"^\[vdso\]$",
    r"^linux-gate.so.*$",
    r".*libc\.so",
    r".*libc\+\+\.so",
    r".*libc\+\+_shared\.so",
    r".*libstdc\+\+\.so",
    r".*libc-.*\.so",
    r".*libpthread\.so",
];
