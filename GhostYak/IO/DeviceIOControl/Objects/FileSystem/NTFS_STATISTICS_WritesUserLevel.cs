using System.Runtime.InteropServices;

namespace GhostYak.IO.DeviceIOControl.Objects.FileSystem
{
    [StructLayout(LayoutKind.Sequential)]
    public struct NTFS_STATISTICS_WritesUserLevel
    {
        public ushort Write;
        public ushort Create;
        public ushort SetInfo;
        public ushort Flush;
    }
}