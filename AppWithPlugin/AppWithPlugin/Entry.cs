using Foundation;
using Native;
using System;
using System.Runtime.CompilerServices;
using System.Runtime.InteropServices;

namespace AppWithPlugin
{
    public static unsafe class Entry
    {
        private static Locker<Action> sourceFileChangedHandle = new();

        [UnmanagedCallersOnly(CallConvs = new[] { typeof(CallConvCdecl) })]
        public static void Main(IntPtr nativeEntryInfoPtr)
        {
            NativeEntryInfo nativeEntryInfo = Marshal.PtrToStructure<NativeEntryInfo>(nativeEntryInfoPtr);

            string[] args = new string[nativeEntryInfo.argsLength];
            for (int i = 0; i < nativeEntryInfo.argsLength; i++)
            {
                args[i] = Marshal.PtrToStringUTF8(Marshal.ReadIntPtr(IntPtr.Add(nativeEntryInfo.argsPtr, i * IntPtr.Size)));
            }

            NativeStudent.Init(nativeEntryInfo.nativeStudentFuncPtr);

            nativeEntryInfo.nativeFileWatchSetFunc(&FileDidChanged);

            Program.Main(args);
        }

        [UnmanagedCallersOnly]
        private static unsafe void FileDidChanged()
        {
            sourceFileChangedHandle.Read(delegate (Action sourceFileChangedHandle)
            {
                if (sourceFileChangedHandle != null)
                {
                    sourceFileChangedHandle();
                }
            });
        }

        public static void SetSourceFileChangedListenter(Action action)
        {
            sourceFileChangedHandle.Write(delegate (ref Action sourceFileChangedHandle)
            {
                sourceFileChangedHandle += action;
            });
        }

    }

    [StructLayout(LayoutKind.Sequential)]
    public unsafe struct NativeEntryInfo
    {
        public delegate* unmanaged<delegate* unmanaged<void>, void> nativeFileWatchSetFunc;
        public IntPtr argsPtr;
        public int argsLength;
        public NativeStudentFuncPtr nativeStudentFuncPtr;
    }

}
