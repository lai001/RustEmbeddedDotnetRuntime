
task("project_setup")
    on_run(function ()
        import("net.http")
        import("utils.archive")
        import("lib.detect.find_program")

        local function runProgram(programName, argv) 
            local program = find_program(programName)
            if program == programName then
                os.execv(program, argv)
            end            
        end

        if os.exists(".xmake/dotnet-sdk-5.0.408-win-x64.zip") == false then
            local link = "https://download.visualstudio.microsoft.com/download/pr/57776397-c87d-4eb8-9080-d58d180ccbe6/920afd9e178bdcd10fcfe696c1fdb88c/dotnet-sdk-5.0.408-win-x64.zip"
            http.download(link, ".xmake/dotnet-sdk-5.0.408-win-x64.zip")
        end
        if os.exists(".xmake/dotnet_runtime") == false and os.exists(".xmake/dotnet-sdk-5.0.408-win-x64.zip") then
            archive.extract(".xmake/dotnet-sdk-5.0.408-win-x64.zip", ".xmake/dotnet_runtime")
        end
        os.mkdir("rust_embedded_dotnet_runtime/target/debug")

        runProgram("dotnet", { "build", "./AppWithPlugin/AppWithPlugin.sln" })

        os.cp(".xmake/dotnet_runtime/packs/Microsoft.NETCore.App.Host.win-x64/5.0.17/runtimes/win-x64/native/nethost.dll", 
            "rust_embedded_dotnet_runtime/target/debug/nethost.dll")
        os.cp(".xmake/dotnet_runtime/packs/Microsoft.NETCore.App.Host.win-x64/5.0.17/runtimes/win-x64/native/nethost.lib", 
            "rust_embedded_dotnet_runtime/target/debug/nethost.lib")

        os.cd("rust_embedded_dotnet_runtime")
        runProgram("cargo", { "build" })
    end)

    set_menu {
        usage = "xmake project_setup",
        description = "Setup project.",
        options = {
            {nil, "project_setup", "setup", nil, "" },
        }
    }