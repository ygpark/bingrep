namespace GhostYak.IO.DeviceIOControl.Objects.Disk
{
    public enum DISK_CACHE_RETENTION_PRIORITY : uint
    {
        EqualPriority = 0,
        KeepPrefetchedData = 1,
        KeepReadData = 2
    }
}