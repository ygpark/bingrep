using System.Runtime.InteropServices;

namespace GhostYak.IO.DeviceIOControl.Objects.Volume
{
    [StructLayout(LayoutKind.Sequential)]
    public struct VOLUME_DISK_EXTENTS
    {
        public uint NumberOfDiskExtents;
        public DISK_EXTENT[] Extents;
    }
}