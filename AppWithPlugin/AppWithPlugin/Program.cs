﻿using Microsoft.Build.Evaluation;
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
using System.Threading;

namespace AppWithPlugin
{
    public static unsafe class Program
    {
        static PluginLoadContext loadContext = null;

        private static readonly Locker<List<Action>> pendingTasks = new(new List<Action>());

        public static void Main(string[] args)
        {
            //Debugger.Launch();
            //Console.WriteLine("Waiting for debugger to attach");
            //while (!Debugger.IsAttached)
            //{
            //    Thread.Sleep(3000);
            //}
            Entry.SetSourceFileChangedListenter(delegate ()
            {
                pendingTasks.Write(delegate (ref List<Action> pendingTasks) {
                    pendingTasks.Add(delegate ()
                    {
                        UnloadAssembly();
                        if (BuildAssembly())
                        {
                            LoadAssembly();
                        }
                    });
                });
            });

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

            LoadAssembly();
            while (true)
            {
                pendingTasks.Write(delegate (ref List<Action> pendingTasks)
                {
                    foreach (Action pendingTask in pendingTasks)
                    {
                        pendingTask();
                    }
                    pendingTasks.Clear();
                });
                Thread.Sleep((int)(1.0 / 60.0 * 1000.0));
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
        private static bool BuildAssembly()
        {
#if DEBUG
    string buildType = "Debug";
#else
    string buildType = "Release";
#endif
            BuildManager.DefaultBuildManager.ResetCaches();
            string path = @"../../../AppWithPlugin/HelloPlugin/HelloPlugin.csproj";
            ProjectCollection projectCollection = new();
            BuildParameters buildParameters = new(projectCollection);
            BuildRequestData buildRequestData = new(path, new Dictionary<string, string>
            {
                { "Configuration", buildType }
            }, null, new[] { "Build" }, null);
            BuildResult buildResult = BuildManager.DefaultBuildManager.Build(buildParameters, buildRequestData);
            return buildResult.OverallResult == BuildResultCode.Success;
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
