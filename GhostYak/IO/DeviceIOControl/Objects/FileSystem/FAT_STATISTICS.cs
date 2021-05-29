using System.Runtime.InteropServices;

namespace GhostYak.IO.DeviceIOControl.Objects.FileSystem
{
    [StructLayout(LayoutKind.Sequential)]
    public struct FAT_STATISTICS : IFSStats
    {
        public uint CreateHits;
        public uint SuccessfulCreates;
        public uint FailedCreates;
        public uint NonCachedReads;
        public uint NonCachedReadBytes;
        public uint NonCachedWrites;
        public uint NonCachedWriteBytes;
        public uint NonCachedDiskReads;
        public uint NonCachedDiskWrites;
    }
}