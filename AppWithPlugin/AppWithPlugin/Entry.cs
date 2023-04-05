using System;
using System.Runtime.CompilerServices;
using System.Runtime.InteropServices;

namespace AppWithPlugin
{
    public static unsafe class Entry
    {
        [UnmanagedCallersOnly(CallConvs = new[] { typeof(CallConvCdecl) })]
        public static void Main(IntPtr argsPtr, int length, NativeStudentFuncPtr funcPtr)
        {
            string[] args = new string[length];
            for (int i = 0; i < length; i++)
            {
                args[i] = Marshal.PtrToStringUTF8(Marshal.ReadIntPtr(IntPtr.Add(argsPtr, i * IntPtr.Size)));
            }

            NativeStudent.Init(funcPtr);
            Program.Main(args);
        }
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