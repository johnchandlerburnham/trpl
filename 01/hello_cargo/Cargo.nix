# Generated by carnix 0.7.2: carnix build
with import <nixpkgs> {};
let kernel = buildPlatform.parsed.kernel.name;
    abi = buildPlatform.parsed.abi.name;
    include = includedFiles: src: builtins.filterSource (path: type:
      lib.lists.any (f:
        let p = toString (src + ("/" + f)); in
        (path == p) || (type == "directory" && lib.strings.hasPrefix path p)
      ) includedFiles
    ) src;
    updateFeatures = f: up: functions: builtins.deepSeq f (lib.lists.foldl' (features: fun: fun features) (lib.attrsets.recursiveUpdate f up) functions);
    mapFeatures = features: map (fun: fun { features = features; });
    mkFeatures = feat: lib.lists.foldl (features: featureName:
      if feat.${featureName} or false then
        [ featureName ] ++ features
      else
        features
    ) [] (builtins.attrNames feat);
in
rec {
  hello_cargo = f: hello_cargo_0_1_0 { features = hello_cargo_0_1_0_features { hello_cargo_0_1_0 = f; }; };
  __all = [ (hello_cargo {}) ];
  hello_cargo_0_1_0_ = { dependencies?[], buildDependencies?[], features?[] }: buildRustCrate {
    crateName = "hello_cargo";
    version = "0.1.0";
    authors = [ "John C. Burnham <jcb@johnchandlerburnham.com>" ];
    src = ./.;
    inherit dependencies buildDependencies features;
  };
  hello_cargo_0_1_0 = { features?(hello_cargo_0_1_0_features {}) }: hello_cargo_0_1_0_ {};
  hello_cargo_0_1_0_features = f: updateFeatures f (rec {
    hello_cargo_0_1_0.default = (f.hello_cargo_0_1_0.default or true);
  }) [];
}