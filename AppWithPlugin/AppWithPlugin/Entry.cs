using System;
using System.Runtime.CompilerServices;
using System.Runtime.InteropServices;

namespace AppWithPlugin
{
    public static unsafe class Entry
    {
        private static Action sourceFileChangedHandle;

        [UnmanagedCallersOnly(CallConvs = new[] { typeof(CallConvCdecl) })]
        public static void Main(NativeEntryInfo nativeEntryInfo)
        {
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
            if (sourceFileChangedHandle != null)
            {
                lock (sourceFileChangedHandle)
                {
                    sourceFileChangedHandle();
                }
            }
        }

        public static void SetSourceFileChangedListenter(Action action)
        {
            if (sourceFileChangedHandle == null)
            {
                sourceFileChangedHandle += action;
            }
            else
            {
                lock (sourceFileChangedHandle)
                {
                    sourceFileChangedHandle += action;
                }
            }
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

    [StructLayout(LayoutKind.Sequential)]
    public unsafe struct NativeStudentFuncPtr
    {
        public delegate* unmanaged<int, IntPtr, IntPtr> nativeStudentNew;
        public delegate* unmanaged<IntPtr, void> nativeStudentDelete;
        public delegate* unmanaged<IntPtr, int, void> nativeStudentSetAge;
        public delegate* unmanaged<IntPtr, int> nativeStudentGetAge;
        public delegate* unmanaged<IntPtr, IntPtr, void> nativeStudentSetName;
        public delegate* unmanaged<IntPtr, IntPtr> nativeStudentGetName;
    }
}
