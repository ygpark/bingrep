using System;

namespace GhostYak.IO.DeviceIOControl.Objects.Enums
{
    [Flags]
    public enum IOMethod : uint
    {
        Buffered = 0,
        InDirect = 1,
        OutDirect = 2,
        Neither = 3
    }
}