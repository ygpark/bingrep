using System;

namespace GhostYak.IO.DeviceIOControl.Objects.Enums
{
    /// <summary>
    /// File attributes are metadata values stored by the file system on disk and are used by the system and are available to developers via various file I/O APIs.
    /// </summary>
    [Flags]
    //[CLSCompliant(false)]
    public enum FileAttributesEx : uint
    {
        /// <summary>
        /// ���Ͽ��� �б� �����Դϴ�.
        /// </summary>
        Readonly = 0x00000001,

        /// <summary>
        /// ���� �Ǵ� ���丮�� �������� ���� �Ϲ� ���͸� ��Ͽ� ���� ���� �ʽ��ϴ�.
        /// </summary>
        Hidden = 0x00000002,

        /// <summary>
        /// ������ �ý��� �����Դϴ�. ��, ������ � ü���� �Ϻ� �̰ų� � ü������ ���������� ��� �˴ϴ�.
        /// </summary>
        System = 0x00000004,

        /// <summary>
        /// ���� ���͸��Դϴ�.
        /// </summary>
        Directory = 0x00000010,

        /// <summary>
        /// ���Ͽ� ��� �Ǵ� ���ſ� �� �� �ĺ��Դϴ�.
        /// </summary>
        Archive = 0x00000020,

        /// <summary>
        /// ���߿� ����ϱ� ���� ����Ǿ� �ֽ��ϴ�.
        /// </summary>
        Device = 0x00000040,

        /// <summary>
        /// ������ ǥ�� ������ Ư�� �� Ư���� �����ϴ�. �� Ư���� �ܵ� ��� �ϴ� ��쿡 ����� �� �ֽ��ϴ�.
        /// </summary>
        Normal = 0x00000080,

        /// <summary>
        ///     �ӽ� �������Դϴ�. �ӽ� ���Ͽ��� ���� ���α׷� ���� ������ ���� ���α׷��� �Ϸ� �� �Ŀ� �ʿ� ���� �����Ƿ� �ϴ� ���� �ʿ��� �����͸�
        ///     ���� �մϴ�. ���� �ý��ۿ� ������ �׼��� ���� �ʰ� �����͸� �÷����ϴ� �޸𸮿� �����͸� �ٽ� ��뷮 ���� ��ġ�� ��� ���� �Ϸ��� �մϴ�.
        ///     �ӽ� ������ ������ ���� ���α׷����� �� �̻� �ʿ� �������� �մϴ�.
        /// </summary>
        Temporary           = 0x00000100,

        /// <summary>
        /// ������ ���Ľ� �����Դϴ�. ���Ľ� ������ �Ϲ������� ū ���� ��κ� 0 �� ������ ���� �˴ϴ�.
        /// </summary>
        SparseFile          = 0x00000200,

        /// <summary>
        /// ���Ͽ��� ���� �Ǵ� ���͸��� ���� �� ����� ���� ������ ����� ��м� ������ ���� �Ǿ� �ֽ��ϴ�.
        /// </summary>
        ReparsePoint        = 0x00000400,

        /// <summary>
        /// ���� �� ���� �Ǵ� ���丮. ������ ��� ������ ��� �����Ͱ� ����˴ϴ�. ���丮�� ���, ���� �ۼ��� ���� �� ���� ���丮�� ��� ������ �⺻���Դϴ�.
        /// </summary>
        Compressed          = 0x00000800,

        /// <summary>
        /// ������ �����͸� ��� ����� �� �����ϴ�. �� �Ӽ��� ���� �����Ͱ� ���������� �������� ���丮���� �Ű������� ����ŵ�ϴ�.
        /// �� �Ӽ��� �������� ����� ���� ����Ʈ������ ���� ����ҿ��� ���˴ϴ�.
        /// ���� ���α׷����� �Ӽ��� ���Ƿ� �����ϸ� �ȵ˴ϴ�.
        /// </summary>
        Offline             = 0x00001000,

        /// <summary>
        /// The file or directory is not to be indexed by the content indexing service.
        /// </summary>
        NotContentIndexed   = 0x00002000,

        /// <summary>
        /// A file or directory that is encrypted. For a file, all data streams in the file are encrypted. For a directory, encryption is the default for newly created files and subdirectories.
        /// </summary>
        Encrypted           = 0x00004000,

        //
        // ���:
        //     ���� �Ǵ� ���͸����� ������ ���Ἲ ������ ���� �˴ϴ�. �� ���� ���Ͽ� ���� �� �� ��� ������ ��Ʈ�� ���Ͽ��� ���Ἲ ���� �մϴ�.
        //     �� ���� �⺻������ ���͸�, ��� �� ���� �� �ش� ���͸� ������ ���� ���͸��� ���� �� �� ���Ἲ ������ ���� �˴ϴ�.
        IntegrityStream     = 0x00008000,

        /// <summary>
        /// This value is reserved for system use.
        /// </summary>
        Virtual             = 0x00010000,
        //
        // ���:
        //     ���� �Ǵ� ���͸� ������ ���Ἲ �˻翡�� ���� �˴ϴ�. �⺻�������� ���� ���͸��� ���� �Ǵ� ��� �� ���� �� ���� ���͸� ������ ������
        //     ���Ἲ���� ���� �˴ϴ�.
        NoScrubData         = 0x00020000,

        
    }
}