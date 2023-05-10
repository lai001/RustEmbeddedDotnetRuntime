task("setup_project")
    on_run(function ()
        local http = import("net.http")
        local archive = import("utils.archive")
        local find_program = import("lib.detect.find_program")

        local function runProgram(programName, argv) 
            local program = find_program(programName)
            if program ~= nil then
                os.execv(program, argv)
            end            
        end

        local function setup(buildType) 
            os.mkdir("rust_embedded_dotnet_runtime/target/" .. buildType)
            os.mkdir("rust_embedded_dotnet_runtime/target/" .. buildType .. "/deps")
            if is_host("linux") then
            local target_nethost = "rust_embedded_dotnet_runtime/target/" .. buildType .. "/deps/libnethost"
                local nethost = ".xmake/dotnetSDK/packs/Microsoft.NETCore.App.Host.linux-x64/6.0.15/runtimes/linux-x64/native/libnethost"
                os.cp(nethost .. ".a", target_nethost .. ".a")
                os.cp(nethost .. ".so", target_nethost .. ".so")
            elseif is_host("windows") then
                local target_nethost = "rust_embedded_dotnet_runtime/target/" .. buildType .. "/nethost"
                local nethost = ".xmake/dotnetSDK/packs/Microsoft.NETCore.App.Host.win-x64/6.0.16/runtimes/win-x64/native/nethost"
                os.cp(nethost .. ".dll", target_nethost .. ".dll")
                os.cp(nethost .. ".lib", target_nethost .. ".lib")
            end            
        end

        if is_host("linux") then
            local dotnet = "$(env DOTNET_ROOT)/dotnet"
            if os.exists(dotnet) == false then
                os.raise("")
            end
        end

        local link = ""
        local dotnetSDKFilename = ""
        if is_host("linux") then
            link = "https://download.visualstudio.microsoft.com/download/pr/868b2f38-62ca-4fd8-93ea-e640cf4d2c5b/1e615b6044c0cf99806b8f6e19c97e03/dotnet-sdk-6.0.407-linux-x64.tar.gz"
            dotnetSDKFilename = "dotnet-sdk-6.0.407-linux-x64.tar.gz"
        elseif is_host("windows") then
            link = "https://download.visualstudio.microsoft.com/download/pr/ca13c6f1-3107-4cf8-991c-f70edc1c1139/a9f90579d827514af05c3463bed63c22/dotnet-sdk-6.0.408-win-x64.zip"
            dotnetSDKFilename = "dotnet-sdk-6.0.408-win-x64.zip"
        end
        if os.exists(".xmake/" .. dotnetSDKFilename) == false then
            http.download(link, ".xmake/" .. dotnetSDKFilename)
        end
        if os.exists(".xmake/dotnetSDK") == false and os.exists(".xmake/" .. dotnetSDKFilename) then
            archive.extract(".xmake/" .. dotnetSDKFilename, ".xmake/dotnetSDK")
        end

        setup("debug")
        setup("release")

        if is_host("linux") then
            runProgram(".xmake/dotnetSDK/dotnet", { "build", "./AppWithPlugin/AppWithPlugin.sln" })
            runProgram(".xmake/dotnetSDK/dotnet", { "build", "-c", "Release", "./AppWithPlugin/AppWithPlugin.sln" })
        elseif is_host("windows") then
            runProgram("dotnet", { "build", "./AppWithPlugin/AppWithPlugin.sln" })
            runProgram("dotnet", { "build", "-c", "Release", "./AppWithPlugin/AppWithPlugin.sln" })
        end            

        os.cd("rust_embedded_dotnet_runtime")
        runProgram("cargo", { "build" })
        runProgram("cargo", { "build", "--release" })

    end)

    set_menu {
        usage = "xmake setup_project",
        description = "Setup project.",
        options = {
            {nil, "setup_project", "setup", nil, "" },
        }
    }