using System.Runtime.InteropServices;

namespace GhostYak.IO.DeviceIOControl.Objects.FileSystem
{
    [StructLayout(LayoutKind.Sequential)]
    public struct NTFS_FILE_RECORD_OUTPUT_BUFFER
    {
        public ulong FileReferenceNumber;
        public uint FileRecordLength;
        public byte[] FileRecordBuffer;
    }
}