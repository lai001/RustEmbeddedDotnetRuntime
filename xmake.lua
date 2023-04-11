
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
        local dotnetSDKFilename = "dotnet-sdk-5.0.408-win-x64.zip"
        if os.exists(".xmake/" .. dotnetSDKFilename) == false then
            local link = "https://download.visualstudio.microsoft.com/download/pr/57776397-c87d-4eb8-9080-d58d180ccbe6/920afd9e178bdcd10fcfe696c1fdb88c/dotnet-sdk-5.0.408-win-x64.zip"
            http.download(link, ".xmake/" .. dotnetSDKFilename)
        end
        if os.exists(".xmake/dotnetSDK") == false and os.exists(".xmake/" .. dotnetSDKFilename) then
            archive.extract(".xmake/" .. dotnetSDKFilename, ".xmake/dotnetSDK")
        end

        local function setup(buildType) 
            local nethost = ".xmake/dotnetSDK/packs/Microsoft.NETCore.App.Host.win-x64/5.0.17/runtimes/win-x64/native/nethost"
            local target_nethost = "rust_embedded_dotnet_runtime/target/" .. buildType .. "/nethost"
            os.mkdir("rust_embedded_dotnet_runtime/target/" .. buildType)
            os.cp(nethost .. ".dll", target_nethost .. ".dll")
            os.cp(nethost .. ".dll", target_nethost .. ".lib")
        end
        setup("debug")
        setup("release")

        runProgram("dotnet", { "build", "./AppWithPlugin/AppWithPlugin.sln" })
        runProgram("dotnet", { "build", "-c", "Release", "./AppWithPlugin/AppWithPlugin.sln" })

        os.cd("rust_embedded_dotnet_runtime")
        runProgram("cargo", { "build" })
        runProgram("cargo", { "build", "--release" })
    end)

    set_menu {
        usage = "xmake project_setup",
        description = "Setup project.",
        options = {
            {nil, "project_setup", "setup", nil, "" },
        }
    }