using System.Runtime.InteropServices;

namespace GhostYak.IO.DeviceIOControl.Objects.Disk
{
    [StructLayout(LayoutKind.Sequential)]
    public struct BLOCK_PREFETCH
    {
        public short Minimum;
        public short Maximum;
    }
}