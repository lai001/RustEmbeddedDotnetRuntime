using PluginBase;
using System;

namespace HelloPlugin
{
    public class HelloCommand : ICommand
    {
        public string Name { get => "hello"; }
        public string Description { get => "Displays hello message."; }

        public int Execute()
        {
            Test();
            return 0;
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