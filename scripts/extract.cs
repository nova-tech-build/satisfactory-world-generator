#r "nuget: CUE4Parse, 1.2.2.21"

using System.Diagnostics;
using CUE4Parse.Compression;
using CUE4Parse.FileProvider;
using CUE4Parse.MappingsProvider;
using CUE4Parse.UE4.Objects.Core.Math;
using CUE4Parse.UE4.Objects.Engine;
using CUE4Parse.UE4.Objects.UObject;
using CUE4Parse.UE4.Versions;

const string SATISFACTORY_PATH = ".../steam/steamapps/common/Satisfactory/";

var packageDir = $"{SATISFACTORY_PATH}FactoryGame/Content/Paks/";
var mappingsFile = $"{SATISFACTORY_PATH}CommunityResources/FactoryGame.usmap";
var mainDllFile = $"{SATISFACTORY_PATH}FactoryGame/Binaries/Win64/FactoryGameSteam-FactoryGame-Win64-Shipping.dll";

var version = FileVersionInfo.GetVersionInfo(mainDllFile);
var ueVersion = $"{version.FileMajorPart}_{version.FileMinorPart}";
var gameVersion = version.FileVersion;

if (string.IsNullOrWhiteSpace(gameVersion) || ueVersion == "0_0")
{
    Console.Error.WriteLine("FileVersionInfo doesn't work. trying again using mono csharp cli...");
    var script = @"
var version = System.Diagnostics.FileVersionInfo.GetVersionInfo(Args[0]);
Console.WriteLine($""{version.FileMajorPart}_{version.FileMinorPart}"");
Console.Write(version.FileVersion);
";
    var process = Process.Start(new ProcessStartInfo()
    {
        FileName = "csharp",
        ArgumentList = { "-s", "/dev/stdin", mainDllFile },
        RedirectStandardInput = true,
        RedirectStandardOutput = true,
        UseShellExecute = false,
    });
    if (process is null)
    {
        Console.Error.WriteLine("could not start csharp process. exiting");
        return;
    }

    process.StandardInput.WriteLine(script);
    process.StandardInput.Close();

    ueVersion = process.StandardOutput.ReadLine();
    gameVersion = process.StandardOutput.ReadToEnd();
}

var ueVersionString = $"GAME_UE{ueVersion}";
EGame game;
try
{
    game = Enum.Parse<EGame>(ueVersionString);
}
catch (Exception)
{
    Console.Error.WriteLine($"unsupported ue version {ueVersionString}. exiting");
    return;
}

var oodlePath = Path.Combine(".", "liboo2corelinux64.so.9");
if (Path.Exists(oodlePath))
    OodleHelper.Initialize(oodlePath);

var provider = new DefaultFileProvider(packageDir, SearchOption.AllDirectories, new VersionContainer(game),
    StringComparer.Ordinal);
try
{
    provider.MappingsContainer = new FileUsmapTypeMappingsProvider(mappingsFile);
}
catch (Exception)
{
    Console.Error.WriteLine("could not load mappings file. continuing without mappings...");
}

provider.Initialize();
provider.Mount();


const string levelPath = "FactoryGame/Content/FactoryGame/Map/GameLevel01/Persistent_Level.umap.PersistentLevel";
var level = provider.LoadPackageObject<ULevel>(levelPath);

// BP_ResourceNode_C, BP_FrackingSatellite_C, BP_FrackingCore_C, BP_ResourceNodeGeyser_C

using var writer = new StreamWriter("extracted-resources.json");
writer.WriteLine("[");
writer.WriteLine($"""["GameVersion", "{gameVersion}"],""");

foreach (var node in level.Actors.Select(a => a.Load()).Where(a => a is { ExportType: "BP_ResourceNode_C" }))
{
    if (node is null) continue;

    var name = node.Name;
    var location = node.Get<FPackageIndex>("mBoxComponent").Load().Get<FVector>("RelativeLocation");
    var resource = node.Get<FPackageIndex>("mResourceClass").Name;
    var purity = node.GetOrDefault<FName>("mPurity", "RP_Normal").ToString();

    writer.WriteLine(
        $"""["{node.ExportType}", "{name}", [{location.X}, {location.Y}, {location.Z}], "{resource}", "{purity}"],""");
}

foreach (var node in level.Actors.Select(a => a.Load()).Where(a => a is { ExportType: "BP_ResourceNodeGeyser_C" }))
{
    if (node is null) continue;

    var name = node.Name;
    var location = node.Get<FPackageIndex>("mBoxComponent").Load().Get<FVector>("RelativeLocation");
    var purity = node.GetOrDefault<FName>("mPurity", "RP_Normal").ToString();

    writer.WriteLine(
        $"""["{node.ExportType}", "{name}", [{location.X}, {location.Y}, {location.Z}], "{purity}"],""");
}

foreach (var node in level.Actors.Select(a => a.Load()).Where(a => a is { ExportType: "BP_FrackingCore_C" }))
{
    if (node is null) continue;

    var name = node.Name;
    var location = node.Get<FPackageIndex>("mBoxComponent").Load().Get<FVector>("RelativeLocation");

    writer.WriteLine(
        $"""["{node.ExportType}", "{name}", [{location.X}, {location.Y}, {location.Z}]],""");
}

foreach (var node in level.Actors.Select(a => a.Load()).Where(a => a is { ExportType: "BP_FrackingSatellite_C" }))
{
    if (node is null) continue;

    var name = node.Name;
    var location = node.Get<FPackageIndex>("mBoxComponent").Load().Get<FVector>("RelativeLocation");
    var resource = node.Get<FPackageIndex>("mResourceClass").Name;
    var purity = node.GetOrDefault<FName>("mPurity", "RP_Normal").ToString();
    var core = node.Get<FPackageIndex>("mCore").Name;

    writer.WriteLine(
        $"""["{node.ExportType}", "{name}", [{location.X}, {location.Y}, {location.Z}], "{resource}", "{purity}", "{core}"],""");
}

writer.WriteLine("[null]]");
