using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using System.Runtime.InteropServices;

namespace test_app_cs
{

    internal class Program
    {
        [DllImport("countchars.dll")]
        public static extern UInt32 count_characters(String str);

        // https://notes.huy.rocks/en/string-ffi-rust.html
        [DllImport("countchars.dll")]
        public static extern String string_from_rust();

        // https://stackoverflow.com/questions/30154541/how-do-i-concatenate-strings
        [DllImport("countchars.dll")]
        public static extern String echo_from_rust(String str);

        static void Main(string[] args)
        {
            String str1 = "Hello world!";
            UInt32 count = 0;

            count = count_characters(str1);
            Console.WriteLine(count + " characters in \"" + str1 + "\"");

            String str2 = echo_from_rust(str1);
            count = (UInt32)str2.Length;
            Console.WriteLine(count + " characters in \"" + str2 + "\"");

            str2 = string_from_rust();
            count = (UInt32)str2.Length;
            Console.WriteLine(count + " characters in \"" + str2 + "\"");

            Console.WriteLine("Press ENTER to exit...");
            Console.ReadLine();
        }
    }
}
