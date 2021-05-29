using System.Collections;
using System.Runtime.InteropServices;

namespace GhostYak.IO.DeviceIOControl.Objects.FileSystem
{
    [StructLayout(LayoutKind.Sequential)]
    public struct VOLUME_BITMAP_BUFFER
    {
        public ulong StartingLcn;
        public ulong BitmapSize;

        public BitArray Buffer;
    }
}