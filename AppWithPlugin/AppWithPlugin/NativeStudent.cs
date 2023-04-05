using System;
using System.Runtime.InteropServices;

namespace AppWithPlugin
{
    public unsafe class NativeStudent : IDisposable
    {
        private IntPtr nativeHandle = IntPtr.Zero;
        private static delegate* unmanaged<int, IntPtr, IntPtr> nativeStudentNew;
        private static delegate* unmanaged<IntPtr, void> nativeStudentDelete;
        private static delegate* unmanaged<IntPtr, int, void> nativeStudentSetAge;
        private static delegate* unmanaged<IntPtr, int> nativeStudentGetAge;
        private static delegate* unmanaged<IntPtr, IntPtr, void> nativeStudentSetName;
        private static delegate* unmanaged<IntPtr, IntPtr> nativeStudentGetName;

        private bool disposed;

        public int Age
        {
            set => nativeStudentSetAge(nativeHandle, value);
            get => nativeStudentGetAge(nativeHandle);
        }

        public string Name
        {
            set => nativeStudentSetName(nativeHandle, Marshal.StringToBSTR(value));
            get => Marshal.PtrToStringUni(nativeStudentGetName(nativeHandle));
        }

        public NativeStudent(int age, string name)
        {
            nativeHandle = nativeStudentNew(age, Marshal.StringToBSTR(name));
            if (nativeHandle != IntPtr.Zero)
            {
                Console.WriteLine($"\nNativeStudent {{ name: {name}, age: {age} }} constructed.");
            }
        }

        public static void Init(NativeStudentFuncPtr funcPtr)
        {
            nativeStudentNew = funcPtr.nativeStudentNew;
            nativeStudentDelete = funcPtr.nativeStudentDelete;
            nativeStudentSetAge = funcPtr.nativeStudentSetAge;
            nativeStudentGetAge = funcPtr.nativeStudentGetAge;
            nativeStudentSetName = funcPtr.nativeStudentSetName;
            nativeStudentGetName = funcPtr.nativeStudentGetName;
        }

        protected virtual void Dispose(bool disposing)
        {
            if (!disposed)
            {
                if (disposing)
                {
                    // Manual release of managed resources.
                }

                // Release unmanaged resources.
                Console.WriteLine($"NativeStudent {{ name: {Name}, age: {Age} }} destructed.");
                nativeStudentDelete(nativeHandle);
                nativeHandle = IntPtr.Zero;
                disposed = true;
            }
        }

        ~NativeStudent()
        {
            Dispose(disposing: false);
        }

        public void Dispose()
        {
            Dispose(disposing: true);
            GC.SuppressFinalize(this);
        }
    }
}
