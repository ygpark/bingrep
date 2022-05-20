using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using System.IO;
using Re2.Net;

namespace TestCode
{
    class Program
    {
        static void Main(string[] args)
        {
            string path = @"F:\\WORK\\VideoRestore\\samplefile\\2.mp4\3.프레임추출\\drf0_20120214_193023.mp4";

            using (FileStream fs = File.Open(path, FileMode.Open, FileAccess.Read, FileShare.Read))
            {
                fs.Position = 0;

                byte[] buffer = new byte[4 * 1024];
                int numberOfRead;

                //string search = "\\x00\\x00\\x01\\xC7";
                //string search = "\\x31\\x0D\\x2C\\x18";
                //string search = "\\x2C\\x18\\x21\\xD2";
                string search = "\\x80\\x03\\xBF\\xE2";
                int searchLen = search.Length / 4;
                //string search = "stco";
                //int searchLen = search.Length;

                List<long> hitOffset = new List<long>();
                List<string> hitString = new List<string>();

                while ((numberOfRead = fs.Read(buffer, 0, buffer.Length)) != 0)
                {
                    MatchCollection matches = Regex.Matches(buffer, search, RegexOptions.Multiline | RegexOptions.Latin1);
                    
                    foreach (Match match in matches)
                    {
                        string value = BitConverter.ToString(buffer, match.Index, searchLen);
                        long offset = fs.Position - numberOfRead + match.Index;

                        hitOffset.Add(offset);
                        hitString.Add(value);
                    }

                    if (numberOfRead == buffer.Length)
                    {
                         // 다음 버퍼를 읽어올 때 '검색어길이 - 1' 만큼 파일 offset을 이동해준다.
                         // 데이터가 현재 버퍼와 다음 버퍼 사이에 끼어있을 때를 방지하기 위한 장치이다.
                        fs.Position = fs.Position - (searchLen - 1);
                    }
                }

                Console.WriteLine(hitOffset.Count());
            }
        }
    }
}
