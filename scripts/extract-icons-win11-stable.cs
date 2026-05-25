#r "nuget: CUE4Parse, 1.2.2"
#r "nuget: CUE4Parse-Conversion, 1.2.1"

using CUE4Parse.Compression;
using CUE4Parse.FileProvider;
using CUE4Parse.MappingsProvider;
using CUE4Parse.UE4.Assets.Exports.Texture;
using CUE4Parse.UE4.Versions;
using CUE4Parse_Conversion.Textures;
using SkiaSharp;

const string PakDir = "C:/Program Files (x86)/steam/steamapps/common/Satisfactory/FactoryGame/Content/Paks/";
const string MappingFile = "C:/Program Files (x86)/steam/steamapps/common/Satisfactory/CommunityResources/FactoryGame.usmap";
const string OutDir = "../assets/icons/";

OodleHelper.Initialize("./oodle-data-shared.dll");

var provider = new DefaultFileProvider(
    PakDir,
    SearchOption.AllDirectories,
    new VersionContainer(EGame.GAME_UE5_3),
    StringComparer.OrdinalIgnoreCase
)
{
    MappingsContainer = new FileUsmapTypeMappingsProvider(MappingFile),
};
provider.Initialize();
provider.Mount();

Directory.CreateDirectory(OutDir);

var resourceIcons = new Dictionary<string, string>
{
    ["Resource/RawResources/Nodes/UI/IconDesc_iron_new_256.uasset"] = "Desc_OreIron_C",
    ["Resource/RawResources/Coal/UI/IconDesc_CoalOre_256.uasset"] = "Desc_Coal_C",
    ["Resource/RawResources/Nodes/UI/IconDesc_copper_new_256.uasset"] = "Desc_OreCopper_C",
    ["Resource/RawResources/Stone/UI/Stone_256.uasset"] = "Desc_Stone_C",
    ["Resource/Parts/QuartzCrystal/UI/IconDesc_QuartzCrystal_256.uasset"] = "Desc_RawQuartz_C",
    ["Resource/RawResources/CrudeOil/UI/LiquidOil_Pipe_256.uasset"] = "Desc_LiquidOil_C",
    ["Resource/RawResources/Water/UI/LiquidWater_Pipe_256.uasset"] = "Desc_Water_C",
    ["Resource/RawResources/SAM/UI/IconDesc_SameOre_256.uasset"] = "Desc_SAM_C",
    ["Resource/Parts/PackagedNitrogen/UI/IconDesc_NitrogenGas_256.uasset"] = "Desc_NitrogenGas_C",
    ["Resource/RawResources/Nodes/UI/IconDesc_Bauxite_256.uasset"] = "Desc_OreBauxite_C",
    ["Resource/RawResources/Nodes/UI/IconDesc_CateriumOre_256.uasset"] = "Desc_OreGold_C",
    ["Resource/RawResources/Sulfur/UI/Sulfur_256.uasset"] = "Desc_Sulfur_C",
    ["Resource/RawResources/OreUranium/UI/IconDesc_UraniumOre_256.uasset"] = "Desc_OreUranium_C",
    ["Buildable/Factory/GeneratorGeoThermal/UI/GeoThermalPowerGenerator_512.uasset"] = "Desc_GeneratorGeoThermal_C",
};

foreach (var (relativePath, pngName) in resourceIcons)
{
    var path = $"FactoryGame/Content/FactoryGame/{relativePath}";

    foreach (var texture in provider.LoadPackage(path).GetExports().OfType<UTexture2D>())
    {
        var bitmap = texture.Decode(ETexturePlatform.DesktopMobile);
        if (bitmap is null) continue;

        var outputPath = Path.Combine(OutDir, $"{pngName}.png");

        using var image = SKImage.FromBitmap(bitmap);
        using var data = image.Encode(SKEncodedImageFormat.Png, 100);
        using var stream = File.Create(outputPath);

        data.SaveTo(stream);
        Console.WriteLine(outputPath);
    }
}
