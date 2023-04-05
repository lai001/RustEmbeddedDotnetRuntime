using Microsoft.Build.Evaluation;
using Microsoft.Build.Execution;
using Microsoft.Build.Locator;
using PluginBase;
using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.IO;
using System.Linq;
using System.Reflection;
using System.Runtime.CompilerServices;

namespace AppWithPlugin
{
    public static unsafe class Program
    {
        static PluginLoadContext loadContext = null;

        public static void Main(string[] args)
        {
            //Debugger.Launch();
            //Console.WriteLine("Waiting for debugger to attach");
            //while (!Debugger.IsAttached)
            //{
            //    Thread.Sleep(3000);
            //}

            Console.WriteLine($"CurrentDirectory: {Directory.GetCurrentDirectory()}");

            string[] commandLineArgs = Environment.GetCommandLineArgs();
            for (int i = 0; i < commandLineArgs.Length; i++)
            {
                Debug.Assert(commandLineArgs[i] == args[i]);
            }

            List<NativeStudent> students = GetStudents(2);
            foreach (NativeStudent student in students)
            {
                student.Dispose();
            }
            MSBuildLocator.RegisterDefaults();

            while (true)
            {
                string line = Console.ReadLine();

                if (line == null)
                {
                    break;
                }
                else
                {
                    if (line == "quit")
                    {
                        break;
                    }
                    else if (line == "load")
                    {
                        LoadAssembly();
                    }
                    else if (line == "unload")
                    {
                        UnloadAssembly();
                    }
                    else if (line == "reload")
                    {
                        UnloadAssembly();
                        BuildAssembly();
                        LoadAssembly();
                    }
                }
            }
        }

        static Assembly LoadPlugin(string pluginLocation)
        {
            Console.WriteLine($"Loading commands from: {pluginLocation}");
            loadContext = new PluginLoadContext(pluginLocation);
            return loadContext.LoadFromAssemblyName(new AssemblyName(Path.GetFileNameWithoutExtension(pluginLocation)));
        }

        static IEnumerable<ICommand> CreateCommands(Assembly assembly)
        {
            int count = 0;

            foreach (Type type in assembly.GetTypes())
            {
                if (typeof(ICommand).IsAssignableFrom(type))
                {
                    ICommand result = Activator.CreateInstance(type) as ICommand;
                    if (result != null)
                    {
                        count++;
                        yield return result;
                    }
                }
            }

            if (count == 0)
            {
                string availableTypes = string.Join(",", assembly.GetTypes().Select(t => t.FullName));
                throw new ApplicationException(
                    $"Can't find any type which implements ICommand in {assembly} from {assembly.Location}.\n" +
                    $"Available types: {availableTypes}");
            }
        }

        [MethodImpl(MethodImplOptions.NoInlining)]
        private static void LoadAssembly()
        {
            try
            {
                Assembly pluginAssembly = LoadPlugin(@"./HelloPlugin.dll");
                List<ICommand> commands = CreateCommands(pluginAssembly).ToList();
                foreach (ICommand command in commands)
                {
                    command.Execute();
                }
            }
            catch (Exception ex)
            {
                Console.WriteLine(ex);
            }
        }

        [MethodImpl(MethodImplOptions.NoInlining)]
        private static void UnloadAssembly()
        {
            WeakReference alcWeakRef = new(loadContext, trackResurrection: true);

            loadContext?.Unload();

            for (var i = 0; alcWeakRef.IsAlive && i < 10; i++)
            {
                GC.Collect();
                GC.WaitForPendingFinalizers();
            }
        }

        [MethodImpl(MethodImplOptions.NoInlining)]
        private static void BuildAssembly()
        {
            BuildManager.DefaultBuildManager.ResetCaches();
            string path = @"../../../AppWithPlugin/HelloPlugin/HelloPlugin.csproj";
            ProjectCollection projectCollection = new();
            BuildParameters buildParameters = new(projectCollection);
            BuildRequestData buildRequestData = new(path, new Dictionary<string, string>
            {
                { "Configuration", "Debug" }
            }, null, new[] { "Build" }, null);
            BuildManager.DefaultBuildManager.Build(buildParameters, buildRequestData);
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
    }

}
