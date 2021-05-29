using System.Runtime.InteropServices;

namespace GhostYak.IO.DeviceIOControl.Objects.FileSystem
{
    [StructLayout(LayoutKind.Sequential)]
    public struct FILE_SET_SPARSE_BUFFER
    {
        [MarshalAs(UnmanagedType.U1)]
        public bool SetSparse;
    }
}