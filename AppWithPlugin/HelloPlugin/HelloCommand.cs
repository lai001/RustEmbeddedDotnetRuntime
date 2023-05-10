using Native;
using PluginBase;
using System;
using System.Collections.Generic;

namespace HelloPlugin
{
    public class HelloCommand : ICommand
    {
        public string Name { get => "hello"; }
        public string Description { get => "Displays hello message."; }

        public int Execute()
        {
            List<NativeStudent> students = GetStudents(2);
            foreach (NativeStudent student in students)
            {
                student.Dispose();
            }
            Test();
            return 0;
        }

        private static List<NativeStudent> GetStudents(int number)
        {
            List<NativeStudent> students = new();
            for (var i = 0; i < number; i++)
            {
                NativeStudent student = new(i, $"{i}");
                students.Add(student);
            }
            return students;
        }

        void Test()
        {
            int time = Environment.TickCount;
            for (int i = 0; i < 45; i++)
            {
                Fib(i);
            }
            Console.WriteLine(Environment.TickCount - time);
        }

        int Fib(int n)
        {
            if (n <= 0)
                return 0;
            if (n < 3)
                return 1;
            return Fib(n - 1) + Fib(n - 2);
        }
    }
}