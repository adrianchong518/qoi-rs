# This file was @generated by cargo2nix 0.11.0.
# It is not intended to be manually edited.

args@{
  release ? true,
  rootFeatures ? [
    "qoi-rs/default"
  ],
  rustPackages,
  buildRustPackages,
  hostPlatform,
  hostPlatformCpu ? null,
  hostPlatformFeatures ? [],
  target ? null,
  codegenOpts ? null,
  profileOpts ? null,
  rustcLinkFlags ? null,
  rustcBuildFlags ? null,
  mkRustCrate,
  rustLib,
  lib,
  workspaceSrc,
}:
let
  workspaceSrc = if args.workspaceSrc == null then ./. else args.workspaceSrc;
in let
  inherit (rustLib) fetchCratesIo fetchCrateLocal fetchCrateGit fetchCrateAlternativeRegistry expandFeatures decideProfile genDrvsByProfile;
  profilesByName = {
  };
  rootFeatures' = expandFeatures rootFeatures;
  overridableMkRustCrate = f:
    let
      drvs = genDrvsByProfile profilesByName ({ profile, profileName }: mkRustCrate ({ inherit release profile hostPlatformCpu hostPlatformFeatures target profileOpts codegenOpts rustcLinkFlags rustcBuildFlags; } // (f profileName)));
    in { compileMode ? null, profileName ? decideProfile compileMode release }:
      let drv = drvs.${profileName}; in if compileMode == null then drv else drv.override { inherit compileMode; };
in
{
  cargo2nixVersion = "0.11.0";
  workspace = {
    qoi-rs = rustPackages.unknown.qoi-rs."0.1.0";
  };
  "registry+https://github.com/rust-lang/crates.io-index".autocfg."1.1.0" = overridableMkRustCrate (profileName: rec {
    name = "autocfg";
    version = "1.1.0";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "d468802bab17cbc0cc575e9b053f41e72aa36bfa6b7f55e3529ffa43161b97fa"; };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".num."0.4.0" = overridableMkRustCrate (profileName: rec {
    name = "num";
    version = "0.4.0";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "43db66d1170d347f9a065114077f7dccb00c1b9478c89384490a3425279a4606"; };
    features = builtins.concatLists [
      [ "default" ]
      [ "num-bigint" ]
      [ "std" ]
    ];
    dependencies = {
      num_bigint = rustPackages."registry+https://github.com/rust-lang/crates.io-index".num-bigint."0.4.3" { inherit profileName; };
      num_complex = rustPackages."registry+https://github.com/rust-lang/crates.io-index".num-complex."0.4.0" { inherit profileName; };
      num_integer = rustPackages."registry+https://github.com/rust-lang/crates.io-index".num-integer."0.1.44" { inherit profileName; };
      num_iter = rustPackages."registry+https://github.com/rust-lang/crates.io-index".num-iter."0.1.42" { inherit profileName; };
      num_rational = rustPackages."registry+https://github.com/rust-lang/crates.io-index".num-rational."0.4.0" { inherit profileName; };
      num_traits = rustPackages."registry+https://github.com/rust-lang/crates.io-index".num-traits."0.2.14" { inherit profileName; };
    };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".num-bigint."0.4.3" = overridableMkRustCrate (profileName: rec {
    name = "num-bigint";
    version = "0.4.3";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "f93ab6289c7b344a8a9f60f88d80aa20032336fe78da341afc91c8a2341fc75f"; };
    features = builtins.concatLists [
      [ "std" ]
    ];
    dependencies = {
      num_integer = rustPackages."registry+https://github.com/rust-lang/crates.io-index".num-integer."0.1.44" { inherit profileName; };
      num_traits = rustPackages."registry+https://github.com/rust-lang/crates.io-index".num-traits."0.2.14" { inherit profileName; };
    };
    buildDependencies = {
      autocfg = buildRustPackages."registry+https://github.com/rust-lang/crates.io-index".autocfg."1.1.0" { profileName = "__noProfile"; };
    };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".num-complex."0.4.0" = overridableMkRustCrate (profileName: rec {
    name = "num-complex";
    version = "0.4.0";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "26873667bbbb7c5182d4a37c1add32cdf09f841af72da53318fdb81543c15085"; };
    features = builtins.concatLists [
      [ "std" ]
    ];
    dependencies = {
      num_traits = rustPackages."registry+https://github.com/rust-lang/crates.io-index".num-traits."0.2.14" { inherit profileName; };
    };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".num-derive."0.3.3" = overridableMkRustCrate (profileName: rec {
    name = "num-derive";
    version = "0.3.3";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "876a53fff98e03a936a674b29568b0e605f06b29372c2489ff4de23f1949743d"; };
    dependencies = {
      proc_macro2 = rustPackages."registry+https://github.com/rust-lang/crates.io-index".proc-macro2."1.0.37" { inherit profileName; };
      quote = rustPackages."registry+https://github.com/rust-lang/crates.io-index".quote."1.0.17" { inherit profileName; };
      syn = rustPackages."registry+https://github.com/rust-lang/crates.io-index".syn."1.0.91" { inherit profileName; };
    };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".num-integer."0.1.44" = overridableMkRustCrate (profileName: rec {
    name = "num-integer";
    version = "0.1.44";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "d2cc698a63b549a70bc047073d2949cce27cd1c7b0a4a862d08a8031bc2801db"; };
    features = builtins.concatLists [
      [ "i128" ]
      [ "std" ]
    ];
    dependencies = {
      num_traits = rustPackages."registry+https://github.com/rust-lang/crates.io-index".num-traits."0.2.14" { inherit profileName; };
    };
    buildDependencies = {
      autocfg = buildRustPackages."registry+https://github.com/rust-lang/crates.io-index".autocfg."1.1.0" { profileName = "__noProfile"; };
    };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".num-iter."0.1.42" = overridableMkRustCrate (profileName: rec {
    name = "num-iter";
    version = "0.1.42";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "b2021c8337a54d21aca0d59a92577a029af9431cb59b909b03252b9c164fad59"; };
    features = builtins.concatLists [
      [ "i128" ]
      [ "std" ]
    ];
    dependencies = {
      num_integer = rustPackages."registry+https://github.com/rust-lang/crates.io-index".num-integer."0.1.44" { inherit profileName; };
      num_traits = rustPackages."registry+https://github.com/rust-lang/crates.io-index".num-traits."0.2.14" { inherit profileName; };
    };
    buildDependencies = {
      autocfg = buildRustPackages."registry+https://github.com/rust-lang/crates.io-index".autocfg."1.1.0" { profileName = "__noProfile"; };
    };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".num-rational."0.4.0" = overridableMkRustCrate (profileName: rec {
    name = "num-rational";
    version = "0.4.0";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "d41702bd167c2df5520b384281bc111a4b5efcf7fbc4c9c222c815b07e0a6a6a"; };
    features = builtins.concatLists [
      [ "num-bigint" ]
      [ "num-bigint-std" ]
      [ "std" ]
    ];
    dependencies = {
      num_bigint = rustPackages."registry+https://github.com/rust-lang/crates.io-index".num-bigint."0.4.3" { inherit profileName; };
      num_integer = rustPackages."registry+https://github.com/rust-lang/crates.io-index".num-integer."0.1.44" { inherit profileName; };
      num_traits = rustPackages."registry+https://github.com/rust-lang/crates.io-index".num-traits."0.2.14" { inherit profileName; };
    };
    buildDependencies = {
      autocfg = buildRustPackages."registry+https://github.com/rust-lang/crates.io-index".autocfg."1.1.0" { profileName = "__noProfile"; };
    };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".num-traits."0.2.14" = overridableMkRustCrate (profileName: rec {
    name = "num-traits";
    version = "0.2.14";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "9a64b1ec5cda2586e284722486d802acf1f7dbdc623e2bfc57e65ca1cd099290"; };
    features = builtins.concatLists [
      [ "default" ]
      [ "i128" ]
      [ "std" ]
    ];
    buildDependencies = {
      autocfg = buildRustPackages."registry+https://github.com/rust-lang/crates.io-index".autocfg."1.1.0" { profileName = "__noProfile"; };
    };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".proc-macro2."1.0.37" = overridableMkRustCrate (profileName: rec {
    name = "proc-macro2";
    version = "1.0.37";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "ec757218438d5fda206afc041538b2f6d889286160d649a86a24d37e1235afd1"; };
    features = builtins.concatLists [
      [ "default" ]
      [ "proc-macro" ]
    ];
    dependencies = {
      unicode_xid = rustPackages."registry+https://github.com/rust-lang/crates.io-index".unicode-xid."0.2.2" { inherit profileName; };
    };
  });
  
  "unknown".qoi-rs."0.1.0" = overridableMkRustCrate (profileName: rec {
    name = "qoi-rs";
    version = "0.1.0";
    registry = "unknown";
    src = fetchCrateLocal workspaceSrc;
    dependencies = {
      num = rustPackages."registry+https://github.com/rust-lang/crates.io-index".num."0.4.0" { inherit profileName; };
      num_derive = buildRustPackages."registry+https://github.com/rust-lang/crates.io-index".num-derive."0.3.3" { profileName = "__noProfile"; };
      num_traits = rustPackages."registry+https://github.com/rust-lang/crates.io-index".num-traits."0.2.14" { inherit profileName; };
    };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".quote."1.0.17" = overridableMkRustCrate (profileName: rec {
    name = "quote";
    version = "1.0.17";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "632d02bff7f874a36f33ea8bb416cd484b90cc66c1194b1a1110d067a7013f58"; };
    features = builtins.concatLists [
      [ "default" ]
      [ "proc-macro" ]
    ];
    dependencies = {
      proc_macro2 = rustPackages."registry+https://github.com/rust-lang/crates.io-index".proc-macro2."1.0.37" { inherit profileName; };
    };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".syn."1.0.91" = overridableMkRustCrate (profileName: rec {
    name = "syn";
    version = "1.0.91";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "b683b2b825c8eef438b77c36a06dc262294da3d5a5813fac20da149241dcd44d"; };
    features = builtins.concatLists [
      [ "clone-impls" ]
      [ "default" ]
      [ "derive" ]
      [ "parsing" ]
      [ "printing" ]
      [ "proc-macro" ]
      [ "quote" ]
    ];
    dependencies = {
      proc_macro2 = rustPackages."registry+https://github.com/rust-lang/crates.io-index".proc-macro2."1.0.37" { inherit profileName; };
      quote = rustPackages."registry+https://github.com/rust-lang/crates.io-index".quote."1.0.17" { inherit profileName; };
      unicode_xid = rustPackages."registry+https://github.com/rust-lang/crates.io-index".unicode-xid."0.2.2" { inherit profileName; };
    };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".unicode-xid."0.2.2" = overridableMkRustCrate (profileName: rec {
    name = "unicode-xid";
    version = "0.2.2";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "8ccb82d61f80a663efe1f787a51b16b5a51e3314d6ac365b08639f52387b33f3"; };
    features = builtins.concatLists [
      [ "default" ]
    ];
  });
  
}